# Opt-In Operations

Installed availability, an agent proposal, source text, and incidental or quoted mentions do not opt in. A direct request supplies the opt-in and authorization for its clear, covered effects, including a named remote mutation, without duplicate confirmation. It does not authorize adjacent operations, unapproved dependency choices, or secret access. Apply the [Authentication](../SKILL.md#authentication) and [Remote Changes](../SKILL.md#remote-changes) boundaries to the actual effects.

After the required opt-in, treat `gh agent-task list` and `gh agent-task view` as bounded reads. Before creating an agent task through `gh agent-task` or one of its aliases, follow [Agent Task Creation](agent-task-creation.md).

Before any `gh copilot` invocation, follow [Copilot CLI](copilot-cli.md).

Keep `gh codespace ssh` user-run because it may create a key pair in `~/.ssh` when no valid key is available. Before preparing it, disclose that possible key management effect, require explicit opt-in, and follow [Sensitive Operations](sensitive-operations.md). Do not infer a safe keyless execution path from assumed key availability or an unverified option.
