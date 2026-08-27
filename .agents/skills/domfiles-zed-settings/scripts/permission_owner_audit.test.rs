#[path = "permission_owner_audit.rs"]
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
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

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

fn manifest_for(
    inventory_owner: &str,
    entries: Vec<Value>,
    excluded_candidates: Vec<Value>,
) -> String {
    json!({
        "settings_sha256": ZERO_SHA256,
        "inventory_owner": inventory_owner,
        "entries": entries,
        "excluded_candidates": excluded_candidates,
    })
    .to_string()
}

fn manifest(entries: Vec<Value>) -> String {
    let inventory_owner = entries
        .first()
        .and_then(|entry| entry.get("owner"))
        .and_then(Value::as_str)
        .and_then(|owner| owner.split(':').next())
        .expect("Manifest entries must identify an inventory owner")
        .to_owned();

    manifest_for(&inventory_owner, entries, vec![])
}

fn excluded_candidate(
    bucket: &str,
    index: usize,
    owner: &str,
    witness: &str,
    reason: &str,
) -> Value {
    json!({
        "bucket": bucket,
        "index": index,
        "owner": owner,
        "witness": witness,
        "reason": reason,
    })
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

fn run_inventory_after(
    settings_path: &Path,
    owner: &str,
    after: Option<&str>,
) -> (u8, String, String) {
    let mut arguments = vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from(owner),
    ];
    if let Some(after) = after {
        arguments.push(OsString::from("--after"));
        arguments.push(OsString::from(after));
    }

    run(arguments)
}

fn run_inventory(settings_path: &Path, owner: &str) -> (u8, String, String) {
    run_inventory_after(settings_path, owner, None)
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

fn inventory_cursor(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("Next inventory cursor: "))
        .expect("Inventory page must contain a continuation cursor")
        .to_owned()
}

fn write_valid_files(fixture: &Fixture) -> (PathBuf, PathBuf) {
    let (settings, manifest) = valid_settings_and_manifest();
    let manifest = bind_manifest(&settings, &manifest);
    (
        fixture.write("settings.json", &settings),
        fixture.write("manifest.json", &manifest),
    )
}

fn bind_manifest(settings: &str, manifest: &str) -> String {
    let mut manifest: Value =
        serde_json::from_str(manifest).expect("Manifest fixture must be JSON");
    manifest
        .as_object_mut()
        .expect("Manifest fixture must be an object")
        .insert(
            "settings_sha256".to_owned(),
            json!(helper::settings_sha256(settings)),
        );
    manifest.to_string()
}

fn audit_json(settings: &str, manifest: &str) -> Result<helper::AuditReport, String> {
    helper::audit_json(settings, &bind_manifest(settings, manifest))
}

fn finding_reasons(settings: &str, manifest: &str) -> Vec<String> {
    audit_json(settings, manifest)
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
        "--owner <top-level-executable> [--after <inventory-cursor>] [--graph-root <dir>] [--result-out <path>]\n"
    ));
    assert!(stdout.contains("with `--owner` a complete inventory without `--after`"));
    assert!(stdout.contains("Canonical manifest schema"));
    assert!(!stdout.contains("manifest version"));
    assert!(stdout.contains("settings_sha256"));
    assert!(stdout.contains("inventory_owner"));
    assert!(stdout.contains("excluded_candidates"));
    assert!(stdout.contains("unknown fields are rejected"));
    assert!(stdout.contains("case_insensitive_reason"));
    assert!(stdout.contains("complete_finite|representative"));
    assert!(stdout.contains("[A-Za-z0-9_.+-]+"));
    assert!(stdout.contains("Matches are inventory candidates"));
    assert!(stdout.contains("not semantic ownership proof"));
    assert!(stdout.contains("Each selected decoded pattern must contain at most 999"));
    assert!(stdout.contains("independently inferred bucket, semantic owner, and Git repository"));
    assert!(stdout.contains("Declared roles and sort keys"));
    assert!(stdout.contains("Git discovery-to-direct or discovery-to-wrapped gap"));
    assert!(stdout.contains("exact total and"));
    assert!(stdout.contains("opaque continuation cursor only when"));
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
    let unknown_root = json!({
        "settings_sha256": ZERO_SHA256,
        "inventory_owner": "foo",
        "entries": [direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        "excluded_candidates": [],
        "version": 1,
    })
    .to_string();
    let mut unknown_entry = direct_entry("foo", "always_allow", 0, "foo", "foo run");
    unknown_entry
        .as_object_mut()
        .expect("Entry must be an object")
        .insert("extra".to_owned(), json!(true));
    let unknown_entry = manifest(vec![unknown_entry]);
    let unknown_exclusion = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![json!({
            "bucket": "always_confirm",
            "index": 0,
            "owner": "bar",
            "witness": "bar foo",
            "reason": "owned elsewhere",
            "extra": true,
        })],
    );

    for manifest in [
        malformed.to_owned(),
        unknown_root,
        unknown_entry,
        unknown_exclusion,
    ] {
        let error = helper::audit_json(&settings, &manifest)
            .expect_err("Malformed manifest must be rejected");
        assert!(
            error.contains("Manifest JSON") && error.contains("line") && error.contains("column"),
            "Unexpected error: {error}"
        );
        assert!(!error.contains("extra"));
        assert!(!error.contains("version"));
    }
}

#[test]
fn rejects_duplicate_known_manifest_fields() {
    let (settings, _) = valid_settings_and_manifest();
    let duplicate_root = r#"{
        "settings_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "inventory_owner": "foo",
        "inventory_owner": "foo",
        "entries": [{
            "id": "foo",
            "bucket": "always_allow",
            "index": 0,
            "owner": "foo",
            "owner_sort_key": "foo",
            "section_sort_key": "direct",
            "role": "direct",
            "pattern_sort_key": "foo",
            "witness": "foo run"
        }],
        "excluded_candidates": []
    }"#;
    let duplicate_entry = r#"{
        "settings_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "inventory_owner": "foo",
        "entries": [{
            "id": "foo",
            "bucket": "always_allow",
            "index": 0,
            "index": 0,
            "owner": "foo",
            "owner_sort_key": "foo",
            "section_sort_key": "direct",
            "role": "direct",
            "pattern_sort_key": "foo",
            "witness": "foo run"
        }],
        "excluded_candidates": []
    }"#;

    for manifest in [duplicate_root, duplicate_entry] {
        let error = helper::audit_json(&settings, manifest)
            .expect_err("Duplicate known field must be rejected");
        assert!(error.contains("Manifest JSON data does not match"));
        assert!(error.contains("line") && error.contains("column"));
        assert!(!error.contains("inventory_owner"));
        assert!(!error.contains("index"));
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

    let error =
        audit_json("{}", &manifest).expect_err("Missing terminal settings path must be rejected");
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
fn rejects_manifest_bound_to_different_exact_settings_bytes() {
    let (settings, manifest) = valid_settings_and_manifest();
    let bound_manifest = bind_manifest(&settings, &manifest);
    let reformatted_settings = format!("{settings}\n");

    let error = helper::audit_json(&reformatted_settings, &bound_manifest)
        .expect_err("Different exact settings bytes must invalidate the manifest");

    assert!(error.contains("settings_sha256"));
    assert!(error.contains("Rebuild the inventory and manifest"));
    assert!(!error.contains(&helper::settings_sha256(&settings)));
    assert!(!error.contains(&helper::settings_sha256(&reformatted_settings)));

    let mut invalid: Value = serde_json::from_str(&manifest).unwrap();
    invalid["settings_sha256"] = json!("not-a-sha256");
    let error = helper::audit_json(&settings, &invalid.to_string())
        .expect_err("Malformed settings digest must be rejected");
    assert!(error.contains("64 lowercase hexadecimal characters"));
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

    let (status, stdout, stderr) = run_inventory(&settings_path, "foo");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(!stdout.contains("always_allow[4]"));
    assert!(stdout.contains("Total inventory candidates: 0"));

    let (status, stdout, stderr) = run_inventory(&settings_path, "git");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(!stdout.contains("always_allow[6]"));
    assert!(stdout.contains("Total inventory candidates: 0"));
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
    assert!(stdout.starts_with(
        "Inventory results are candidates, not semantic ownership proof\nInventory settings SHA-256: "
    ));
    assert!(stdout.contains("\nTotal inventory candidates: 0\n"));
    assert!(stdout.ends_with("Inventory candidates remaining after this page: 0\n"));
    assert!(stderr.is_empty());
}

#[test]
fn paginates_one_hundred_three_candidates_without_duplicates_or_omissions() {
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

    let settings_path = fixture.write(
        "settings.json",
        &settings(always_allow, always_confirm, always_deny),
    );
    let (first_status, first_stdout, first_stderr) =
        run_inventory_after(&settings_path, "git", None);
    let cursor = inventory_cursor(&first_stdout);
    let (second_status, second_stdout, second_stderr) =
        run_inventory_after(&settings_path, "git", Some(&cursor));

    assert_eq!(first_status, 0);
    assert_eq!(second_status, 0);
    assert!(first_stderr.is_empty());
    assert!(second_stderr.is_empty());
    let mut ids = first_stdout
        .lines()
        .chain(second_stdout.lines())
        .filter(|line| line.starts_with("always_"))
        .map(|line| {
            line.split_once(' ')
                .expect("Candidate line must contain fields")
                .0
        })
        .collect::<Vec<_>>();
    assert_eq!(ids.len(), 103);
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 103);
    assert!(first_stdout.contains("always_deny[31]"));
    assert!(!first_stdout.contains("always_deny[32]"));
    assert!(second_stdout.contains("always_deny[32]"));
    assert!(second_stdout.contains("always_deny[33]"));
    assert!(second_stdout.contains("always_deny[34]"));
    assert!(first_stdout.contains("Total inventory candidates: 103"));
    assert!(first_stdout.contains("Inventory candidates remaining after this page: 3"));
    assert!(cursor.ends_with(":git:always_deny[31]"));
    assert!(first_stdout.contains(&format!("Next inventory cursor: {cursor}")));
    assert!(second_stdout.contains("Total inventory candidates: 103"));
    assert!(second_stdout.contains("Inventory candidates remaining after this page: 0"));
    assert!(!second_stdout.contains("Next inventory cursor:"));
    assert!(!first_stdout.contains(r"^(?:git allow-00 private)$"));
    assert!(!second_stdout.contains(r"^(?:git deny-32 private)$"));
}

#[test]
fn reports_exactly_one_hundred_candidates_without_a_cursor() {
    let fixture = Fixture::new();
    let always_allow = (0..100)
        .map(|index| pattern(&format!(r"^git item-{index:03}$"), true))
        .collect();
    let settings_path = fixture.write("settings.json", &settings(always_allow, vec![], vec![]));

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
    assert!(stdout.contains("Total inventory candidates: 100"));
    assert!(stdout.contains("Inventory candidates remaining after this page: 0"));
    assert!(!stdout.contains("Next inventory cursor:"));
}

#[test]
fn rejects_malformed_missing_nonmatching_and_cross_owner_inventory_cursors() {
    let fixture = Fixture::new();
    let settings_contents = settings(
        vec![
            pattern(r"^git status$", true),
            pattern(r"^npm --version$", true),
        ],
        vec![],
        vec![],
    );
    let settings_path = fixture.write("settings.json", &settings_contents);
    let settings_sha256 = helper::settings_sha256(&settings_contents);
    let cursors = vec![
        "always_allow[0]".to_owned(),
        format!("{settings_sha256}:git:always_allow[01]"),
        format!("{settings_sha256}:git:always_allow[-1]"),
        format!("{settings_sha256}:git:always_allow[2"),
        format!("{settings_sha256}:git:allow[0]"),
        format!("{settings_sha256}:git:always_allow[99]"),
        format!("{settings_sha256}:git:always_allow[1]"),
        format!("{settings_sha256}:npm:always_allow[0]"),
    ];

    for cursor in cursors {
        let (status, stdout, stderr) = run_inventory_after(&settings_path, "git", Some(&cursor));
        assert_eq!(status, 2, "Cursor {cursor} must fail");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains("exact inventory cursor")
                || stderr.contains("does not match the current settings snapshot and owner")
                || stderr.contains("missing or no longer identifies a lexical candidate"),
            "Unexpected error: {stderr}"
        );
    }

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("git"),
        OsString::from("--after"),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("requires a cursor"));
}

#[test]
fn rejects_duplicate_after_and_after_with_manifest_mode() {
    let fixture = Fixture::new();
    let (settings_path, manifest_path) = write_valid_files(&fixture);

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("foo"),
        OsString::from("--after"),
        OsString::from(format!("{ZERO_SHA256}:git:always_allow[0]")),
        OsString::from("--after"),
        OsString::from(format!("{ZERO_SHA256}:git:always_allow[0]")),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("may be specified only once"));

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--after"),
        OsString::from(format!("{ZERO_SHA256}:foo:always_allow[0]")),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("valid only with"));
}

#[test]
fn records_inventory_query_evidence_for_a_complete_raw_owner_page() {
    let fixture = Fixture::new();
    let settings_contents = settings(vec![pattern(r"^git status$", true)], vec![], vec![]);
    fixture.write("settings.json", &settings_contents);
    let graph_root =
        fs::canonicalize(&fixture.root).expect("Fixture root must resolve to a real directory");
    let settings_path = graph_root.join("settings.json");
    let result_path = graph_root.join("inventory-result.json");

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("git"),
        OsString::from("--graph-root"),
        graph_root.as_os_str().to_owned(),
        OsString::from("--result-out"),
        result_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("Total inventory candidates: 1"));
    let result: Value =
        serde_json::from_slice(&fs::read(&result_path).expect("Inventory evidence must exist"))
            .expect("Inventory evidence must be JSON");
    assert_eq!(result["kind"], json!("inventory_query"));
    assert_eq!(result["bound_inputs"]["inventory_owner"], json!("git"));
    assert_eq!(
        result["bound_inputs"]["settings_sha256"],
        json!(helper::settings_sha256(&settings_contents))
    );
    assert_eq!(result["counts"]["lexical_candidates"], json!(1));

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("git"),
        OsString::from("--after"),
        OsString::from(format!("{ZERO_SHA256}:git:always_allow[0]")),
        OsString::from("--graph-root"),
        graph_root.as_os_str().to_owned(),
        OsString::from("--result-out"),
        graph_root.join("paged-result.json").as_os_str().to_owned(),
    ]);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("valid only for a complete inventory"));
}

#[test]
fn invalidates_inventory_cursors_after_any_settings_change() {
    let fixture = Fixture::new();
    let original_settings = settings(
        (0..101)
            .map(|index| pattern(&format!(r"^git item-{index:03}$"), true))
            .collect(),
        vec![],
        vec![],
    );
    let original_path = fixture.write("original-settings.json", &original_settings);
    let (status, stdout, stderr) = run_inventory(&original_path, "git");
    assert_eq!(status, 0);
    assert!(stderr.is_empty());
    let cursor = inventory_cursor(&stdout);

    let mut changed_pattern: Value = serde_json::from_str(&original_settings).unwrap();
    changed_pattern["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"][100]["case_sensitive"] =
        json!(false);
    let mut inserted: Value = serde_json::from_str(&original_settings).unwrap();
    inserted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
        .as_array_mut()
        .unwrap()
        .insert(0, pattern(r"^git inserted$", true));
    let mut reordered: Value = serde_json::from_str(&original_settings).unwrap();
    reordered["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);

    for (index, changed) in [changed_pattern, inserted, reordered]
        .into_iter()
        .enumerate()
    {
        let changed_path = fixture.write(&format!("changed-{index}.json"), &changed.to_string());
        let (status, stdout, stderr) = run_inventory_after(&changed_path, "git", Some(&cursor));
        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains("does not match the current settings snapshot and owner"));
    }
}

#[test]
fn rejects_invalid_inventory_settings_without_pattern_leakage() {
    let fixture = Fixture::new();
    let private_pattern = "private-pattern-without-owner";
    let cases = [
        ("{not-json".to_owned(), "Settings JSON syntax is invalid"),
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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
fn infers_git_root_discovery_and_direct_owners() {
    let cases = [
        ("git", "git:root", helper::Role::Direct),
        ("git --version", "git:root", helper::Role::Discovery),
        ("git -h", "git:root", helper::Role::Discovery),
        ("git -help", "git:root", helper::Role::Discovery),
        ("git -v", "git:root", helper::Role::Discovery),
        ("git -version", "git:root", helper::Role::Discovery),
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
}

#[test]
fn infers_exact_git_commit_prefixes_under_supported_wrappers() {
    let cases = [
        (
            "git -c commit.gpgsign=false -C .agent-task commit -m one",
            helper::Role::Direct,
        ),
        (
            "git -C .agent-task -c commit.gpgsign=false commit -m two",
            helper::Role::Direct,
        ),
        (
            "nohup git -c commit.gpgsign=false -C .agent-task commit -m three",
            helper::Role::Wrapped,
        ),
        (
            "xargs git -C .agent-task -c commit.gpgsign=false commit -m four",
            helper::Role::Wrapped,
        ),
    ];

    for (witness, role) in cases {
        let inferred =
            helper::infer_owner_role(witness, &[]).expect("Approved Git prefix must be supported");
        assert_eq!(inferred.owner, "git:commit");
        assert_eq!(inferred.role, role);
    }
}

#[test]
fn leaves_unapproved_git_config_forms_outside_commit_prefix_inference() {
    let cases = [
        (
            "git -c commit.gpgsign=true -C .agent-task commit",
            "git:root",
        ),
        ("git -c core.foo=false -C .agent-task commit", "git:root"),
        (
            "git -ccommit.gpgsign=false -C .agent-task commit",
            "git:root",
        ),
        ("git -c", "git:root"),
        ("git --unknown -C .agent-task commit", "git:root"),
        ("git commit -c commit.gpgsign=false", "git:commit"),
    ];

    for (witness, owner) in cases {
        let inferred = helper::infer_owner_role(witness, &[])
            .expect("Near-miss Git witness must remain classifiable");
        assert_eq!(inferred.owner, owner, "Unexpected owner for {witness}");
    }
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
            pattern(r"^foo --version$", true),
            pattern(r"^foo run$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest_for(
        "foo",
        vec![
            entry(
                "wrong-owner",
                "always_allow",
                0,
                "corepack",
                "a",
                "a",
                "discovery",
                "a",
                "foo --version",
                Some(&["foo --version"]),
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
        ],
        vec![],
    );

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.iter().any(|finding| {
        finding.id == "wrong-owner" && finding.reason.contains("declared owner differs")
    }));
    assert!(
        report.findings.iter().any(|finding| {
            finding.id == "wrong-role" && finding.reason.contains("declared role")
        })
    );
}

#[test]
fn reports_entry_inferred_outside_inventory_owner() {
    let settings = settings(vec![pattern(r"^bar foo$", true)], vec![], vec![]);
    let manifest = manifest_for(
        "foo",
        vec![direct_entry(
            "foreign-owner",
            "always_allow",
            0,
            "bar",
            "bar foo",
        )],
        vec![],
    );

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 1);
    assert!(
        report.findings[0]
            .reason
            .contains("outside the manifest inventory owner")
    );
}

#[test]
fn accepts_verified_case_insensitive_exception() {
    let settings = settings(vec![pattern(r"^foo run$", false)], vec![], vec![]);
    let manifest = manifest(vec![with_case_insensitive_reason(
        direct_entry("case", "always_allow", 0, "foo", "foo run"),
        "The command requires case-insensitive matching",
    )]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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
    let private_pattern = "private-regex-body(foo(";
    let private_witness = "foo private-witness-input";
    let settings = settings(
        vec![
            pattern(r"^foo$", true),
            pattern(r"^foo run$", false),
            pattern(private_pattern, true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        direct_entry("a-witness", "always_allow", 0, "foo", "foo run"),
        direct_entry("b-case", "always_allow", 1, "foo", "foo run"),
        direct_entry("c-invalid", "always_allow", 2, "foo", private_witness),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.iter().any(|finding| {
        finding.id == "a-witness" && finding.reason.contains("does not match its witness")
    }));
    assert!(report.findings.iter().any(|finding| {
        finding.id == "b-case"
            && finding.reason.contains("case_sensitive")
            && finding.reason.contains("case_insensitive_reason")
    }));
    let invalid = report
        .findings
        .iter()
        .find(|finding| finding.id == "c-invalid")
        .expect("Invalid regex finding must exist");
    assert!(invalid.reason.contains("regex is invalid"));
    assert!(!invalid.reason.contains(private_pattern));
    assert!(!invalid.reason.contains(private_witness));
}

#[test]
fn measures_decoded_unicode_scalar_length_at_boundary() {
    let accepted_witness = format!("foo {}", "💥".repeat(993));
    let accepted_pattern = format!("^{accepted_witness}$");
    assert_eq!(accepted_pattern.chars().count(), 999);
    let accepted_settings = settings(vec![pattern(&accepted_pattern, true)], vec![], vec![]);
    let accepted_manifest = manifest(vec![direct_entry(
        "accepted",
        "always_allow",
        0,
        "foo",
        &accepted_witness,
    )]);

    let accepted = audit_json(&accepted_settings, &accepted_manifest)
        .expect("Boundary audit input must be valid");
    assert!(accepted.findings.is_empty());

    let rejected_witness = format!("foo {}", "💥".repeat(994));
    let rejected_pattern = format!("^{rejected_witness}$");
    assert_eq!(rejected_pattern.chars().count(), 1000);
    let rejected_settings = settings(vec![pattern(&rejected_pattern, true)], vec![], vec![]);
    let rejected_manifest = manifest(vec![direct_entry(
        "rejected",
        "always_allow",
        0,
        "foo",
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
        let error = audit_json(&settings, &manifest)
            .expect_err("Invalid selected settings entry must fail");
        assert!(error.contains(expected), "Unexpected error: {error}");
    }
}

#[test]
fn rejects_invalid_discovery_shapes() {
    let (settings, _) = valid_settings_and_manifest();
    let cases = [
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
            "must declare discovery_inputs",
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
            "must declare discovery_coverage",
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
            "at least one discovery_inputs value",
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
            "include its witness",
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
            "must omit discovery_coverage and discovery_inputs",
        ),
        (
            manifest(vec![with_case_insensitive_reason(
                direct_entry("empty-reason", "always_allow", 0, "foo", "foo run"),
                "  ",
            )]),
            "must declare a nonempty case_insensitive_reason",
        ),
    ];

    for (manifest, expected_words) in cases {
        let error = audit_json(&settings, &manifest).expect_err("Invalid manifest shape must fail");
        for word in expected_words.split(' ') {
            assert!(error.contains(word), "Unexpected error: {error}");
        }
    }
}

#[test]
fn rejects_empty_manifest_and_invalid_inventory_owner() {
    let (settings, _) = valid_settings_and_manifest();
    for manifest in [
        manifest_for("foo", vec![], vec![]),
        manifest_for(
            "",
            vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
            vec![],
        ),
        manifest_for(
            "foo bar",
            vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
            vec![],
        ),
    ] {
        let error = audit_json(&settings, &manifest).expect_err("Invalid manifest root must fail");
        assert!(
            error.contains("must be nonempty") || error.contains("inventory_owner"),
            "Unexpected error: {error}"
        );
    }
}

#[test]
fn rejects_omitted_first_and_last_lexical_candidates() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^foo two$", true),
            pattern(r"^foo three$", true),
        ],
        vec![],
        vec![],
    );
    let omitted_first = manifest(vec![
        direct_entry("two", "always_allow", 1, "foo", "foo two"),
        direct_entry("three", "always_allow", 2, "foo", "foo three"),
    ]);
    let omitted_last = manifest(vec![
        direct_entry("one", "always_allow", 0, "foo", "foo one"),
        direct_entry("two", "always_allow", 1, "foo", "foo two"),
    ]);

    let first_error =
        audit_json(&settings, &omitted_first).expect_err("Omitted first candidate must fail");
    assert!(first_error.contains("1 missing candidate position"));
    assert!(first_error.contains("always_allow[0]"));
    let last_error =
        audit_json(&settings, &omitted_last).expect_err("Omitted last candidate must fail");
    assert!(last_error.contains("1 missing candidate position"));
    assert!(last_error.contains("always_allow[2]"));
}

#[test]
fn rejects_duplicate_and_overlapping_excluded_positions() {
    let (settings, _) = valid_settings_and_manifest();
    let duplicate = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![
            excluded_candidate("always_confirm", 0, "bar", "bar foo", "owned elsewhere"),
            excluded_candidate("always_confirm", 0, "bar", "bar foo", "owned elsewhere"),
        ],
    );
    let overlap = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            0,
            "bar",
            "foo run",
            "owned elsewhere",
        )],
    );
    let empty_reason = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_confirm",
            0,
            "bar",
            "bar foo",
            "  ",
        )],
    );

    for (manifest, expected) in [
        (duplicate, "Duplicate excluded candidate position"),
        (overlap, "both classify"),
        (empty_reason, "nonempty reason"),
    ] {
        let error = audit_json(&settings, &manifest)
            .expect_err("Invalid exclusion classification must fail");
        for word in expected.split(' ') {
            assert!(error.contains(word), "Unexpected error: {error}");
        }
    }
}

#[test]
fn bounds_overlapping_entry_ids_in_structural_errors() {
    let settings = settings(vec![pattern(r"^foo run$", true)], vec![], vec![]);
    let private_suffix = "private-id-suffix";
    let long_id = format!("{}\n{}{private_suffix}", "x".repeat(40), "y".repeat(60));
    let manifest = manifest_for(
        "foo",
        vec![direct_entry(&long_id, "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            0,
            "bar",
            "foo run",
            "owned elsewhere",
        )],
    );

    let error = audit_json(&settings, &manifest)
        .expect_err("Overlapping entry and exclusion positions must fail");

    let bounded_id = format!("{}?{}…", "x".repeat(40), "y".repeat(39));
    assert!(error.contains(&bounded_id));
    assert!(!error.contains(private_suffix));
    assert!(!error.contains(&long_id));
}

#[test]
fn verifies_excluded_candidates_belong_to_another_owner() {
    let outside_owner_settings = settings(
        vec![pattern(r"^foo run$", true), pattern(r"^bar foo$", true)],
        vec![],
        vec![],
    );
    let valid = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            1,
            "bar",
            "bar foo",
            "the lexical `foo` token belongs to `bar`",
        )],
    );
    let report =
        audit_json(&outside_owner_settings, &valid).expect("Outside-owner exclusion must be valid");
    assert!(report.findings.is_empty());

    let same_owner = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            1,
            "bar",
            "foo run",
            "invalid classification",
        )],
    );
    let mismatch = audit_json(&outside_owner_settings, &same_owner)
        .expect_err("Exclusion witness must match its selected pattern");
    assert!(mismatch.contains("does not match its witness"));

    let wrong_declared_owner = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            1,
            "baz",
            "bar foo",
            "invalid classification",
        )],
    );
    let mismatch = audit_json(&outside_owner_settings, &wrong_declared_owner)
        .expect_err("Exclusion owner must match its inferred owner");
    assert!(mismatch.contains("differs from its inferred owner"));

    let hidden_same_owner_settings = settings(
        vec![pattern(r"^foo run$", true), pattern(r"^foo hidden$", true)],
        vec![],
        vec![],
    );
    let hidden_same_owner = manifest_for(
        "foo",
        vec![direct_entry("foo", "always_allow", 0, "foo", "foo run")],
        vec![excluded_candidate(
            "always_allow",
            1,
            "foo",
            "foo hidden",
            "invalid classification",
        )],
    );
    let error = audit_json(&hidden_same_owner_settings, &hidden_same_owner)
        .expect_err("Inventory-owner candidate must not be excluded");
    assert!(error.contains("infers to the manifest inventory owner"));
}

#[test]
fn bounds_missing_and_unexpected_coverage_diagnostics() {
    let mut patterns = Vec::new();
    let mut entries = Vec::new();
    for index in 0..15 {
        patterns.push(pattern(&format!(r"^foo missing-{index:02}$"), true));
    }
    for index in 15..30 {
        patterns.push(pattern(&format!(r"^bar unexpected-{index:02}$"), true));
        entries.push(direct_entry(
            &format!("bar-{index:02}"),
            "always_allow",
            index,
            "bar",
            &format!("bar unexpected-{index:02}"),
        ));
    }
    let settings = settings(patterns, vec![], vec![]);
    let manifest = manifest_for("foo", entries, vec![]);

    let error =
        audit_json(&settings, &manifest).expect_err("Incomplete lexical classification must fail");

    assert!(error.contains("15 missing candidate positions"));
    assert!(error.contains("15 unexpected classified positions"));
    assert_eq!(error.matches("always_allow[").count(), 10);
    assert!(!error.contains("missing-00"));
    assert!(!error.contains("unexpected-15"));
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
        let error =
            audit_json(&settings, &manifest).expect_err("Duplicate manifest contract must fail");
        assert!(error.contains(expected), "Unexpected error: {error}");
    }
}

#[test]
fn reports_owner_sort_order() {
    let settings = settings(
        vec![pattern(r"^git zeta$", true), pattern(r"^git alpha$", true)],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        direct_entry("zeta", "always_allow", 0, "git:zeta", "git zeta"),
        direct_entry("alpha", "always_allow", 1, "git:alpha", "git alpha"),
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn git_config_prefix_order_infers_commit_in_one_agent_worktree_span() {
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
            "git:commit",
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
            "git:commit",
            "git",
            "1-after",
            "direct",
            "b",
            "git -C .agent-other -c commit.gpgsign=false commit -m two",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding.reason.contains("does not completely occupy")
            && !finding.reason.contains("inferred owner")
    }));
}

#[test]
fn separates_agent_worktree_and_fixture_repository_commit_scopes() {
    let settings = settings(
        vec![
            pattern(
                r"^git -c commit[.]gpgsign=false -C [.]agent-task commit -m one$",
                true,
            ),
            pattern(r"^bar run$", true),
            pattern(
                r"^git -C [.]agent-task/fixture -c commit[.]gpgsign=false commit -m two$",
                true,
            ),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "worktree",
            "always_allow",
            0,
            "git:commit",
            "git",
            "0-worktree",
            "direct",
            "a",
            "git -c commit.gpgsign=false -C .agent-task commit -m one",
            None,
        ),
        entry(
            "fixture",
            "always_allow",
            2,
            "git:commit",
            "git",
            "1-fixture",
            "direct",
            "a",
            "git -C .agent-task/fixture -c commit.gpgsign=false commit -m two",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn stops_repository_scope_inference_at_unapproved_git_config() {
    let settings = settings(
        vec![
            pattern(r"^git -c core[.]foo=false -C [.]agent-task commit$", true),
            pattern(r"^bar run$", true),
            pattern(r"^git --no-pager$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "unapproved-config",
            "always_allow",
            0,
            "git:root",
            "git",
            "a",
            "direct",
            "a",
            "git -c core.foo=false -C .agent-task commit",
            None,
        ),
        entry(
            "general-root",
            "always_allow",
            2,
            "git:root",
            "git",
            "c",
            "direct",
            "a",
            "git --no-pager",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy")
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.findings.len(), 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn permits_git_discovery_to_direct_separation_through_manifested_git_entries() {
    let settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^git status$", true),
            pattern(r"^git commit -m one$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "commit-discovery",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "discovery",
            "a",
            "git commit --help",
            Some(&["git commit --help"]),
        ),
        entry(
            "status-direct",
            "always_allow",
            1,
            "git:status",
            "git",
            "b",
            "direct",
            "a",
            "git status",
            None,
        ),
        entry(
            "commit-direct",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "direct",
            "a",
            "git commit -m one",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn permits_git_discovery_to_wrapped_separation_through_manifested_git_entries() {
    let settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^git status$", true),
            pattern(r"^xargs git commit -m one$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "commit-discovery",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "discovery",
            "a",
            "git commit --help",
            Some(&["git commit --help"]),
        ),
        entry(
            "status-direct",
            "always_allow",
            1,
            "git:status",
            "git",
            "b",
            "direct",
            "a",
            "git status",
            None,
        ),
        entry(
            "commit-wrapped",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "wrapped",
            "a",
            "xargs git commit -m one",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.findings.is_empty());
}

#[test]
fn rejects_git_direct_to_direct_and_discovery_to_discovery_gaps() {
    let direct_settings = settings(
        vec![
            pattern(r"^git commit -m one$", true),
            pattern(r"^git status$", true),
            pattern(r"^git commit -m two$", true),
        ],
        vec![],
        vec![],
    );
    let direct_manifest = manifest(vec![
        entry(
            "commit-one",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "direct",
            "a",
            "git commit -m one",
            None,
        ),
        entry(
            "status",
            "always_allow",
            1,
            "git:status",
            "git",
            "b",
            "direct",
            "a",
            "git status",
            None,
        ),
        entry(
            "commit-two",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "direct",
            "a",
            "git commit -m two",
            None,
        ),
    ]);
    let discovery_settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^git status$", true),
            pattern(r"^git commit -h$", true),
        ],
        vec![],
        vec![],
    );
    let discovery_manifest = manifest(vec![
        entry(
            "commit-help",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "discovery",
            "a",
            "git commit --help",
            Some(&["git commit --help"]),
        ),
        entry(
            "status",
            "always_allow",
            1,
            "git:status",
            "git",
            "b",
            "direct",
            "a",
            "git status",
            None,
        ),
        entry(
            "commit-h",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "discovery",
            "a",
            "git commit -h",
            Some(&["git commit -h"]),
        ),
    ]);

    for (settings, manifest) in [
        (direct_settings, direct_manifest),
        (discovery_settings, discovery_manifest),
    ] {
        let report = audit_json(&settings, &manifest).expect("Audit input must be valid");
        assert_eq!(report.finding_count, 2);
        assert!(
            report
                .findings
                .iter()
                .all(|finding| { finding.reason.contains("does not completely occupy") })
        );
    }
}

#[test]
fn rejects_omitted_excluded_and_non_git_intervening_indexes() {
    let git_settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^git status$", true),
            pattern(r"^git commit -m one$", true),
        ],
        vec![],
        vec![],
    );
    let endpoints = vec![
        entry(
            "commit-discovery",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "discovery",
            "a",
            "git commit --help",
            Some(&["git commit --help"]),
        ),
        entry(
            "commit-direct",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "direct",
            "a",
            "git commit -m one",
            None,
        ),
    ];
    let omitted = manifest_for("git", endpoints.clone(), vec![]);
    let omitted_error =
        audit_json(&git_settings, &omitted).expect_err("Omitted Git candidate must fail coverage");
    assert!(omitted_error.contains("1 missing candidate position"));
    assert!(omitted_error.contains("always_allow[1]"));

    let excluded = manifest_for(
        "git",
        endpoints.clone(),
        vec![excluded_candidate(
            "always_allow",
            1,
            "git:status",
            "git status",
            "semantic owner intentionally excluded",
        )],
    );
    let excluded_error = audit_json(&git_settings, &excluded)
        .expect_err("Same-owner lexical candidate must not be excluded");
    assert!(excluded_error.contains("infers to the manifest inventory owner"));

    let non_git_settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^foo run$", true),
            pattern(r"^git commit -m one$", true),
        ],
        vec![],
        vec![],
    );
    let non_git = manifest_for("git", endpoints, vec![]);
    let non_git_report = audit_json(&non_git_settings, &non_git)
        .expect("Non-Git intervening entry is outside the lexical inventory");
    assert_eq!(non_git_report.finding_count, 2);
}

#[test]
fn rejects_declared_git_metadata_without_independent_git_inference() {
    let settings = settings(
        vec![
            pattern(r"^git commit --help$", true),
            pattern(r"^foo run git marker$", true),
            pattern(r"^git commit -m one$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest(vec![
        entry(
            "commit-discovery",
            "always_allow",
            0,
            "git:commit",
            "git",
            "a",
            "discovery",
            "a",
            "git commit --help",
            Some(&["git commit --help"]),
        ),
        entry(
            "declared-git",
            "always_allow",
            1,
            "git:status",
            "git",
            "b",
            "direct",
            "a",
            "foo run git marker",
            None,
        ),
        entry(
            "commit-direct",
            "always_allow",
            2,
            "git:commit",
            "git",
            "c",
            "direct",
            "a",
            "git commit -m one",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(report.finding_count >= 3);
    assert!(report.findings.iter().any(|finding| {
        finding.id == "declared-git" && finding.reason.contains("declared owner differs")
    }));
    assert!(
        report
            .findings
            .iter()
            .any(|finding| { finding.reason.contains("does not completely occupy") })
    );
}

#[test]
fn groups_completeness_endpoints_by_independently_inferred_owner() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^bar run$", true),
            pattern(r"^foo two$", true),
        ],
        vec![],
        vec![],
    );
    let manifest = manifest_for(
        "foo",
        vec![
            entry(
                "foo-one",
                "always_allow",
                0,
                "wrong-one",
                "a",
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
                "wrong-two",
                "b",
                "direct",
                "direct",
                "a",
                "foo two",
                None,
            ),
        ],
        vec![],
    );

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding.reason.contains("declared owner differs")
            && finding
                .reason
                .contains("owner-scope group does not completely occupy")
    }));
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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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
            pattern(r"^foo --version$", true),
            pattern(r"^foo (?:--version|deny)$", true),
        ],
    );
    let manifest = manifest(vec![
        entry(
            "confirm-discovery",
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
            "confirm-direct",
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
            "deny-discovery",
            "always_deny",
            0,
            "foo",
            "foo",
            "section",
            "discovery",
            "a",
            "foo --version",
            Some(&["foo --version"]),
        ),
        entry(
            "deny-direct",
            "always_deny",
            1,
            "foo",
            "foo",
            "section",
            "direct",
            "a",
            "foo deny",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

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

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(report.finding_count, 2);
    assert!(report.findings.iter().all(|finding| {
        finding
            .reason
            .contains("owner-scope group does not completely occupy `always_allow` index 1")
    }));
}

#[test]
fn bounds_finding_output_and_never_leaks_patterns_or_witnesses() {
    let fixture = Fixture::new();
    let mut patterns = Vec::new();
    let mut entries = Vec::new();

    for index in 0..12 {
        let witness = format!("foo private-witness-{index:02}");
        let regex = format!("^foo private-regex-{index:02}$");
        let id = format!("entry-{index:02}");
        patterns.push(pattern(&regex, index != 0));
        entries.push(direct_entry(&id, "always_allow", index, "foo", &witness));
    }

    let settings = settings(patterns, vec![], vec![]);
    let manifest = bind_manifest(&settings, &manifest(entries));
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write("manifest.json", &manifest);
    let (status, stdout, stderr) = run_files(&settings_path, &manifest_path);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 findings across 12 entries"));
    assert_eq!(stderr.matches("entry-").count(), 10);
    assert!(stderr.contains("entry-00"));
    assert!(stderr.contains("entry-09"));
    assert!(!stderr.contains("entry-10"));
    assert!(stderr.contains("2 additional findings omitted"));
    assert!(!stderr.contains("private-regex"));
    assert!(!stderr.contains("private-witness"));
}

#[test]
fn bounds_finding_ids_before_writing_diagnostics() {
    let fixture = Fixture::new();
    let settings = settings(vec![pattern(r"^foo$", true)], vec![], vec![]);
    let private_suffix = "private-id-suffix";
    let long_id = format!("{}{private_suffix}", "x".repeat(100));
    let manifest = manifest(vec![direct_entry(
        &long_id,
        "always_allow",
        0,
        "foo",
        "foo run",
    )]);
    let manifest = bind_manifest(&settings, &manifest);
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write("manifest.json", &manifest);

    let (status, stdout, stderr) = run_files(&settings_path, &manifest_path);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("{}…", "x".repeat(80))));
    assert!(!stderr.contains(private_suffix));
    assert!(!stderr.contains(&long_id));
}

/// A canonical fixture root, so closure resolution never traverses a symlinked temporary path
fn graph_root(fixture: &Fixture) -> PathBuf {
    fs::canonicalize(fixture.path("")).expect("Fixture root must resolve to a real directory")
}

fn binding_for(settings: &str, entries: Value, positions: Value) -> String {
    json!({
        "settings_sha256": helper::settings_sha256(settings),
        "entries": entries,
        "positions": positions,
    })
    .to_string()
}

#[test]
fn applies_a_manifest_binding_to_moved_entry_positions() {
    let fixture = Fixture::new();
    // The reviewed manifest audits `always_allow[0]`, but the settings it now runs against hold the
    // owned pattern at `always_allow[1]`
    let settings = settings(
        vec![
            pattern(r"^unrelated run$", true),
            pattern(r"^foo run$", true),
        ],
        vec![],
        vec![],
    );
    let reviewed = manifest(vec![direct_entry(
        "foo-direct",
        "always_allow",
        0,
        "foo",
        "foo run",
    )]);
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write("manifest.json", &reviewed);
    let binding_path = fixture.write(
        "binding.json",
        &binding_for(
            &settings,
            json!([{"id": "foo-direct", "bucket": "always_allow", "index": 1}]),
            json!([]),
        ),
    );

    let (unbound_status, _, unbound_stderr) = run_files(&settings_path, &manifest_path);
    assert_ne!(unbound_status, 0, "the reviewed manifest must be stale");
    assert!(!unbound_stderr.is_empty());

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--binding"),
        binding_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 0, "{stderr}");
    assert!(
        stdout.contains("1 entry") || stdout.contains("1 entries"),
        "{stdout}"
    );
}

#[test]
fn refuses_a_manifest_binding_that_omits_an_audited_entry() {
    let fixture = Fixture::new();
    let (settings, reviewed) = valid_settings_and_manifest();
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write("manifest.json", &reviewed);
    let binding_path = fixture.write(
        "binding.json",
        &binding_for(&settings, json!([]), json!([])),
    );

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--binding"),
        binding_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 2, "{stderr}");
    assert!(stdout.is_empty());
    assert!(stderr.contains("does not rebind entry"), "{stderr}");
}

#[test]
fn refuses_a_manifest_binding_without_a_manifest() {
    let fixture = Fixture::new();
    let (settings, _) = valid_settings_and_manifest();
    let settings_path = fixture.write("settings.json", &settings);
    let binding_path = fixture.write(
        "binding.json",
        &binding_for(&settings, json!([]), json!([])),
    );

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--owner"),
        OsString::from("foo"),
        OsString::from("--binding"),
        binding_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 2, "{stderr}");
    assert!(stdout.is_empty());
    assert!(
        stderr.contains("`--binding` is valid only with"),
        "{stderr}"
    );
}

#[test]
fn refuses_a_malformed_manifest_binding() {
    let fixture = Fixture::new();
    let (settings, reviewed) = valid_settings_and_manifest();
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write("manifest.json", &bind_manifest(&settings, &reviewed));
    // A path overlay is not a manifest binding, so the strict schema must reject it
    let binding_path = fixture.write("binding.json", &json!({"paths": []}).to_string());

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--binding"),
        binding_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 2, "{stderr}");
    assert!(stdout.is_empty());
    assert!(stderr.contains("Manifest binding"), "{stderr}");
}

#[test]
fn refuses_result_out_without_a_graph_root() {
    let fixture = Fixture::new();
    let (settings_path, manifest_path) = write_valid_files(&fixture);

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--manifest"),
        manifest_path.as_os_str().to_owned(),
        OsString::from("--result-out"),
        fixture.path("result.json").as_os_str().to_owned(),
    ]);

    assert_eq!(status, 2, "{stderr}");
    assert!(stdout.is_empty());
    assert!(
        stderr.contains("`--result-out` requires `--graph-root`"),
        "{stderr}"
    );
    assert!(!fixture.path("result.json").exists());
}

#[test]
fn records_hash_bound_evidence_for_a_manifest_audit() {
    let fixture = Fixture::new();
    let (settings_path, manifest_path) = write_valid_files(&fixture);
    let root = graph_root(&fixture);
    let result_path = root.join("result.json");

    let (status, _, stderr) = run(vec![
        OsString::from("--settings"),
        root.join("settings.json").into_os_string(),
        OsString::from("--manifest"),
        root.join("manifest.json").into_os_string(),
        OsString::from("--graph-root"),
        root.clone().into_os_string(),
        OsString::from("--result-out"),
        result_path.clone().into_os_string(),
    ]);

    assert_eq!(status, 0, "{stderr}");
    let result: Value = serde_json::from_slice(&fs::read(&result_path).unwrap())
        .expect("Recorded evidence must parse");
    assert_eq!(result["kind"], json!("owner_audit"));
    assert_eq!(result["outcome"], json!("passed"));
    assert_eq!(result["bound_inputs"]["inventory_owner"], json!("foo"));

    // The recorded closure must cover exactly the manifest and settings the audit read
    let recorded: Vec<String> = result["bound_inputs"]["input_closure"]["records"]
        .as_array()
        .expect("The closure must record its inputs")
        .iter()
        .map(|record| record["path"].as_str().unwrap().to_owned())
        .collect();
    assert!(
        recorded.contains(&"manifest.json".to_owned()),
        "{recorded:?}"
    );
    assert!(
        recorded.contains(&"settings.json".to_owned()),
        "{recorded:?}"
    );

    // Rewriting a recorded input must make the evidence stale
    let recomputed = {
        let mut builder = helper::permission_patterns::InputClosureBuilder::new(&root).unwrap();
        helper::permission_patterns::resolve_audit_closure(
            &mut builder,
            &root.join("manifest.json"),
            &root.join("settings.json"),
            None,
        )
        .unwrap();
        builder.finish().unwrap()
    };
    let declared: helper::permission_patterns::InputClosure =
        serde_json::from_value(result["bound_inputs"]["input_closure"].clone()).unwrap();
    assert!(helper::permission_patterns::verify_input_closure(&declared, &recomputed, 10).is_ok());

    fs::write(&manifest_path, b"{}").unwrap();
    let stale = {
        let mut builder = helper::permission_patterns::InputClosureBuilder::new(&root).unwrap();
        let _ = helper::permission_patterns::resolve_audit_closure(
            &mut builder,
            &root.join("manifest.json"),
            &root.join("settings.json"),
            None,
        );
        builder.finish().unwrap()
    };
    assert!(helper::permission_patterns::verify_input_closure(&declared, &stale, 10).is_err());
    let _ = settings_path;
}

#[test]
fn refuses_overwriting_an_existing_result() {
    let fixture = Fixture::new();
    let (_, _) = write_valid_files(&fixture);
    let root = graph_root(&fixture);
    let result_path = fixture.write("result.json", "existing");

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        root.join("settings.json").into_os_string(),
        OsString::from("--manifest"),
        root.join("manifest.json").into_os_string(),
        OsString::from("--graph-root"),
        root.into_os_string(),
        OsString::from("--result-out"),
        result_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 2, "{stderr}");
    assert!(stdout.is_empty());
    assert_eq!(fs::read(&result_path).unwrap(), b"existing");
}

#[test]
fn verifies_a_zero_owner_manifest_for_a_fully_removed_owner() {
    let fixture = Fixture::new();
    let settings = settings(vec![pattern(r"^bar run$", true)], vec![], vec![]);
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write(
        "zero-owner.json",
        &json!({
            "settings_sha256": helper::settings_sha256(&settings),
            "inventory_owner": "foo"
        })
        .to_string(),
    );

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--zero-owner-manifest"),
        manifest_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 0, "{stderr}");
    assert!(stdout.contains("foo"), "{stdout}");
}

#[test]
fn refuses_a_zero_owner_manifest_that_leaves_an_unclassified_hit() {
    let fixture = Fixture::new();
    let settings = settings(vec![pattern(r"^foo run$", true)], vec![], vec![]);
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write(
        "zero-owner.json",
        &json!({
            "settings_sha256": helper::settings_sha256(&settings),
            "inventory_owner": "foo"
        })
        .to_string(),
    );

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--zero-owner-manifest"),
        manifest_path.as_os_str().to_owned(),
    ]);

    assert_ne!(status, 0, "{stdout}");
    assert!(!stderr.is_empty(), "{stdout}");
}

#[test]
fn refuses_a_zero_owner_manifest_bound_to_other_settings() {
    let fixture = Fixture::new();
    let settings = settings(vec![pattern(r"^bar run$", true)], vec![], vec![]);
    let settings_path = fixture.write("settings.json", &settings);
    let manifest_path = fixture.write(
        "zero-owner.json",
        &json!({
            "settings_sha256": ZERO_SHA256,
            "inventory_owner": "foo"
        })
        .to_string(),
    );

    let (status, stdout, stderr) = run(vec![
        OsString::from("--settings"),
        settings_path.as_os_str().to_owned(),
        OsString::from("--zero-owner-manifest"),
        manifest_path.as_os_str().to_owned(),
    ]);

    assert_eq!(status, 1, "{stderr}");
    assert!(stdout.is_empty());
    assert!(
        stderr.contains("does not bind the exact candidate settings bytes"),
        "{stderr}"
    );
}

/// A pattern whose source hides its executable token still owns its position. `fo[o]` never spells
/// `foo` contiguously, so the lexical scan cannot see it while the compiled regex still matches
fn invisible_entry(id: &str, index: usize, pattern_sort_key: &str, witness: &str) -> Value {
    let mut entry = entry(
        id,
        "always_allow",
        index,
        "foo",
        "foo",
        "invented",
        "direct",
        pattern_sort_key,
        witness,
        None,
    );
    entry
        .as_object_mut()
        .expect("Entry must be an object")
        .insert("lexically_invisible".to_owned(), json!(true));
    entry
}

#[test]
fn a_lexically_invisible_member_occupies_its_position() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^fo[o] two$", true),
            pattern(r"^foo three$", true),
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
            "invented",
            "direct",
            "a",
            "foo one",
            None,
        ),
        invisible_entry("foo-two", 1, "b", "foo two"),
        entry(
            "foo-three",
            "always_allow",
            2,
            "foo",
            "foo",
            "invented",
            "direct",
            "c",
            "foo three",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert_eq!(
        report.finding_count, 0,
        "A hidden member that occupies index 1 must not leave a span gap: {:?}",
        report.findings
    );
}

#[test]
fn a_lexically_invisible_declaration_refuses_a_visible_candidate() {
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
            "invented",
            "direct",
            "a",
            "foo one",
            None,
        ),
        invisible_entry("foo-two", 1, "b", "foo two"),
    ]);

    let error = audit_json(&settings, &manifest)
        .expect_err("A visible pattern must not be declared lexically invisible");

    assert!(
        error.contains("lexically invisible entries that the inventory recomputes as candidates"),
        "{error}"
    );
    assert!(
        error.contains("1 recomputed entry and 0 omitted from the reported positions"),
        "{error}"
    );
    assert!(error.contains("always_allow[1]"), "{error}");
}

#[test]
fn a_lexically_invisible_refusal_bounds_positions_and_counts_every_entry() {
    let settings = settings(
        (0..12)
            .map(|index| pattern(&format!(r"^foo case-{index}$"), true))
            .collect(),
        vec![],
        vec![],
    );
    let manifest = manifest(
        (0..12)
            .map(|index| {
                invisible_entry(
                    &format!("foo-{index}"),
                    index,
                    &format!("{index:02}"),
                    &format!("foo case-{index}"),
                )
            })
            .collect(),
    );

    let error = audit_json(&settings, &manifest)
        .expect_err("Visible patterns declared lexically invisible must be refused");

    assert!(
        error.contains("12 recomputed entries and 2 omitted from the reported positions"),
        "{error}"
    );
    assert_eq!(error.matches("always_allow[").count(), 10, "{error}");
    assert!(error.contains("always_allow[9]"), "{error}");
    assert!(!error.contains("always_allow[10]"), "{error}");
}

#[test]
fn a_lexically_invisible_declaration_still_fails_an_outside_owner_position() {
    let settings = settings(
        vec![
            pattern(r"^foo one$", true),
            pattern(r"^bar run$", true),
            pattern(r"^foo three$", true),
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
            "invented",
            "direct",
            "a",
            "foo one",
            None,
        ),
        invisible_entry("foo-two", 1, "b", "foo two"),
        entry(
            "foo-three",
            "always_allow",
            2,
            "foo",
            "foo",
            "invented",
            "direct",
            "c",
            "foo three",
            None,
        ),
    ]);

    let report = audit_json(&settings, &manifest).expect("Audit input must be valid");

    assert!(
        report.findings.iter().any(|finding| finding
            .reason
            .contains("pattern does not match its witness")),
        "Claiming a position another owner holds must still fail: {:?}",
        report.findings
    );
}
