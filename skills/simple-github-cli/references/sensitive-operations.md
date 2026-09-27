# Sensitive Operations

Require explicit user direction for the exact operation and apply the entrypoint’s [Authentication](../SKILL.md#authentication) and [Remote Changes](../SKILL.md#remote-changes) boundaries. Authentication, key management, `gh secret` operations, and commands that directly handle credentials or private machine or account values remain user-run, including changes to authentication sources or scopes.

A direct, scoped request may authorize non-secret `gh variable` operations without duplicate confirmation. Execute only when the target and effects are clear and the necessary inputs and outputs are established as non-secret without inspecting private values to decide. A variable’s command family or name alone does not establish that its value is safe to read or disclose. If that boundary cannot be established, keep the operation user-run. Do not enumerate unrelated values or use a variable operation to retrieve credentials.

For a user-run operation, resolve the applicable host, account, repository, or resource target and every required remote mutation or key management authorization, then provide the exact command in a `sh` code block for the user to copy, paste, and run locally.

Keep credentials, tokens, private keys, secret values, one-time codes, and other private authentication material out of command literals, environment variable examples, repository files, and the conversation. Use named placeholders and an interactive terminal prompt or an established secure machine-local source.

Tell the user to enter secret material only into the local terminal prompt and never share it in chat. Do not ask the user to paste authentication or secret-bearing command output into the conversation. Ask only whether the operation succeeded or for a sanitized error containing no private values.

Do not proceed when the operation would expose secret material, rely on plaintext credential storage, or require a secure machine-local source that has not been established. Report the boundary instead.
