# Copilot CLI

`gh copilot` runs a separate agent CLI, can grant it tool permissions, and may download it automatically. A direct, scoped request may authorize agent execution under [Execution Choice](#execution-choice). Otherwise, prepare a command for the user to run locally.

## Availability and Downloads

Use read-only machine-local evidence to establish whether a Copilot CLI is already available. Don’t invoke `copilot` or `gh copilot` to check.

If the CLI is unavailable or you can’t establish its presence, tell the user that `gh copilot` may download the GitHub Copilot CLI into GitHub CLI’s machine-local data directory. Apply [Tool Acquisition](opt-in-operations.md#tool-acquisition) before proceeding.

## Task Handoffs

When the command gives Copilot a task or tool permission and the entrypoint resolves `agent-task-relay`, use it for applicable confirmation and assignment composition only. Carry the user’s actual authorization so covered effects don’t trigger duplicate confirmation. Return to [Execution Choice](#execution-choice) instead of the peer’s normal delivery, and don’t also return the assignment as a relay.

When the entrypoint doesn’t resolve `agent-task-relay`, don’t compose or expand a task or its tool permissions. Preserve task text and permissions the user supplied directly, and leave unresolved values as named placeholders for the user to review and fill locally.

## Execution Choice

Execute only when a direct user request covers the concrete task, target, tool permissions, and any data sent to Copilot, and all applicable access, acquisition, and remote mutation requirements are satisfied. Verify a supported invocation for those effects without choosing a different source or version on the user’s behalf. Keep scope and runtime bounded, preserve approval controls, and don’t enable unrestricted tool permissions or untrusted startup configuration to make execution unattended.

Don’t execute with unresolved placeholders or infer permission for operations the receiving agent might choose. Carry explicit commit or publication authority only when the user supplied it. Credential and key handling remain user-run. If the invocation can’t preserve required prompts or another genuine human-only checkpoint, use the user-run path instead.

## User-Run Command

When execution wasn’t requested or a human-only step remains, provide the exact `gh copilot …` command in a `sh` code block after resolving applicable opt-ins and approvals. Use named placeholders for unresolved task text and tool permissions, and clearly identify what the user must review and fill locally.

Tell the user to copy, paste, and run the command locally, review every prompt and tool permission before accepting it, and never share credentials, secret values, private material, or secret-bearing output in chat. Ask only whether the operation succeeded or for a sanitized error containing no private values.
