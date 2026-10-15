# Codex CLI

## Apply Runtime Configuration

Follow the [shared runtime rules](session-coordination.md#apply-runtime-configuration). Use the installed `codex` command through `PATH` and its existing login.

Run every turn through `codex exec --json --approve-for-me`. That option routes approval requests through Codex’s automatic review under the `workspace-write` sandbox rather than to a person. It does not authorize otherwise prohibited effects. Never use `--dangerously-bypass-approvals-and-sandbox` or `--dangerously-bypass-hook-trust`. Do not use `--ephemeral`, which stores no session files for the monitor to read. Do not read Codex configuration or run model or version discovery commands before launch or resumption.

Unless the user explicitly selects otherwise, use `-m gpt-6.1-sol` and `-c 'model_reasoning_effort="xhigh"'`.

## Run Host Jobs

`codex exec` runs one turn in the foreground and exits when that turn ends. When the coordinating host can run a background job that reports its exit and can be stopped on request, run each launch and resumption that way, with an explicit timeout of at most 2 hours. Reaching the timeout stops the turn like a [host job stop](#stop-a-session).

Otherwise, run the same command in the foreground, with a host timeout of at most 2 hours, rather than detaching the process another way. The coordinator then waits for the command and cannot observe or stop the turn, so the shared [inactivity policy](session-coordination.md#inactivity) does not apply and the timeout is the turn’s only limit. Other sessions keep running and are checked after it returns. The command’s return counts as the job’s exit report under [Read Codex Signals](#read-codex-signals): read the session UUID from the events file, then check the session once with `--wait 0`. If the host rejects that timeout, use the longest one it accepts and tell the user about the shorter limit.

A host’s own sandbox can keep Codex from starting or from applying its own sandbox to the commands it runs. Zed 1.24.2’s terminal sandbox does both, which stays established while that version and its sandbox settings apply. Elsewhere, a launch failing with `Failed to synchronize managed preferences` or commands failing with `sandbox_apply: Operation not permitted` are leads, not proof, because both errors can have other causes. Attribute such a failure to the host sandbox with evidence that needs no inspection of permission configuration, such as the host’s denial logs, before relying on it. When no such evidence is available, ask the user to attribute the failure. Only for an established incompatibility that no narrower grant the host supports resolves, run each Codex launch and resumption outside the host sandbox through the host’s approval. Reuse an applicable existing grant. Otherwise, ask the user to approve that command alone, because some hosts, including Zed, can extend such an approval to every later command in the thread. Outside the host sandbox, Codex still sandboxes its commands under `--approve-for-me`. If the host cannot run the command outside its sandbox or the user declines, report the limitation.

Codex stops its running subagents when `exec` exits, discarding their unfinished work. The shared [command limits](session-coordination.md#commands) therefore matter for every Codex turn.

## Establish Readiness

Follow the [shared readiness procedure](session-coordination.md#establish-readiness). If launch reports a trust, hook, or configuration requirement, pause and ask the user to resolve it as the CLI’s message describes, then retry the intended operation. Do not work around the requirement with another option.

## Prepare Implementation Assignments

Codex edits the supplied checkout directly. Do not pass `--worktree`, which creates another checkout. Keep its write scope apart from other concurrent work under the global **Concurrent work** policy, and establish branch authority under the **Commit gate** policy before edits begin.

## Launch a Fresh Conversation

Compose the commands below under the [shared command rules](session-coordination.md#compose-commands), and give each dispatch its own events and standard error files. Launch with the assignment on stdin:

```sh
codex exec --json --approve-for-me -m gpt-6.1-sol -c 'model_reasoning_effort="xhigh"' - < '<assignment-file>' > '<events-file>' 2> '<stderr-file>'
```

The `-` argument makes Codex read the assignment from stdin, so the command never carries its text. Retain the host job ID of a background job, the explicit `-m` and `model_reasoning_effort` values, and both output paths with the [shared session state](session-coordination.md#retain-session-state).

Obtain the full session UUID from the first event:

```sh
head -n 1 '<events-file>'
```

Expect a `thread.started` event and take its `thread_id`. While the file is still empty, check again. A job that exits before writing that event failed to launch. Diagnose it from the end of the standard error file. Do not otherwise read the events file, which repeats the turn’s items, including command output, except to [recover partial output](#stop-a-session). The monitor supplies a finished turn’s result.

## Resume the Same Exchange

Follow the [shared resumption procedure](session-coordination.md#resume-the-same-exchange). When the retained launch values still match current requirements, pass them again before `resume`, with new events and standard error files:

```sh
codex exec --json --approve-for-me -m gpt-6.1-sol -c 'model_reasoning_effort="xhigh"' resume '<full-uuid>' - < '<assignment-file>' > '<events-file>' 2> '<stderr-file>'
```

Codex appends the new turn to the session’s existing rollout. Confirm that the new events file’s `thread.started` event carries the intended UUID before the first monitoring check. If it does not, stop that job and report the limitation. A resumption may start before the previous dispatch’s job exits.

## Read Codex Signals

For each `--codex` session, `subagent-signal` reads the `rollout-*-<uuid>.jsonl` file beneath `~/.codex/sessions/`. The turn starts at the latest `task_started` event after `<since>` and ends at a later `task_complete` or `turn_aborted` event. Codex sessions need no session listing and report no `blocked`, `failed`, or `stopped` attention.

Codex compresses a rollout to `.jsonl.zst` after a week without activity, and the monitor does not read that file. A resumption restores the plain file before Codex appends to it, so until then a follow-up reports `working` with `progressAt: null` rather than an `error`.

`progressAt` also covers the rollouts of Codex subagents started during the turn, at any depth. To find them, the monitor reads the first record of every rollout in the root rollout’s date directory and the two newest date directories, including rollouts of unrelated sessions.

`ready` requires a `task_complete` event with a non-empty final message.

| Attention Reason | Meaning and Detail |
| --- | --- |
| `noReply` | The turn ended with a `task_complete` event that has no final message, such as a failed turn, or with a `turn_aborted` event. `detail` contains the abort reason as `abortReason` and the failure message as `error`, each `null` when absent, plus `errorOmitted`. |

When the host reports that the current dispatch’s job exited, check that session once with `--wait 0`. If it still reports `working`, the process ended without closing the turn. Treat the turn as interrupted and diagnose it from the job’s exit status and standard error file. Ignore exit reports from earlier dispatches’ jobs.

## Stop a Session

Stop the current dispatch’s host job and confirm from the host that it ended. A host job stop records no turn end, so stop monitoring that dispatch rather than waiting for a result. A foreground turn cannot be stopped on request. It ends at its timeout or when the user interrupts the command.

After a stop, a timeout, or an exit that leaves the turn open, the monitor reports no partial output. For the shared [interrupted-turn procedure](session-coordination.md#resume-the-same-exchange), read only the last completed agent message from that dispatch’s events file:

```sh
jq -c 'select(.type == "item.completed" and .item.type == "agent_message") | .item.text' '<events-file>' | tail -n 1
```

An unfinished final event makes `jq` report a parse error after printing the earlier messages. When the file holds no completed agent message, the turn left no partial output.

For completion cleanup, no stop is needed, because the job exits after the turn. Keep the events and standard error files while the exchange may continue, and clean them up with the task directory.
