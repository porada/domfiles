# Sensitive Operations

Require explicit user direction covering the operation, target, and effects, including authentication setup, key management, `gh secret` operations, and changes to authentication sources or scopes. Apply the entrypoint’s [Authentication](../SKILL.md#authentication), [Remote Changes](../SKILL.md#remote-changes), and [Secrets and Authentication](../SKILL.md#secrets-and-authentication) boundaries.

Use a qualifying client mechanism for execution and any secret handling involved. Establish the selected command’s input, output, and storage behavior before running it. A protected prompt does not make a command that prints credentials safe. Preserve user consent and identity verification, and keep authentication in the environment that owns it.

Pass a client-managed reference or delivery path only to its documented consumer. Keep secret values out of command literals, environment assignments, arguments, repository files, task artifacts, logs, and agent-visible output. Follow the documented temporary-delivery cleanup lifecycle, including on failure. Temporary delivery does not authorize persistent plaintext credential storage or an unestablished credential source. If safe storage cannot be established, report that boundary rather than providing an execution command.

A direct, scoped request may authorize non-secret `gh variable` operations without duplicate confirmation. Execute only when the target and effects are clear and the necessary inputs and outputs are established as non-secret without inspecting private values to decide. A variable’s command family or name alone doesn’t establish that its value is safe to read or disclose. If you can’t establish that boundary, keep the operation user-run. Don’t enumerate unrelated values or use a variable operation to retrieve credentials.

For a user-run operation, resolve the applicable host, account, repository, or resource target and every required remote mutation or key management authorization, then provide the exact command in a `sh` code block for the user to copy, paste, and run locally.

Use named placeholders and direct secret entry to a protected prompt or established secure machine-local source, never chat. Don’t ask for authentication or secret-bearing command output. Ask only whether the operation succeeded or for a sanitized error containing no private values. Don’t recommend a fallback that exposes secret material or persists plaintext credentials.
