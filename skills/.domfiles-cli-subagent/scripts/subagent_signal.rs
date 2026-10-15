use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::OsString,
    fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    str::FromStr,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const DETAIL_LIMIT: usize = 500;
const LISTING_INTERVAL: Duration = Duration::from_secs(10);
const MAX_WAIT_SECONDS: u64 = 90;
const NAME: &str = "subagent-signal";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const SYNTHETIC_MODEL: &str = "<synthetic>";

#[derive(Default)]
struct Listing {
    state: Option<String>,
    status: Option<String>,
    waiting_for: Option<String>,
}

#[derive(Default)]
struct Message {
    error: Option<String>,
    model: Option<String>,
    stop_reason: Option<String>,
    text: String,
}

impl Message {
    fn is_reply(&self) -> bool {
        self.model.as_deref() != Some(SYNTHETIC_MODEL)
            && self.stop_reason.as_deref() == Some("end_turn")
            && !self.text.trim().is_empty()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Provider {
    Claude,
    Codex,
}

struct Session {
    provider: Provider,
    since: usize,
    uuid: String,
}

struct Options {
    sessions: Vec<Session>,
    wait: Duration,
}

enum Route {
    Help,
    Observe(Options),
}

enum Signal {
    Attention {
        detail: Option<Value>,
        reason: &'static str,
    },
    Ready {
        compacted_at: Option<usize>,
        replies: Vec<String>,
    },
    Working {
        progress_at: Option<u64>,
    },
}

enum Observation {
    Error(String),
    Observed { next_since: usize, signal: Signal },
}

impl Observation {
    fn is_working(&self) -> bool {
        matches!(
            self,
            Self::Observed {
                signal: Signal::Working { .. },
                ..
            }
        )
    }
}

struct Transcript {
    anchor: Option<usize>,
    path: PathBuf,
    records: Vec<Value>,
}

struct Turn<'a> {
    closed: bool,
    final_message: Option<Message>,
    pending_work: bool,
    replies: Vec<String>,
    unresolved: Vec<&'a Value>,
}

fn help() -> String {
    format!(
        concat!(
            "Usage:\n",
            "  subagent-signal [--wait <seconds>] (--claude <since>:<uuid> | --codex <since>:<uuid>)...\n",
            "  subagent-signal --help\n",
            "\n",
            "Report whether the latest requested turn of each listed Claude or Codex background session is working, ready, or needs attention. Return as soon as any session leaves `working`\n",
            "\n",
            "Options:\n",
            "  --claude <since>:<uuid>  A Claude session to observe, given once per session. `<since>` is the number of complete transcript lines recorded before the latest dispatch, or `0` for a fresh launch. Keep it fixed for every check of that turn. `<uuid>` selects the `~/.claude/projects/*/<uuid>.jsonl` transcript and the session’s `claude agents --json --all` row\n",
            "  --codex <since>:<uuid>   A Codex session to observe, given once per session. `<since>` works as for `--claude`. `<uuid>` selects the `~/.codex/sessions/*/*/*/rollout-*-<uuid>.jsonl` rollout\n",
            "  --wait <seconds>         Wait up to this many seconds, from `0` to `{max_wait_seconds}`, for any session to become `ready`, need `attention`, or report an `error`. Defaults to `{max_wait_seconds}`\n",
            "\n",
            "Output:\n",
            "  One JSON object on standard output with `observedAt`, the observation time in Unix milliseconds, and `sessions`, one entry per session in the given order\n",
            "  Each entry has `session` and `state`. Every state except `error` adds `nextSince`, the `<since>` value for the next dispatch\n",
            "  `working` adds `progressAt`, the latest write to the session’s transcripts in Unix milliseconds, or `null` before the turn start is recorded\n",
            "  `ready` adds `replies`, each turn’s final reply since the turn start, and `compactedAt`, the latest compaction line or `null`\n",
            "  `attention` adds `reason`, one of `blocked`, `failed`, `noReply`, or `stopped`, and `detail` for `blocked` and `noReply`. Codex sessions report only `noReply`, whose `detail` has `abortReason`, `error`, and `errorOmitted`\n",
            "  `error` adds `message`, the reason the session could not be observed, such as a missing transcript when `<since>` is above `0`\n",
            "\n",
            "Exit statuses:\n",
            "  0  The sessions were observed, including any `error` entries, or help was displayed\n",
            "  2  Invalid arguments or a failed write to standard output\n",
        ),
        max_wait_seconds = MAX_WAIT_SECONDS,
    )
}

fn parse_arguments(arguments: &[OsString]) -> Result<Route, String> {
    if let [argument] = arguments
        && argument == "--help"
    {
        return Ok(Route::Help);
    }

    let mut sessions: Vec<Session> = Vec::new();
    let mut wait: Option<u64> = None;
    let mut remaining = arguments.iter();

    while let Some(argument) = remaining.next() {
        let option = match argument.to_string_lossy().as_ref() {
            "--claude" => "--claude",
            "--codex" => "--codex",
            "--wait" => "--wait",
            "--help" => return Err("`--help` must be the only argument".to_owned()),
            text => return Err(format!("Unknown argument `{text}`")),
        };
        let value = remaining
            .next()
            .ok_or_else(|| format!("`{option}` requires a value"))?
            .to_str()
            .ok_or_else(|| format!("The `{option}` value must be valid UTF-8"))?;

        if option == "--wait" {
            if wait.replace(count(value, "--wait")?).is_some() {
                return Err("`--wait` must be given only once".to_owned());
            }
            continue;
        }

        let provider = if option == "--claude" {
            Provider::Claude
        } else {
            Provider::Codex
        };
        let session = parse_session(value, provider, option)?;

        if sessions
            .iter()
            .any(|existing| existing.uuid == session.uuid)
        {
            return Err(format!(
                "Session `{}` must be given only once",
                session.uuid
            ));
        }

        sessions.push(session);
    }

    if sessions.is_empty() {
        return Err("At least one `--claude` or `--codex` session is required".to_owned());
    }

    let wait = wait.unwrap_or(MAX_WAIT_SECONDS);

    if wait > MAX_WAIT_SECONDS {
        return Err(format!("`--wait` must be at most {MAX_WAIT_SECONDS}"));
    }

    Ok(Route::Observe(Options {
        sessions,
        wait: Duration::from_secs(wait),
    }))
}

fn parse_session(value: &str, provider: Provider, option: &str) -> Result<Session, String> {
    let (since, uuid) = value
        .split_once(':')
        .filter(|(_, uuid)| is_uuid(uuid))
        .ok_or_else(|| format!("`{option}` must be `<since>:<uuid>` with a full session UUID"))?;

    Ok(Session {
        provider,
        since: count(since, "<since>")?,
        uuid: uuid.to_ascii_lowercase(),
    })
}

fn count<T: FromStr>(text: &str, name: &str) -> Result<T, String> {
    Some(text)
        .filter(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| format!("`{name}` must be a non-negative integer"))
}

fn is_uuid(text: &str) -> bool {
    text.len() == 36
        && text.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

fn millis(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).map_or(0, |elapsed| {
        u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
    })
}

fn home_directory() -> Result<PathBuf, String> {
    env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| "`HOME` is not set".to_owned())
}

fn child_directories(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(format!("Failed to read `{}`: {error}", directory.display()));
        }
    };
    let mut directories = Vec::new();

    for entry in entries {
        let failure =
            |error: io::Error| format!("Failed to read `{}`: {error}", directory.display());
        let entry = entry.map_err(failure)?;

        if entry.file_type().map_err(failure)?.is_dir() {
            directories.push(entry.path());
        }
    }

    directories.sort();
    Ok(directories)
}

// Codex files each rollout under the `YYYY/MM/DD` directory of its local creation date
fn date_directories(sessions: &Path) -> Result<Vec<PathBuf>, String> {
    let mut days = Vec::new();

    for year in child_directories(sessions)? {
        for month in child_directories(&year)? {
            days.extend(child_directories(&month)?);
        }
    }

    Ok(days)
}

fn find_rollout(sessions: &Path, uuid: &str) -> Result<Option<PathBuf>, String> {
    let mut matches = rollouts(sessions, &format!("-{uuid}.jsonl"))?;

    match matches.len() {
        0 | 1 => Ok(matches.pop()),
        found => Err(format!("Found {found} rollouts for the session")),
    }
}

fn rollouts(sessions: &Path, suffix: &str) -> Result<Vec<PathBuf>, String> {
    let mut matches = Vec::new();

    for day in date_directories(sessions)? {
        for entry in fs::read_dir(&day)
            .map_err(|error| format!("Failed to read `{}`: {error}", day.display()))?
        {
            let path = entry
                .map_err(|error| format!("Failed to read `{}`: {error}", day.display()))?
                .path();

            if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("rollout-") && name.ends_with(suffix))
            {
                matches.push(path);
            }
        }
    }

    Ok(matches)
}

fn find_transcript(projects: &Path, uuid: &str) -> Result<Option<PathBuf>, String> {
    let entries = match fs::read_dir(projects) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!("Failed to read `{}`: {error}", projects.display()));
        }
    };
    let name = format!("{uuid}.jsonl");
    let mut matches = Vec::new();

    for entry in entries {
        let candidate = entry
            .map_err(|error| format!("Failed to read `{}`: {error}", projects.display()))?
            .path()
            .join(&name);

        match fs::metadata(&candidate) {
            Ok(metadata) if metadata.is_file() => matches.push(candidate),
            Ok(_) => {}
            // A plain file such as `.DS_Store` cannot hold a project’s transcripts
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotADirectory | io::ErrorKind::NotFound
                ) => {}
            Err(error) => {
                return Err(format!("Failed to read `{}`: {error}", candidate.display()));
            }
        }
    }

    match matches.len() {
        0 | 1 => Ok(matches.pop()),
        found => Err(format!("Found {found} transcripts for the session")),
    }
}

fn read_records(path: &Path) -> Result<Vec<Value>, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("Failed to read `{}`: {error}", path.display()))?;
    let mut lines: Vec<&[u8]> = bytes.split(|byte| *byte == b'\n').collect();

    // An unterminated final record may still be in progress, so a later check reads it
    lines.pop();

    lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_slice::<Value>(line)
                .ok()
                .filter(Value::is_object)
                .ok_or_else(|| {
                    format!(
                        "Malformed JSON record at line {} of `{}`",
                        index + 1,
                        path.display()
                    )
                })
        })
        .collect()
}

fn subagent_transcripts(transcript: &Path) -> Result<Vec<PathBuf>, String> {
    let directory = transcript.with_extension("").join("subagents");
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(format!("Failed to read `{}`: {error}", directory.display()));
        }
    };
    let mut paths = Vec::new();

    for entry in entries {
        let path = entry
            .map_err(|error| format!("Failed to read `{}`: {error}", directory.display()))?
            .path();

        if path
            .extension()
            .is_some_and(|extension| extension == "jsonl")
        {
            paths.push(path);
        }
    }

    Ok(paths)
}

fn modified(path: &Path) -> Result<u64, String> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map(millis)
        .map_err(|error| format!("Failed to read `{}`: {error}", path.display()))
}

// Any write by the session or its subagents counts as progress. Older subagent transcripts were
// last written before this turn’s prompt, so they never move the result. A subagent transcript
// removed after the listing, or a dangling link, has no write to report
fn latest_write(transcript: &Path) -> Result<u64, String> {
    let mut latest = modified(transcript)?;

    for path in subagent_transcripts(transcript)? {
        match fs::metadata(&path).and_then(|metadata| metadata.modified()) {
            Ok(time) => latest = latest.max(millis(time)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Failed to read `{}`: {error}", path.display())),
        }
    }

    Ok(latest)
}

fn blocks(record: &Value) -> impl Iterator<Item = &Value> {
    record["message"]["content"]
        .as_array()
        .into_iter()
        .flatten()
}

fn is_sidechain(record: &Value) -> bool {
    record["isSidechain"] == true
}

// Compaction summaries and command output are user records without a human origin
fn is_prompt(record: &Value) -> bool {
    record["type"] == "user"
        && record["isMeta"] != true
        && !is_sidechain(record)
        && record["origin"]["kind"] == "human"
}

fn notified_tool_uses(content: &str) -> impl Iterator<Item = &str> {
    content
        .split("<tool-use-id>")
        .skip(1)
        .filter_map(|tail| tail.split_once("</tool-use-id>"))
        .map(|(id, _)| id)
}

fn analyze_turn(records: &[Value]) -> Turn<'_> {
    let mut finals: Vec<Option<String>> = Vec::new();
    let mut last_activity = None;
    let mut last_duration: Option<(usize, &Value)> = None;
    let mut last_notification = None;
    let mut messages: HashMap<String, Message> = HashMap::new();
    let mut notified: HashSet<&str> = HashSet::new();
    let mut results: HashMap<&str, bool> = HashMap::new();
    let mut segment_message: Option<String> = None;
    let mut tool_uses: Vec<&Value> = Vec::new();

    for (index, record) in records
        .iter()
        .enumerate()
        .filter(|(_, record)| !is_sidechain(record))
    {
        match record["type"].as_str() {
            Some("assistant") => {
                let key = record["message"]["id"]
                    .as_str()
                    .map_or_else(|| format!("record {index}"), str::to_owned);
                let message = messages.entry(key.clone()).or_default();

                if let Some(model) = record["message"]["model"].as_str() {
                    message.model = Some(model.to_owned());
                }
                if let Some(reason) = record["message"]["stop_reason"].as_str() {
                    message.stop_reason = Some(reason.to_owned());
                }
                if let Some(error) = record["error"].as_str() {
                    message.error = Some(error.to_owned());
                }

                for block in blocks(record) {
                    match block["type"].as_str() {
                        Some("text") => {
                            if let Some(text) = block["text"].as_str() {
                                if !message.text.is_empty() {
                                    message.text.push_str("\n\n");
                                }
                                message.text.push_str(text);
                            }
                        }
                        Some("tool_use") => tool_uses.push(block),
                        _ => {}
                    }
                }

                last_activity = Some(index);
                segment_message = Some(key);
            }
            Some("queue-operation") => {
                if let Some(content) = record["content"].as_str() {
                    let mut ids = notified_tool_uses(content).peekable();

                    if record["operation"] == "enqueue" && ids.peek().is_some() {
                        last_notification = Some(index);
                    }
                    notified.extend(ids);
                }
            }
            Some("system") if record["subtype"] == "turn_duration" => {
                finals.push(segment_message.take());
                last_duration = Some((index, record));
            }
            Some("user") => {
                for block in blocks(record).filter(|block| block["type"] == "tool_result") {
                    if let Some(id) = block["tool_use_id"].as_str() {
                        results.insert(id, block["is_error"] == true);
                        last_activity = Some(index);
                    }
                }
            }
            _ => {}
        }
    }

    let closed = last_duration
        .is_some_and(|(index, _)| last_activity.is_none_or(|activity| index > activity));
    let unresolved: Vec<&Value> = tool_uses
        .iter()
        .copied()
        .filter(|block| {
            block["id"]
                .as_str()
                .is_none_or(|id| !results.contains_key(id))
        })
        .collect();
    let background_pending = tool_uses.iter().any(|block| {
        block["input"]["run_in_background"] == true
            && block["id"]
                .as_str()
                .is_some_and(|id| results.get(id) == Some(&false) && !notified.contains(id))
    });
    let duration_pending = last_duration.is_some_and(|(_, record)| {
        ["pendingBackgroundAgentCount", "pendingWorkflowCount"]
            .iter()
            .any(|field| record[*field].as_u64().is_some_and(|count| count > 0))
    });
    // Claude answers a notification that arrives after its turn ended with a new turn
    let notification_pending = last_notification.is_some_and(|notification| {
        last_duration.is_none_or(|(duration, _)| duration < notification)
    });
    let replies = finals
        .iter()
        .flatten()
        .filter_map(|key| messages.get(key))
        .filter(|message| message.is_reply())
        .map(|message| message.text.clone())
        .collect();
    let final_message = finals
        .last()
        .cloned()
        .flatten()
        .and_then(|key| messages.remove(&key));

    Turn {
        closed,
        final_message,
        pending_work: background_pending || duration_pending || notification_pending,
        replies,
        unresolved,
    }
}

fn read_listing() -> Result<Vec<Value>, String> {
    let output = Command::new("claude")
        .args(["agents", "--json", "--all"])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("Failed to run `claude agents`: {error}"))?;

    if !output.status.success() {
        return Err(format!("`claude agents` failed with {}", output.status));
    }

    let rows: Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| "`claude agents` printed invalid JSON".to_owned())?;

    match rows {
        Value::Array(rows) => Ok(rows),
        _ => Err("`claude agents` did not print a JSON array".to_owned()),
    }
}

fn listing_for(rows: &[Value], uuid: &str) -> Result<Listing, String> {
    let matches: Vec<&Value> = rows.iter().filter(|row| row["sessionId"] == uuid).collect();
    let [row] = matches[..] else {
        return Err(format!(
            "Expected one `claude agents` row for the session, found {}",
            matches.len()
        ));
    };
    let field = |name: &str| {
        row[name]
            .as_str()
            .filter(|text| !text.is_empty())
            .map(str::to_owned)
    };

    Ok(Listing {
        state: field("state"),
        status: field("status"),
        waiting_for: field("waitingFor"),
    })
}

fn truncate(text: &str) -> (String, usize) {
    let total = text.chars().count();

    (
        text.chars().take(DETAIL_LIMIT).collect(),
        total.saturating_sub(DETAIL_LIMIT),
    )
}

fn blocked(tool: Option<&Value>, listing: &Listing) -> Signal {
    let tool = tool.map_or(Value::Null, |block| {
        let (input, omitted) = truncate(&block["input"].to_string());

        json!({ "input": input, "inputOmitted": omitted, "name": block["name"] })
    });

    Signal::Attention {
        detail: Some(json!({ "tool": tool, "waitingFor": listing.waiting_for })),
        reason: "blocked",
    }
}

fn no_reply(message: Option<Message>) -> Signal {
    let message = message.unwrap_or_default();
    let (text, omitted) = truncate(&message.text);

    Signal::Attention {
        detail: Some(json!({
            "error": message.error,
            "model": message.model,
            "stopReason": message.stop_reason,
            "text": text,
            "textOmitted": omitted,
        })),
        reason: "noReply",
    }
}

// Returns `None` while the turn is still working
fn decide(turn: Turn<'_>, listing: &Listing, compacted_at: Option<usize>) -> Option<Signal> {
    // Only an open prompt counts as blocked. A `blocked` session state alone can mean that a reply
    // asks a question, which the coordinator reads in the reply itself
    let prompting = listing.status.as_deref() == Some("waiting") || listing.waiting_for.is_some();
    let tool = turn.unresolved.last().copied();

    if turn.closed {
        match turn.final_message {
            Some(message) if tool.is_none() && message.is_reply() => {}
            message => return Some(no_reply(message)),
        }

        // Pending background work keeps a replied turn unfinished
        if !turn.pending_work {
            return if prompting {
                Some(blocked(None, listing))
            } else if listing.status.as_deref() == Some("busy") {
                None
            } else {
                Some(Signal::Ready {
                    compacted_at,
                    replies: turn.replies,
                })
            };
        }
    }

    if prompting {
        return Some(blocked(tool, listing));
    }

    let reason = match listing.state.as_deref() {
        Some("failed") => "failed",
        Some("stopped") => "stopped",
        _ => return None,
    };

    Some(Signal::Attention {
        detail: None,
        reason,
    })
}

fn compacted_at(records: &[Value]) -> Option<usize> {
    records
        .iter()
        .rposition(|record| record["type"] == "system" && record["subtype"] == "compact_boundary")
        .map(|index| index + 1)
}

fn is_turn_start(record: &Value) -> bool {
    record["type"] == "event_msg" && record["payload"]["type"] == "task_started"
}

fn is_turn_end(record: &Value) -> bool {
    record["type"] == "event_msg"
        && (record["payload"]["type"] == "task_complete"
            || record["payload"]["type"] == "turn_aborted")
}

fn codex_reply(record: &Value) -> Option<&str> {
    Some(&record["payload"])
        .filter(|payload| payload["type"] == "task_complete")
        .and_then(|payload| payload["last_agent_message"].as_str())
        .filter(|text| !text.trim().is_empty())
}

// Returns the first record once its line is complete, so a file still being created or removed is
// skipped until a later check. A complete line that is not JSON identifies no session
fn first_record(path: &Path) -> Result<Option<Value>, String> {
    let failure = |error: io::Error| format!("Failed to read `{}`: {error}", path.display());
    let mut line = Vec::new();
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(failure(error)),
    };

    BufReader::new(file)
        .read_until(b'\n', &mut line)
        .map_err(failure)?;

    if line.last() != Some(&b'\n') {
        return Ok(None);
    }

    Ok(Some(serde_json::from_slice(&line).unwrap_or_default()))
}

// Descendants at any depth carry the root thread’s `session_id`, and only those started during
// this turn count. A turn shorter than a day spans at most the two newest date directories besides
// the root’s own. Each file’s start time is cached, so a newer turn start still filters it
fn latest_codex_write(
    root: &Path,
    uuid: &str,
    start: &str,
    sessions: &Path,
    starts: &mut HashMap<PathBuf, Option<String>>,
) -> Result<u64, String> {
    let mut latest = modified(root)?;
    let mut days = date_directories(sessions)?;
    let recent = days.split_off(days.len().saturating_sub(2));

    for day in recent
        .iter()
        .map(PathBuf::as_path)
        .chain(root.parent())
        .collect::<HashSet<&Path>>()
    {
        let failure = |error: io::Error| format!("Failed to read `{}`: {error}", day.display());

        for entry in fs::read_dir(day).map_err(failure)? {
            let path = entry.map_err(failure)?.path();

            if path == root
                || path
                    .extension()
                    .is_none_or(|extension| extension != "jsonl")
            {
                continue;
            }

            if !starts.contains_key(&path) {
                let Some(record) = first_record(&path)? else {
                    continue;
                };
                let started = Some(&record)
                    .filter(|record| {
                        record["type"] == "session_meta"
                            && record["payload"]["session_id"] == uuid
                            && record["payload"]["id"] != uuid
                    })
                    .and_then(|record| record["timestamp"].as_str())
                    .map(str::to_owned);

                starts.insert(path.clone(), started);
            }

            if starts[&path].as_deref().is_some_and(|time| time >= start) {
                match fs::metadata(&path).and_then(|metadata| metadata.modified()) {
                    Ok(time) => latest = latest.max(millis(time)),
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(format!("Failed to read `{}`: {error}", path.display()));
                    }
                }
            }
        }
    }

    Ok(latest)
}

fn observe_codex(
    transcript: &Transcript,
    anchor: usize,
    uuid: &str,
    sessions: &Path,
    starts: &mut HashMap<PathBuf, Option<String>>,
) -> Result<Signal, String> {
    let records = &transcript.records;
    let after = &records[anchor + 1..];

    if let Some(end) = after.iter().rfind(|record| is_turn_end(record)) {
        if codex_reply(end).is_some() {
            return Ok(Signal::Ready {
                compacted_at: records
                    .iter()
                    .rposition(|record| record["type"] == "compacted")
                    .map(|index| index + 1),
                replies: after
                    .iter()
                    .filter_map(codex_reply)
                    .map(str::to_owned)
                    .collect(),
            });
        }

        let (error, omitted) = end["payload"]["error"]["message"]
            .as_str()
            .map(truncate)
            .map_or((None, 0), |(error, omitted)| (Some(error), omitted));

        return Ok(Signal::Attention {
            detail: Some(json!({
                "abortReason": end["payload"]["reason"].as_str(),
                "error": error,
                "errorOmitted": omitted,
            })),
            reason: "noReply",
        });
    }

    let progress_at = match records[anchor]["timestamp"].as_str() {
        Some(start) => latest_codex_write(&transcript.path, uuid, start, sessions, starts)?,
        // Without the turn’s start time, no subagent rollout can be attributed to this turn
        None => modified(&transcript.path)?,
    };

    Ok(Signal::Working {
        progress_at: Some(progress_at),
    })
}

// Every check finds the transcript again, so a later duplicate or a removed file is reported
fn read_transcript(
    session: &Session,
    home: &Result<PathBuf, String>,
) -> Result<Option<Transcript>, String> {
    let home = home.as_ref()?;
    let sessions = home.join(".codex/sessions");
    let found = match session.provider {
        Provider::Claude => find_transcript(&home.join(".claude/projects"), &session.uuid)?,
        Provider::Codex => find_rollout(&sessions, &session.uuid)?,
    };
    let Some(path) = found else {
        // Only a fresh launch may precede its transcript, because a follow-up appends to it.
        // Codex compresses a rollout that stays idle for a week, and a resumption restores the plain
        // file before appending, so a compressed-only rollout is still pending
        if session.since == 0
            || (matches!(session.provider, Provider::Codex)
                && !rollouts(&sessions, &format!("-{}.jsonl.zst", session.uuid))?.is_empty())
        {
            return Ok(None);
        }
        return Err("Found no transcript, but `<since>` is above 0".to_owned());
    };
    let records = read_records(&path)?;

    if session.since > records.len() {
        return Err(format!(
            "`<since>` exceeds the transcript’s {} complete lines",
            records.len()
        ));
    }

    let anchor = (session.since..records.len())
        .rev()
        .find(|index| match session.provider {
            Provider::Claude => is_prompt(&records[*index]),
            Provider::Codex => is_turn_start(&records[*index]),
        });

    Ok(Some(Transcript {
        anchor,
        path,
        records,
    }))
}

fn observe_turn(
    transcript: &Transcript,
    turn: Turn<'_>,
    rows: Option<&Result<Vec<Value>, String>>,
    uuid: &str,
) -> Result<Signal, String> {
    let rows = rows
        .ok_or_else(|| "The session lookup did not run".to_owned())?
        .as_ref()
        .map_err(String::clone)?;

    match decide(
        turn,
        &listing_for(rows, uuid)?,
        compacted_at(&transcript.records),
    ) {
        Some(signal) => Ok(signal),
        None => Ok(Signal::Working {
            progress_at: Some(latest_write(&transcript.path)?),
        }),
    }
}

fn render(uuid: &str, observation: Observation) -> String {
    let mut fields: Vec<(&str, Value)> = vec![("session", uuid.into())];

    match observation {
        Observation::Error(message) => {
            fields.extend([("state", "error".into()), ("message", message.into())]);
        }
        Observation::Observed { next_since, signal } => match signal {
            Signal::Attention { detail, reason } => {
                fields.extend([
                    ("state", "attention".into()),
                    ("nextSince", next_since.into()),
                    ("reason", reason.into()),
                ]);
                fields.extend(detail.map(|detail| ("detail", detail)));
            }
            Signal::Ready {
                compacted_at,
                replies,
            } => {
                fields.extend([
                    ("state", "ready".into()),
                    ("nextSince", next_since.into()),
                    ("replies", replies.into()),
                    ("compactedAt", compacted_at.into()),
                ]);
            }
            Signal::Working { progress_at } => {
                fields.extend([
                    ("state", "working".into()),
                    ("nextSince", next_since.into()),
                    ("progressAt", progress_at.into()),
                ]);
            }
        },
    }

    let fields: Vec<String> = fields
        .into_iter()
        .map(|(key, value)| format!("{}:{value}", Value::from(key)))
        .collect();

    format!("{{{}}}", fields.join(","))
}

fn observe(options: &Options) -> String {
    let deadline = Instant::now() + options.wait;
    let home = home_directory();
    let sessions_directory = home
        .as_ref()
        .map(|home| home.join(".codex/sessions"))
        .unwrap_or_default();
    let mut starts: Vec<HashMap<PathBuf, Option<String>>> =
        vec![HashMap::new(); options.sessions.len()];
    let mut checked_at: Option<Instant> = None;
    let mut looked_up: Vec<Option<usize>> = vec![None; options.sessions.len()];
    let mut rows: Option<Result<Vec<Value>, String>> = None;

    loop {
        let transcripts: Vec<Result<Option<Transcript>, String>> = options
            .sessions
            .iter()
            .map(|session| read_transcript(session, &home))
            .collect();
        let claude_anchor =
            |session: &Session, transcript: &Result<Option<Transcript>, String>| match (
                session.provider,
                transcript,
            ) {
                (Provider::Claude, Ok(Some(transcript))) => transcript.anchor,
                _ => None,
            };
        let turns: Vec<Option<Turn<'_>>> = options
            .sessions
            .iter()
            .zip(&transcripts)
            .map(|(session, transcript)| {
                let anchor = claude_anchor(session, transcript)?;
                let records = &transcript.as_ref().ok()?.as_ref()?.records;

                Some(analyze_turn(&records[anchor + 1..]))
            })
            .collect();
        let anchors: Vec<Option<usize>> = options
            .sessions
            .iter()
            .zip(&transcripts)
            .map(|(session, transcript)| claude_anchor(session, transcript))
            .collect();

        // One lookup serves every session. A closed turn without pending work needs fresh session
        // state to become ready, and a newly recorded prompt needs state from after that prompt
        if turns.iter().any(Option::is_some)
            && (anchors != looked_up
                || checked_at.is_none_or(|at| {
                    at.elapsed() >= LISTING_INTERVAL
                        || turns
                            .iter()
                            .flatten()
                            .any(|turn| turn.closed && !turn.pending_work)
                }))
        {
            rows = Some(read_listing());
            checked_at = Some(Instant::now());
            looked_up = anchors;
        }

        let observations: Vec<Observation> = options
            .sessions
            .iter()
            .zip(&transcripts)
            .zip(turns)
            .zip(starts.iter_mut())
            .map(|(((session, transcript), turn), starts)| {
                let transcript = match transcript {
                    Err(message) => return Observation::Error(message.clone()),
                    Ok(None) => {
                        return Observation::Observed {
                            next_since: session.since,
                            signal: Signal::Working { progress_at: None },
                        };
                    }
                    Ok(Some(transcript)) => transcript,
                };
                let signal = match (session.provider, transcript.anchor, turn) {
                    (Provider::Claude, _, Some(turn)) => {
                        observe_turn(transcript, turn, rows.as_ref(), &session.uuid)
                    }
                    (Provider::Codex, Some(anchor), _) => observe_codex(
                        transcript,
                        anchor,
                        &session.uuid,
                        &sessions_directory,
                        starts,
                    ),
                    _ => Ok(Signal::Working { progress_at: None }),
                };

                match signal {
                    Ok(signal) => Observation::Observed {
                        next_since: transcript.records.len(),
                        signal,
                    },
                    Err(message) => Observation::Error(message),
                }
            })
            .collect();
        let now = Instant::now();

        if !observations.iter().all(Observation::is_working) || now >= deadline {
            let sessions: Vec<String> = options
                .sessions
                .iter()
                .zip(observations)
                .map(|(session, observation)| render(&session.uuid, observation))
                .collect();

            return format!(
                "{{\"observedAt\":{},\"sessions\":[{}]}}\n",
                millis(SystemTime::now()),
                sessions.join(",")
            );
        }

        thread::sleep(POLL_INTERVAL.min(deadline - now));
    }
}

fn run(arguments: &[OsString]) -> Result<String, String> {
    match parse_arguments(arguments)? {
        Route::Help => Ok(help()),
        Route::Observe(options) => Ok(observe(&options)),
    }
}

fn main() -> ExitCode {
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    let diagnostic = match run(&arguments) {
        Ok(output) => {
            let mut stdout = io::stdout().lock();
            match stdout
                .write_all(output.as_bytes())
                .and_then(|()| stdout.flush())
            {
                Ok(()) => return ExitCode::SUCCESS,
                Err(error) => format!("Failed to write to standard output: {error}"),
            }
        }
        Err(diagnostic) => diagnostic,
    };

    // The failure status still reports the error when standard error rejects the diagnostic
    let _ = writeln!(io::stderr(), "{NAME}: {diagnostic}");

    ExitCode::from(2)
}
