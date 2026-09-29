# Early Git Access

Assess access before implementation or substantial commit preparation, not after the changes are ready. This checkpoint establishes capability needs, not commit contents or execution authority.

## Establish the Required Access

Keep the preflight read-only. Verify the repository, supplied checkout, current branch, requested Git effects, and existing index and working tree state. Identify conflicts or an in-progress operation without changing them. For commit-writing scripts or tests, use the [invocation checks](commit-execution.md#commit-writing-scripts-and-tests) to establish their targets before requesting access.

Use the client’s declared permission model to identify the Git metadata writes and other access the requested operations need. Inspect applicable hook and signing requirements through non-disclosing evidence without executing hooks or retrieving credentials. Report known separate requirements early, but do not acquire speculative access or inspect unrelated configuration.

## Resolve the Checkpoint

Use the first applicable case:

1. **Existing access:** Reuse an applicable grant without another prompt. Do not infer its scope or lifetime from a previous successful command alone.
2. **Supported advance grant:** When the client exposes a supported way to request access without executing a repository operation, request the narrowest grant for the verified target and known required effects now. A grant supplies capability only, not authority for another operation.
3. **Command-bound grant:** When permission requests must accompany command execution, state that advance acquisition is unavailable. Continue otherwise authorized work, then request access at the first genuinely required Git operation after its normal workflow prerequisites are satisfied. Do not manufacture a fetch, staging change, empty commit, index mutation, or elevated read-only probe solely to provoke a prompt.

If required access is declined or unavailable, report the affected Git requirement and pause that operation. Do not silently replace a requested committed result with working tree changes or claim that later Git execution is available. Resolve any resulting change to the deliverable with the user.

Retain the access assessment with the task context and return to the selected workflow. Respect the client’s actual grant scope and lifetime, and reassess when the target, required effects, or access changes. Early access does not bypass the [commit proposal, authorization, validation, or verification](commit-execution.md), authorize publication, or relax required hooks and signing. Do not attempt a write already known to be forbidden in the current sandbox.
