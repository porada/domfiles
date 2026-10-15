use serde_json::{Value, json};
use std::{
    env,
    ffi::OsStr,
    fs,
    os::unix::{
        ffi::OsStrExt,
        fs::{PermissionsExt, symlink},
    },
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const BINARY: &str = env!("CARGO_BIN_EXE_subagent-signal");
const CARGO_STUB: &str = concat!(
    "#!/bin/sh\n",
    "pwd -P > \"${0%/*}/cargo-directory\"\n",
    "printf '%s\\n' \"$@\" > \"${0%/*}/cargo-arguments\"\n",
    "printf '%s\\n' 'cargo: building' >&2\n",
    "printf '{\"reason\":\"compiler-artifact\",\"target\":{\"name\":\"%s\"},\"executable\":\"%s\"}\\n' \"$3\" \"$(cat \"${0%/*}/cargo-executable\")\"\n",
    "exit \"$(cat \"${0%/*}/cargo-status\")\"\n",
);
// A lookup is counted only after the listing is read, so a test may replace the listing as soon
// as it sees the count change
const CLAUDE_STUB: &str = concat!(
    "#!/bin/sh\n",
    "[ \"$*\" = 'agents --json --all' ] || exit 64\n",
    "cat \"${0%/*}/listing.json\"\n",
    "status=$?\n",
    "printf x >> \"${0%/*}/calls\"\n",
    "exit \"$status\"\n",
);
const LAUNCHER: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/home/.local/bin/subagent-signal"
);
const NAME: &str = "subagent-signal";
const OTHER_UUID: &str = "ffffffff-ffff-ffff-ffff-ffffffffffff";
const UUID: &str = "0123abcd-4567-89ab-cdef-0123456789ab";

static FIXTURES: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    bin: PathBuf,
    home: PathBuf,
    project: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "{NAME}-test-{}-{}",
            process::id(),
            FIXTURES.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("bin");
        let home = root.join("home é");
        let project = home.join(".claude/projects/-Users-dom-Projects-a b");

        // A previous run that was interrupted before cleanup may have left state behind
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&bin).expect("The stub directory must be created");
        fs::create_dir_all(&project).expect("The project directory must be created");

        for (name, contents) in [("cargo", CARGO_STUB), ("claude", CLAUDE_STUB)] {
            let stub = bin.join(name);

            fs::write(&stub, contents).expect("The stub must be written");
            fs::set_permissions(&stub, fs::Permissions::from_mode(0o755))
                .expect("The stub must be executable");
        }

        Self {
            bin,
            home,
            project,
            root,
        }
    }

    fn transcript(&self, uuid: &str) -> PathBuf {
        self.project.join(format!("{uuid}.jsonl"))
    }

    fn write(&self, uuid: &str, records: &[Value]) {
        let lines: String = records.iter().map(|record| format!("{record}\n")).collect();

        fs::write(self.transcript(uuid), lines).expect("The transcript must be written");
    }

    fn listing(&self, rows: &Value) {
        fs::write(self.bin.join("listing.json"), rows.to_string())
            .expect("The listing must be written");
    }

    fn rollout(&self, uuid: &str, day: u32, contents: &str) -> PathBuf {
        let directory = self.home.join(format!(".codex/sessions/2026/10/{day:02}"));
        let path = directory.join(format!("rollout-2026-10-{day:02}T18-00-00-{uuid}.jsonl"));

        fs::create_dir_all(&directory).expect("The date directory must be created");
        fs::write(&path, contents).expect("The rollout must be written");
        path
    }

    fn write_rollout(&self, uuid: &str, day: u32, records: &[Value]) -> PathBuf {
        let lines: String = records.iter().map(|record| format!("{record}\n")).collect();

        self.rollout(uuid, day, &lines)
    }

    fn subagent(&self, uuid: &str, name: &str, contents: &str) -> PathBuf {
        let directory = self.transcript(uuid).with_extension("").join("subagents");
        let path = directory.join(name);

        fs::create_dir_all(&directory).expect("The subagent directory must be created");
        fs::write(&path, contents).expect("The subagent transcript must be written");
        path
    }

    fn calls(&self) -> usize {
        fs::read(self.bin.join("calls")).map_or(0, |calls| calls.len())
    }

    fn cargo(&self, status: u8, executable: &str) {
        fs::write(self.bin.join("cargo-status"), status.to_string())
            .expect("The Cargo status must be written");
        fs::write(self.bin.join("cargo-executable"), executable)
            .expect("The Cargo executable must be written");
    }

    fn recorded(&self, name: &str) -> String {
        fs::read_to_string(self.bin.join(name)).expect("The stub must record its invocation")
    }

    fn run(&self, directory: Option<&Path>, arguments: &[&str]) -> (Option<i32>, String, String) {
        execute(
            BINARY,
            &format!("{}:/usr/bin:/bin", self.bin.display()),
            &self.home,
            directory,
            arguments,
        )
    }

    fn observe_raw(&self, sessions: &[(&str, usize)], extra: &[&str]) -> String {
        let values: Vec<String> = sessions
            .iter()
            .map(|(uuid, since)| format!("{since}:{uuid}"))
            .collect();
        let mut arguments: Vec<&str> = values
            .iter()
            .flat_map(|value| ["--claude", value.as_str()])
            .collect();

        arguments.extend_from_slice(extra);
        if !extra.contains(&"--wait") {
            arguments.extend_from_slice(&["--wait", "0"]);
        }

        let (status, stdout, stderr) = self.run(None, &arguments);

        assert_eq!(status, Some(0), "{arguments:?}: {stderr}");
        assert!(stderr.is_empty(), "{arguments:?}: {stderr}");
        assert!(
            stdout.ends_with('\n') && stdout.matches('\n').count() == 1,
            "{stdout}"
        );
        stdout
    }

    fn observe_all(&self, sessions: &[(&str, usize)], extra: &[&str]) -> Value {
        let output: Value = serde_json::from_str(&self.observe_raw(sessions, extra))
            .expect("The output must be JSON");

        assert!(output["observedAt"].is_u64(), "{output}");
        output
    }

    fn observe(&self, since: usize, extra: &[&str]) -> Value {
        let mut sessions = self.observe_all(&[(UUID, since)], extra)["sessions"].take();

        sessions[0].take()
    }

    fn observe_with(&self, arguments: &[&str]) -> Value {
        let mut arguments = arguments.to_vec();

        if !arguments.contains(&"--wait") {
            arguments.extend_from_slice(&["--wait", "0"]);
        }

        let (status, stdout, stderr) = self.run(None, &arguments);

        assert_eq!(status, Some(0), "{arguments:?}: {stderr}");
        assert!(stderr.is_empty(), "{arguments:?}: {stderr}");
        serde_json::from_str(&stdout).expect("The output must be JSON")
    }

    fn refuse(&self, arguments: &[&str]) -> String {
        let (status, stdout, stderr) = self.run(None, arguments);

        assert_eq!(status, Some(2), "{arguments:?}: {stderr}");
        assert!(stdout.is_empty(), "{arguments:?}: {stdout}");
        assert!(
            stderr.starts_with(&format!("{NAME}: "))
                && stderr.ends_with('\n')
                && stderr.matches('\n').count() == 1,
            "{arguments:?}: {stderr}"
        );
        stderr
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn execute(
    program: &str,
    path: &str,
    home: &Path,
    directory: Option<&Path>,
    arguments: &[&str],
) -> (Option<i32>, String, String) {
    let mut command = Command::new(program);

    command.args(arguments).env("HOME", home).env("PATH", path);

    if let Some(directory) = directory {
        command.current_dir(directory);
    }

    let output = command.output().expect("The command must start");

    (
        output.status.code(),
        String::from_utf8(output.stdout).expect("Standard output must be UTF-8"),
        String::from_utf8(output.stderr).expect("Standard error must be UTF-8"),
    )
}

fn help_section<'a>(help: &'a str, heading: &str) -> Vec<&'a str> {
    help.lines()
        .skip_while(|line| *line != heading)
        .skip(1)
        .take_while(|line| !line.is_empty())
        .collect()
}

fn at(second: u32) -> String {
    format!("2026-10-09T18:00:{second:02}.000Z")
}

fn millis(time: SystemTime) -> u64 {
    u64::try_from(
        time.duration_since(UNIX_EPOCH)
            .expect("The time must follow the Unix epoch")
            .as_millis(),
    )
    .expect("The time must fit in milliseconds")
}

fn set_modified(path: &Path, milliseconds: u64) {
    fs::File::options()
        .write(true)
        .open(path)
        .and_then(|file| file.set_modified(UNIX_EPOCH + Duration::from_millis(milliseconds)))
        .expect("The modification time must be set");
}

fn bookkeeping() -> Value {
    json!({ "type": "ai-title", "aiTitle": "Review" })
}

fn prompt(second: u32) -> Value {
    json!({
        "type": "user",
        "origin": { "kind": "human" },
        "message": { "role": "user", "content": "Review the change" },
        "timestamp": at(second),
    })
}

fn reply(id: &str, text: &str, stop_reason: &str, second: u32) -> Value {
    json!({
        "type": "assistant",
        "message": {
            "id": id,
            "model": "claude-opus-5-5",
            "stop_reason": stop_reason,
            "content": [{ "type": "text", "text": text }],
        },
        "timestamp": at(second),
    })
}

fn thinking(id: &str, second: u32) -> Value {
    json!({
        "type": "assistant",
        "message": {
            "id": id,
            "model": "claude-opus-5-5",
            "stop_reason": "end_turn",
            "content": [{ "type": "thinking", "thinking": "Considering the change" }],
        },
        "timestamp": at(second),
    })
}

fn synthetic(text: &str, error: Option<&str>, second: u32) -> Value {
    json!({
        "type": "assistant",
        "message": {
            "id": format!("synthetic-{second}"),
            "model": "<synthetic>",
            "stop_reason": "stop_sequence",
            "content": [{ "type": "text", "text": text }],
        },
        "error": error,
        "isApiErrorMessage": error.is_some(),
        "timestamp": at(second),
    })
}

fn call(id: &str, input: &Value, second: u32) -> Value {
    json!({
        "type": "assistant",
        "message": {
            "id": format!("message-{id}"),
            "model": "claude-opus-5-5",
            "stop_reason": "tool_use",
            "content": [{ "type": "tool_use", "id": id, "name": "Bash", "input": input }],
        },
        "timestamp": at(second),
    })
}

fn result(id: &str, is_error: bool, second: u32) -> Value {
    json!({
        "type": "user",
        "message": {
            "role": "user",
            "content": [{
                "type": "tool_result",
                "tool_use_id": id,
                "is_error": is_error,
                "content": "output",
            }],
        },
        "timestamp": at(second),
    })
}

fn turn_end(second: u32) -> Value {
    json!({ "type": "system", "subtype": "turn_duration", "durationMs": 1000, "timestamp": at(second) })
}

fn sidechain(mut record: Value) -> Value {
    record["isSidechain"] = json!(true);
    record
}

fn compaction(second: u32) -> Value {
    json!({ "type": "system", "subtype": "compact_boundary", "timestamp": at(second) })
}

fn notification(id: &str, second: u32) -> Value {
    json!({
        "type": "queue-operation",
        "operation": "enqueue",
        "content": format!(
            "<task-notification>\n<task-id>task</task-id>\n<tool-use-id>{id}</tool-use-id>\n<status>completed</status>\n</task-notification>"
        ),
        "timestamp": at(second),
    })
}

fn row(uuid: &str, state: &str, status: Option<&str>, waiting_for: Option<&str>) -> Value {
    json!({ "id": &uuid[..8], "sessionId": uuid, "state": state, "status": status, "waitingFor": waiting_for })
}

fn session(state: &str, status: Option<&str>, waiting_for: Option<&str>) -> Value {
    json!([
        row(OTHER_UUID, "working", Some("busy"), None),
        row(UUID, state, status, waiting_for),
    ])
}

fn completed_turn() -> Vec<Value> {
    vec![
        bookkeeping(),
        prompt(1),
        call("call", &json!({ "command": "git diff" }), 2),
        result("call", false, 3),
        thinking("final", 4),
        reply("final", "No findings", "end_turn", 5),
        turn_end(6),
        bookkeeping(),
    ]
}

fn session_meta(id: &str, root: &str, second: u32) -> Value {
    json!({
        "type": "session_meta",
        "payload": { "id": id, "session_id": root },
        "timestamp": at(second),
    })
}

fn event(payload: &Value, second: u32) -> Value {
    json!({ "type": "event_msg", "payload": payload, "timestamp": at(second) })
}

fn turn_started(second: u32) -> Value {
    event(&json!({ "type": "task_started" }), second)
}

fn turn_complete(message: Option<&str>, error: Option<&str>, second: u32) -> Value {
    event(
        &json!({
            "type": "task_complete",
            "last_agent_message": message,
            "error": error.map(|message| json!({ "message": message, "codex_error_info": "other" })),
        }),
        second,
    )
}

fn turn_aborted(reason: &str, second: u32) -> Value {
    event(&json!({ "type": "turn_aborted", "reason": reason }), second)
}

fn reasoning(second: u32) -> Value {
    json!({ "type": "response_item", "payload": { "type": "reasoning" }, "timestamp": at(second) })
}

fn codex_turn(reply: &str, start: u32) -> Vec<Value> {
    vec![
        turn_started(start),
        reasoning(start + 1),
        turn_complete(Some(reply), None, start + 2),
    ]
}

#[test]
fn prints_help_alone() {
    let fixture = Fixture::new();
    let (status, stdout, stderr) = fixture.run(None, &["--help"]);

    assert_eq!(status, Some(0));
    assert!(stdout.starts_with("Usage:\n"));
    assert!(stderr.is_empty());
}

#[test]
fn documents_exactly_the_accepted_options() {
    let fixture = Fixture::new();
    let (_, help, _) = fixture.run(None, &["--help"]);
    let usage = help_section(&help, "Usage:");
    let mut usage_options: Vec<&str> = usage[0]
        .split_whitespace()
        .map(|token| token.trim_matches(['[', ']', '(', ')']))
        .filter(|token| token.starts_with("--"))
        .collect();
    let documented: Vec<&str> = help_section(&help, "Options:")
        .into_iter()
        .filter_map(|line| line.split_whitespace().next())
        .collect();

    usage_options.sort_unstable();

    assert_eq!(usage.len(), 2);
    assert!(
        usage
            .iter()
            .all(|line| line.split_whitespace().next() == Some(NAME))
    );
    assert!(documented.is_sorted());
    assert_eq!(documented, usage_options);

    fixture.write(UUID, &[prompt(1)]);
    assert_eq!(fixture.observe(1, &["--wait", "0"])["state"], "working");
}

#[test]
fn rejects_invalid_invocations() {
    let fixture = Fixture::new();
    let session = format!("0:{UUID}");
    let upper = format!("0:{}", UUID.to_ascii_uppercase());
    let invalid_values = [
        UUID.to_owned(),
        format!("0:{}", &UUID[1..]),
        format!("0:{}", UUID.replace('-', "_")),
        format!("0:{UUID}.jsonl"),
        format!(":{UUID}"),
        format!(" 1:{UUID}"),
        format!("+1:{UUID}"),
        format!("-1:{UUID}"),
        format!("1.5:{UUID}"),
        format!("1e3:{UUID}"),
    ];
    let mut cases: Vec<Vec<&str>> = vec![
        vec![],
        vec!["-h"],
        vec!["--help", "--claude", &session],
        vec!["--claude", &session, "--help"],
        vec!["--wait", "0"],
        vec!["--claude"],
        vec!["--claude", &session, "--claude", &session],
        vec!["--claude", &session, "--claude", &upper],
        vec!["--claude", &session, "--verbose"],
        vec!["--claude", &session, "--wait", "0", "--wait", "0"],
        vec!["--claude", &session, "--wait", "x"],
        vec!["--claude", &session, "--wait"],
        vec!["--claude", &session, "--codex", &session],
        vec!["--codex", &session, "--codex", &upper],
        vec!["--codex", UUID],
        vec!["--codex"],
    ];

    for value in &invalid_values {
        cases.push(vec!["--claude", value]);
    }
    for arguments in &cases {
        fixture.refuse(arguments);
    }

    assert_eq!(fixture.calls(), 0);
}

#[test]
fn rejects_option_values_that_are_not_utf_8() {
    let fixture = Fixture::new();
    let output = Command::new(BINARY)
        .arg("--claude")
        .arg(OsStr::from_bytes(b"0:\xff"))
        .env("HOME", &fixture.home)
        .env("PATH", format!("{}:/usr/bin:/bin", fixture.bin.display()))
        .output()
        .expect("The command must start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("{NAME}: The `--claude` value must be valid UTF-8\n")
    );
}

#[test]
fn accepts_the_documented_maximum_wait_and_rejects_larger_values() {
    let fixture = Fixture::new();
    let (_, help, _) = fixture.run(None, &["--help"]);
    let maximum: u64 = help_section(&help, "Options:")
        .into_iter()
        .find(|line| line.trim_start().starts_with("--wait "))
        .and_then(|line| line.split_once("from `0` to `"))
        .and_then(|(_, bound)| bound.split('`').next())
        .expect("Help must document the wait bound")
        .parse()
        .expect("The wait bound must be an integer");
    let accepted = maximum.to_string();
    let rejected = (maximum + 1).to_string();
    let value = format!("0:{UUID}");

    assert!(help.contains(&format!("Defaults to `{maximum}`")));

    fixture.write(UUID, &completed_turn());
    fixture.listing(&session("done", Some("idle"), None));

    assert_eq!(fixture.observe(0, &["--wait", &accepted])["state"], "ready");

    let stderr = fixture.refuse(&["--claude", &value, "--wait", &rejected]);

    assert!(
        stderr.contains(&format!("`--wait` must be at most {maximum}")),
        "{stderr}"
    );
}

#[test]
fn reports_unreadable_and_malformed_transcripts_as_errors() {
    let fixture = Fixture::new();
    let transcript = fixture.transcript(UUID);

    fs::write(&transcript, format!("{}\n{{\"secret\": \n", prompt(1)))
        .expect("The transcript must be written");

    let output = fixture.observe(0, &[]);

    assert_eq!(output["state"], "error", "{output}");
    assert!(output.get("nextSince").is_none(), "{output}");

    let message = output["message"]
        .as_str()
        .expect("The error must have a message");

    assert!(message.contains("line 2"), "{message}");
    assert!(!message.contains("secret"), "{message}");

    fs::write(&transcript, "[1]\n").expect("The transcript must be written");
    assert_eq!(fixture.observe(0, &[])["state"], "error");

    fs::set_permissions(&transcript, fs::Permissions::from_mode(0o000))
        .expect("The transcript must become unreadable");
    assert_eq!(fixture.observe(0, &[])["state"], "error");
    fs::set_permissions(&transcript, fs::Permissions::from_mode(0o644))
        .expect("The transcript must become readable");
}

#[test]
fn reports_a_boundary_beyond_the_transcript_as_an_error() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1), bookkeeping()]);

    let output = fixture.observe(3, &[]);

    assert_eq!(output["state"], "error");
    assert!(
        output["message"]
            .as_str()
            .is_some_and(|message| message.contains("2 complete lines")),
        "{output}"
    );
}

#[test]
fn reports_a_missing_transcript_as_working() {
    let fixture = Fixture::new();

    assert_eq!(
        fixture.observe_raw(&[(UUID, 0)], &[]).split_once(",\"sessions\":").map(|(_, rest)| rest),
        Some(format!("[{{\"session\":\"{UUID}\",\"state\":\"working\",\"nextSince\":0,\"progressAt\":null}}]}}\n").as_str())
    );
    assert_eq!(fixture.calls(), 0);

    fs::remove_dir_all(fixture.home.join(".claude")).expect("The projects must be removed");
    assert_eq!(fixture.observe(0, &[])["state"], "working");
}

#[test]
fn reports_a_missing_follow_up_transcript_as_an_error() {
    let fixture = Fixture::new();
    let message = "Found no transcript, but `<since>` is above 0";
    let codex = |since: usize| {
        fixture.observe_with(&["--codex", &format!("{since}:{OTHER_UUID}")])["sessions"][0].clone()
    };

    assert_eq!(
        fixture.observe(4, &[]),
        json!({ "session": UUID, "state": "error", "message": message })
    );
    assert_eq!(
        codex(4),
        json!({ "session": OTHER_UUID, "state": "error", "message": message })
    );

    fs::remove_dir_all(fixture.home.join(".claude")).expect("The projects must be removed");
    assert_eq!(fixture.observe(4, &[])["message"], message);

    // The monitor cannot read a rollout that Codex compressed while it stayed idle
    let rollout = fixture.rollout(OTHER_UUID, 9, "");

    fs::rename(&rollout, rollout.with_extension("jsonl.zst"))
        .expect("The rollout must be compressed");
    assert_eq!(
        codex(4),
        json!({ "session": OTHER_UUID, "state": "working", "nextSince": 4, "progressAt": null })
    );
    assert_eq!(fixture.calls(), 0);
}

#[test]
fn reports_inaccessible_projects_as_errors() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1)]);
    fs::set_permissions(&fixture.project, fs::Permissions::from_mode(0o000))
        .expect("The project must become inaccessible");

    let output = fixture.observe(0, &[]);

    fs::set_permissions(&fixture.project, fs::Permissions::from_mode(0o755))
        .expect("The project must become accessible");
    assert_eq!(output["state"], "error", "{output}");
}

#[test]
fn ignores_plain_files_beside_projects() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1)]);
    fs::write(fixture.home.join(".claude/projects/.DS_Store"), "")
        .expect("The plain file must be written");
    fixture.listing(&session("working", Some("busy"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "working");
}

#[test]
fn reports_ambiguous_transcripts_as_errors() {
    let fixture = Fixture::new();
    let other = fixture
        .home
        .join(".claude/projects/-Users-dom-Projects-other");

    fixture.write(UUID, &[prompt(1)]);
    fs::create_dir_all(&other).expect("The other project must be created");
    fs::write(
        other.join(format!("{UUID}.jsonl")),
        format!("{}\n", prompt(1)),
    )
    .expect("The duplicate transcript must be written");

    let output = fixture.observe(0, &[]);

    assert_eq!(output["state"], "error");
    assert!(
        output["message"]
            .as_str()
            .is_some_and(|message| message.contains("Found 2 transcripts")),
        "{output}"
    );
}

#[test]
fn ignores_an_unterminated_final_record() {
    let fixture = Fixture::new();
    let transcript = fixture.transcript(UUID);

    fs::write(&transcript, format!("{}\n{{\"type\":\"assist", prompt(1)))
        .expect("The transcript must be written");
    set_modified(&transcript, 1_000);
    fixture.listing(&session("working", Some("busy"), None));

    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "session": UUID, "state": "working", "nextSince": 1, "progressAt": 1_000 })
    );
}

#[test]
fn reports_working_before_the_prompt_without_a_lookup() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[prompt(1), reply("a", "Done", "end_turn", 2), turn_end(3)],
    );

    assert_eq!(
        fixture.observe(3, &[]),
        json!({ "session": UUID, "state": "working", "nextSince": 3, "progressAt": null })
    );
    assert_eq!(fixture.calls(), 0);
}

#[test]
fn anchors_the_turn_on_the_latest_prompt() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            reply("a", "Done", "end_turn", 2),
            turn_end(3),
            synthetic("No response requested.", None, 4),
            prompt(5),
        ],
    );
    set_modified(&fixture.transcript(UUID), 5_000);
    fixture.listing(&session("done", Some("idle"), None));

    assert_eq!(
        fixture.observe(3, &[]),
        json!({ "session": UUID, "state": "working", "nextSince": 5, "progressAt": 5_000 })
    );
}

#[test]
fn anchors_only_on_prompts_with_a_human_origin() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &background, 2),
            result("a", false, 3),
            compaction(4),
            json!({
                "type": "user",
                "isCompactSummary": true,
                "message": { "role": "user", "content": "Summary of the earlier conversation" },
                "timestamp": at(5),
            }),
            json!({
                "type": "user",
                "message": { "role": "user", "content": "<local-command-stdout></local-command-stdout>" },
                "timestamp": at(6),
            }),
            reply("b", "Started the tests", "end_turn", 7),
            turn_end(8),
        ],
    );
    fixture.listing(&session("done", Some("idle"), None));

    let output = fixture.observe(0, &[]);

    assert_eq!(output["state"], "working", "{output}");
    assert_eq!(output["nextSince"], 8);
}

#[test]
fn reports_progress_from_the_latest_transcript_write() {
    let fixture = Fixture::new();
    let transcript = fixture.transcript(UUID);

    fixture.write(
        UUID,
        &[prompt(10), call("a", &json!({ "command": "ls" }), 11)],
    );
    set_modified(&transcript, 20_000);
    fixture.listing(&session("working", Some("busy"), None));
    assert_eq!(fixture.observe(0, &[])["progressAt"], 20_000);

    let earlier = fixture.subagent(UUID, "agent-earlier.jsonl", "{}\n");
    let current = fixture.subagent(UUID, "agent-current.jsonl", "{}\n");
    let metadata = fixture.subagent(UUID, "agent-current.meta.json", "{}");

    set_modified(&earlier, 5_000);
    set_modified(&current, 30_000);
    set_modified(&metadata, 40_000);
    assert_eq!(fixture.observe(0, &[])["progressAt"], 30_000);

    // Subagent transcripts count by their writes alone, so their contents are never parsed
    fs::write(&current, "{\n").expect("The subagent transcript must be written");
    set_modified(&current, 35_000);
    assert_eq!(fixture.observe(0, &[])["progressAt"], 35_000);

    // A subagent transcript removed after the listing has no write to report
    symlink(
        fixture.home.join("missing.jsonl"),
        current.with_file_name("agent-removed.jsonl"),
    )
    .expect("The dangling link must be created");
    assert_eq!(fixture.observe(0, &[])["progressAt"], 35_000);
}

#[test]
fn reports_the_final_reply_when_ready() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());

    let before = fs::read(fixture.transcript(UUID)).expect("The transcript must be readable");

    fixture.listing(&session("done", None, None));

    let output = fixture.observe_raw(&[(UUID, 0)], &[]);

    assert!(output.starts_with("{\"observedAt\":"), "{output}");
    assert!(
        output.ends_with(&format!(
            ",\"sessions\":[{{\"session\":\"{UUID}\",\"state\":\"ready\",\"nextSince\":8,\"replies\":[\"No findings\"],\"compactedAt\":null}}]}}\n"
        )),
        "{output}"
    );
    assert_eq!(fs::read(fixture.transcript(UUID)).ok(), Some(before));
}

#[test]
fn reports_observation_time_in_unix_milliseconds() {
    let fixture = Fixture::new();
    let before = millis(SystemTime::now());

    fixture.write(UUID, &[prompt(1)]);

    let observed = fixture.observe_all(&[(UUID, 1)], &[])["observedAt"]
        .as_u64()
        .expect("The observation time must be an integer");

    assert!(observed >= before && observed <= millis(SystemTime::now()));
}

#[test]
fn waits_for_a_session_that_is_not_busy() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());
    set_modified(&fixture.transcript(UUID), 5_000);
    fixture.listing(&session("working", Some("busy"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "session": UUID, "state": "working", "nextSince": 8, "progressAt": 5_000 })
    );

    fixture.listing(&session("working", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "ready");
}

#[test]
fn reports_the_latest_compaction() {
    let mut records = vec![compaction(0), bookkeeping()];

    records.extend(completed_turn());

    let fixture = Fixture::new();

    fixture.write(UUID, &records);
    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &[])["compactedAt"], 1);
}

#[test]
fn reports_turns_that_end_without_a_reply() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            synthetic(
                "Please run /login · API Error: 401",
                Some("authentication_failed"),
                2,
            ),
            turn_end(3),
        ],
    );
    fixture.listing(&session("blocked", Some("idle"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
            "session": UUID,
            "state": "attention",
            "nextSince": 3,
            "reason": "noReply",
            "detail": {
                "error": "authentication_failed",
                "model": "<synthetic>",
                "stopReason": "stop_sequence",
                "text": "Please run /login · API Error: 401",
                "textOmitted": 0,
            },
        })
    );

    fixture.write(
        UUID,
        &[
            prompt(1),
            reply("a", &"é".repeat(520), "max_tokens", 2),
            turn_end(3),
        ],
    );
    fixture.listing(&session("done", None, None));

    let output = fixture.observe(0, &[]);

    assert_eq!(output["detail"]["stopReason"], "max_tokens");
    assert_eq!(output["detail"]["text"], "é".repeat(500));
    assert_eq!(output["detail"]["textOmitted"], 20);

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &json!({ "command": "ls" }), 2),
            turn_end(3),
        ],
    );
    assert_eq!(fixture.observe(0, &[])["reason"], "noReply");
}

#[test]
fn reports_blocked_sessions_with_the_pending_tool() {
    let input = json!({ "command": "y".repeat(600) });
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1), call("a", &input, 2)]);

    let input = input.to_string();

    fixture.listing(&session(
        "working",
        Some("waiting"),
        Some("permission prompt"),
    ));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
            "session": UUID,
            "state": "attention",
            "nextSince": 2,
            "reason": "blocked",
            "detail": {
                "tool": { "input": input[..500], "inputOmitted": input.len() - 500, "name": "Bash" },
                "waitingFor": "permission prompt",
            },
        })
    );

    // A `blocked` session state without an open prompt is not a pending approval
    fixture.listing(&session("blocked", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "working");
}

#[test]
fn waits_for_background_work_despite_a_blocked_session_state() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let mut records = vec![
        prompt(1),
        call("a", &background, 2),
        result("a", false, 3),
        reply("b", "Should I also run lint?", "end_turn", 4),
        turn_end(5),
    ];
    let fixture = Fixture::new();

    fixture.write(UUID, &records);
    fixture.listing(&session("blocked", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "working");

    records.push(notification("a", 6));
    fixture.write(UUID, &records);
    assert_eq!(fixture.observe(0, &[])["state"], "working");

    records.extend([reply("c", "The tests passed", "end_turn", 7), turn_end(8)]);
    fixture.write(UUID, &records);
    assert_eq!(
        fixture.observe(0, &[])["replies"],
        json!(["Should I also run lint?", "The tests passed"])
    );
}

#[test]
fn reports_a_finished_reply_despite_a_blocked_session_state() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());

    for status in [Some("idle"), None] {
        fixture.listing(&session("blocked", status, None));
        assert_eq!(
            fixture.observe(0, &[])["replies"],
            json!(["No findings"]),
            "{status:?}"
        );
    }
}

#[test]
fn reports_failed_and_stopped_turns() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[prompt(1), call("a", &json!({ "command": "ls" }), 2)],
    );

    for state in ["failed", "stopped"] {
        fixture.listing(&session(state, None, None));
        assert_eq!(
            fixture.observe(0, &[]),
            json!({ "session": UUID, "state": "attention", "nextSince": 2, "reason": state })
        );
    }
}

#[test]
fn reports_open_prompts_after_a_reply_as_blocked() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());
    fixture.listing(&session("working", Some("waiting"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
            "session": UUID,
            "state": "attention",
            "nextSince": 8,
            "reason": "blocked",
            "detail": { "tool": null, "waitingFor": null },
        })
    );

    fixture.write(
        UUID,
        &[prompt(1), call("a", &json!({ "command": "ls" }), 2)],
    );
    fixture.listing(&session("working", Some("busy"), Some("dialog open")));
    assert_eq!(
        fixture.observe(0, &[])["detail"]["waitingFor"],
        "dialog open"
    );
}

#[test]
fn waits_for_pending_agents_and_workflows() {
    let fixture = Fixture::new();

    fixture.listing(&session("done", None, None));

    for (field, count, state) in [
        ("pendingBackgroundAgentCount", 1, "working"),
        ("pendingWorkflowCount", 1, "working"),
        ("pendingBackgroundAgentCount", 0, "ready"),
        ("pendingWorkflowCount", 0, "ready"),
    ] {
        let mut records = completed_turn();

        records[6][field] = json!(count);
        fixture.write(UUID, &records);
        assert_eq!(fixture.observe(0, &[])["state"], state, "{field}: {count}");
    }
}

#[test]
fn reports_failed_replies_despite_pending_background_work() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &background, 2),
            result("a", false, 3),
            synthetic("API Error: 500", Some("server_error"), 4),
            turn_end(5),
        ],
    );
    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &[])["reason"], "noReply");

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &background, 2),
            result("a", false, 3),
            reply("b", "Started the tests", "end_turn", 4),
            turn_end(5),
        ],
    );
    fixture.listing(&session("stopped", None, None));
    assert_eq!(fixture.observe(0, &[])["reason"], "stopped");
}

#[test]
fn ignores_sidechain_records_when_deciding_the_turn() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &json!({ "command": "ls" }), 2),
            sidechain(reply("s", "Sidechain finding", "end_turn", 4)),
            sidechain(result("s", false, 5)),
            sidechain(turn_end(6)),
            sidechain(prompt(7)),
        ],
    );
    fixture.listing(&session("done", Some("idle"), None));

    let output = fixture.observe(0, &[]);

    assert_eq!(output["state"], "working", "{output}");
    assert_eq!(output["nextSince"], 6);
}

#[test]
fn waits_for_background_commands_to_report() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let mut records = vec![
        prompt(1),
        call("a", &background, 2),
        result("a", false, 3),
        reply("b", "Started the tests", "end_turn", 4),
        turn_end(5),
    ];
    let fixture = Fixture::new();

    fixture.write(UUID, &records);
    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "working");

    // Claude answers a notification recorded after its turn ended, even while the row shows `idle`
    records.push(notification("a", 6));
    fixture.write(UUID, &records);
    assert_eq!(fixture.observe(0, &[])["state"], "working");

    records.extend([reply("c", "The tests passed", "end_turn", 7), turn_end(8)]);
    fixture.write(UUID, &records);
    assert_eq!(
        fixture.observe(0, &[])["replies"],
        json!(["Started the tests", "The tests passed"])
    );

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &background, 2),
            result("a", true, 3),
            reply("b", "The tests could not start", "end_turn", 4),
            turn_end(5),
        ],
    );
    assert_eq!(fixture.observe(0, &[])["state"], "ready");
}

#[test]
fn collects_the_reply_of_every_turn_since_the_prompt() {
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            reply("a", "First", "end_turn", 2),
            turn_end(3),
            notification("x", 4),
            reply("b", "Second", "end_turn", 5),
            turn_end(6),
        ],
    );
    fixture.listing(&session("done", None, None));
    assert_eq!(
        fixture.observe(0, &[])["replies"],
        json!(["First", "Second"])
    );
}

#[test]
fn reports_unexpected_session_lookups_as_errors() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());
    assert_eq!(fixture.observe(0, &[])["state"], "error");

    for rows in [
        json!([row(OTHER_UUID, "done", None, None)]),
        json!([row(UUID, "done", None, None), row(UUID, "done", None, None)]),
        row(UUID, "done", None, None),
    ] {
        fixture.listing(&rows);
        assert_eq!(fixture.observe(0, &[])["state"], "error", "{rows}");
    }

    fs::write(fixture.bin.join("listing.json"), "not json").expect("The listing must be written");
    assert_eq!(fixture.observe(0, &[])["state"], "error");

    fs::remove_file(fixture.bin.join("claude")).expect("The Claude stub must be removed");
    assert_eq!(fixture.observe(0, &[])["state"], "error");
}

#[test]
fn observes_several_sessions_in_argument_order() {
    let fixture = Fixture::new();

    fixture.write(UUID, &completed_turn());
    fixture.write(OTHER_UUID, &[prompt(1)]);
    set_modified(&fixture.transcript(OTHER_UUID), 7_000);
    fixture.listing(&session("done", None, None));

    let output = fixture.observe_all(&[(OTHER_UUID, 0), (UUID, 0)], &[]);

    assert_eq!(
        output["sessions"],
        json!([
            { "session": OTHER_UUID, "state": "working", "nextSince": 1, "progressAt": 7_000 },
            { "session": UUID, "state": "ready", "nextSince": 8, "replies": ["No findings"], "compactedAt": null },
        ])
    );
    assert_eq!(fixture.calls(), 1);
}

#[test]
fn keeps_reporting_healthy_sessions_beside_errors() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1)]);
    fixture.write(OTHER_UUID, &[prompt(1)]);
    fixture.listing(&json!([row(OTHER_UUID, "working", Some("busy"), None)]));

    let started = Instant::now();
    let sessions = fixture.observe_all(
        &[
            (UUID, 0),
            (OTHER_UUID, 0),
            (UUID.replace('0', "9").as_str(), 0),
        ],
        &["--wait", "60"],
    )["sessions"]
        .take();

    // An `error` ends the wait like any other state that is not `working`
    assert!(started.elapsed() < Duration::from_secs(10));
    assert_eq!(sessions[0]["state"], "error");
    assert_eq!(sessions[1]["state"], "working");
    assert_eq!(
        sessions[2],
        json!({ "session": UUID.replace('0', "9"), "state": "working", "nextSince": 0, "progressAt": null })
    );
}

#[test]
fn returns_as_soon_as_any_session_leaves_working() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1)]);
    fixture.write(OTHER_UUID, &[prompt(1)]);
    fixture.listing(&json!([
        row(OTHER_UUID, "working", Some("busy"), None),
        row(UUID, "done", Some("idle"), None),
    ]));

    let started = Instant::now();
    let output = thread::scope(|scope| {
        scope.spawn(|| {
            let deadline = Instant::now() + Duration::from_secs(10);

            while fixture.calls() == 0 {
                assert!(Instant::now() < deadline, "The initial lookup must occur");
                thread::sleep(Duration::from_millis(10));
            }
            fixture.write(UUID, &completed_turn());
        });
        fixture.observe_all(&[(OTHER_UUID, 0), (UUID, 0)], &["--wait", "60"])
    });

    assert!(started.elapsed() < Duration::from_secs(20));
    assert_eq!(output["sessions"][0]["state"], "working");
    assert_eq!(output["sessions"][1]["state"], "ready");
}

#[test]
fn refreshes_the_lookup_when_a_prompt_is_recorded() {
    let fixture = Fixture::new();
    let wait_for_calls = |count: usize| {
        let deadline = Instant::now() + Duration::from_secs(10);

        while fixture.calls() < count {
            assert!(Instant::now() < deadline, "Lookup {count} must occur");
            thread::sleep(Duration::from_millis(10));
        }
    };

    fixture.write(UUID, &[]);
    fixture.write(OTHER_UUID, &[prompt(1)]);
    fixture.listing(&json!([
        row(OTHER_UUID, "working", Some("busy"), None),
        row(UUID, "stopped", None, None),
    ]));

    let output = thread::scope(|scope| {
        scope.spawn(|| {
            wait_for_calls(1);

            // A resumed session’s row changes before its prompt is recorded
            fixture.listing(&json!([
                row(OTHER_UUID, "done", Some("idle"), None),
                row(UUID, "working", Some("busy"), None),
            ]));
            fixture.write(UUID, &[prompt(1)]);
            wait_for_calls(2);
            fixture.write(OTHER_UUID, &completed_turn());
        });
        fixture.observe_all(&[(UUID, 0), (OTHER_UUID, 0)], &["--wait", "60"])
    });

    assert_eq!(output["sessions"][0]["state"], "working", "{output}");
    assert_eq!(output["sessions"][1]["state"], "ready", "{output}");
}

#[test]
fn waits_until_ready_or_the_deadline() {
    let fixture = Fixture::new();
    let started = Instant::now();

    fixture.write(UUID, &completed_turn());
    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &["--wait", "60"])["state"], "ready");
    assert!(started.elapsed() < Duration::from_secs(10));

    fixture.write(UUID, &[prompt(1)]);
    fixture.listing(&session("working", Some("busy"), None));

    let started = Instant::now();

    assert_eq!(fixture.observe(0, &["--wait", "1"])["state"], "working");
    assert!(started.elapsed() >= Duration::from_secs(1));
    assert!(started.elapsed() < Duration::from_secs(10));
    assert_eq!(fixture.calls(), 2);
}

#[test]
fn waits_for_readiness_by_default() {
    let fixture = Fixture::new();

    fixture.write(UUID, &[prompt(1)]);
    fixture.listing(&session("done", Some("idle"), None));

    let (status, stdout, stderr) = thread::scope(|scope| {
        scope.spawn(|| {
            let deadline = Instant::now() + Duration::from_secs(10);

            while fixture.calls() == 0 {
                assert!(Instant::now() < deadline, "The initial lookup must occur");
                thread::sleep(Duration::from_millis(10));
            }
            fixture.write(UUID, &completed_turn());
        });
        fixture.run(None, &["--claude", &format!("0:{UUID}")])
    });

    assert_eq!((status, stderr.as_str()), (Some(0), ""));
    assert_eq!(
        serde_json::from_str::<Value>(&stdout).expect("The output must be JSON")["sessions"][0]["state"],
        "ready"
    );
}

#[test]
fn keeps_the_lookup_interval_while_background_work_runs() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new();

    fixture.write(
        UUID,
        &[
            prompt(1),
            call("a", &background, 2),
            result("a", false, 3),
            reply("b", "Started the tests", "end_turn", 4),
            turn_end(5),
        ],
    );
    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(fixture.observe(0, &["--wait", "2"])["state"], "working");
    assert_eq!(fixture.calls(), 1);
}

#[test]
fn reports_a_finished_codex_turn() {
    let fixture = Fixture::new();
    let mut records = vec![
        session_meta(UUID, UUID, 0),
        json!({ "type": "compacted", "timestamp": at(1) }),
    ];

    records.extend(codex_turn("First", 2));
    records.extend(codex_turn("Second", 10));
    fixture.write_rollout(UUID, 9, &records);

    for since in [0, 5] {
        assert_eq!(
            fixture.observe_with(&["--codex", &format!("{since}:{UUID}")])["sessions"][0],
            json!({ "session": UUID, "state": "ready", "nextSince": 8, "replies": ["Second"], "compactedAt": 2 }),
            "{since}"
        );
    }
    assert_eq!(fixture.calls(), 0);
}

#[test]
fn reports_codex_turns_without_a_reply() {
    let fixture = Fixture::new();
    let session = format!("0:{UUID}");

    fixture.write_rollout(
        UUID,
        9,
        &[
            session_meta(UUID, UUID, 0),
            turn_started(1),
            turn_complete(None, Some(&"é".repeat(520)), 2),
        ],
    );
    assert_eq!(
        fixture.observe_with(&["--codex", &session])["sessions"][0],
        json!({
            "session": UUID,
            "state": "attention",
            "nextSince": 3,
            "reason": "noReply",
            "detail": { "abortReason": null, "error": "é".repeat(500), "errorOmitted": 20 },
        })
    );

    fixture.write_rollout(
        UUID,
        9,
        &[
            session_meta(UUID, UUID, 0),
            turn_started(1),
            reasoning(2),
            turn_aborted("interrupted", 3),
        ],
    );
    assert_eq!(
        fixture.observe_with(&["--codex", &session])["sessions"][0]["detail"],
        json!({ "abortReason": "interrupted", "error": null, "errorOmitted": 0 })
    );
}

#[test]
fn reports_codex_working_before_the_turn_starts() {
    let fixture = Fixture::new();
    let session = format!("0:{UUID}");

    assert_eq!(
        fixture.observe_with(&["--codex", &session])["sessions"][0],
        json!({ "session": UUID, "state": "working", "nextSince": 0, "progressAt": null })
    );

    let rollout = fixture.rollout(UUID, 9, "");

    fs::rename(&rollout, rollout.with_extension("jsonl.zst"))
        .expect("The rollout must be compressed");
    assert_eq!(
        fixture.observe_with(&["--codex", &session])["sessions"][0]["progressAt"],
        Value::Null
    );

    fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0)]);
    assert_eq!(
        fixture.observe_with(&["--codex", &session])["sessions"][0],
        json!({ "session": UUID, "state": "working", "nextSince": 1, "progressAt": null })
    );
    assert_eq!(fixture.calls(), 0);
}

#[test]
fn reports_ambiguous_codex_rollouts_as_errors() {
    let fixture = Fixture::new();

    fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0)]);
    fixture.write_rollout(UUID, 10, &[session_meta(UUID, UUID, 0)]);

    let output = fixture.observe_with(&["--codex", &format!("0:{UUID}")]);

    assert_eq!(output["sessions"][0]["state"], "error", "{output}");
    assert!(
        output["sessions"][0]["message"]
            .as_str()
            .is_some_and(|message| message.contains("Found 2 rollouts")),
        "{output}"
    );
}

#[test]
fn reports_codex_progress_from_descendants_started_during_the_turn() {
    const CHILD: &str = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa";
    const EARLIER: &str = "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb";
    const GRANDCHILD: &str = "cccccccc-cccc-cccc-cccc-cccccccccccc";
    const PARTIAL: &str = "dddddddd-dddd-dddd-dddd-dddddddddddd";
    const UNRELATED: &str = "eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee";

    let fixture = Fixture::new();
    let root = fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0), turn_started(10)]);

    set_modified(&root, 10_000);
    assert_eq!(
        fixture.observe_with(&["--codex", &format!("0:{UUID}")])["sessions"][0]["progressAt"],
        10_000
    );

    for (path, time) in [
        (
            fixture.write_rollout(CHILD, 9, &[session_meta(CHILD, UUID, 11)]),
            20_000,
        ),
        (
            fixture.write_rollout(GRANDCHILD, 10, &[session_meta(GRANDCHILD, UUID, 12)]),
            30_000,
        ),
        (
            fixture.write_rollout(EARLIER, 10, &[session_meta(EARLIER, UUID, 5)]),
            90_000,
        ),
        (
            fixture.write_rollout(UNRELATED, 10, &[session_meta(UNRELATED, OTHER_UUID, 13)]),
            80_000,
        ),
        (
            fixture.rollout(PARTIAL, 10, &session_meta(PARTIAL, UUID, 14).to_string()),
            70_000,
        ),
    ] {
        set_modified(&path, time);
    }

    assert_eq!(
        fixture.observe_with(&["--codex", &format!("0:{UUID}")])["sessions"][0],
        json!({ "session": UUID, "state": "working", "nextSince": 2, "progressAt": 30_000 })
    );

    // Without the turn’s start time, every subagent rollout could predate the turn
    let mut started = turn_started(10);

    started
        .as_object_mut()
        .expect("The turn start must be an object")
        .remove("timestamp");
    fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0), started]);
    set_modified(&root, 10_000);
    assert_eq!(
        fixture.observe_with(&["--codex", &format!("0:{UUID}")])["sessions"][0]["progressAt"],
        10_000
    );
}

#[test]
fn scans_the_root_and_two_newest_date_directories() {
    const CURRENT: &str = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa";
    const NEWEST: &str = "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb";
    const OLDER: &str = "cccccccc-cccc-cccc-cccc-cccccccccccc";
    const RECENT: &str = "dddddddd-dddd-dddd-dddd-dddddddddddd";

    let fixture = Fixture::new();
    let root = fixture.write_rollout(UUID, 1, &[session_meta(UUID, UUID, 0), turn_started(10)]);

    set_modified(&root, 10_000);

    let [current, _, recent, _] = [
        (CURRENT, 1, 20_000),
        (OLDER, 8, 90_000),
        (RECENT, 9, 30_000),
        (NEWEST, 10, 40_000),
    ]
    .map(|(uuid, day, time)| {
        let path = fixture.write_rollout(uuid, day, &[session_meta(uuid, UUID, 11)]);

        set_modified(&path, time);
        path
    });

    // The descendant in the third-newest directory never counts, whichever in-window write is latest
    for (path, time) in [
        (None, 40_000),
        (Some(&current), 60_000),
        (Some(&recent), 70_000),
    ] {
        if let Some(path) = path {
            set_modified(path, time);
        }
        assert_eq!(
            fixture.observe_with(&["--codex", &format!("0:{UUID}")])["sessions"][0]["progressAt"],
            time
        );
    }
}

#[test]
fn reports_unreadable_rollouts_but_ignores_unidentified_ones() {
    let fixture = Fixture::new();
    let root = fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0), turn_started(10)]);
    let directory = root.parent().expect("The rollout must have a directory");
    let unreadable = directory.join("rollout-unreadable.jsonl");

    set_modified(&root, 10_000);
    fs::write(directory.join("rollout-notes.jsonl"), "not json\n")
        .expect("The unidentified rollout must be written");
    assert_eq!(
        fixture.observe_with(&["--codex", &format!("0:{UUID}")])["sessions"][0]["progressAt"],
        10_000
    );

    fs::write(
        &unreadable,
        format!("{}\n", session_meta(OTHER_UUID, OTHER_UUID, 0)),
    )
    .expect("The unreadable rollout must be written");
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000))
        .expect("The rollout must become unreadable");

    let output = fixture.observe_with(&["--codex", &format!("0:{UUID}")]);

    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o644))
        .expect("The rollout must become readable");
    assert_eq!(output["sessions"][0]["state"], "error", "{output}");
}

#[test]
fn applies_a_new_turn_start_to_known_descendants() {
    const CHILD: &str = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa";

    let fixture = Fixture::new();
    let mut records = vec![session_meta(UUID, UUID, 0), turn_started(10)];
    let root = fixture.write_rollout(UUID, 9, &records);
    let child = fixture.write_rollout(CHILD, 9, &[session_meta(CHILD, UUID, 11)]);
    let later = millis(SystemTime::now()) + 3_600_000;

    set_modified(&root, 10_000);
    set_modified(&child, 20_000);

    let output = thread::scope(|scope| {
        scope.spawn(|| {
            // The first check caches the child as a descendant of the turn started at second 10
            thread::sleep(Duration::from_secs(1));
            records.push(turn_started(30));
            fixture.write_rollout(UUID, 9, &records);
            set_modified(&root, 50_000);
            set_modified(&child, later);
        });
        fixture.observe_with(&["--codex", &format!("0:{UUID}"), "--wait", "4"])
    });

    assert_eq!(output["sessions"][0]["progressAt"], 50_000, "{output}");
}

#[test]
fn reports_a_duplicate_rollout_that_appears_while_waiting() {
    let fixture = Fixture::new();

    fixture.write_rollout(UUID, 9, &[session_meta(UUID, UUID, 0), turn_started(1)]);

    let started = Instant::now();
    let output = thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            fixture.write_rollout(UUID, 10, &[session_meta(UUID, UUID, 0)]);
        });
        fixture.observe_with(&["--codex", &format!("0:{UUID}"), "--wait", "30"])
    });

    assert!(started.elapsed() < Duration::from_secs(20));
    assert_eq!(output["sessions"][0]["state"], "error", "{output}");
}

#[test]
fn observes_claude_and_codex_sessions_together() {
    let fixture = Fixture::new();
    let root = fixture.write_rollout(
        OTHER_UUID,
        9,
        &[session_meta(OTHER_UUID, OTHER_UUID, 0), turn_started(1)],
    );

    set_modified(&root, 4_000);
    fixture.write(UUID, &completed_turn());
    fixture.listing(&session("done", None, None));

    let output = fixture.observe_with(&[
        "--codex",
        &format!("0:{OTHER_UUID}"),
        "--claude",
        &format!("0:{UUID}"),
    ]);

    assert_eq!(
        output["sessions"],
        json!([
            { "session": OTHER_UUID, "state": "working", "nextSince": 2, "progressAt": 4_000 },
            { "session": UUID, "state": "ready", "nextSince": 8, "replies": ["No findings"], "compactedAt": null },
        ])
    );
    assert_eq!(fixture.calls(), 1);
}

#[test]
fn launches_from_the_domfiles_root_and_runs_in_the_caller_directory() {
    let fixture = Fixture::new();
    let caller = fixture.project.clone();
    let root = fs::canonicalize(env!("CARGO_MANIFEST_DIR")).expect("The root must resolve");
    let session = format!("1:{UUID}");
    // The launcher also needs the provisioned `jq`, so it keeps the inherited search path after the stubs
    let path = format!(
        "{}:{}",
        fixture.bin.display(),
        env::var("PATH").unwrap_or_default()
    );
    let launch =
        |arguments: &[&str]| execute(LAUNCHER, &path, &fixture.home, Some(&caller), arguments);
    let reported = fixture.bin.join("reported");

    fixture.write(UUID, &[prompt(1)]);
    fs::create_dir_all(caller.join(".cargo")).expect("The caller configuration must be created");
    fs::write(
        caller.join(".cargo/config.toml"),
        "[build]\ntarget-dir = \"conflicting\"\n",
    )
    .expect("The caller configuration must be written");
    fixture.cargo(0, BINARY);

    let (status, stdout, stderr) = launch(&["--claude", &session, "--wait", "0"]);

    assert_eq!((status, stderr.as_str()), (Some(0), "cargo: building\n"));
    assert!(
        stdout.ends_with(&format!(
            ",\"sessions\":[{{\"session\":\"{UUID}\",\"state\":\"working\",\"nextSince\":1,\"progressAt\":null}}]}}\n"
        )),
        "{stdout}"
    );
    assert_eq!(
        fixture.recorded("cargo-directory"),
        format!("{}\n", root.display())
    );
    assert_eq!(
        fixture.recorded("cargo-arguments"),
        "build\n--bin\nsubagent-signal\n--locked\n--message-format=json-render-diagnostics\n--quiet\n"
    );
    assert_eq!(
        launch(&["--claude", &session, "--claude", &session]),
        (
            Some(2),
            String::new(),
            format!("cargo: building\n{NAME}: Session `{UUID}` must be given only once\n")
        )
    );

    fs::write(&reported, "#!/bin/sh\npwd -P\n").expect("The reported command must be written");
    fs::set_permissions(&reported, fs::Permissions::from_mode(0o755))
        .expect("The reported command must be executable");
    fixture.cargo(0, reported.to_str().expect("The path must be UTF-8"));
    assert_eq!(
        launch(&["--wait", "0"]),
        (
            Some(0),
            format!(
                "{}\n",
                fs::canonicalize(&caller)
                    .expect("The caller directory must resolve")
                    .display()
            ),
            "cargo: building\n".to_owned()
        )
    );

    for (executable, diagnostic) in [
        (
            "",
            "Cargo did not report an executable for `subagent-signal`",
        ),
        ("\"", "Failed to read Cargo’s build messages"),
    ] {
        fixture.cargo(0, executable);

        let (status, stdout, stderr) = launch(&["--wait", "0"]);

        assert_eq!((status, stdout.as_str()), (Some(1), ""));
        assert!(stderr.contains(diagnostic), "{stderr}");
    }

    fixture.cargo(101, BINARY);
    assert_eq!(
        launch(&["--claude", &session, "--wait", "0"]),
        (Some(101), String::new(), "cargo: building\n".to_owned())
    );
}
