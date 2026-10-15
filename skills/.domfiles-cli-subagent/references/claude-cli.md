# Claude CLI

## Apply Runtime Configuration

Follow the [shared runtime rules](session-coordination.md#apply-runtime-configuration). Use the installed `claude` command through `PATH` and the existing subscription login. Do not use `--bare` or replace the system prompt.

Use `--permission-mode auto` for every fresh launch. Do not read Zed settings or run configuration or version discovery commands before launch or resumption. Never use `bypassPermissions`. The selected mode does not transfer interactive approvals into an unattended CLI process or authorize otherwise prohibited effects.

Unless the user explicitly selects otherwise, use `--model opus` for the latest available Opus and `--effort xhigh`. Do not add a monetary budget or automatic fallback model. Native background summaries may use auxiliary Haiku-class requests, so the primary assignment’s model selection does not imply that every CLI request uses Opus.

The procedures below use the native background interface, not an async print mode runner.

## Establish Readiness

Follow the [shared readiness procedure](session-coordination.md#establish-readiness). If launch requires workspace trust, pause and ask the user to run `claude` interactively in the exact supplied checkout, without an initial prompt, review the trust request, and exit without submitting a task. Interactive startup can run configured hooks and integrations. After the user resolves the requirement, retry the intended background operation. Do not infer readiness from a parent directory’s trust or print mode success, which skips the workspace trust dialog.

## Prepare Implementation Assignments

Native background sessions can require an isolated worktree before edits. Their worktree tool may default to a remote default-branch base rather than the coordinator’s current `HEAD`. For implementation, establish the intended base and any necessary uncommitted inputs, checkout and branch authority under the global **Concurrent work** and **Commit gate** policies, delivery into the coordinator’s checkout, and cleanup ownership before edits begin. Do not assume the tool’s default base contains the assigned changes.

If the runtime’s checkout requirements are uncertain, have Claude establish them read-only before requesting any uncovered lifecycle effects. Verify the supported way to prepare the intended state rather than inventing tool parameters. If the required isolation or base cannot be established within current authority, return that limitation instead of editing in place around the requirement. This preparation does not authorize creating worktrees, branches, commits, or remote changes.

## Launch a Fresh Conversation

Compose the commands below under the [shared command rules](session-coordination.md#compose-commands).

Launch with the complete assignment on stdin:

```sh
claude --bg --effort xhigh --model opus --permission-mode auto < '<assignment-file>'
```

Without a positional prompt, `--bg` reads its prompt from stdin, for a launch and for `--resume` alike. `claude --help` does not document this, so treat a launch or resumption that never records its prompt, and keeps reporting `working` with `progressAt: null`, as a compatibility problem under [Handle Errors and Stop](session-coordination.md#handle-errors-and-stop). Do not use `--print`. Leave browser and MCP configuration to the existing Claude setup. Stdin holds nothing beyond the assignment, so it leaves no input to answer a human-only prompt. This does not turn an approval requirement into permission.

Do not supply `--session-id` with `--bg`, which assigns its own session identity. Capture the returned job ID, then obtain the full session UUID from the exact matching row:

```sh
claude agents --all --cwd '<checkout>' --json | jq -e --arg job '<job-id>' '[.[] | select(.id == $job) | {id, sessionId, state, status, waitingFor}] | if length == 1 and (.[0].sessionId | if type == "string" then test("^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$") else false end) then .[0] else error("Expected one task-owned job with a full session UUID") end'
```

The listing is an array. Require exactly one match and a full `sessionId`. Do not prefix-match optional UUID fields or emit unrelated rows or the potentially large `name` field. Inspect only the task row’s process metadata when needed.

Retain the job ID and the explicit launch values for `--effort`, `--model`, and `--permission-mode` with the [shared session state](session-coordination.md#retain-session-state).

## Resume the Same Exchange

Follow the [shared resumption procedure](session-coordination.md#resume-the-same-exchange). Before a completed-turn follow-up, record the task row’s `pid` when available, then stop the task-owned worker while preserving the conversation:

```sh
claude stop '<job-id>'
```

Confirm exit from the stop response and task row, using a host process status check of the recorded PID when needed. If exit cannot be established, report the limitation and do not resume. Reuse applicable termination evidence rather than stopping an already confirmed exited worker again.

When the retained launch values still match current requirements, resume with only the background and resume arguments, passing the follow-up on stdin as [Launch a Fresh Conversation](#launch-a-fresh-conversation) describes:

```sh
claude --bg --resume '<full-uuid>' < '<assignment-file>'
```

A worker with session `state: done` and `status: idle` can still be alive. Resuming it can create a copy even with the minimal command above. Adding configuration flags can also create a copy even when their values match. Let the CLI restore its saved options, check the restoration confirmation, and verify that the returned job resolves to the intended full UUID before the first monitoring check.

If a launch unexpectedly creates a copy, stop only the task-owned copy, preserve its transcript, and correct the demonstrated cause within existing authority. Do not adopt the copy or use blind restart or `respawn` loops. Report any remaining limitation or material effect on continuity or pending work.

## Read Claude Signals

For each `--claude` session, `subagent-signal` reads the `<uuid>.jsonl` transcript beneath `~/.claude/projects/`, the modification times of its subagent transcripts, and the session’s native CLI state from `claude agents`. The turn starts at the latest human prompt after `<since>`, and a session whose prompt is not yet recorded needs no `claude agents` lookup. `progressAt` covers the transcript and its subagent transcripts.

`ready` requires a closed turn, answered tool calls, completed recorded background commands, a later turn for any task notification recorded after the last turn ended, no pending agents or workflows, a usable final reply, and a session that is neither busy nor showing an open prompt. Claude also marks a session `blocked` when its reply asks a question, so a finished reply reports `ready` and the question appears in `replies`. Use that result rather than deriving completion again from one session field. It does not terminate the worker.

| Attention Reason | Meaning and Detail |
| --- | --- |
| `blocked` | An open prompt, where `status` is `waiting` or `waitingFor` is populated. A session `state` of `blocked` alone does not qualify. `detail` contains `waitingFor` and a pending `tool` with `name`, serialized `input`, and `inputOmitted`, or `null` when no tool is identified. |
| `failed` | The turn is still open or awaiting background work, and the session reports failure. |
| `noReply` | The turn ended without a usable final reply, including synthetic API errors, output limits, refusals, or unanswered tool calls. `detail` contains `error`, `model`, `stopReason`, `text`, and `textOmitted`. |
| `stopped` | The turn is still open or awaiting background work, and the session reports that it stopped. |

A `blocked` signal does not authorize answering a user-only prompt or establish which approval is needed. Return that decision or an unclear requirement to the user.

## Stop a Session

Use `claude stop` on the identified task-owned worker, then confirm its exit as [Resume the Same Exchange](#resume-the-same-exchange) describes. Background shell commands the session started can keep running after the stop, and they keep a later resumption of that session `busy` until they finish. When the work of a stopped, unfinished turn must continue, do not resume the stopped session. Replace it as the [shared resumption procedure](session-coordination.md#resume-the-same-exchange) describes for changed requirements.

For completion cleanup, stop any remaining task-owned worker as [Resume the Same Exchange](#resume-the-same-exchange) describes. Do not automatically delete sessions with `claude rm`, which can also remove job or worktree state.
