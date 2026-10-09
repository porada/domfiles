use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    thread,
    time::{Duration, Instant},
};

const DETAIL_LIMIT: usize = 500;
const HELP: &str = concat!(
    "Usage:\n",
    "  claude-signal --since <line> --transcript <path> [--wait <seconds>]\n",
    "  claude-signal --help\n",
    "\n",
    "Report whether the latest requested turn of one Claude background session is working, ready, or needs attention\n",
    "\n",
    "Options:\n",
    "  --since <line>       Complete transcript lines recorded before the latest dispatch, or `0` for a fresh launch. Keep it fixed for every check of that turn\n",
    "  --transcript <path>  The session’s `<uuid>.jsonl` transcript. Its UUID selects the session’s `claude agents --json --all` row\n",
    "  --wait <seconds>     Wait up to this many seconds, from `0` to `60`, for `ready` or `attention`. Defaults to `0`\n",
    "\n",
    "Output:\n",
    "  One JSON object on standard output with `state` and `nextSince`, the `--since` value for the next dispatch\n",
    "  `working` adds `progressAt`, the time of the latest progress, or `null` before the turn’s prompt is recorded\n",
    "  `ready` adds `replies`, each turn’s final reply since the prompt, and `compactedAt`, the latest compaction line or `null`\n",
    "  `attention` adds `reason`, one of `blocked`, `failed`, `noReply`, or `stopped`, and `detail` for `blocked` and `noReply`\n",
    "\n",
    "Exit statuses:\n",
    "  0  The turn was observed, or help was displayed\n",
    "  2  Invalid arguments, an unreadable or malformed transcript, or a failed session lookup\n",
);
const LISTING_INTERVAL: Duration = Duration::from_secs(10);
const MAX_WAIT_SECONDS: u64 = 60;
const NAME: &str = "claude-signal";
const OPTIONS: [&str; 3] = ["--since", "--transcript", "--wait"];
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

struct Options {
    since: usize,
    transcript: PathBuf,
    uuid: String,
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
        progress_at: Option<String>,
    },
}

struct Turn<'a> {
    closed: bool,
    final_message: Option<Message>,
    pending_work: bool,
    replies: Vec<String>,
    unresolved: Vec<&'a Value>,
}

fn parse_arguments(arguments: &[OsString]) -> Result<Route, String> {
    if let [argument] = arguments
        && argument == "--help"
    {
        return Ok(Route::Help);
    }

    let mut values: [Option<&OsString>; 3] = [None; 3];
    let mut remaining = arguments.iter();

    while let Some(argument) = remaining.next() {
        let text = argument.to_string_lossy();
        let Some(index) = OPTIONS.iter().position(|option| *option == text) else {
            return Err(if text == "--help" {
                "`--help` must be the only argument".to_owned()
            } else {
                format!("Unknown argument `{text}`")
            });
        };
        let option = OPTIONS[index];
        let value = remaining
            .next()
            .ok_or_else(|| format!("`{option}` requires a value"))?;

        if values[index].replace(value).is_some() {
            return Err(format!("`{option}` must be given only once"));
        }
    }

    let [since, transcript, wait] = values;
    let since = count(since.ok_or("`--since` is required")?, "--since")?;
    let transcript = PathBuf::from(transcript.ok_or("`--transcript` is required")?);
    let wait = match wait {
        Some(value) => count(value, "--wait")?,
        None => 0,
    };

    if wait > MAX_WAIT_SECONDS {
        return Err(format!("`--wait` must be at most {MAX_WAIT_SECONDS}"));
    }

    Ok(Route::Observe(Options {
        since: usize::try_from(since).map_err(|_| "`--since` is too large".to_owned())?,
        uuid: session_uuid(&transcript)?,
        transcript,
        wait: Duration::from_secs(wait),
    }))
}

fn count(value: &OsString, option: &str) -> Result<u64, String> {
    value
        .to_str()
        .filter(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| format!("`{option}` must be a nonnegative integer"))
}

fn session_uuid(transcript: &Path) -> Result<String, String> {
    transcript
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".jsonl"))
        .filter(|stem| is_uuid(stem))
        .map(str::to_owned)
        .ok_or_else(|| "`--transcript` must name a `<uuid>.jsonl` file".to_owned())
}

fn is_uuid(text: &str) -> bool {
    text.len() == 36
        && text.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

fn is_timestamp(text: &str) -> bool {
    text.len() == 24
        && text.bytes().enumerate().all(|(index, byte)| match index {
            4 | 7 => byte == b'-',
            10 => byte == b'T',
            13 | 16 => byte == b':',
            19 => byte == b'.',
            23 => byte == b'Z',
            _ => byte.is_ascii_digit(),
        })
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

    paths.sort();
    Ok(paths)
}

fn blocks(record: &Value) -> impl Iterator<Item = &Value> {
    record["message"]["content"]
        .as_array()
        .into_iter()
        .flatten()
}

fn timestamp(record: &Value) -> Option<&str> {
    record["timestamp"]
        .as_str()
        .filter(|text| is_timestamp(text))
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
                    notified.extend(notified_tool_uses(content));
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
        pending_work: background_pending || duration_pending,
        replies,
        unresolved,
    }
}

fn advance(latest: &mut Option<String>, record: &Value) {
    if let Some(timestamp) = timestamp(record)
        && latest.as_deref().is_none_or(|current| timestamp > current)
    {
        *latest = Some(timestamp.to_owned());
    }
}

// Progress is new reply text or a successful tool result. A result for a call that repeats an
// earlier identical call in the same transcript does not count
fn record_progress<'a>(records: impl IntoIterator<Item = &'a Value>, latest: &mut Option<String>) {
    let mut calls: HashSet<String> = HashSet::new();
    let mut repeats: HashSet<&str> = HashSet::new();

    for record in records {
        let qualifies = match record["type"].as_str() {
            Some("assistant") => {
                for block in blocks(record).filter(|block| block["type"] == "tool_use") {
                    if !calls.insert(format!("{}\0{}", block["name"], block["input"]))
                        && let Some(id) = block["id"].as_str()
                    {
                        repeats.insert(id);
                    }
                }

                record["message"]["model"] != SYNTHETIC_MODEL
                    && blocks(record).any(|block| {
                        block["type"] == "text"
                            && block["text"]
                                .as_str()
                                .is_some_and(|text| !text.trim().is_empty())
                    })
            }
            Some("user") => blocks(record).any(|block| {
                block["type"] == "tool_result"
                    && block["is_error"] != true
                    && block["tool_use_id"]
                        .as_str()
                        .is_some_and(|id| !repeats.contains(id))
            }),
            _ => false,
        };

        if qualifies {
            advance(latest, record);
        }
    }
}

fn progress(records: &[Value], anchor: usize, transcript: &Path) -> Result<Option<String>, String> {
    let mut latest = None;
    let after = &records[anchor + 1..];

    advance(&mut latest, &records[anchor]);
    record_progress(
        after.iter().filter(|record| !is_sidechain(record)),
        &mut latest,
    );
    record_progress(
        after.iter().filter(|record| is_sidechain(record)),
        &mut latest,
    );

    // Only subagents started during this turn count toward its progress
    if let Some(start) = timestamp(&records[anchor]) {
        for path in subagent_transcripts(transcript)? {
            let subagent = read_records(&path)?;

            if subagent
                .iter()
                .find_map(timestamp)
                .is_some_and(|first| first > start)
            {
                record_progress(&subagent, &mut latest);
            }
        }
    }

    Ok(latest)
}

fn read_listing(uuid: &str) -> Result<Listing, String> {
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
    let matches: Vec<&Value> = rows
        .as_array()
        .ok_or("`claude agents` did not print a JSON array")?
        .iter()
        .filter(|row| row["sessionId"] == uuid)
        .collect();
    let [row] = matches[..] else {
        return Err(format!(
            "Expected one `claude agents` row for session `{uuid}`, found {}",
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
    let waiting = listing.state.as_deref() == Some("blocked")
        || listing.status.as_deref() == Some("waiting")
        || listing.waiting_for.is_some();
    let tool = turn.unresolved.last().copied();

    if turn.closed {
        match turn.final_message {
            Some(message) if tool.is_none() && message.is_reply() => {}
            message => return Some(no_reply(message)),
        }

        // Pending background work keeps a replied turn unfinished
        if !turn.pending_work {
            return if waiting {
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

    if waiting {
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

fn render(signal: Signal, next_since: usize) -> String {
    let mut fields: Vec<(&str, Value)> = Vec::new();

    match signal {
        Signal::Attention { detail, reason } => {
            fields.extend([
                ("state", "attention".into()),
                ("nextSince", next_since.into()),
            ]);
            fields.push(("reason", reason.into()));
            fields.extend(detail.map(|detail| ("detail", detail)));
        }
        Signal::Ready {
            compacted_at,
            replies,
        } => {
            fields.extend([("state", "ready".into()), ("nextSince", next_since.into())]);
            fields.extend([
                ("replies", replies.into()),
                ("compactedAt", compacted_at.into()),
            ]);
        }
        Signal::Working { progress_at } => {
            fields.extend([
                ("state", "working".into()),
                ("nextSince", next_since.into()),
            ]);
            fields.push(("progressAt", progress_at.into()));
        }
    }

    let fields: Vec<String> = fields
        .into_iter()
        .map(|(key, value)| format!("{}:{value}", Value::from(key)))
        .collect();

    format!("{{{}}}\n", fields.join(","))
}

fn observe(options: &Options) -> Result<String, String> {
    let deadline = Instant::now() + options.wait;
    let mut checked_at: Option<Instant> = None;
    let mut listing = Listing::default();

    loop {
        let records = read_records(&options.transcript)?;

        if options.since > records.len() {
            return Err(format!(
                "`--since` exceeds the transcript’s {} complete lines",
                records.len()
            ));
        }

        let anchor = (options.since..records.len())
            .rev()
            .find(|index| is_prompt(&records[*index]));
        let signal = match anchor {
            None => Signal::Working { progress_at: None },
            Some(anchor) => {
                let turn = analyze_turn(&records[anchor + 1..]);

                // A closed turn without pending work needs fresh session state to become ready
                let settled = turn.closed && !turn.pending_work;

                if checked_at.is_none_or(|at| settled || at.elapsed() >= LISTING_INTERVAL) {
                    listing = read_listing(&options.uuid)?;
                    checked_at = Some(Instant::now());
                }

                match decide(turn, &listing, compacted_at(&records)) {
                    Some(signal) => signal,
                    None => Signal::Working {
                        progress_at: progress(&records, anchor, &options.transcript)?,
                    },
                }
            }
        };
        let now = Instant::now();

        if !matches!(signal, Signal::Working { .. }) || now >= deadline {
            return Ok(render(signal, records.len()));
        }

        thread::sleep(POLL_INTERVAL.min(deadline - now));
    }
}

fn run(arguments: &[OsString]) -> Result<String, String> {
    match parse_arguments(arguments)? {
        Route::Help => Ok(HELP.to_owned()),
        Route::Observe(options) => observe(&options),
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
