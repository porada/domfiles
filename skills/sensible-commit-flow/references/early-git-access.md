# Early Git Access

This checkpoint establishes capability needs, not commit contents or execution authority.

## Establish Required Access

Keep target and state inspection read-only. Verify the repository, supplied checkout, current branch, requested Git effects, and existing index and working tree state. Identify conflicts or an in-progress operation without changing them. For commit-writing scripts or tests, use the [invocation checks](commit-execution.md#inspect-commit-writing-scripts-and-tests) to establish their targets before requesting access.

Use the client’s declared permission model to identify the Git metadata writes and other access the requested operations need. Inspect applicable hook and signing requirements through non-disclosing evidence without executing hooks or retrieving credentials. Report known separate requirements early, but do not acquire speculative access or inspect unrelated configuration. Dry runs, filesystem permission checks, successful read-only commands, and writable worktrees do not establish Git metadata write access.

## Resolve Access Checkpoint

Use the first applicable case:

1. **Existing access:** Reuse an explicit, applicable client grant without another prompt. Do not infer its scope or lifetime from a previous successful command alone.
2. **Supported advance grant:** When the client exposes a supported way to request access without executing a repository operation, request the narrowest grant for the verified target and known required effects now. A grant supplies capability only, not authority for another operation.
3. **Command-bound grant:** When permission requests must accompany command execution, use the [empty-file staging probe](#probe-index-access) now. Request the required grant with the probe rather than waiting until the task’s changes are ready. If the probe lacks user authorization, cannot preserve existing state, or has no permitted nonignored path, report that access remains unestablished and request it at the first otherwise authorized Git operation. Do not substitute an elevated read-only probe or an unrelated Git mutation.

If required access is declined or unavailable, report the affected Git requirement and pause that operation. Do not silently replace a requested committed result with working tree changes or claim that later Git execution is available. Resolve any resulting change to the deliverable with the user.

For every access case, retain the access assessment with the task context and return to the selected workflow. Respect the client’s actual grant scope and lifetime, and reassess when the target, required effects, or access changes. Early access does not bypass the [commit proposal, authorization, validation, or verification](commit-execution.md), authorize publication, or relax required hooks and signing. Do not attempt a write already known to be forbidden in the current sandbox.

## Probe Index Access

Run this probe only when the user has requested commit creation or a history update, or separately authorized the probe. When the command-bound grant case selects this probe, run it now rather than waiting for a commit proposal or confirmation. It does not authorize staging task changes or creating commits beyond the user’s request.

1. Select one fresh, nonignored file path directly under the supplied worktree root, using `.agent-<name>` unless applicable project instructions require another approved namespace. The path must be absent from `HEAD` when present, the index, and the working tree. Record existing staging for comparison. Do not reuse a file, change ignore rules, or force-add an ignored artifact.
2. Create an empty file and stage only that path with `git add -- <probe-path>`. Explain the temporary staging and immediate cleanup when requesting the supported sandbox grant. If the client declares Git metadata protected, request that grant with the operation rather than first attempting a write known to be forbidden.
3. Verify the new staged entry, immediately unstage only it with `git rm --cached -- <probe-path>`, then delete the empty working tree file. This cleanup also works before the repository’s first commit. Never reset or restore the whole index.
4. Verify that the file and its index entry are gone and that pre-existing staging is unchanged. If a step fails, clean up only the probe’s known state where permitted, report any remainder, and pause the affected operation.

A successful probe establishes index write access under the grant used, not that commit hooks or signing will succeed. A grant limited to the probe does not cover later commands, which still require their own grants.
