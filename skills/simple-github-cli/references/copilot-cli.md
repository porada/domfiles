# Copilot CLI

`gh copilot` runs a separate agent CLI, can grant it tool permissions, and may download it automatically. After the entrypoint’s opt-in, do not execute the command. This workflow ends with a command for the user to run locally.

## Availability and Dependency Approval

Establish from read-only machine-local evidence whether a Copilot CLI is already available. Do not invoke `copilot` or `gh copilot` to check.

If the CLI is unavailable or its presence cannot be established, tell the user that `gh copilot` may download the GitHub Copilot CLI into GitHub CLI’s machine-local data directory. Treat that possible download as a dependency addition and apply the entrypoint’s [Dependency Changes](../SKILL.md#dependency-changes) policy before preparing the command. Opting into `gh copilot` does not provide dependency approval.

## Task Handoffs

When the command gives Copilot a task or tool permission and the entrypoint resolves `agent-task-relay`, use it for confirmation and assignment composition only. The [User-Run Command](#user-run-command) replaces its normal delivery, so do not also return the assignment as a relay.

When the entrypoint does not resolve `agent-task-relay`, do not compose or expand a task or its tool permissions. Preserve task text and permissions the user supplied directly, and leave unresolved values as named placeholders for the user to review and fill locally.

## User-Run Command

After resolving every applicable opt-in, dependency approval, handoff confirmation, and remote-mutation authorization, provide the exact `gh copilot …` command in a `sh` code block. Use named placeholders for unresolved task text and tool permissions.

Tell the user to copy, paste, and run the command locally, review every prompt and tool permission before accepting it, and never share credentials, secret values, private material, or secret-bearing output in chat. Ask only whether the operation succeeded or for a sanitized error containing no private values.
