# Session Coordination

These procedures apply to every CLI participant. The [Claude CLI](claude-cli.md) and [Codex CLI](codex-cli.md) references supply each participant’s runtime defaults, its launch, resume, and stop commands, and what the monitor reads for its sessions.

## Apply Runtime Configuration

Resolve availability, login, and configuration failures through [Handle Errors and Stop](#handle-errors-and-stop).

Global instruction alignment belongs to domfiles provisioning. Normal startup of each CLI loads the canonical global instructions through its managed installation. Do not audit that setup on each exchange or paste the global instructions into assignments. Repairing provisioning is a separate task.

Treat unsupported options or changed CLI behavior as a concrete compatibility problem under [Handle Errors and Stop](#handle-errors-and-stop), not a reason to install, upgrade, or improvise a runner.

## Establish Readiness

Use the intended session in the supplied checkout, not a parent directory or another worktree. Reuse a successful exchange when its checkout, required launch configuration, and access conditions remain applicable. Otherwise, use the participant’s normal launch or resumption procedure. If the assignment is ready, dispatch it directly. If it depends on later work, send a readiness-only assignment: acknowledge readiness without inspecting task material, using tools, delegating, or making changes. This is a real turn with normal startup, configured integrations, and model usage, not a side-effect-free local query.

Await a usable reply through the normal monitor, then apply the participant’s completion cleanup and retain the session state for the actual assignment. A readiness-only reply establishes startup and response capability, not tool permissions or task completion. Resume that session when the work is ready rather than creating a second session.

## Compose Commands

Write each assignment, including every follow-up, to a file in the task directory and pass it on stdin, so commands never carry assignment text. Keep these files while the exchange may continue, and clean them up with the task directory.

The command shapes in this skill assume POSIX `sh`, not the default interactive shell. They are schematic, not raw substitution templates. Pass each placeholder as one literal argument, using a host interface that accepts an argument array when available. With an argument array, apply the command’s redirections, such as the assignment file or output files, through the host’s stream settings rather than passing them as arguments. If neither invocation form can preserve the arguments and connect the streams, stop and report the capability limitation. For POSIX `sh` command strings, surround each value with single quotes and replace every embedded `'` with `'\''`. Do not apply that encoding in another shell or use shell interpolation or command substitution to insert values. Write each command with literal values, because some hosts reject shell variable expansion and command substitution in agent commands. Run launch and resumption commands from the supplied checkout, using the host’s working directory parameter.

## Retain Session State

For each session, retain the full UUID, checkout, explicit launch values, the current dispatch’s `<since>`, the participant-specific values its reference names, and the context and monitoring state defined below in the coordinator’s context.

Session bookkeeping, launch and monitoring diagnostics, and recovered invocation errors normally stay with the coordinator. Include them in an assignment only when they affect the participant’s task or are its subject.

## Resume the Same Exchange

Use resumption for a justified continuation, not as the default for an unrelated request. Use the recorded full UUID, never implicit “latest session” selection. Before composing the follow-up, [check retained context](#check-retained-context).

Before a completed-turn follow-up, obtain the [turn’s `ready` result](#read-turn-signals) and retain its replies and `nextSince`. Compare the retained launch values with current requirements. When they still match, follow the participant’s resumption procedure, including its check that the new turn belongs to the intended UUID before the first monitoring check. If requirements have changed, do not silently reuse stale options. For delegated reviews, `subagent-review-flow` governs whether the original session may be replaced. For other exchanges, start a fresh conversation with the required configuration and a self-contained handoff, telling the user when losing continuity materially changes the exchange.

Interrupted-turn recovery is not established by completed-turn resumption. After interruption, first inspect the task’s last complete output and pending work. Do not replay an uncertain turn or assume child work ended merely because the session stopped. Preserve partial findings and return any recovery decision that the evidence cannot settle.

### Check Retained Context

Use the initial assignment as the context reference point until a later self-contained handoff refreshes it. Retain the fixed `<since>` value used for that handoff as its context boundary. Obtain `compactedAt` from the completed turn’s [signal](#read-turn-signals), not from a separate transcript reader.

A delta-only follow-up requires an identifiable handoff, a successful `ready` result, and `compactedAt` either `null` or no greater than the retained context boundary. A larger value requires a self-contained handoff. This comparison is conservative because the boundary precedes the dispatch’s first record. The participant’s resumption checks must still succeed. This supports context reuse, not a claim that every earlier message reaches the model verbatim.

When compaction or uncertain retention requires a refreshed handoff, retain its new pre-dispatch boundary so earlier compaction does not force repeated refreshes. Do not add a turn solely to test recall or refresh context. Resolve monitoring failures under [Handle Errors and Stop](#handle-errors-and-stop) before dispatch rather than using a context refresh to bypass them.

## Observe the Current Turn

Run this skill’s `subagent-signal` monitor with `node` instead of reconstructing transcript readers or polling session listings separately. Its path is `<skill-directory>/scripts/subagent-signal.ts`, where `<skill-directory>` is the directory containing this skill’s `SKILL.md`. Apply the global **Output economy** policy. The command finds each session’s transcript itself.

The observation is read-only. It does not dispatch prompts or stop sessions.

### Read Turn Signals

Run from any directory. Pass one argument per pending session, `--claude` for a Claude session and `--codex` for a Codex session. Pending sessions are those whose last result was `working`, plus any session just dispatched. A session that has already reported another state would make every call return immediately, so check it again with `--wait 0` only when there is a reason to.

For a non-waiting check during independent work:

```sh
node '<skill-directory>/scripts/subagent-signal.ts' --wait 0 --claude '<since>:<uuid>' --codex '<since>:<uuid>'
```

`<since>` is a non-negative complete-line count recorded before the session’s latest dispatch: use `0` for a fresh launch or the previous completed turn’s `nextSince` for a follow-up. Keep it fixed for every check of that dispatch. Never advance it to a polling result’s `nextSince` while the turn is still running, which would exclude the requested turn.

Omit `--wait` to use the command’s default bounded wait, which returns as soon as any session becomes `ready`, needs `attention`, or reports an `error`. Use that [waiting form](#inactivity) when CLI sessions are the last outstanding dependencies. The wait is not a hard process deadline: a blocking Claude session lookup can extend the invocation. Give the host invocation a finite timeout of at least 120 seconds. `--help` reports the default and override bounds and must be used alone.

A successful observation prints one JSON line with `observedAt`, the observation time in Unix milliseconds, and `sessions`, one entry per session argument in the given order. Each entry has `session` and `state`, which describes the requested turn, not a state reported by the participant’s CLI. Every state except `error` adds `nextSince`, the current complete-line count for the next dispatch, not a read cursor. The command anchors on the latest turn start after `<since>`, as the participant reference defines it. Until that start is recorded, it reports `working` with `progressAt: null`. That includes a fresh launch whose transcript does not exist yet, but a follow-up whose transcript is missing reports an `error` unless the participant reference describes an exception.

| Turn State | Additional Fields | Coordinator Action |
| --- | --- | --- |
| `attention` | `reason`, plus `detail` when available. | Follow [Handle Errors and Stop](#handle-errors-and-stop), rather than waiting for the inactivity threshold. |
| `error` | `message`, the reason the session could not be observed. | Follow the [error procedure](#handle-errors-and-stop). |
| `ready` | `replies`, the final reply of each turn since the turn start, and `compactedAt`, the latest compaction record’s one-based line or `null`. | Read all replies, retain the result for context reuse and the next dispatch, then follow [completion cleanup](#handle-errors-and-stop). |
| `working` | `progressAt`, the latest write to the session’s transcripts in Unix milliseconds, or `null`. | Continue coordination under the [inactivity policy](#inactivity). |

The participant reference defines what `ready` requires and which attention reasons its sessions report. A `ready` result does not prove success of the assigned task. Tool input, reply text, and Codex error messages in attention details are limited to 500 characters each, with exact omitted-character counts. Retrieve only the bounded additional evidence needed when truncation prevents a decision.

If the host cuts off the output, run the same call again with `--wait 0` and standard output redirected to a task file, then read that file. A finished turn reports the same result on every check.

The command consumes newline-terminated JSON records and leaves an unfinished final record for a later check. Transcript structure remains an internal, version-sensitive interface. Do not replace a monitor failure with an ad hoc reader, raw transcript dump, or CLI log stream. Do not emit thinking blocks, unrelated sessions, authentication data, or global debug logs.

## Enforce Progress and Command Limits

### Inactivity

Stop a session after 15 minutes without progress. Apart from the [Codex job timeout](codex-cli.md#run-host-jobs), there is no total runtime cap. Track each session’s clock start and last recorded `progressAt`, both in Unix milliseconds:

1. The first observation after dispatch sets the clock start to its `observedAt`. Take it as soon as the dispatch is confirmed, in one `--wait 0` call covering every session just dispatched, before returning to independent work.
2. Whenever an observation reports a non-null `progressAt` that differs from the last one recorded for that session, record it and set the clock start to it. This includes the first non-null value.
3. Stop the session once `observedAt` minus the clock start reaches 15 minutes.

A `null` `progressAt` leaves the clock unchanged, and so does entering the final waiting phase.

`progressAt` is the latest write to the session’s transcripts, including those of its own subagents as the participant reference describes. Any write counts, including thinking, tool calls, and failed tool results. It is an activity proxy, not proof of useful reasoning or a command’s semantic success, and a session looping on identical calls keeps restarting the clock. Prohibit artificial keepalives, unnecessary commands, and heartbeat files, and do not request internal thinking or extra tool calls solely to make progress visible.

During the final waiting phase, combine the wait and observation using the command’s default:

```sh
node '<skill-directory>/scripts/subagent-signal.ts' --claude '<since>:<uuid>' --codex '<since>:<uuid>'
```

Do not add a separate sleep or status call. Stop each session at the first successful check that establishes its inactivity threshold. Opportunistic checks cannot guarantee immediate error detection or exact deadline enforcement between checks. After a monitoring gap, a successful check examines the intervening records using the same fixed boundary. If observation fails, report a monitoring limitation rather than proven inactivity. A single long model request or blocking command writes nothing until it finishes.

### Commands

Give the participant and its delegates a default five-minute actual execution cap per command. Before shell work, require them to establish that their available execution mechanism terminates commands at their deadlines. Reuse that capability evidence while its relevant conditions remain unchanged, rather than testing it with a potentially long-running command. A tool returning early, timing out its own wait, or backgrounding a still-running process does not enforce this cap.

Require pending command identity, start time, deadline, and completion or termination evidence where execution outlives a tool response. Before a necessary longer command, the participant must return the reason and proposed bound to the coordinator. The coordinator may resolve the exception only within existing authority and applicable host timeout, retry, and approval requirements.

If termination cannot be established, use a suitable read-only tool that does not launch a shell command, or report the limitation and omit the affected command or return the exception before execution. Do not introduce a timeout dependency or wrapper as an implicit repair. Do not claim that periodic observation enforces a process deadline. The command cap does not limit the entire conversation or a native subagent’s lifetime, but delegated commands retain it. Before its final reply, the participant must finish or stop the native subagents and background commands it started and summarize their results in that reply.

## Handle Errors and Stop

When coordinator action is needed or a stopping condition is reached, the participant must end the turn with partial results and the required decision or material limitation rather than leave that report only in interim text. Do not rely on interim prose to deliver required reports.

Apply the shared **Progress and recovery**, **Outcomes**, and **Validation reporting** policies to errors. Report material limitations and required user actions, not an inventory of successfully recovered errors.

`subagent-signal` exits `0` after an observation, including `attention` and `error` entries, or help. This departs from the repository’s portable default so that one session that cannot be observed does not hide another session’s result. Exit `2` reports an invalid invocation or a failed write to standard output, with diagnostics on standard error. Treat any unsuccessful check as a monitoring limitation.

An `error` entry reports one session the command could not observe, such as an unreadable or malformed transcript, a missing transcript when `<since>` is above 0, a `<since>` beyond the complete transcript, several transcripts for one UUID, or a failed or non-unique Claude session lookup. Malformed records are identified by path and line number without their contents. Check that session alone with `--wait 0` up to two more times. If the error persists, report a monitoring limitation and pause or stop that exchange while continuing to watch the others. Do not advance `<since>`, restart the inactivity clock, or infer inactivity from an error.

For `noReply`, inspect the error and other detail rather than treating any partial text as a completed result. Authentication and quota failures follow the stopping rules below.

Stop the collaboration and tell the user when the CLI is unavailable or subscription usage quota is exhausted. Do not repeatedly retry quota failures, change billing or authentication sources, select another model, or manufacture a budget to continue. Configuration requirements, human-only decisions, denied access, and unsatisfied approval gates also pause the affected work.

On demonstrated inactivity, user cancellation, or a stopping boundary, preserve available partial results and stop the session through the participant’s stop procedure. For an unfinished turn, `subagent-signal` does not enumerate pending commands or delegates. Use identities and lifecycle evidence already available from the participant’s reports or earlier observations to cancel only identified task-owned work through permitted mechanisms. When identities or termination evidence are unavailable, report background commands and delegates as possibly still running, state what is known, and identify any required user action. Do not claim complete cancellation or require an ad hoc transcript reader to fill this gap.

After establishing [completion](#read-turn-signals), apply the participant’s completion cleanup. Preserve the UUID and transcript for justified follow-ups. Return the result and any limitation to the coordinator’s [finish or pause procedure](../SKILL.md#finish-or-pause).

## Validate Monitor Changes

For a monitor change or a reproduced compatibility problem, run its focused contract tests:

```sh
node --test '<skill-directory>/scripts/subagent-signal.test.ts'
```

The tests use temporary fixtures, including a temporary home directory, stub the Claude session listing, and write Codex rollouts directly. They exercise parsing, turn states for both participants, several sessions at once, progress, and waits without starting a live exchange.
