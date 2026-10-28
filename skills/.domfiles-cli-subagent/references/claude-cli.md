# Claude CLI

## Apply Runtime Configuration

Follow the [shared runtime rules](session-coordination.md#apply-runtime-configuration). Use the installed `claude` command through `PATH` and its existing subscription login. Don’t use `--bare` or replace the system prompt.

Use `--permission-mode auto` for every fresh launch. Don’t read Zed settings or run configuration or version discovery commands before launch or resumption. Never use `bypassPermissions`. The selected mode doesn’t transfer interactive approvals into an unattended CLI process or authorize otherwise prohibited effects.

Unless the user explicitly selects otherwise, use `--model opus` for the latest available Opus and `--effort xhigh`. Don’t add a monetary budget or automatic fallback model. Native background summaries may use auxiliary Haiku-class requests, so choosing Opus for the primary assignment doesn’t mean every CLI request uses it.

The procedures below use the native background interface, not an async print mode runner.

Match authorized lifecycle access to the operation: launch and resumption read and write Claude’s session and job data and start a worker, monitoring reads that data and queries live process state through `claude agents`, stopping uses `claude stop` to terminate the task-owned worker, and a host process check queries that worker’s existence. Establish the needed access from the host’s execution contract or retained task-specific evidence, not command success alone.

## Establish Readiness

Follow the [shared readiness procedure](session-coordination.md#establish-readiness). If launch requires workspace trust, pause and ask the user to run `claude` interactively in the exact supplied checkout, without an initial prompt, review the trust request, and exit without submitting a task. Interactive startup can run configured hooks and integrations. After the user resolves the requirement, retry the intended background operation. Don’t infer readiness from a parent directory’s trust or success in print mode, which skips the workspace trust dialog.

## Prepare Implementation Assignments

Keep Claude’s implementation and validation in the coordinator’s supplied checkout, including any Zed-managed worktree. Keep its write scope apart from other concurrent work under the global **Concurrent work** policy, and establish branch authority under the **Commit gate** policy before edits begin. Include that checkout in the assignment and require native subagents to use it without separate worktree isolation. Leave worktree creation, switching, and cleanup to the user and host application.

The domfiles-managed Claude user settings disable [background worktree isolation](https://code.claude.com/docs/en/agent-view#how-file-edits-are-isolated), allowing background sessions to edit the current checkout. If the runtime still requires another worktree before edits, report the configuration or compatibility limitation instead of creating one or bypassing the requirement.

## Limit Command Runtime

Apply the [shared command limits](session-coordination.md#commands) with the Bash tool’s native `timeout`, using `300000` milliseconds by default. A [foreground command can move to the background at its timeout](https://code.claude.com/docs/en/tools-reference#foreground-commands-that-move-to-the-background), so a hard-termination guarantee isn’t a prerequisite for running it.

If a command remains running when its timeout expires, stop that task immediately through the native `TaskStop` tool and confirm termination. This also applies when the timeout moves the command to the background. Don’t reset the deadline or treat backgrounding as permission to continue. Stopping may occur after the deadline, so don’t describe this as an exact execution cap. If stopping fails or can’t be confirmed, report the command as possibly still running under [Handle Errors and Stop](session-coordination.md#handle-errors-and-stop).

## Launch a Fresh Conversation

Compose the commands below under the [shared command rules](session-coordination.md#compose-commands).

Claude [refuses to start a session when workspace trust is required and no dialog can appear](https://code.claude.com/docs/en/errors#workspace-not-trusted-when-dispatching-a-background-session). Keep all three standard streams disconnected from terminals on launch and resumption to use that refusal path. Give each dispatch its own output file in the task directory, passing the complete assignment file on stdin and capturing stdout and stderr:

```sh
claude --bg --effort xhigh --model opus --permission-mode auto < '<assignment-file>' > '<output-file>' 2>&1
```

Read the captured output on success or failure and retain the invocation’s exit status. Continue to session identification only after a successful invocation.

Without a positional prompt, `--bg` reads its assignment from stdin on launch and `--resume`. `claude --help` doesn’t document this input form. Treat a launch or resumption that never records its prompt and keeps reporting `working` with `progressAt: null` as a compatibility problem under [Handle Errors and Stop](session-coordination.md#handle-errors-and-stop). Don’t use `--print`. Leave browser and MCP configuration to the existing Claude setup.

On `Workspace not trusted`, use the reported cause to choose recovery. Follow [Establish Readiness](#establish-readiness) when accepting workspace trust in the supplied checkout can resolve it. Handle other causes and invocation failures through [Handle Errors and Stop](session-coordination.md#handle-errors-and-stop).

Don’t supply `--session-id` with `--bg`, which assigns its own session identity. Capture the returned job ID from the output file, then obtain the full session UUID from the exact matching row:

```sh
claude agents --all --cwd '<checkout>' --json | jq -ce --arg job '<job-id>' '[.[] | select(.id == $job) | {id, sessionId}] | if length == 1 and (.[0].sessionId | if type == "string" then test("^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$") else false end) then .[0] else error("Expected one task-owned job with a full session UUID") end'
```

The listing is an array. Require exactly one match and a full `sessionId`. Don’t match optional UUID fields by prefix or emit unrelated rows or the potentially large `name` field. For lifecycle checks and diagnostics, keep the exact job filter and compact output, adding only the fields needed for that decision. Use the [shared monitor](session-coordination.md#observe-the-current-turn) for routine state polling.

Retain the job ID and the explicit launch values for `--effort`, `--model`, and `--permission-mode` with the [shared session state](session-coordination.md#retain-session-state).

## Resume the Same Exchange

Follow the [shared resumption procedure](session-coordination.md#resume-the-same-exchange). Before sending a follow-up after a completed turn, check whether the task-owned worker has exited as described below, reusing applicable evidence. If exit is not established, record the task row’s `pid` when available, then stop the worker while preserving the conversation:

```sh
claude stop '<job-id>'
```

Confirm exit from the exact task row using access established to read Claude’s session and job data and query live process state. When a PID was recorded, reuse the access conditions that exposed it, with a host process check of that PID when needed. A prior PID is not required: under Claude’s [documented session listing contract](https://code.claude.com/docs/en/agent-view#list-sessions-as-json), a native task row that omits both `pid` and `status` establishes that its worker has exited, provided the query’s access to live process state is independently established. Without that access evidence, a successful stop message or absent process fields cannot prove exit.

If a query with established access to live process state confirms that the same task-owned worker survived the stop, retry `claude stop` once with authorized access to terminate that worker, then repeat the exit check. If you can’t establish exit, report the limitation and don’t resume.

When the retained launch values still match current requirements, resume with only the background and resume arguments. Apply the stream redirections and result checks from [Launch a Fresh Conversation](#launch-a-fresh-conversation):

```sh
claude --bg --resume '<full-uuid>' < '<assignment-file>' > '<output-file>' 2>&1
```

A worker with session `state: done` and `status: idle` can still be alive. Resuming it can create a copy even with the minimal command above. Adding configuration flags can also create a copy even when their values match. Let the CLI restore its saved options, check the restoration confirmation, and verify that the returned job resolves to the intended full UUID before the first monitoring check.

If a launch unexpectedly creates a copy, stop only the task-owned copy, preserve its transcript, and correct the demonstrated cause within existing authority. Don’t adopt the copy or use blind restart or `respawn` loops. Report any remaining limitation or material effect on continuity or pending work.

## Read Claude Signals

For each `--claude` session, `subagent-signal` reads the `<uuid>.jsonl` transcript beneath `~/.claude/projects/`, the modification times of its subagent transcripts, and the session’s native CLI state from `claude agents`. The turn starts at the latest human prompt after `<since>`. If the session’s prompt has not yet been recorded, no `claude agents` lookup is needed. `progressAt` covers the transcript and its subagent transcripts.

`ready` requires a closed turn, answered tool calls, completed recorded background commands, a later turn for any task notification recorded after the last turn ended, no pending agents or workflows, a usable final reply, and a session that is neither busy nor showing an open prompt. Claude also marks a session `blocked` when its reply asks a question, so a finished reply reports `ready` and the question appears in `replies`. Use that result instead of inferring completion from a single session field. Reporting `ready` doesn’t terminate the worker.

| Attention Reason | Meaning and Detail |
| --- | --- |
| `blocked` | An open prompt, where `status` is `waiting` or `waitingFor` is populated. A session `state` of `blocked` alone doesn’t qualify. `detail` contains `waitingFor` and a pending `tool` with `name`, serialized `input`, and `inputOmitted`, or `null` when no tool is identified. |
| `failed` | The turn is still open or awaiting background work, and the session reports failure. |
| `noReply` | The turn ended without a usable final reply, including synthetic API errors, output limits, refusals, or unanswered tool calls. `detail` contains `error`, `model`, `stopReason`, `text`, and `textOmitted`. |
| `stopped` | The turn is still open or awaiting background work, and the session reports that it stopped. |

Queries in a host context that restricts filesystem reads or process queries can report `failed` while the worker remains active. If this failure signal conflicts with an increase in the monitor’s `nextSince` across observations or comes from a context with narrower access to filesystem reads or process queries than launch, recheck the same session using access established to read its session and job data and query live process state before stopping or replacing it. This read-only recheck does not require permission to terminate the worker. Use the prescribed monitor with the same UUID and fixed `<since>` value. If that access or reliable observation is unavailable, report the monitoring limitation and follow [Handle Errors and Stop](session-coordination.md#handle-errors-and-stop) rather than infer worker failure or inactivity.

A `blocked` signal doesn’t authorize answering a user-only prompt or establish which approval is needed. Ask the user to resolve the approval or clarify an unclear requirement.

## Stop a Session

Use `claude stop` on the identified task-owned worker, then confirm its exit as [Resume the Same Exchange](#resume-the-same-exchange) describes. Background shell commands the session started can keep running after the stop, and they keep a later resumption of that session `busy` until they finish. If an unfinished turn’s work must continue after a stop, don’t resume the stopped session. Replace it as the [shared resumption procedure](session-coordination.md#resume-the-same-exchange) describes for changed requirements.

For completion cleanup, stop any remaining task-owned worker as [Resume the Same Exchange](#resume-the-same-exchange) describes. Don’t automatically delete sessions with `claude rm`, which can also remove job or worktree state.
