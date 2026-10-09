use serde_json::{Value, json};
use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};

const BINARY: &str = env!("CARGO_BIN_EXE_claude-signal");
const CARGO_STUB: &str = concat!(
    "#!/bin/sh\n",
    "pwd -P > \"${0%/*}/cargo-directory\"\n",
    "printf '%s\\n' \"$@\" > \"${0%/*}/cargo-arguments\"\n",
    "printf '%s\\n' 'cargo: building' >&2\n",
    "printf '{\"reason\":\"compiler-artifact\",\"target\":{\"name\":\"%s\"},\"executable\":\"%s\"}\\n' \"$3\" \"$(cat \"${0%/*}/cargo-executable\")\"\n",
    "exit \"$(cat \"${0%/*}/cargo-status\")\"\n",
);
const CLAUDE_STUB: &str = concat!(
    "#!/bin/sh\n",
    "printf x >> \"${0%/*}/calls\"\n",
    "[ \"$*\" = 'agents --json --all' ] || exit 64\n",
    "exec cat \"${0%/*}/listing.json\"\n",
);
const LAUNCHER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/home/.local/bin/claude-signal");
const NAME: &str = "claude-signal";
const OTHER_UUID: &str = "ffffffff-ffff-ffff-ffff-ffffffffffff";
const UUID: &str = "0123abcd-4567-89ab-cdef-0123456789ab";

static FIXTURES: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    bin: PathBuf,
    root: PathBuf,
    transcript: PathBuf,
}

impl Fixture {
    fn new(records: &[Value]) -> Self {
        let root = env::temp_dir().join(format!(
            "{NAME}-test-{}-{}",
            process::id(),
            FIXTURES.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("bin");
        let project = root.join("projects é").join("-Users-dom-Projects-a b");

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

        let fixture = Self {
            bin,
            root,
            transcript: project.join(format!("{UUID}.jsonl")),
        };

        fixture.write(records);
        fixture
    }

    fn write(&self, records: &[Value]) {
        let lines: String = records.iter().map(|record| format!("{record}\n")).collect();

        fs::write(&self.transcript, lines).expect("The transcript must be written");
    }

    fn listing(&self, rows: &Value) {
        fs::write(self.bin.join("listing.json"), rows.to_string())
            .expect("The listing must be written");
    }

    fn subagent(&self, name: &str, contents: &str) {
        let directory = self.transcript.with_extension("").join("subagents");

        fs::create_dir_all(&directory).expect("The subagent directory must be created");
        fs::write(directory.join(name), contents).expect("The subagent transcript must be written");
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

    fn path(&self) -> &str {
        self.transcript
            .to_str()
            .expect("The transcript path must be UTF-8")
    }

    fn observe_raw(&self, since: usize, extra: &[&str]) -> String {
        let since = since.to_string();
        let mut arguments = vec!["--transcript", self.path(), "--since", &since];

        arguments.extend_from_slice(extra);

        let (status, stdout, stderr) = run(&self.bin, None, &arguments);

        assert_eq!(status, Some(0), "{arguments:?}: {stderr}");
        assert!(stderr.is_empty(), "{arguments:?}: {stderr}");
        assert!(
            stdout.ends_with('\n') && stdout.matches('\n').count() == 1,
            "{stdout}"
        );
        stdout
    }

    fn observe(&self, since: usize, extra: &[&str]) -> Value {
        serde_json::from_str(&self.observe_raw(since, extra)).expect("The output must be JSON")
    }

    fn refuse(&self, arguments: &[&str]) -> String {
        refuse(&self.bin, arguments)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn run(path: &Path, directory: Option<&Path>, arguments: &[&str]) -> (Option<i32>, String, String) {
    execute(
        BINARY,
        &format!("{}:/usr/bin:/bin", path.display()),
        directory,
        arguments,
    )
}

fn execute(
    program: &str,
    path: &str,
    directory: Option<&Path>,
    arguments: &[&str],
) -> (Option<i32>, String, String) {
    let mut command = Command::new(program);

    command.args(arguments).env("PATH", path);

    if let Some(directory) = directory {
        command.current_dir(directory);
    }

    let output = command.output().expect("The checker must start");

    (
        output.status.code(),
        String::from_utf8(output.stdout).expect("Standard output must be UTF-8"),
        String::from_utf8(output.stderr).expect("Standard error must be UTF-8"),
    )
}

fn refuse(path: &Path, arguments: &[&str]) -> String {
    let (status, stdout, stderr) = run(path, None, arguments);

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

fn session(state: &str, status: Option<&str>, waiting_for: Option<&str>) -> Value {
    json!([
        { "id": "ffffffff", "sessionId": OTHER_UUID, "state": "working", "status": "busy", "waitingFor": null },
        { "id": "0123abcd", "sessionId": UUID, "state": state, "status": status, "waitingFor": waiting_for },
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

#[test]
fn prints_help_alone() {
    let (status, stdout, stderr) = run(Path::new("/nonexistent"), None, &["--help"]);

    assert_eq!(status, Some(0));
    assert!(stdout.starts_with("Usage:\n"));
    assert!(stderr.is_empty());
}

#[test]
fn documents_exactly_the_accepted_options() {
    let (_, help, _) = run(Path::new("/nonexistent"), None, &["--help"]);
    let usage = help_section(&help, "Usage:");
    let mut usage_options: Vec<&str> = usage[0]
        .split_whitespace()
        .map(|token| token.trim_matches(['[', ']']))
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

    let fixture = Fixture::new(&[prompt(1)]);

    assert_eq!(fixture.observe(1, &["--wait", "0"])["state"], "working");
}

#[test]
fn rejects_invalid_invocations() {
    let fixture = Fixture::new(&[prompt(1)]);
    let transcript = fixture.path().to_owned();
    let invalid_names = [
        "session.jsonl".to_owned(),
        format!("{UUID}.json"),
        format!("{}.jsonl", &UUID[1..]),
        format!("{}.jsonl", UUID.replace('-', "_")),
    ];
    let mut cases: Vec<Vec<&str>> = vec![
        vec![],
        vec!["-h"],
        vec!["--help", "--since", "0"],
        vec!["--since", "0", "--help"],
        vec!["--since", "0"],
        vec!["--transcript", &transcript],
        vec!["--transcript", &transcript, "--since"],
        vec!["--transcript", &transcript, "--since", "0", "--since", "0"],
        vec!["--transcript", &transcript, "--since", "0", "--verbose"],
        vec!["--transcript", &transcript, "--since", "0", "--wait", "61"],
        vec!["--transcript", &transcript, "--since", "0", "--wait", "x"],
    ];

    for since in ["", " 1", "+1", "-1", "1.5", "1e3"] {
        cases.push(vec!["--transcript", &transcript, "--since", since]);
    }
    for name in &invalid_names {
        cases.push(vec!["--transcript", name, "--since", "0"]);
    }
    for arguments in &cases {
        fixture.refuse(arguments);
    }

    assert_eq!(fixture.calls(), 0);
}

#[test]
fn rejects_unreadable_and_malformed_transcripts() {
    let fixture = Fixture::new(&[]);
    let arguments = ["--transcript", fixture.path(), "--since", "0"];

    fs::write(
        &fixture.transcript,
        format!("{}\n{{\"secret\": \n", prompt(1)),
    )
    .expect("The transcript must be written");
    let stderr = fixture.refuse(&arguments);

    assert!(stderr.contains("line 2"), "{stderr}");
    assert!(!stderr.contains("secret"), "{stderr}");

    fs::write(&fixture.transcript, "[1]\n").expect("The transcript must be written");
    fixture.refuse(&arguments);

    fs::remove_file(&fixture.transcript).expect("The transcript must be removed");
    fixture.refuse(&arguments);
}

#[test]
fn rejects_a_boundary_beyond_the_transcript() {
    let fixture = Fixture::new(&[prompt(1), bookkeeping()]);
    let stderr = fixture.refuse(&["--transcript", fixture.path(), "--since", "3"]);

    assert!(stderr.contains("2 complete lines"), "{stderr}");
}

#[test]
fn ignores_an_unterminated_final_record() {
    let fixture = Fixture::new(&[]);

    fs::write(
        &fixture.transcript,
        format!("{}\n{{\"type\":\"assist", prompt(1)),
    )
    .expect("The transcript must be written");
    fixture.listing(&session("working", Some("busy"), None));

    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "state": "working", "nextSince": 1, "progressAt": at(1) })
    );
}

#[test]
fn reports_working_before_the_prompt_without_a_lookup() {
    let fixture = Fixture::new(&[prompt(1), reply("a", "Done", "end_turn", 2), turn_end(3)]);

    assert_eq!(
        fixture.observe(3, &[]),
        json!({ "state": "working", "nextSince": 3, "progressAt": null })
    );
    assert_eq!(fixture.calls(), 0);
}

#[test]
fn anchors_the_turn_on_the_latest_prompt() {
    let fixture = Fixture::new(&[
        prompt(1),
        reply("a", "Done", "end_turn", 2),
        turn_end(3),
        synthetic("No response requested.", None, 4),
        prompt(5),
    ]);

    fixture.listing(&session("done", Some("idle"), None));

    assert_eq!(
        fixture.observe(3, &[]),
        json!({ "state": "working", "nextSince": 5, "progressAt": at(5) })
    );
}

#[test]
fn reports_progress_from_new_text_and_successful_results() {
    let mut records = vec![
        prompt(1),
        call("a", &json!({ "command": "ls" }), 2),
        result("a", false, 3),
        call("b", &json!({ "command": "ls" }), 4),
        result("b", false, 5),
        call("c", &json!({ "command": "false" }), 6),
        result("c", true, 7),
    ];
    let fixture = Fixture::new(&records);

    fixture.listing(&session("working", Some("busy"), None));
    assert_eq!(fixture.observe(0, &[])["progressAt"], at(3));

    records.push(reply("d", "Checking the tests next", "tool_use", 8));
    fixture.write(&records);
    assert_eq!(fixture.observe(0, &[])["progressAt"], at(8));
}

#[test]
fn reports_the_final_reply_when_ready() {
    let fixture = Fixture::new(&completed_turn());
    let before = fs::read(&fixture.transcript).expect("The transcript must be readable");

    fixture.listing(&session("done", None, None));

    assert_eq!(
        fixture.observe_raw(0, &[]),
        "{\"state\":\"ready\",\"nextSince\":8,\"replies\":[\"No findings\"],\"compactedAt\":null}\n"
    );
    assert_eq!(fs::read(&fixture.transcript).ok(), Some(before));
}

#[test]
fn waits_for_a_session_that_is_not_busy() {
    let fixture = Fixture::new(&completed_turn());

    fixture.listing(&session("working", Some("busy"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "state": "working", "nextSince": 8, "progressAt": at(5) })
    );

    fixture.listing(&session("working", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "ready");
}

#[test]
fn reports_the_latest_compaction() {
    let mut records = vec![compaction(0), bookkeeping()];

    records.extend(completed_turn());

    let fixture = Fixture::new(&records);

    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &[])["compactedAt"], 1);
}

#[test]
fn reports_turns_that_end_without_a_reply() {
    let fixture = Fixture::new(&[
        prompt(1),
        synthetic(
            "Please run /login · API Error: 401",
            Some("authentication_failed"),
            2,
        ),
        turn_end(3),
    ]);

    fixture.listing(&session("blocked", Some("idle"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
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

    fixture.write(&[
        prompt(1),
        reply("a", &"é".repeat(520), "max_tokens", 2),
        turn_end(3),
    ]);
    fixture.listing(&session("done", None, None));

    let output = fixture.observe(0, &[]);

    assert_eq!(output["detail"]["stopReason"], "max_tokens");
    assert_eq!(output["detail"]["text"], "é".repeat(500));
    assert_eq!(output["detail"]["textOmitted"], 20);

    fixture.write(&[
        prompt(1),
        call("a", &json!({ "command": "ls" }), 2),
        turn_end(3),
    ]);
    assert_eq!(fixture.observe(0, &[])["reason"], "noReply");
}

#[test]
fn reports_blocked_sessions_with_the_pending_tool() {
    let input = json!({ "command": "y".repeat(600) });
    let fixture = Fixture::new(&[prompt(1), call("a", &input, 2)]);
    let input = input.to_string();

    fixture.listing(&session(
        "working",
        Some("waiting"),
        Some("permission prompt"),
    ));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
            "state": "attention",
            "nextSince": 2,
            "reason": "blocked",
            "detail": {
                "tool": { "input": input[..500], "inputOmitted": input.len() - 500, "name": "Bash" },
                "waitingFor": "permission prompt",
            },
        })
    );

    fixture.listing(&session("blocked", Some("idle"), None));

    let output = fixture.observe(0, &[]);

    assert_eq!(output["reason"], "blocked");
    assert_eq!(output["detail"]["waitingFor"], Value::Null);
}

#[test]
fn reports_failed_and_stopped_turns() {
    let fixture = Fixture::new(&[prompt(1), call("a", &json!({ "command": "ls" }), 2)]);

    for state in ["failed", "stopped"] {
        fixture.listing(&session(state, None, None));
        assert_eq!(
            fixture.observe_raw(0, &[]),
            format!("{{\"state\":\"attention\",\"nextSince\":2,\"reason\":\"{state}\"}}\n")
        );
    }
}

#[test]
fn reports_waiting_sessions_as_blocked() {
    let fixture = Fixture::new(&completed_turn());

    fixture.listing(&session("working", Some("waiting"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({
            "state": "attention",
            "nextSince": 8,
            "reason": "blocked",
            "detail": { "tool": null, "waitingFor": null },
        })
    );

    fixture.write(&[prompt(1), call("a", &json!({ "command": "ls" }), 2)]);
    fixture.listing(&session("working", Some("busy"), Some("dialog open")));
    assert_eq!(
        fixture.observe(0, &[])["detail"]["waitingFor"],
        "dialog open"
    );
}

#[test]
fn waits_for_pending_agents_and_workflows() {
    let fixture = Fixture::new(&[]);

    fixture.listing(&session("done", None, None));

    for (field, count, state) in [
        ("pendingBackgroundAgentCount", 1, "working"),
        ("pendingWorkflowCount", 1, "working"),
        ("pendingBackgroundAgentCount", 0, "ready"),
        ("pendingWorkflowCount", 0, "ready"),
    ] {
        let mut records = completed_turn();

        records[6][field] = json!(count);
        fixture.write(&records);
        assert_eq!(fixture.observe(0, &[])["state"], state, "{field}: {count}");
    }
}

#[test]
fn reports_failed_replies_despite_pending_background_work() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new(&[
        prompt(1),
        call("a", &background, 2),
        result("a", false, 3),
        synthetic("API Error: 500", Some("server_error"), 4),
        turn_end(5),
    ]);

    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &[])["reason"], "noReply");

    fixture.write(&[
        prompt(1),
        call("a", &background, 2),
        result("a", false, 3),
        reply("b", "Started the tests", "end_turn", 4),
        turn_end(5),
    ]);
    fixture.listing(&session("stopped", None, None));
    assert_eq!(fixture.observe(0, &[])["reason"], "stopped");
}

#[test]
fn counts_sidechain_progress_without_changing_the_turn() {
    let fixture = Fixture::new(&[
        prompt(1),
        call("a", &json!({ "command": "ls" }), 2),
        synthetic("Interrupted", None, 9),
        sidechain(reply("s", "Sidechain finding", "end_turn", 4)),
        sidechain(result("s", false, 5)),
        sidechain(turn_end(6)),
        sidechain(prompt(7)),
    ]);

    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "state": "working", "nextSince": 7, "progressAt": at(5) })
    );
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
    let fixture = Fixture::new(&records);

    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(fixture.observe(0, &[])["state"], "working");

    records.push(notification("a", 6));
    fixture.write(&records);
    assert_eq!(
        fixture.observe(0, &[])["replies"],
        json!(["Started the tests"])
    );

    fixture.write(&[
        prompt(1),
        call("a", &background, 2),
        result("a", true, 3),
        reply("b", "The tests could not start", "end_turn", 4),
        turn_end(5),
    ]);
    assert_eq!(fixture.observe(0, &[])["state"], "ready");
}

#[test]
fn collects_the_reply_of_every_turn_since_the_prompt() {
    let fixture = Fixture::new(&[
        prompt(1),
        reply("a", "First", "end_turn", 2),
        turn_end(3),
        notification("x", 4),
        reply("b", "Second", "end_turn", 5),
        turn_end(6),
    ]);

    fixture.listing(&session("done", None, None));
    assert_eq!(
        fixture.observe(0, &[])["replies"],
        json!(["First", "Second"])
    );
}

#[test]
fn rejects_unexpected_session_lookups() {
    let fixture = Fixture::new(&completed_turn());
    let arguments = ["--transcript", fixture.path(), "--since", "0"];

    fixture.refuse(&arguments);

    for rows in [
        json!([{ "sessionId": OTHER_UUID, "state": "done" }]),
        json!([{ "sessionId": UUID, "state": "done" }, { "sessionId": UUID, "state": "done" }]),
        json!({ "sessionId": UUID, "state": "done" }),
    ] {
        fixture.listing(&rows);
        fixture.refuse(&arguments);
    }

    fs::write(fixture.bin.join("listing.json"), "not json").expect("The listing must be written");
    fixture.refuse(&arguments);
    refuse(Path::new("/nonexistent"), &arguments);
}

#[test]
fn counts_progress_from_subagents_started_during_the_turn() {
    let fixture = Fixture::new(&[
        prompt(10),
        call("a", &json!({ "command": "ls" }), 11),
        result("a", false, 12),
        call("task", &json!({ "prompt": "Review" }), 13),
    ]);
    let start = json!({ "type": "user", "message": { "content": "Review" }, "timestamp": at(15) });
    let earlier = json!({ "type": "user", "message": { "content": "Review" }, "timestamp": at(5) });

    fixture.listing(&session("working", Some("busy"), None));
    fixture.subagent(
        "agent-current.jsonl",
        &format!(
            "{start}\n{}\n{}\n",
            call("s", &json!({ "command": "ls" }), 16),
            result("s", false, 30)
        ),
    );
    fixture.subagent("agent-current.meta.json", "{}");
    fixture.subagent(
        "agent-earlier.jsonl",
        &format!("{earlier}\n{}\n", result("t", false, 40)),
    );
    assert_eq!(fixture.observe(0, &[])["progressAt"], at(30));

    fixture.subagent("agent-malformed.jsonl", "{\n");

    let stderr = fixture.refuse(&["--transcript", fixture.path(), "--since", "0"]);

    assert!(stderr.contains("agent-malformed.jsonl"), "{stderr}");
}

#[test]
fn waits_until_ready_or_the_deadline() {
    let fixture = Fixture::new(&completed_turn());
    let started = Instant::now();

    fixture.listing(&session("done", None, None));
    assert_eq!(fixture.observe(0, &["--wait", "60"])["state"], "ready");
    assert!(started.elapsed() < Duration::from_secs(10));

    fixture.write(&[prompt(1)]);
    fixture.listing(&session("working", Some("busy"), None));

    let started = Instant::now();

    assert_eq!(fixture.observe(0, &["--wait", "1"])["state"], "working");
    assert!(started.elapsed() >= Duration::from_secs(1));
    assert!(started.elapsed() < Duration::from_secs(10));
    assert_eq!(fixture.calls(), 2);
}

#[test]
fn resolves_relative_transcript_paths_from_the_working_directory() {
    let fixture = Fixture::new(&[prompt(1)]);
    let name = format!("{UUID}.jsonl");
    let directory = fixture
        .transcript
        .parent()
        .expect("The transcript must have a parent");
    let (status, stdout, stderr) = run(
        &fixture.bin,
        Some(directory),
        &["--transcript", &name, "--since", "1"],
    );

    assert_eq!((status, stderr.as_str()), (Some(0), ""));
    assert_eq!(
        serde_json::from_str::<Value>(&stdout).ok(),
        Some(json!({ "state": "working", "nextSince": 1, "progressAt": null }))
    );
}

#[test]
fn launches_from_the_domfiles_root_and_runs_in_the_caller_directory() {
    let fixture = Fixture::new(&[prompt(1)]);
    let caller = fixture
        .transcript
        .parent()
        .expect("The transcript must have a parent");
    let root = fs::canonicalize(env!("CARGO_MANIFEST_DIR")).expect("The root must resolve");
    let name = format!("{UUID}.jsonl");
    // The launcher also needs the provisioned `jq`, so it keeps the inherited search path after the stubs
    let path = format!(
        "{}:{}",
        fixture.bin.display(),
        env::var("PATH").unwrap_or_default()
    );
    let launch = |arguments: &[&str]| execute(LAUNCHER, &path, Some(caller), arguments);
    let reported = fixture.bin.join("reported");

    fs::create_dir_all(caller.join(".cargo")).expect("The caller configuration must be created");
    fs::write(
        caller.join(".cargo/config.toml"),
        "[build]\ntarget-dir = \"conflicting\"\n",
    )
    .expect("The caller configuration must be written");
    fixture.cargo(0, BINARY);

    assert_eq!(
        launch(&["--transcript", &name, "--since", "1"]),
        (
            Some(0),
            "{\"state\":\"working\",\"nextSince\":1,\"progressAt\":null}\n".to_owned(),
            "cargo: building\n".to_owned()
        )
    );
    assert_eq!(
        fixture.recorded("cargo-directory"),
        format!("{}\n", root.display())
    );
    assert_eq!(
        fixture.recorded("cargo-arguments"),
        "build\n--bin\nclaude-signal\n--locked\n--message-format=json-render-diagnostics\n--quiet\n"
    );

    let (status, stdout, stderr) = launch(&["--transcript", fixture.path(), "--since", "2"]);

    assert_eq!((status, stdout.as_str()), (Some(2), ""));
    assert_eq!(
        stderr,
        format!("cargo: building\n{NAME}: `--since` exceeds the transcript’s 1 complete lines\n")
    );

    fs::write(&reported, "#!/bin/sh\npwd -P\n").expect("The reported command must be written");
    fs::set_permissions(&reported, fs::Permissions::from_mode(0o755))
        .expect("The reported command must be executable");
    fixture.cargo(0, reported.to_str().expect("The path must be UTF-8"));
    assert_eq!(
        launch(&["--since", "1"]),
        (
            Some(0),
            format!(
                "{}\n",
                fs::canonicalize(caller)
                    .expect("The caller directory must resolve")
                    .display()
            ),
            "cargo: building\n".to_owned()
        )
    );

    for (executable, diagnostic) in [
        ("", "Cargo did not report an executable for `claude-signal`"),
        ("\"", "Failed to read Cargo’s build messages"),
    ] {
        fixture.cargo(0, executable);

        let (status, stdout, stderr) = launch(&["--since", "1"]);

        assert_eq!((status, stdout.as_str()), (Some(1), ""));
        assert!(stderr.contains(diagnostic), "{stderr}");
    }

    fixture.cargo(101, BINARY);
    assert_eq!(
        launch(&["--transcript", &name, "--since", "1"]),
        (Some(101), String::new(), "cargo: building\n".to_owned())
    );
}

#[test]
fn anchors_only_on_prompts_with_a_human_origin() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new(&[
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
    ]);

    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(
        fixture.observe(0, &[]),
        json!({ "state": "working", "nextSince": 8, "progressAt": at(7) })
    );
}

#[test]
fn keeps_the_lookup_interval_while_background_work_runs() {
    let background = json!({ "command": "pnpm test", "run_in_background": true });
    let fixture = Fixture::new(&[
        prompt(1),
        call("a", &background, 2),
        result("a", false, 3),
        reply("b", "Started the tests", "end_turn", 4),
        turn_end(5),
    ]);

    fixture.listing(&session("done", Some("idle"), None));
    assert_eq!(fixture.observe(0, &["--wait", "2"])["state"], "working");
    assert_eq!(fixture.calls(), 1);
}
