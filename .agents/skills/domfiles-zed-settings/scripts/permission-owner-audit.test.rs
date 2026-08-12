#[path = "permission-owner-audit.rs"]
mod helper;

use serde_json::{Value, json};
use std::{
    env,
    ffi::OsString,
    fs,
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
        let root = env::temp_dir().join(format!(
            "domfiles-permission-owner-audit-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");

        Self { root }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.path(name);
        fs::write(&path, contents).expect("Failed to write fixture file");
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn pattern(pattern: &str, case_sensitive: bool) -> Value {
    json!({
        "pattern": pattern,
        "case_sensitive": case_sensitive,
    })
}

fn settings(
    always_allow: Vec<Value>,
    always_confirm: Vec<Value>,
    always_deny: Vec<Value>,
) -> String {
    json!({
        "agent": {
            "tool_permissions": {
                "tools": {
                    "terminal": {
                        "always_allow": always_allow,
                        "always_confirm": always_confirm,
                        "always_deny": always_deny,
                    }
                }
            }
        }
    })
    .to_string()
}

#[allow(clippy::too_many_arguments)]
fn entry(
    id: &str,
    bucket: &str,
    index: usize,
    owner: &str,
    owner_sort_key: &str,
    section_sort_key: &str,
    role: &str,
    pattern_sort_key: &str,
    witness: &str,
    discovery_inputs: Option<&[&str]>,
) -> Value {
    let mut entry = json!({
        "id": id,
        "bucket": bucket,
        "index": index,
        "owner": owner,
        "owner_sort_key": owner_sort_key,
        "section_sort_key": section_sort_key,
        "role": role,
        "pattern_sort_key": pattern_sort_key,
        "witness": witness,
    });
    if let Some(inputs) = discovery_inputs {
        let object = entry.as_object_mut().expect("Entry must be an object");
        object.insert("discovery_coverage".to_owned(), json!("complete_finite"));
        object.insert("discovery_inputs".to_owned(), json!(inputs));
    }
    entry
}

fn direct_entry(id: &str, bucket: &str, index: usize, owner: &str, witness: &str) -> Value {
    entry(
        id, bucket, index, owner, owner, "direct", "direct", id, witness, None,
    )
}

fn with_case_insensitive_reason(mut entry: Value, reason: &str) -> Value {
    entry
        .as_object_mut()
        .expect("Entry must be an object")
        .insert("case_insensitive_reason".to_owned(), json!(reason));
    entry
}

fn manifest(entries: Vec<Value>) -> String {
    json!({
        "version": 1,
        "entries": entries,
    })
    .to_string()
}

fn valid_settings_and_manifest() -> (String, String) {
    (
        settings(vec![pattern(r"^foo run$", true)], vec![], vec![]),
        manifest(vec![direct_entry(
            "foo-direct",
            "always_allow",
            0,
            "foo",
            "foo run",
        )]),
    )
}

fn run(arguments: Vec<OsString>) -> (u8, String, String) {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

fn run_files(settings_path: &Path, manifest_path: &Path) -> (u8, String, String) {
    run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
    ])
}

fn run_inventory(settings_path: &Path, owner: &str) -> (u8, String, String) {
    run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from(owner),
    ])
}

fn inventory_preview(stdout: &str, id: &str) -> String {
    let prefix = format!("{id} ");
    let line = stdout
        .lines()
        .find(|line| line.starts_with(&prefix))
        .expect("Inventory hit must be present");
    let (_, preview) = line
        .split_once(" preview=")
        .expect("Inventory hit must contain a preview");
    serde_json::from_str(preview).expect("Inventory preview must be a JSON string")
}

fn write_valid_files(fixture: &Fixture) -> (PathBuf, PathBuf) {
    let (settings, manifest) = valid_settings_and_manifest();
    (
        fixture.write("settings.json", &settings),
        fixture.write("manifest.json", &manifest),
    )
}

fn finding_reasons(settings: &str, manifest: &str) -> Vec<String> {
    helper::audit_json(settings, manifest)
        .expect("Audit input must be structurally valid")
        .findings
        .into_iter()
        .map(|finding| finding.reason)
        .collect()
}

#[test]
fn prints_help_only_for_exact_help() {
    let (status, stdout, stderr) = run(vec![OsString::from("--help")]);

    assert_eq!(status, 0);
    assert!(stdout.starts_with("Usage:\n  permission-owner-audit"));
    assert!(stdout.contains(
        "permission-owner-audit --settings <settings-path> --owner <top-level-executable>"
    ));
    assert!(stdout.contains("Version-1 manifest schema"));
    assert!(stdout.contains("unknown fields are rejected"));
    assert!(stdout.contains("case_insensitive_reason"));
    assert!(stdout.contains("discovery_coverage"));
    assert!(stdout.contains("complete_finite|representative"));
    assert!(stdout.contains("discovery_inputs"));
    assert!(stdout.contains("[A-Za-z0-9_.+-]+"));
    assert!(stdout.contains("Matches are inventory candidates"));
    assert!(stdout.contains("not semantic ownership proof"));
    assert!(stdout.contains("Each selected decoded pattern must contain at most 999"));
    assert!(stdout.contains("independently inferred bucket, semantic owner, and Git repository"));
    assert!(stdout.contains("exact top-level agent worktree"));
    assert!(stdout.contains("`section_sort_key` participates in ordering"));
    assert!(stdout.contains("(`owner_sort_key`, `section_sort_key`, role order"));
    assert!(stdout.contains("written to standard output"));
    assert!(stdout.contains("written to standard error"));
    assert!(stdout.contains("Exit statuses:"));
    assert!(stderr.is_empty());

    let (status, stdout, stderr) = run(vec![
        OsString::from("--help"),
        OsString::from("--settings"),
        OsString::from("settings.json"),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("must be used alone"));
}

#[test]
fn rejects_missing_unknown_and_duplicate_arguments() {
    let cases = [
        (vec![], "--settings"),
        (vec!["--unknown"], "Unknown option"),
        (
            vec!["--settings", "first.json", "--settings", "second.json"],
            "may be specified only once",
        ),
        (
            vec!["--manifest", "first.json", "--manifest", "second.json"],
            "may be specified only once",
        ),
        (
            vec!["--owner", "git", "--owner", "npm"],
            "may be specified only once",
        ),
        (vec!["--settings"], "requires a path"),
        (
            vec!["--settings", "settings.json", "--manifest"],
            "requires a path",
        ),
    ];

    for (arguments, expected) in cases {
        let (status, stdout, stderr) = run(arguments.into_iter().map(OsString::from).collect());
        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains(expected));
    }
}

#[test]
fn rejects_malformed_and_unknown_field_manifests() {
    let (settings, _) = valid_settings_and_manifest();
    let malformed = "{not-json";
    let unknown_root = json!({"version": 1, "entries": [], "extra": true}).to_string();
    let mut unknown_entry = direct_entry("foo", "always_allow", 0, "foo", "foo run");
    unknown_entry
        .as_object_mut()
        .expect("Entry must be an object")
        .insert("extra".to_owned(), json!(true));
    let unknown_entry = manifest(vec![unknown_entry]);

    for manifest in [malformed.to_owned(), unknown_root, unknown_entry] {
        let error = helper::audit_json(&settings, &manifest)
            .expect_err("Malformed manifest must be rejected");
        assert!(error.contains("Invalid manifest JSON"));
    }
}

#[test]
fn rejects_missing_settings_file_and_settings_path() {
    let fixture = Fixture::new();
    let (_, manifest) = valid_settings_and_manifest();
    let manifest_path = fixture.write("manifest.json", &manifest);

    let (status, stdout, stderr) = run_files(&fixture.path("missing.json"), &manifest_path);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to read settings file"));

    let error = helper::audit_json("{}", &manifest)
        .expect_err("Missing terminal settings path must be rejected");
    assert!(error.contains(".agent.tool_permissions.tools.terminal"));
}

#[test]
fn reports_run_success_with_counts_only() {
    let fixture = Fixture::new();
    let (settings_path, manifest_path) = write_valid_files(&fixture);

    let (status, stdout, stderr) = run_files(&settings_path, &manifest_path);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Audited 1 entry across 1 owner group and 1 bucket\n"
    );
    assert!(stderr.is_empty());
    assert!(!stdout.contains("foo"));
}

#[test]
fn inventories_lexical_owner_tokens_across_all_buckets() {
    let fixture = Fixture::new();
    let settings_path = fixture.write(
        "settings.json",
        &settings(
            vec![
                pattern(r"^(?:git)(?: |$)", true),
                pattern(r"^github status$", true),
                pattern(r"^git-lfs$", true),
                pattern(r"^git_like$", true),
            ],
            vec![pattern(r"^xargs git$", false), pattern(r"^legit$", true)],
            vec![pattern("git", true)],
        ),
    );

    let (status, stdout, stderr) = run_inventory(&settings_path, "git");

    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("Inventory results are candidates, not semantic ownership proof"));
    assert!(stdout.contains("always_allow[0] characters=15 case_sensitive=true"));
    assert!(stdout.contains("always_confirm[0] characters=11 case_sensitive=false"));
    assert!(stdout.contains("always_deny[0] characters=3 case_sensitive=true"));
    assert!(!stdout.contains("always_allow[1]"));
    assert!(!stdout.contains("always_allow[2]"));
    assert!(!stdout.contains("always_allow[3]"));
    assert!(!stdout.contains("always_confirm[1]"));
    assert!(stdout.contains("Total inventory candidates: 3"));
}

#[test]
fn excludes_manager_substrings_and_escapes_literal_owner_tokens() {
    let fixture = Fixture::new();
    let settings_path = fixture.write(
        "settings.json",
        &settings(
            vec![
                pattern(r"^pnpm install$", true),
                pattern(r"^(?:npm)$", true),
                pattern(r"^corepack npm --version$", true),
                pattern(r"^npm-cli$", true),
                pattern(r"^(?:foo.bar)$", true),
                pattern(r"^(?:fooxbar)$", true),
                pattern(r"^(?:git+)$", true),
                pattern(r"^(?:gitt)$", true),
            ],
            vec![],
            vec![],
        ),
    );

    let (status, stdout, stderr) = run_inventory(&settings_path, "npm");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(!stdout.contains("always_allow[0]"));
    assert!(stdout.contains("always_allow[1]"));
    assert!(stdout.contains("always_allow[2]"));
    assert!(!stdout.contains("always_allow[3]"));
    assert!(stdout.contains("Total inventory candidates: 2"));

    let (status, stdout, stderr) = run_inventory(&settings_path, "foo.bar");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("always_allow[4]"));
    assert!(!stdout.contains("always_allow[5]"));
    assert!(stdout.contains("Total inventory candidates: 1"));

    let (status, stdout, stderr) = run_inventory(&settings_path, "git+");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("always_allow[6]"));
    assert!(!stdout.contains("always_allow[7]"));
    assert!(stdout.contains("Total inventory candidates: 1"));
}

#[test]
fn reports_unicode_length_and_caps_preview_without_full_pattern_leakage() {
    let fixture = Fixture::new();
    let private_prefix = "PRIVATE_PREFIX_秘密";
    let private_tail = "PRIVATE_PATTERN_TAIL";
    let suffix = "界".repeat(200);
    let source = format!(r"^{private_prefix}(?:git{suffix}){private_tail}$");
    assert_ne!(source.len(), source.chars().count());
    let settings_path = fixture.write(
        "settings.json",
        &settings(vec![pattern(&source, true)], vec![], vec![]),
    );

    let (status, stdout, stderr) = run_inventory(&settings_path, "git");

    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains(&format!("characters={}", source.chars().count())));
    let preview = inventory_preview(&stdout, "always_allow[0]");
    assert!(preview.starts_with("git"));
    assert_eq!(preview.chars().count(), 160);
    assert!(!stdout.contains(private_prefix));
    assert!(!stdout.contains(private_tail));
    assert!(!stdout.contains(&source));
}

#[test]
fn succeeds_with_zero_inventory_hits() {
    let fixture = Fixture::new();
    let settings_path = fixture.write(
        "settings.json",
        &settings(vec![pattern(r"^npm --version$", true)], vec![], vec![]),
    );

    let (status, stdout, stderr) = run_inventory(&settings_path, "git");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Inventory results are candidates, not semantic ownership proof\nTotal inventory candidates: 0\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn bounds_inventory_at_one_hundred_hits_with_exact_total() {
    let fixture = Fixture::new();
    let mut always_allow = Vec::new();
    let mut always_confirm = Vec::new();
    let mut always_deny = Vec::new();

    for index in 0..34 {
        always_allow.push(pattern(
            &format!(r"^(?:git allow-{index:02} private)$"),
            true,
        ));
        always_confirm.push(pattern(
            &format!(r"^(?:git confirm-{index:02} private)$"),
            true,
        ));
    }
    for index in 0..35 {
        always_deny.push(pattern(
            &format!(r"^(?:git deny-{index:02} private)$"),
            true,
        ));
    }

    let first_pattern = always_allow[0]
        .get("pattern")
        .and_then(Value::as_str)
        .expect("Fixture pattern must be a string")
        .to_owned();
    let settings_path = fixture.write(
        "settings.json",
        &settings(always_allow, always_confirm, always_deny),
    );
    let (status, stdout, stderr) = run_inventory(&settings_path, "git");

    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert_eq!(
        stdout
            .lines()
            .filter(|line| line.starts_with("always_"))
            .count(),
        100
    );
    assert!(stdout.contains("always_deny[31]"));
    assert!(!stdout.contains("always_deny[32]"));
    assert!(stdout.contains("… 3 additional inventory candidates omitted"));
    assert!(stdout.contains("Total inventory candidates: 103"));
    assert!(!stdout.contains(&first_pattern));
}

#[test]
fn rejects_invalid_inventory_settings_without_pattern_leakage() {
    let fixture = Fixture::new();
    let private_pattern = "private-pattern-without-owner";
    let cases = [
        ("{not-json".to_owned(), "Invalid settings JSON"),
        ("{}".to_owned(), ".agent.tool_permissions.tools.terminal"),
        (
            json!({
                "agent": {
                    "tool_permissions": {
                        "tools": {
                            "terminal": {
                                "always_allow": {},
                                "always_confirm": [],
                                "always_deny": [],
                            }
                        }
                    }
                }
            })
            .to_string(),
            "bucket `always_allow` must be an array",
        ),
        (
            settings(vec![json!(private_pattern)], vec![], vec![]),
            "entry `always_allow[0]` must be an object",
        ),
        (
            settings(vec![json!({"case_sensitive": true})], vec![], vec![]),
            "must contain string `pattern`",
        ),
        (
            settings(vec![json!({"pattern": private_pattern})], vec![], vec![]),
            "must contain boolean `case_sensitive`",
        ),
        (
            settings(
                vec![json!({"pattern": private_pattern, "case_sensitive": "true"})],
                vec![],
                vec![],
            ),
            "must contain boolean `case_sensitive`",
        ),
    ];

    for (index, (contents, expected)) in cases.into_iter().enumerate() {
        let settings_path = fixture.write(&format!("settings-{index}.json"), &contents);
        let (status, stdout, stderr) = run_inventory(&settings_path, "git");
        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains(expected), "Unexpected error: {stderr}");
        assert!(!stderr.contains(private_pattern));
    }
}

#[test]
fn rejects_invalid_owner_tokens_and_mutually_exclusive_modes() {
    let fixture = Fixture::new();
    let (settings_path, manifest_path) = write_valid_files(&fixture);

    for owner in ["", "git status", "git/", "git:status", "💥"] {
        let (status, stdout, stderr) = run_inventory(&settings_path, owner);
        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains("[A-Za-z0-9_.+-]+"));
    }

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("foo"),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("mutually exclusive"));
}

#[test]
fn infers_optional_corepack_manager_ownership() {
    for (witness, owner, role) in [
        ("npm --version", "npm", helper::Role::Discovery),
        ("corepack npm --version", "npm", helper::Role::Discovery),
        ("corepack pnpm install", "pnpm", helper::Role::Direct),
        ("corepack yarn run test", "yarn", helper::Role::Direct),
        (
            "MODE=check nohup corepack pnpm --version",
            "pnpm",
            helper::Role::Wrapped,
        ),
    ] {
        let discovery_inputs = if witness.ends_with("--version") {
            vec![witness.to_owned()]
        } else {
            Vec::new()
        };
        let inferred = helper::infer_owner_role(witness, &discovery_inputs)
            .expect("Manager witness must be supported");
        assert_eq!(inferred.owner, owner);
        assert_eq!(inferred.role, role);
    }
}

#[test]
fn audits_nohup_forms_as_wrapped() {
    let settings = settings(
        vec![
            pattern(r"^foo run$", true),
            pattern(r"^nohup foo run$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        direct_entry("direct", "always_allow", 0, "foo", "foo run"),
        entry(
            "wrapped",
            "always_allow",
            1,
            "foo",
            "foo",
            "direct",
            "wrapped",
            "a",
            "nohup foo run",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn retains_corepack_for_its_own_forms() {
    for witness in [
        "corepack --help",
        "corepack enable",
        "corepack npm@latest --help",
        "nohup corepack --version",
    ] {
        let discovery_inputs = if witness.ends_with("--help") || witness.ends_with("--version") {
            vec![witness.to_owned()]
        } else {
            Vec::new()
        };
        let inferred = helper::infer_owner_role(witness, &discovery_inputs)
            .expect("Corepack witness must be supported");
        assert_eq!(inferred.owner, "corepack");
    }
}

#[test]
fn infers_xargs_child_ownership_for_documented_options() {
    let cases = [
        ("xargs -0rtx basename", "basename"),
        ("xargs -L2 cat", "cat"),
        ("xargs -L 2 cksum", "cksum"),
        ("xargs -n3 cmp", "cmp"),
        ("xargs -n 3 col", "col"),
        ("xargs --exit column", "column"),
        ("xargs --max-args=4 comm", "comm"),
        ("xargs --no-run-if-empty cut", "cut"),
        ("xargs --null dirname", "dirname"),
        (
            "xargs --verbose git --no-pager hash-object --stdin",
            "git:hash-object",
        ),
    ];

    for (witness, owner) in cases {
        let inferred = helper::infer_owner_role(witness, &[])
            .expect("Documented `xargs` wrapper must be supported");
        assert_eq!(inferred.owner, owner);
        assert_eq!(inferred.role, helper::Role::Wrapped);
    }
}

#[test]
fn rejects_unsupported_xargs_options_and_values() {
    for witness in [
        "xargs -P2 cat",
        "xargs -L0 cat",
        "xargs -L 0 cat",
        "xargs --max-args=0 cat",
        "xargs --replace cat",
        "xargs -n",
    ] {
        assert!(helper::infer_owner_role(witness, &[]).is_err());
    }
}

#[test]
fn infers_git_root_and_direct_owners() {
    let cases = [
        ("git", "git:root", helper::Role::Direct),
        ("git --version", "git:root", helper::Role::Direct),
        (
            "git -C repo --no-optional-locks --no-pager",
            "git:root",
            helper::Role::Direct,
        ),
        (
            "git hash-object --stdin",
            "git:hash-object",
            helper::Role::Direct,
        ),
        (
            "GIT_OPTIONAL_LOCKS=0 nohup git -C repo --no-pager status --short",
            "git:status",
            helper::Role::Wrapped,
        ),
    ];

    for (witness, owner, role) in cases {
        let inferred =
            helper::infer_owner_role(witness, &[]).expect("Git witness must be supported");
        assert_eq!(inferred.owner, owner);
        assert_eq!(inferred.role, role);
    }

    let discovery = vec!["git --version".to_owned()];
    let inferred = helper::infer_owner_role("git --version", &discovery)
        .expect("Git discovery witness must be supported");
    assert_eq!(inferred.owner, "git:root");
    assert_eq!(inferred.role, helper::Role::Discovery);
}

#[test]
fn rejects_ambiguous_or_unnormalized_witnesses() {
    for witness in [
        "",
        " foo run",
        "foo  run",
        "foo run ",
        "foo\trun",
        "NAME=value",
        "nohup",
        "git -C",
    ] {
        assert!(helper::infer_owner_role(witness, &[]).is_err());
    }
}

#[test]
fn reports_wrong_owner_and_wrong_role() {
    let settings = settings(
        vec![
            pattern(r"^corepack npm --version$", true),
            pattern(r"^foo run$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "wrong-owner",
            "always_allow",
            0,
            "corepack",
            "a",
            "a",
            "discovery",
            "a",
            "corepack npm --version",
            Some(&["corepack npm --version"]),
        ),
        entry(
            "wrong-role",
            "always_allow",
            1,
            "foo",
            "b",
            "a",
            "wrapped",
            "a",
            "foo run",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.iter().any(|finding| {
        finding.id == "wrong-owner" && finding.reason.contains("declared owner differs")
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.id == "wrong-role" && finding.reason.contains("declared role `wrapped`")
    }));
}

#[test]
fn accepts_verified_case_insensitive_exception() {
    let settings = settings(vec![pattern(r"^foo run$", false)], vec![], vec![]);
    let manifest = manifest(vec![with_case_insensitive_reason(
        direct_entry("case", "always_allow", 0, "foo", "foo run"),
        "The command requires case-insensitive matching",
    )]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn reports_case_reason_on_case_sensitive_pattern() {
    let settings = settings(vec![pattern(r"^foo run$", true)], vec![], vec![]);
    let manifest = manifest(vec![with_case_insensitive_reason(
        direct_entry("case", "always_allow", 0, "foo", "foo run"),
        "The command requires case-insensitive matching",
    )]);

    let reasons = finding_reasons(&settings, &manifest);

    assert!(reasons.iter().any(|reason| {
        reason.contains("case-sensitive pattern must omit `case_insensitive_reason`")
    }));
}

#[test]
fn reports_witness_mismatch_case_setting_and_invalid_regex() {
    let private_pattern = "private-regex-body(";
    let private_witness = "private-witness-input";
    let settings = settings(
        vec![
            pattern(r"^foo$", true),
            pattern(r"^bar run$", false),
            pattern(private_pattern, true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        direct_entry("witness", "always_allow", 0, "foo", "foo run"),
        direct_entry("case", "always_allow", 1, "bar", "bar run"),
        direct_entry(
            "invalid",
            "always_allow",
            2,
            private_witness,
            private_witness,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.iter().any(|finding| {
        finding.id == "witness" && finding.reason.contains("does not match its witness")
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.id == "case"
            && finding
                .reason
                .contains("`case_sensitive` is `false` without `case_insensitive_reason`")
    }));
    let invalid = report
        .findings
        .iter()
        .find(|finding| finding.id == "invalid")
        .expect("Invalid regex finding must exist");
    assert!(invalid.reason.contains("regex is invalid"));
    assert!(!invalid.reason.contains(private_pattern));
    assert!(!invalid.reason.contains(private_witness));
}

#[test]
fn measures_decoded_unicode_scalar_length_at_boundary() {
    let accepted_witness = "💥".repeat(997);
    let accepted_pattern = format!("^{accepted_witness}$");
    assert_eq!(accepted_pattern.chars().count(), 999);
    let accepted_settings = settings(vec![pattern(&accepted_pattern, true)], vec![], vec![]);
    let accepted_manifest = manifest(vec![direct_entry(
        "accepted",
        "always_allow",
        0,
        &accepted_witness,
        &accepted_witness,
    )]);

    let accepted = helper::audit_json(&accepted_settings, &accepted_manifest)
        .expect("Boundary audit input must be valid");
    assert!(accepted.findings.is_empty());

    let rejected_witness = "💥".repeat(998);
    let rejected_pattern = format!("^{rejected_witness}$");
    assert_eq!(rejected_pattern.chars().count(), 1000);
    let rejected_settings = settings(vec![pattern(&rejected_pattern, true)], vec![], vec![]);
    let rejected_manifest = manifest(vec![direct_entry(
        "rejected",
        "always_allow",
        0,
        &rejected_witness,
        &rejected_witness,
    )]);

    let reasons = finding_reasons(&rejected_settings, &rejected_manifest);
    assert!(reasons.iter().any(|reason| reason.contains("Maximum: 999")));
}

#[test]
fn rejects_invalid_selected_settings_entries() {
    let valid_manifest = |index| {
        manifest(vec![direct_entry(
            "selected",
            "always_allow",
            index,
            "foo",
            "foo run",
        )])
    };
    let cases = [
        (
            settings(vec![], vec![], vec![]),
            valid_manifest(0),
            "missing",
        ),
        (
            settings(vec![json!("not-an-object")], vec![], vec![]),
            valid_manifest(0),
            "must be an object",
        ),
        (
            settings(vec![json!({"case_sensitive": true})], vec![], vec![]),
            valid_manifest(0),
            "string `pattern`",
        ),
        (
            settings(
                vec![json!({"pattern": "^foo$", "case_sensitive": "true"})],
                vec![],
                vec![],
            ),
            valid_manifest(0),
            "boolean `case_sensitive`",
        ),
    ];

    for (settings, manifest, expected) in cases {
        let error = helper::audit_json(&settings, &manifest)
            .expect_err("Invalid selected settings entry must fail");
        assert!(error.contains(expected), "Unexpected error: {error}");
    }
}

#[test]
fn rejects_unsupported_version_and_invalid_discovery_shapes() {
    let (settings, _) = valid_settings_and_manifest();
    let cases = [
        (
            json!({"version": 2, "entries": []}).to_string(),
            "Unsupported manifest version",
        ),
        (
            {
                let mut entry = entry(
                    "missing-inputs",
                    "always_allow",
                    0,
                    "foo",
                    "foo",
                    "discovery",
                    "discovery",
                    "a",
                    "foo --help",
                    None,
                );
                entry
                    .as_object_mut()
                    .expect("Entry must be an object")
                    .insert("discovery_coverage".to_owned(), json!("complete_finite"));
                manifest(vec![entry])
            },
            "must declare `discovery_inputs`",
        ),
        (
            manifest(vec![entry(
                "missing-coverage",
                "always_allow",
                0,
                "foo",
                "foo",
                "discovery",
                "discovery",
                "a",
                "foo --help",
                None,
            )]),
            "must declare `discovery_coverage`",
        ),
        (
            manifest(vec![entry(
                "empty-inputs",
                "always_allow",
                0,
                "foo",
                "foo",
                "discovery",
                "discovery",
                "a",
                "foo --help",
                Some(&[]),
            )]),
            "at least one `discovery_inputs` value",
        ),
        (
            manifest(vec![entry(
                "missing-witness",
                "always_allow",
                0,
                "foo",
                "foo",
                "discovery",
                "discovery",
                "a",
                "foo --help",
                Some(&["foo -h"]),
            )]),
            "include its `witness`",
        ),
        (
            manifest(vec![entry(
                "direct-inputs",
                "always_allow",
                0,
                "foo",
                "foo",
                "direct",
                "direct",
                "a",
                "foo run",
                Some(&["foo run"]),
            )]),
            "must omit `discovery_coverage` and `discovery_inputs`",
        ),
        (
            manifest(vec![with_case_insensitive_reason(
                direct_entry("empty-reason", "always_allow", 0, "foo", "foo run"),
                "  ",
            )]),
            "must declare a nonempty `case_insensitive_reason`",
        ),
    ];

    for (manifest, expected) in cases {
        let error =
            helper::audit_json(&settings, &manifest).expect_err("Invalid manifest shape must fail");
        assert!(error.contains(expected), "Unexpected error: {error}");
    }
}

#[test]
fn rejects_duplicate_ids_indexes_and_sort_tuples() {
    let (settings, _) = valid_settings_and_manifest();
    let empty_id = manifest(vec![direct_entry("", "always_allow", 0, "foo", "foo run")]);
    let duplicate_id = manifest(vec![
        direct_entry("duplicate", "always_allow", 0, "foo", "foo run"),
        direct_entry("duplicate", "always_confirm", 0, "bar", "bar run"),
    ]);
    let duplicate_index = manifest(vec![
        direct_entry("first", "always_allow", 0, "foo", "foo run"),
        direct_entry("second", "always_allow", 0, "bar", "bar run"),
    ]);
    let duplicate_tuple = manifest(vec![
        entry(
            "first",
            "always_allow",
            0,
            "foo",
            "owner",
            "section",
            "direct",
            "pattern",
            "foo one",
            None,
        ),
        entry(
            "second",
            "always_allow",
            1,
            "foo",
            "owner",
            "section",
            "direct",
            "pattern",
            "foo two",
            None,
        ),
    ]);

    for (manifest, expected) in [
        (empty_id, "IDs must be nonempty"),
        (duplicate_id, "Duplicate manifest entry ID"),
        (duplicate_index, "select the same `always_allow` index"),
        (duplicate_tuple, "same sort tuple"),
    ] {
        let error = helper::audit_json(&settings, &manifest)
            .expect_err("Duplicate manifest contract must fail");
        assert!(error.contains(expected), "Unexpected error: {error}");
    }
}

#[test]
fn reports_owner_sort_order() {
    let settings = settings(
        vec![pattern(r"^zeta run$", true), pattern(r"^alpha run$", true)],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        direct_entry("zeta", "always_allow", 0, "zeta", "zeta run"),
        direct_entry("alpha", "always_allow", 1, "alpha", "alpha run"),
    ]);

    let reasons = finding_reasons(&settings, &manifest);

    assert!(reasons.iter().any(|reason| reason.contains("sort order")));
}

#[test]
fn reports_bucket_order_that_violates_section_sort_keys() {
    let settings = settings(
        vec![
            pattern(r"^foo second$", true),
            pattern(r"^foo first$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "second-section",
            "always_allow",
            0,
            "foo",
            "foo",
            "z-section",
            "direct",
            "a",
            "foo second",
            None,
        ),
        entry(
            "first-section",
            "always_allow",
            1,
            "foo",
            "foo",
            "a-section",
            "direct",
            "a",
            "foo first",
            None,
        ),
    ]);

    let reasons = finding_reasons(&settings, &manifest);

    assert!(reasons.iter().any(|reason| reason.contains("sort order")));
}

#[test]
fn reports_discovery_direct_wrapped_phase_order() {
    let settings = settings(
        vec![
            pattern(r"^foo run$", true),
            pattern(r"^foo --help$", true),
            pattern(r"^xargs foo$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "direct",
            "always_allow",
            0,
            "foo",
            "foo",
            "section",
            "direct",
            "a",
            "foo run",
            None,
        ),
        entry(
            "discovery",
            "always_allow",
            1,
            "foo",
            "foo",
            "section",
            "discovery",
            "a",
            "foo --help",
            Some(&["foo --help"]),
        ),
        entry(
            "wrapped",
            "always_allow",
            2,
            "foo",
            "foo",
            "section",
            "wrapped",
            "a",
            "xargs foo",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(
        report
            .findings
            .iter()
            .any(|finding| { finding.id == "direct" && finding.reason.contains("sort order") })
    );
    assert!(
        report
            .findings
            .iter()
            .any(|finding| { finding.id == "discovery" && finding.reason.contains("sort order") })
    );
    assert!(
        !report
            .findings
            .iter()
            .any(|finding| finding.id == "wrapped")
    );
}

#[test]
fn accepts_separated_sections_for_the_same_semantic_owner() {
    let settings = settings(
        vec![
            pattern(r"^git status$", true),
            pattern(r"^bar run$", true),
            pattern(r"^git -C \.agent-task status$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "git-status-general",
            "always_allow",
            0,
            "git:status",
            "git",
            "0-general",
            "direct",
            "a",
            "git status",
            None,
        ),
        entry(
            "git-status-worktree",
            "always_allow",
            2,
            "git:status",
            "git",
            "1-agent-worktree",
            "direct",
            "a",
            "git -C .agent-task status",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn arbitrary_section_sort_keys_do_not_hide_a_same_owner_role_gap() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^bar run$", true),
            pattern(r"^foo two$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "foo-one",
            "always_allow",
            0,
            "foo",
            "foo",
            "invented-a",
            "direct",
            "a",
            "foo one",
            None,
        ),
        entry(
            "foo-two",
            "always_allow",
            2,
            "foo",
            "foo",
            "invented-b",
            "direct",
            "b",
            "foo two",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn discovery_and_direct_roles_do_not_hide_a_same_owner_gap() {
    let settings = settings(
        vec![
            pattern(r"^foo --help$", true),
            pattern(r"^bar run$", true),
            pattern(r"^foo run$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "foo-discovery",
            "always_allow",
            0,
            "foo",
            "foo",
            "0-discovery",
            "discovery",
            "a",
            "foo --help",
            Some(&["foo --help"]),
        ),
        entry(
            "foo-direct",
            "always_allow",
            2,
            "foo",
            "foo",
            "1-direct",
            "direct",
            "a",
            "foo run",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn git_config_prefix_order_does_not_split_an_agent_worktree_span() {
    let settings = settings(
        vec![
            pattern(
                r"^git -c commit[.]gpgsign=false -C [.]agent-task commit -m one$",
                true,
            ),
            pattern(r"^bar run$", true),
            pattern(
                r"^git -C [.]agent-other -c commit[.]gpgsign=false commit -m two$",
                true,
            ),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "config-before-worktree",
            "always_allow",
            0,
            "git:root",
            "git",
            "0-before",
            "direct",
            "a",
            "git -c commit.gpgsign=false -C .agent-task commit -m one",
            None,
        ),
        entry(
            "config-after-worktree",
            "always_allow",
            2,
            "git:root",
            "git",
            "1-after",
            "direct",
            "b",
            "git -C .agent-other -c commit.gpgsign=false commit -m two",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn dotted_agent_names_remain_in_the_agent_worktree_span() {
    let settings = settings(
        vec![
            pattern(r"^git -C [.]agent-task status$", true),
            pattern(r"^bar run$", true),
            pattern(r"^git -C [.]agent-task[.]v1 status$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "plain-name",
            "always_allow",
            0,
            "git:status",
            "git",
            "0-plain",
            "direct",
            "a",
            "git -C .agent-task status",
            None,
        ),
        entry(
            "dotted-name",
            "always_allow",
            2,
            "git:status",
            "git",
            "1-dotted",
            "direct",
            "b",
            "git -C .agent-task.v1 status",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn accepts_inferred_agent_worktree_and_fixture_sections_for_one_owner() {
    let settings = settings(
        vec![
            pattern(r"^git -C \.agent-task status$", true),
            pattern(r"^bar run$", true),
            pattern(r"^git -C \.agent-task/fixture status$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "worktree",
            "always_allow",
            0,
            "git:status",
            "git",
            "0-worktree",
            "direct",
            "a",
            "git -C .agent-task status",
            None,
        ),
        entry(
            "fixture",
            "always_allow",
            2,
            "git:status",
            "git",
            "1-fixture",
            "direct",
            "a",
            "git -C .agent-task/fixture status",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn malformed_repository_paths_cannot_create_a_completeness_section() {
    let settings = settings(
        vec![
            pattern(r"^git -C \.agent-task/\.\./outside status$", true),
            pattern(r"^bar run$", true),
            pattern(r"^git -C other status$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "traversal",
            "always_allow",
            0,
            "git:status",
            "git",
            "invented-a",
            "direct",
            "a",
            "git -C .agent-task/../outside status",
            None,
        ),
        entry(
            "general",
            "always_allow",
            2,
            "git:status",
            "git",
            "invented-b",
            "direct",
            "b",
            "git -C other status",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn reports_a_gap_inside_one_owner_section_span() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^bar run$", true),
            pattern(r"^foo two$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "foo-one",
            "always_allow",
            0,
            "foo",
            "foo",
            "direct",
            "direct",
            "a",
            "foo one",
            None,
        ),
        entry(
            "foo-two",
            "always_allow",
            2,
            "foo",
            "foo",
            "direct",
            "direct",
            "b",
            "foo two",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.findings.len(), 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn accepts_same_owner_entries_in_one_contiguous_section() {
    let settings = settings(
        vec![pattern(r"^foo one$", true), pattern(r"^foo two$", true)],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "foo-one",
            "always_allow",
            0,
            "foo",
            "foo",
            "direct",
            "direct",
            "a",
            "foo one",
            None,
        ),
        entry(
            "foo-two",
            "always_allow",
            1,
            "foo",
            "foo",
            "direct",
            "direct",
            "b",
            "foo two",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn validates_every_declared_discovery_input() {
    let settings = settings(vec![pattern(r"^foo --help$", true)], vec![], vec![]);
    let manifest = manifest(vec![entry(
        "foo-discovery",
        "always_allow",
        0,
        "foo",
        "foo",
        "discovery",
        "discovery",
        "a",
        "foo --help",
        Some(&["foo --help", "foo --version"]),
    )]);

    let reasons = finding_reasons(&settings, &manifest);

    assert!(
        reasons
            .iter()
            .any(|reason| { reason.contains("does not match every declared discovery input") })
    );
}

#[test]
fn accepts_representative_variable_discovery_without_claiming_finite_redundancy() {
    let settings = settings(
        vec![
            pattern(r"^pnpm exec which (?:node|npm)$", true),
            pattern(r"^pnpm (?:exec which (?:node|npm)|run)$", true),
        ],
        vec![],
        vec![],
    );
    let mut discovery = entry(
        "pnpm-variable-discovery",
        "always_allow",
        0,
        "pnpm",
        "pnpm",
        "owner",
        "discovery",
        "a",
        "pnpm exec which node",
        Some(&["pnpm exec which node", "pnpm exec which npm"]),
    );
    discovery
        .as_object_mut()
        .expect("Entry must be an object")
        .insert("discovery_coverage".to_owned(), json!("representative"));
    let manifest = manifest(vec![
        discovery,
        entry(
            "pnpm-direct",
            "always_allow",
            1,
            "pnpm",
            "pnpm",
            "owner",
            "direct",
            "a",
            "pnpm run",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn reports_redundant_allow_discovery_across_git_manager_group() {
    let settings = settings(
        vec![
            pattern(r"^git --(?:help|version)$", true),
            pattern(r"^git (?:--version|hash-object)$", true),
            pattern(r"^git (?:--help|status)$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "git-root-discovery",
            "always_allow",
            0,
            "git:root",
            "git",
            "0-root",
            "discovery",
            "a",
            "git --help",
            Some(&["git --help", "git --version"]),
        ),
        entry(
            "git-hash-object-direct",
            "always_allow",
            1,
            "git:hash-object",
            "git",
            "1-direct-hash-object",
            "direct",
            "a",
            "git hash-object",
            None,
        ),
        entry(
            "git-status-direct",
            "always_allow",
            2,
            "git:status",
            "git",
            "1-direct-status",
            "direct",
            "a",
            "git status",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].id, "git-root-discovery");
    assert!(report.findings[0].reason.contains("redundant"));
}

#[test]
fn ignores_discovery_overlap_outside_always_allow() {
    let settings = settings(
        vec![],
        vec![
            pattern(r"^foo --help$", true),
            pattern(r"^foo (?:--help|run)$", true),
        ],
        vec![
            pattern(r"^bar --help$", true),
            pattern(r"^bar (?:--help|run)$", true),
        ],
    );
    let manifest = manifest(vec![
        entry(
            "foo-discovery",
            "always_confirm",
            0,
            "foo",
            "foo",
            "section",
            "discovery",
            "a",
            "foo --help",
            Some(&["foo --help"]),
        ),
        entry(
            "foo-direct",
            "always_confirm",
            1,
            "foo",
            "foo",
            "section",
            "direct",
            "a",
            "foo run",
            None,
        ),
        entry(
            "bar-discovery",
            "always_deny",
            0,
            "bar",
            "bar",
            "section",
            "discovery",
            "a",
            "bar --help",
            Some(&["bar --help"]),
        ),
        entry(
            "bar-direct",
            "always_deny",
            1,
            "bar",
            "bar",
            "section",
            "direct",
            "a",
            "bar run",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn does_not_treat_unapproved_executables_as_wrappers() {
    let inferred = helper::infer_owner_role("env foo run", &[])
        .expect("Ordinary executable witness must be supported");

    assert_eq!(inferred.owner, "env");
    assert_eq!(inferred.role, helper::Role::Direct);
}

#[test]
fn infers_xargs_own_discovery_without_a_child() {
    let discovery = vec!["xargs --help".to_owned()];
    let inferred = helper::infer_owner_role("xargs --help", &discovery)
        .expect("`xargs` discovery witness must be supported");

    assert_eq!(inferred.owner, "xargs");
    assert_eq!(inferred.role, helper::Role::Discovery);

    let ambiguous = vec!["xargs --replace foo".to_owned()];
    assert!(helper::infer_owner_role("xargs --replace foo", &ambiguous).is_err());
}

#[test]
fn xargs_discovery_participates_in_owner_span_completeness() {
    let settings = settings(
        vec![
            pattern(r"^xargs --help$", true),
            pattern(r"^bar run$", true),
            pattern(r"^xargs$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "xargs-discovery",
            "always_allow",
            0,
            "xargs",
            "xargs",
            "0-discovery",
            "discovery",
            "a",
            "xargs --help",
            Some(&["xargs --help"]),
        ),
        entry(
            "xargs-direct",
            "always_allow",
            2,
            "xargs",
            "xargs",
            "1-direct",
            "direct",
            "a",
            "xargs",
            None,
        ),
    ]);

    let report = helper::audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-section group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn bounds_finding_output_and_never_leaks_patterns_or_witnesses() {
    let fixture = Fixture::new();
    let mut patterns = Vec::new();
    let mut entries = Vec::new();

    for index in 0..12 {
        let owner = format!("secret-owner-{index:02}");
        let witness = format!("{owner} private-witness-{index:02}");
        let regex = format!("^private-regex-{index:02}$");
        let id = format!("entry-{index:02}");
        patterns.push(pattern(&regex, index != 0));
        entries.push(entry(
            &id,
            "always_allow",
            index,
            &owner,
            &owner,
            "direct",
            "direct",
            "a",
            &witness,
            None,
        ));
    }

    let settings_path = fixture.write("settings.json", &settings(patterns, vec![], vec![]));
    let manifest_path = fixture.write("manifest.json", &manifest(entries));
    let (status, stdout, stderr) = run_files(&settings_path, &manifest_path);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 findings across 12 entries"));
    assert_eq!(stderr.matches("  `entry-").count(), 10);
    assert!(
        stderr
            .contains("`entry-00`: `case_sensitive` is `false` without `case_insensitive_reason`; pattern does not match its witness")
    );
    assert!(stderr.contains("`entry-09`"));
    assert!(!stderr.contains("`entry-10`"));
    assert!(stderr.contains("… 2 additional findings omitted"));
    assert!(!stderr.contains("private-regex"));
    assert!(!stderr.contains("private-witness"));
    assert!(!stderr.contains("secret-owner"));
}
