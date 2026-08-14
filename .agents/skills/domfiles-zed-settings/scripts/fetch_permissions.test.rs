#[allow(dead_code)]
#[path = "fetch_permissions.rs"]
mod helper;

use serde_json::{Value, json};
use std::{
    env,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock must be after the Unix epoch")
            .as_nanos();
        let root = fs::canonicalize(env::temp_dir())
            .expect("Failed to resolve fixture directory")
            .join(format!(
                "domfiles-fetch-permissions-{}-{timestamp}-{fixture_id}",
                process::id()
            ));
        fs::create_dir(&root).expect("Failed to create fixture directory");
        Self { root }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn write(&self, relative: &str, bytes: &[u8]) -> PathBuf {
        let path = self.path(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create fixture file parent");
        }
        fs::write(&path, bytes).expect("Failed to write fixture file");
        path
    }

    fn write_json(&self, relative: &str, value: &Value) -> PathBuf {
        let mut bytes = serde_json::to_vec_pretty(value).expect("Fixture JSON must serialize");
        bytes.push(b'\n');
        self.write(relative, &bytes)
    }

    fn capture(&self, settings: &Value) -> Capture {
        let baseline = self.write_json("capture/baseline-settings.json", settings);
        let candidate = self.write_json("capture/candidate-settings.json", settings);
        let state = self.write("capture/state.json", b"opaque-state\n");
        Capture {
            baseline,
            candidate,
            state,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct Capture {
    baseline: PathBuf,
    candidate: PathBuf,
    state: PathBuf,
}

struct RunResult {
    status: u8,
    stderr: String,
    stdout: String,
}

fn run(arguments: Vec<OsString>) -> RunResult {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);
    RunResult {
        status,
        stderr: String::from_utf8(stderr).expect("Standard error must be UTF-8"),
        stdout: String::from_utf8(stdout).expect("Standard output must be UTF-8"),
    }
}

fn run_with_hook<F>(arguments: Vec<OsString>, hook: F) -> RunResult
where
    F: FnOnce(&Path) -> io::Result<()>,
{
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run_apply_with_hook(arguments, &mut stdout, &mut stderr, hook);
    RunResult {
        status,
        stderr: String::from_utf8(stderr).expect("Standard error must be UTF-8"),
        stdout: String::from_utf8(stdout).expect("Standard output must be UTF-8"),
    }
}

fn pattern(pattern: &str) -> Value {
    json!({
        "pattern": pattern,
        "case_sensitive": true
    })
}

fn settings(
    allow: Vec<Value>,
    confirm: Vec<Value>,
    deny: Vec<Value>,
    network_hosts: Vec<&str>,
) -> Value {
    json!({
        "agent": {
            "sandbox_permissions": {
                "network_hosts": network_hosts
            },
            "tool_permissions": {
                "default": "allow",
                "tools": {
                    "fetch": {
                        "default": "confirm",
                        "always_allow": allow,
                        "always_confirm": confirm,
                        "always_deny": deny
                    }
                }
            }
        }
    })
}

fn apply_arguments(
    capture: &Capture,
    output: &Path,
    scope: &str,
    input_option: &str,
    input: &str,
) -> Vec<OsString> {
    vec![
        OsString::from("apply"),
        OsString::from("--baseline"),
        capture.baseline.as_os_str().to_owned(),
        OsString::from("--candidate"),
        capture.candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        capture.state.as_os_str().to_owned(),
        OsString::from("--output"),
        output.as_os_str().to_owned(),
        OsString::from("--scope"),
        OsString::from(scope),
        OsString::from(input_option),
        OsString::from(input),
        OsString::from("--write"),
    ]
}

fn validate_arguments(capture: &Capture, bundle: &Path) -> Vec<OsString> {
    vec![
        OsString::from("validate"),
        OsString::from("--baseline"),
        capture.baseline.as_os_str().to_owned(),
        OsString::from("--candidate"),
        capture.candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        capture.state.as_os_str().to_owned(),
        OsString::from("--bundle"),
        bundle.as_os_str().to_owned(),
    ]
}

fn read_candidate(capture: &Capture) -> Value {
    serde_json::from_slice(&fs::read(&capture.candidate).expect("Candidate must be readable"))
        .expect("Candidate must remain JSON")
}

fn fetch_patterns(candidate: &Value) -> Vec<&str> {
    candidate["agent"]["tool_permissions"]["tools"]["fetch"]["always_allow"]
        .as_array()
        .expect("Fetch allow patterns must be an array")
        .iter()
        .map(|value| value["pattern"].as_str().expect("Pattern must be a string"))
        .collect()
}

fn network_hosts(candidate: &Value) -> Vec<&str> {
    candidate["agent"]["sandbox_permissions"]["network_hosts"]
        .as_array()
        .expect("Network hosts must be an array")
        .iter()
        .map(|value| value.as_str().expect("Network host must be a string"))
        .collect()
}

fn bundle_value(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("Bundle must be readable"))
        .expect("Bundle must be JSON")
}

#[test]
fn help_describes_the_persistent_all_port_boundary() {
    let result = run(vec![OsString::from("--help")]);
    assert_eq!(result.status, 0);
    assert!(result.stderr.is_empty());
    assert!(result.stdout.contains("Each grant covers every port"));
    assert!(result.stdout.contains("later sandboxed terminal processes"));
    assert!(
        result
            .stdout
            .contains("Terminal commands still require their independent terminal permission")
    );
    assert!(result.stdout.contains("No mode reads live settings"));
}

#[test]
fn unknown_arguments_are_rejected_without_echoing_their_values() {
    let marker = "https://domain.example/sensitive-marker";
    for arguments in [
        vec![OsString::from(marker)],
        vec![OsString::from("apply"), OsString::from(marker)],
        vec![OsString::from("validate"), OsString::from(marker)],
    ] {
        let result = run(arguments);
        assert_eq!(result.status, 2);
        assert!(result.stdout.is_empty());
        assert!(!result.stderr.contains(marker));
        assert!(result.stderr.len() < 160);
    }
}

#[test]
fn exact_hostname_adds_one_pattern_and_one_exact_host() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "Docs.Example",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stderr.is_empty());

    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec!["^(?i:https://docs\\.example)(?:[/?#]|$)"]
    );
    assert_eq!(network_hosts(&candidate), vec!["docs.example"]);

    let artifact =
        fs::read(output.join("fetch-pattern-01.regex")).expect("Pattern artifact must be readable");
    assert_eq!(artifact, b"^(?i:https://docs\\.example)(?:[/?#]|$)");
    assert!(!artifact.ends_with(b"\n"));

    let bundle = output.join("fetch-validation.json");
    let value = bundle_value(&bundle);
    assert_eq!(value["cases"].as_array().unwrap().len(), 10);
    let port_case = value["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == "explicit port")
        .expect("Exact-host corpus must include an explicit-port case");
    assert_eq!(port_case["candidate"]["final_decision"], "confirm");
    assert_eq!(port_case["candidate"]["always_allow"], false);

    let validation = run(validate_arguments(&capture, &bundle));
    assert_eq!(validation.status, 0, "{}", validation.stderr);
}

#[test]
fn subdomains_only_adds_one_wildcard_host() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "subdomains-only",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec!["^(?i:https://(?:[^./?#:@]+\\.)+domain\\.example)(?:[/?#]|$)"]
    );
    assert_eq!(network_hosts(&candidate), vec!["*.domain.example"]);
    assert_eq!(
        bundle_value(&output.join("fetch-validation.json"))["cases"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
}

#[test]
fn exact_hostname_plus_subdomains_adds_both_host_entries() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname-plus-subdomains",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec!["^(?i:https://(?:[^./?#:@]+\\.)*domain\\.example)(?:[/?#]|$)"]
    );
    assert_eq!(
        network_hosts(&candidate),
        vec!["*.domain.example", "domain.example"]
    );
}

#[test]
fn path_qualified_url_adds_no_sandbox_host() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "path-qualified-url",
        "--url-prefix",
        "HTTPS://Docs.Example/Reference/API/",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec!["^(?i:https://docs\\.example)/Reference/API/"]
    );
    assert!(network_hosts(&candidate).is_empty());

    let cases = bundle_value(&output.join("fetch-validation.json"));
    let cases = cases["cases"].as_array().unwrap();
    let case_variant = cases
        .iter()
        .find(|case| case["name"] == "path case variant")
        .expect("Path corpus must include a case variant");
    assert_eq!(case_variant["candidate"]["final_decision"], "confirm");
}

#[test]
fn insertion_preserves_scope_groups_and_hostname_order() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![
            pattern("^(?i:https://alpha-only\\.example)(?:[/?#]|$)"),
            pattern("^(?i:https://zeta\\.example)(?:[/?#]|$)"),
            pattern("^(?i:https://(?:[^./?#:@]+\\.)+subdomain\\.example)(?:[/?#]|$)"),
            pattern("^(?i:https://(?:[^./?#:@]+\\.)*alpha\\.example)(?:[/?#]|$)"),
        ],
        Vec::new(),
        Vec::new(),
        vec![
            "*.alpha.example",
            "*.subdomain.example",
            "alpha-only.example",
            "alpha.example",
            "zeta.example",
        ],
    ));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "middle.example",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec![
            "^(?i:https://alpha-only\\.example)(?:[/?#]|$)",
            "^(?i:https://middle\\.example)(?:[/?#]|$)",
            "^(?i:https://zeta\\.example)(?:[/?#]|$)",
            "^(?i:https://(?:[^./?#:@]+\\.)+subdomain\\.example)(?:[/?#]|$)",
            "^(?i:https://(?:[^./?#:@]+\\.)*alpha\\.example)(?:[/?#]|$)",
        ]
    );
    assert_eq!(
        network_hosts(&candidate),
        vec![
            "*.alpha.example",
            "*.subdomain.example",
            "alpha-only.example",
            "alpha.example",
            "middle.example",
            "zeta.example",
        ]
    );
}

#[test]
fn nested_hostname_insertion_uses_each_patterns_own_hostname() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![pattern("^(?i:https://b\\.example)(?:[/?#]|$)")],
        Vec::new(),
        Vec::new(),
        vec!["b.example"],
    ));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "a.b.example",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec![
            "^(?i:https://a\\.b\\.example)(?:[/?#]|$)",
            "^(?i:https://b\\.example)(?:[/?#]|$)",
        ]
    );
    assert_eq!(network_hosts(&candidate), vec!["a.b.example", "b.example"]);
}

#[test]
fn nested_parent_before_child_is_detected_as_misordered() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![
            pattern("^(?i:https://(?:[^./?#:@]+\\.)*b\\.example)(?:[/?#]|$)"),
            pattern("^(?i:https://(?:[^./?#:@]+\\.)*a\\.b\\.example)(?:[/?#]|$)"),
        ],
        Vec::new(),
        Vec::new(),
        vec!["*.a.b.example", "*.b.example", "a.b.example", "b.example"],
    ));
    let result = run(apply_arguments(
        &capture,
        &fixture.path("artifacts"),
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("represented hostname"));
}

#[test]
fn equivalent_existing_allowance_is_reused() {
    let fixture = Fixture::new();
    let settings = settings(
        vec![pattern("^(?i:https://domain\\.example)(?:[/?#]|$)")],
        Vec::new(),
        Vec::new(),
        vec!["domain.example"],
    );
    let capture = fixture.capture(&settings);
    let baseline_bytes = fs::read(&capture.baseline).unwrap();
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 1);
    assert!(result.stderr.contains("already covered"));
    assert!(!output.exists());
    assert_eq!(fs::read(&capture.candidate).unwrap(), baseline_bytes);
}

#[test]
fn noncanonical_path_patterns_use_the_fallback() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![pattern(
            "^(?i:https://domain\\.example)(?:$|/path$|\\?query=value$|#fragment$)",
        )],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("outside the fast path"));
    assert!(!output.exists());
}

#[test]
fn invalid_fetch_regexes_are_rejected_without_echoing_their_bodies() {
    let fixture = Fixture::new();
    let marker = "sensitive-regex-marker(";
    let capture = fixture.capture(&settings(
        vec![pattern(marker)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    let baseline_bytes = fs::read(&capture.baseline).unwrap();
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(!result.stderr.contains(marker));
    assert!(result.stderr.len() < 160);
    assert!(!output.exists());
    assert_eq!(fs::read(&capture.candidate).unwrap(), baseline_bytes);
}

#[test]
fn duplicate_patterns_are_rejected() {
    let fixture = Fixture::new();
    let duplicate = pattern("^(?i:https://domain\\.example)(?:[/?#]|$)");
    let capture = fixture.capture(&settings(
        vec![duplicate.clone(), duplicate],
        Vec::new(),
        Vec::new(),
        vec!["domain.example"],
    ));
    let result = run(apply_arguments(
        &capture,
        &fixture.path("artifacts"),
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("duplicate pattern object"));
}

#[test]
fn duplicate_sandbox_hosts_are_rejected() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![pattern("^(?i:https://domain\\.example)(?:[/?#]|$)")],
        Vec::new(),
        Vec::new(),
        vec!["domain.example", "domain.example"],
    ));
    let result = run(apply_arguments(
        &capture,
        &fixture.path("artifacts"),
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("duplicate entry"));
}

#[test]
fn factored_patterns_account_for_every_represented_hostname() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![pattern(
            "^(?i:https://(?:allowed|ungranted)\\.example)(?:[/?#]|$)",
        )],
        Vec::new(),
        Vec::new(),
        vec!["allowed.example"],
    ));
    let baseline_bytes = fs::read(&capture.baseline).unwrap();
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("misaligned"));
    assert!(!output.exists());
    assert_eq!(fs::read(&capture.candidate).unwrap(), baseline_bytes);
}

#[test]
fn fetch_and_sandbox_scope_mismatches_are_rejected() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![pattern("^(?i:https://domain\\.example)(?:[/?#]|$)")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    let result = run(apply_arguments(
        &capture,
        &fixture.path("artifacts"),
        "exact-hostname",
        "--hostname",
        "other.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("misaligned"));
}

#[test]
fn misordered_represented_hostnames_are_rejected() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        vec![
            pattern("^(?i:https://zeta\\.example)(?:[/?#]|$)"),
            pattern("^(?i:https://alpha\\.example)(?:[/?#]|$)"),
        ],
        Vec::new(),
        Vec::new(),
        vec!["alpha.example", "zeta.example"],
    ));
    let result = run(apply_arguments(
        &capture,
        &fixture.path("artifacts"),
        "exact-hostname",
        "--hostname",
        "middle.example",
    ));
    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("represented hostname"));
}

#[test]
fn confirm_precedence_refuses_an_ineffective_allowance() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(
        Vec::new(),
        vec![pattern("^https://blocked\\.example")],
        Vec::new(),
        Vec::new(),
    ));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "blocked.example",
    ));
    assert_eq!(result.status, 1);
    assert!(result.stderr.contains("confirm precedence"));
    assert!(!output.exists());
}

#[test]
fn validation_rejects_malformed_and_unsafe_bundle_requests() {
    for url_prefix in ["sensitive-url-marker", "https://domain.example/%2F/"] {
        let fixture = Fixture::new();
        let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        let output = fixture.path("artifacts");
        let apply = run(apply_arguments(
            &capture,
            &output,
            "exact-hostname",
            "--hostname",
            "domain.example",
        ));
        assert_eq!(apply.status, 0, "{}", apply.stderr);
        let candidate_bytes = fs::read(&capture.candidate).unwrap();
        let bundle = output.join("fetch-validation.json");
        let mut value = bundle_value(&bundle);
        value["request"] = json!({
            "scope": "path_qualified_url",
            "url_prefix": url_prefix
        });
        fixture.write_json("artifacts/fetch-validation.json", &value);

        let validation = run(validate_arguments(&capture, &bundle));
        assert_eq!(validation.status, 2);
        assert!(!validation.stderr.contains(url_prefix));
        assert!(validation.stderr.len() < 200);
        assert_eq!(fs::read(&capture.candidate).unwrap(), candidate_bytes);
    }
}

#[test]
fn validation_rejects_trailing_artifact_bytes() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let apply = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(apply.status, 0, "{}", apply.stderr);
    let artifact = output.join("fetch-pattern-01.regex");
    let mut bytes = fs::read(&artifact).unwrap();
    bytes.push(b'\n');
    fs::write(&artifact, bytes).unwrap();

    let validation = run(validate_arguments(
        &capture,
        &output.join("fetch-validation.json"),
    ));
    assert_eq!(validation.status, 2);
    assert!(
        validation
            .stderr
            .contains("outside its exact candidate pattern binding")
    );
}

#[test]
fn validation_rejects_state_drift() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let apply = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(apply.status, 0, "{}", apply.stderr);
    fs::write(&capture.state, b"changed-state\n").unwrap();

    let validation = run(validate_arguments(
        &capture,
        &output.join("fetch-validation.json"),
    ));
    assert_eq!(validation.status, 1);
    assert!(validation.stderr.contains("does not bind"));
}

#[test]
fn capture_file_aliases_are_rejected() {
    let fixture = Fixture::new();
    let original = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let capture = Capture {
        baseline: original.baseline.clone(),
        candidate: original.baseline.clone(),
        state: original.state.clone(),
    };
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 1);
    assert!(result.stderr.contains("three distinct regular files"));
    assert!(!output.exists());
}

#[test]
fn concurrent_candidate_changes_roll_back_output() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let candidate = capture.candidate.clone();
    let result = run_with_hook(
        apply_arguments(
            &capture,
            &output,
            "exact-hostname",
            "--hostname",
            "domain.example",
        ),
        move |_| fs::write(&candidate, b"{}\n"),
    );
    assert_eq!(result.status, 1);
    assert!(result.stderr.contains("changed concurrently"));
    assert!(!output.exists());
}

#[test]
fn existing_output_paths_are_refused_without_candidate_mutation() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let baseline_bytes = fs::read(&capture.baseline).unwrap();
    let output = fixture.path("artifacts");
    fs::create_dir(&output).unwrap();
    let result = run(apply_arguments(
        &capture,
        &output,
        "exact-hostname",
        "--hostname",
        "domain.example",
    ));
    assert_eq!(result.status, 1);
    assert!(result.stderr.contains("already exists"));
    assert_eq!(fs::read(&capture.candidate).unwrap(), baseline_bytes);
}

#[test]
fn path_fast_path_rejects_unsupported_url_grammars() {
    for input in [
        "http://domain.example/path/",
        "https://domain.example:8443/path/",
        "https://user@domain.example/path/",
        "https://domain.example/path/?query=value",
        "https://domain.example/path/#fragment",
        "https://domain.example/path",
        "https://domain.example/a b/",
        "https://domain.example/%zz/",
        "https://domain.example/%2f/",
        "https://domain.example/\"quoted\"/",
        "https://domain.example/\0/",
    ] {
        let fixture = Fixture::new();
        let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        let result = run(apply_arguments(
            &capture,
            &fixture.path("artifacts"),
            "path-qualified-url",
            "--url-prefix",
            input,
        ));
        assert_eq!(result.status, 2, "input unexpectedly accepted: {input}");
        assert!(!fixture.path("artifacts").exists());
    }
}

#[test]
fn numeric_path_prefixes_use_the_bounded_corpus_without_a_case_variant() {
    let fixture = Fixture::new();
    let capture = fixture.capture(&settings(Vec::new(), Vec::new(), Vec::new(), Vec::new()));
    let output = fixture.path("artifacts");
    let result = run(apply_arguments(
        &capture,
        &output,
        "path-qualified-url",
        "--url-prefix",
        "https://domain.example/123/",
    ));
    assert_eq!(result.status, 0, "{}", result.stderr);
    let candidate = read_candidate(&capture);
    assert_eq!(
        fetch_patterns(&candidate),
        vec!["^(?i:https://domain\\.example)/123/"]
    );
    let cases = bundle_value(&output.join("fetch-validation.json"));
    assert_eq!(cases["cases"].as_array().unwrap().len(), 9);
    assert!(
        cases["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["name"] != "path case variant")
    );
}
