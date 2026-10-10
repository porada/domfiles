# Claude CLI

## Apply Runtime Configuration

Use the installed `claude` command through `PATH` and the existing subscription login. Authentication uses ordinary non-disclosing CLI operations, not credential inspection or alternate authentication sources. Resolve availability, login, and configuration failures through [Handle Errors and Stop](#handle-errors-and-stop).

Global instruction alignment belongs to domfiles provisioning. Normal Claude startup loads the canonical global instructions through its managed installation. Do not audit that setup on each exchange, use `--bare`, replace the system prompt, or paste the global instructions into assignments. Repairing provisioning is a separate task.

Use `--permission-mode auto` for every fresh launch. Do not read Zed settings or run configuration or version discovery commands before launch or resumption. Never use `bypassPermissions`. The selected mode does not transfer interactive approvals into an unattended CLI process or authorize otherwise prohibited effects.

Omit `allow_hosts` on Claude CLI terminal calls and use existing session network permissions. If access is insufficient, pause for a supported user grant rather than substituting unrestricted access or unsandboxed execution.

Unless the user explicitly selects otherwise, use `--model opus` for the latest available Opus and `--effort xhigh`. Do not add a monetary budget or automatic fallback model. Native background summaries may use auxiliary Haiku-class requests, so the primary assignment’s model selection does not imply that every CLI request uses Opus.

The procedures below use the native background interface, not an async print mode runner. Treat unsupported options or changed CLI behavior as a concrete compatibility problem under [Handle Errors and Stop](#handle-errors-and-stop), not a reason to install, upgrade, or improvise a runner.

## Launch a Fresh Conversation

Run from the supplied checkout, using the host’s working directory parameter. The command shapes below assume POSIX `sh`, not the default interactive shell. They are schematic, not raw substitution templates. Pass each placeholder as one literal argument, using a host interface that accepts an argument array when available. For POSIX `sh` command strings, surround each value with single quotes and replace every embedded `'` with `'\''`. Do not apply that encoding in another shell or use shell interpolation or command substitution to insert the assignment. With an argument array, supply closed, empty stdin through the host’s stdin setting rather than passing `< /dev/null` as an argument. If neither invocation form can preserve the arguments and provide empty stdin, stop and report the capability limitation.

Launch with the complete assignment:

```sh
claude --bg --effort xhigh --model opus --no-chrome --permission-mode auto --strict-mcp-config '<assignment>' < /dev/null
```

Use a positional prompt, not `--print`. Preserve the launch’s MCP and browser restrictions. Redirect stdin from `/dev/null` so an unattended launch does not answer a human-only prompt. This does not turn an approval requirement into permission.

Do not supply `--session-id` with `--bg`, which assigns its own session identity. Capture the returned job ID, then obtain the full session UUID from the exact matching row:

```sh
claude agents --all --cwd '<checkout>' --json | jq -e --arg job '<job-id>' '[.[] | select(.id == $job) | {id, sessionId, state, status, waitingFor}] | if length == 1 and (.[0].sessionId | if type == "string" then test("^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$") else false end) then .[0] else error("Expected one task-owned job with a full session UUID") end'
```

The listing is an array. Require exactly one match and a full `sessionId`. Do not prefix-match optional UUID fields or emit unrelated rows or the potentially large `name` field. Inspect only the task row’s process metadata when needed.

Retain the job ID, full UUID, checkout, effective configuration confirmations, transcript path, and the context and monitoring state defined below in the coordinator’s context. If task files become necessary, use `agent-task-directories` rather than creating an ad hoc log tree.

Session bookkeeping, launch and monitoring diagnostics, and recovered invocation errors normally stay with the coordinator. Include them in an assignment only when they affect Claude’s task or are its subject.

## Resume the Same Exchange

Use resumption for a justified continuation, not as the default for an unrelated request. Use the recorded full UUID, never implicit “latest session” selection. Before composing the follow-up, [check retained context](#check-retained-context).

Before a completed-turn follow-up, obtain the [turn’s `ready` result](#observe-the-current-turn) and retain its replies and `nextSince`. Record the task row’s `pid` when available, then stop the task-owned worker while preserving the conversation:

```sh
claude stop '<job-id>'
```

Confirm exit from the stop response and task row, using a host process status check of the recorded PID when needed. If exit cannot be established, report the limitation and do not resume. Reuse applicable termination evidence rather than stopping an already confirmed exited worker again.

Compare the session’s recorded configuration with current requirements, including `auto` permission mode. When they still match, resume with only the background, resume, and prompt arguments:

```sh
claude --bg --resume '<full-uuid>' '<assignment>' < /dev/null
```

A worker with session `state: done` and `status: idle` can still be alive. Resuming it can create a copy even with the minimal command above. Adding configuration flags can also create a copy even when their values match. Let the CLI restore its saved options, check the restoration confirmation, and verify that the returned job resolves to the intended full UUID before the first monitoring check. If requirements have changed, start a fresh conversation with the required configuration and a self-contained handoff instead of silently reusing stale options. Tell the user when losing continuity materially changes the exchange.

If a launch unexpectedly creates a copy, report the deviation, stop only the task-owned copy, preserve its transcript, and correct the demonstrated cause within existing authority. Do not adopt the copy silently or use blind restart or `respawn` loops.

Interrupted-turn recovery is not established by completed-turn resumption. After interruption, first inspect the task’s last complete output and pending work. Do not replay an uncertain turn or assume child work ended merely because the worker stopped. Preserve partial findings and return any recovery decision that the evidence cannot settle.

### Check Retained Context

Use the initial assignment as the context reference point until a later self-contained handoff refreshes it. Retain the fixed `--since` value used for that handoff as its context boundary. Obtain `compactedAt` from the completed turn’s [signal](#observe-the-current-turn), not from a separate transcript reader.

A delta-only follow-up requires an identifiable handoff, a successful `ready` result, and `compactedAt` either `null` or no greater than the retained context boundary. A larger value requires a self-contained handoff. This comparison is conservative because the boundary precedes the prompt’s record. The subsequent restoration and UUID checks must still succeed. This supports context reuse, not a claim that every earlier message reaches the model verbatim.

When compaction or uncertain retention requires a refreshed handoff, retain its new pre-dispatch boundary so earlier compaction does not force repeated refreshes. Do not add a turn solely to test recall or refresh context. Resolve monitoring failures under [Handle Errors and Stop](#handle-errors-and-stop) before dispatch rather than using a context refresh to bypass them.

## Observe the Current Turn

Use the domfiles-managed `claude-signal` command through `PATH` instead of reconstructing transcript readers or polling `claude agents` separately. Apply the global **Output economy** policy. Locate only the recorded UUID’s `<uuid>.jsonl` transcript beneath `~/.claude/projects/` once per session and retain its exact path.

Before invoking the command, read **Run Command Wrappers** in `skill-development`’s `references/domfiles-script-policy.md` for launcher prerequisites, build effects, and failure statuses. The compiled observation is read-only: it reads the selected transcript, related subagent transcripts, and the matching session’s native CLI state. It does not dispatch prompts or stop workers.

### Read Turn Signals

Run from any directory, resolving relative transcript paths from that directory:

```sh
claude-signal --since '<line>' --transcript '<transcript-path>'
```

Both options are required. `--since` is a nonnegative complete-line count recorded before the latest dispatch: use `0` for a fresh launch or the previous completed turn’s `nextSince` for a follow-up. Keep it fixed for every check of that dispatch. Never advance it to a polling result’s `nextSince` mid-turn, which would exclude the requested prompt.

The optional `--wait` accepts an integer from `0` to `60`, defaults to `0`, and waits up to two seconds between checks until the wait expires or the turn becomes `ready` or needs `attention`. Use the nonwaiting form during independent work and the [bounded waiting form](#inactivity) when Claude is the last outstanding dependency. The wait is not a hard process deadline: build preparation and a blocking CLI lookup can extend the invocation. Give the host invocation a finite timeout that allows for that overhead. `claude-signal --help` must be used alone.

A successful observation prints one JSON line with `state` and `nextSince`. This `state` describes the requested turn, not the session’s `state` in `claude agents`. `nextSince` is the current complete-line count for the next dispatch, not a read cursor. The command anchors on the latest human prompt after `--since`. Before that prompt is recorded, it returns `working` with `progressAt: null` without calling `claude agents`.

| Turn State | Additional Fields | Coordinator Action |
| --- | --- | --- |
| `attention` | `reason`, plus `detail` when available. | Report promptly and follow [Handle Errors and Stop](#handle-errors-and-stop), rather than waiting for the inactivity threshold. |
| `ready` | `replies`, the final reply of each turn since the prompt, and `compactedAt`, the latest compaction record’s one-based line or `null`. | Read all replies, retain the result for context reuse and the next dispatch, then stop the task-owned worker. |
| `working` | `progressAt`, the latest qualifying event timestamp or `null`. | Continue coordination under the [inactivity policy](#inactivity). |

`ready` requires a closed turn, answered tool calls, completed recorded background commands, no pending agents or workflows, a usable final reply, and a session that is neither busy nor waiting. Use that result rather than manually re-deriving completion from one session field. It does not prove success of the assigned task or terminate the worker.

| Attention Reason | Meaning and Detail |
| --- | --- |
| `blocked` | Session `state` is `blocked`, `status` is `waiting`, or `waitingFor` is populated. `detail` contains `waitingFor` and a pending `tool` with `name`, serialized `input`, and `inputOmitted`, or `null` when no tool is identified. |
| `failed` | The turn is still open or awaiting background work, and the session reports failure. |
| `noReply` | The turn ended without a usable final reply, including synthetic API errors, output limits, refusals, or unanswered tool calls. `detail` contains `error`, `model`, `stopReason`, `text`, and `textOmitted`. |
| `stopped` | The turn is still open or awaiting background work, and the session reports that it stopped. |

Tool input and reply text in attention details are limited to 500 characters each, with exact omitted-character counts. Retrieve only the bounded additional evidence needed when truncation prevents a decision. A `blocked` signal does not authorize answering a user-only prompt or establish which approval is needed. Return that decision or an unclear requirement to the user.

The command consumes newline-terminated JSON records and leaves an unfinished final record for a later check. Transcript structure remains an internal, version-sensitive interface. Do not replace a monitor failure with an ad hoc reader, raw transcript dump, or `claude logs` stream. Do not emit thinking blocks, unrelated sessions, authentication data, or global debug logs.

## Enforce Progress and Command Limits

### Inactivity

Stop after 10 minutes without progress reported by `claude-signal`. There is no total runtime cap. Initialize the clock when dispatch is accepted, retain the latest `progressAt`, and reset only when it advances, using the event timestamp rather than the time the check returned. A null value does not reset the clock. Entering the final waiting phase does not reset it either.

`progressAt` tracks the latest timestamp among the prompt, Claude’s own nonempty reply text, and tool results not marked `is_error: true`, including subagents started during the turn. Results for identical repeated calls within the same observed transcript segment do not count. These event kinds replace manual progress judgments and identify the basis for resets. They are an observable-event proxy, not proof of useful reasoning or a command’s semantic success. Prohibit artificial keepalives, unnecessary commands, and heartbeat files, and do not request internal thinking or extra tool calls solely to make progress visible.

During the final waiting phase, combine the wait and observation:

```sh
claude-signal --since '<line>' --transcript '<transcript-path>' --wait 60
```

Do not add a separate sleep or status call. Stop at the first successful check that establishes the inactivity threshold. Opportunistic checks cannot guarantee immediate error detection or exact deadline enforcement between checks. After a monitoring gap, a successful check examines the intervening records using the same fixed boundary. If observation fails, report a monitoring limitation rather than proven inactivity. One long reasoning phase or blocking operation may provide no qualifying event.

### Commands

Give Claude and its delegates a default five-minute actual execution cap per command. Before shell work, require them to establish that their available execution mechanism terminates commands at their deadlines. Reuse that capability evidence while its relevant conditions remain unchanged, rather than testing it with a potentially long-running command. A tool returning early, timing out its own wait, or backgrounding a still-running process does not enforce this cap.

Require pending command identity, start time, deadline, and completion or termination evidence where execution outlives a tool response. Before a necessary longer command, Claude must return the reason and proposed bound to the coordinator. The coordinator may resolve the exception only within existing authority and applicable host timeout, retry, and approval requirements.

If termination cannot be established, use a suitable read-only tool that does not launch a shell command, or report the limitation and omit the affected command or return the exception before execution. Do not introduce a timeout dependency or wrapper as an implicit repair. Do not claim that periodic observation enforces a process deadline. The command cap does not limit the entire conversation or a native subagent’s lifetime, but delegated commands retain it and delegated work must be accounted for at completion or stopping.

## Handle Errors and Stop

Require Claude to include errors, recovered errors, gate requests, and material limitations in its final reply, even when already mentioned in interim prose. When coordinator action is needed or a stopping condition is reached, Claude must end the turn with partial results and the required decision or limitation rather than leave that report only in interim text. Do not rely on interim prose to deliver required reports.

Surface observed CLI, monitoring, permission, and command errors to the user promptly, with the relevant error and known impact, without exposing secrets. Recover ordinary errors through evidence-backed corrections within existing authority and retry limits. Do not hide an error because recovery succeeds.

The compiled `claude-signal` exits `0` after a successful observation, including `attention`, or help. Exit `2` reports an invalid invocation or an observation/output failure, including unreadable or malformed transcripts, a boundary beyond the complete transcript, or a failed or non-unique session lookup. Diagnostics go to standard error, and malformed records are identified by path and line number without their contents. For failures before the monitor runs, use the shared wrapper contract routed from [Observe the Current Turn](#observe-the-current-turn). Treat any unsuccessful check as a monitoring limitation. Do not advance the boundary, reset the clock, or infer inactivity from an error.

For `noReply`, inspect the API error and other detail rather than treating the partial text as a completed review. Authentication and quota failures follow the stopping rules below. The compact monitor does not stream every command result or recovered error, so Claude’s required final error report remains necessary.

Stop the collaboration and tell the user when the CLI is unavailable or subscription usage quota is exhausted. Do not repeatedly retry quota failures, change billing or authentication sources, select another model, or manufacture a budget to continue. Configuration requirements, human-only decisions, denied access, and unsatisfied approval gates also pause the affected work. Never weaken permissions or answer user-only approvals to recover.

On demonstrated inactivity, user cancellation, or a stopping boundary, preserve available partial results and use `claude stop` on the identified task-owned worker. Verify its resulting state without assuming that background work stopped with it.

For an unfinished turn, `claude-signal` does not enumerate pending commands or delegates. Use identities and lifecycle evidence already available from Claude’s reports or prior task observations to cancel only identified task-owned work through permitted mechanisms. When identities or termination evidence are unavailable, report background commands and delegates as possibly still running, state what is known, and identify any required user action. Do not claim complete cancellation or require an ad hoc transcript reader to fill this gap.

After establishing [completion](#observe-the-current-turn), stop any remaining task-owned worker. Preserve the UUID and transcript for justified follow-ups. Do not automatically delete sessions with `claude rm`, which can also remove job or worktree state. Return the result and any limitation to the coordinator’s [finish or pause procedure](../SKILL.md#finish-or-pause).

## Validate Monitor Changes

For a monitor change or a reproduced compatibility problem, run its focused contract tests from the domfiles root:

```sh
cargo test --locked --test claude-signal-test
```

The tests use temporary fixtures and stub the Claude session listing. They exercise parsing, turn states, progress, waits, and launcher behavior without starting a live Claude exchange. For launcher coverage, follow **Validate Command Wrappers** in `skill-development`’s `references/domfiles-script-policy.md`.
