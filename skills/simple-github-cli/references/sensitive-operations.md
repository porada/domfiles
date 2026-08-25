# Sensitive Operations

Do not execute a command routed here. Require explicit user opt-in for its exact authentication, key-management, secret, or variable operation, including any alternate authentication method, host, account, configuration source, or broader scopes. Apply the entrypoint’s [Authentication](../SKILL.md#authentication) and [Remote Changes](../SKILL.md#remote-changes) boundaries before preparing it.

After resolving the applicable host, account, repository, or resource target and every required remote-mutation or key-management authorization, provide the exact command in a `sh` code block for the user to copy, paste, and run locally.

Keep credentials, tokens, private keys, secret values, one-time codes, and other private authentication material out of command literals, environment-variable examples, repository files, and the conversation. Use named placeholders and an interactive terminal prompt or an established secure machine-local source.

Tell the user to enter secret material only into the local terminal prompt and never share it in chat. Do not ask the user to paste authentication or secret-bearing command output into the conversation. Ask only whether the operation succeeded or for a sanitized error containing no private values.

Do not proceed when the operation would expose secret material, rely on plaintext credential storage, or require a secure machine-local source that has not been established. Report the boundary instead.
