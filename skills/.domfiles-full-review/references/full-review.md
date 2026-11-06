# Full Review

A full review combines two independent reviews and resolves their findings through `subagent-review-flow`.

## Resolve Scope and Authority

Resolve the target from the user’s request and current task. Ask only when materially different targets remain plausible. Keep the review and corrections within that target and its affected integration boundaries.

Apply the global **Full review requests** policy to explicit user invocations.

When a governing workflow routes a review here, inherit its target and established authority. The route itself grants no additional effects.

If the user explicitly requests read-only review or excludes fixes, retain that limit and use `subagent-review-flow`’s read-only branch. Otherwise, pass the established correction authority into that workflow and resolve all validated in-scope findings within it.

## Maintain Reviewer Continuity

Load `subagent-review-flow` before selecting execution interfaces and apply its reviewer continuity requirements to both reviewer roles.

For every later full review of the same work, resume the existing reviewer conversations under `subagent-review-flow`’s continuity and recovery requirements, even after an earlier review has completed. An explicit invocation is a user-directed continuation under the global **Explicit user direction** policy. A governing workflow route continues under that workflow’s established authority. Retain each reviewer’s identifier, thread link when available, session type, provider, model, runtime settings, latest reviewed state, and continuation state in the shared review record and handoffs. Each new full review asks both reviewers to assess the current scope again, not merely revisit their earlier findings. Refresh the target, baseline, and scope before dispatch.

Distinguish a top-level thread from a delegate task when interpreting host lifecycle rules. Establish each rule’s scope before choosing the continuation operation. For a top-level thread, read its retained conversation and use the host’s permitted continuation interface without clearing context. A completed turn, settled thread, or new review round alone does not require a replacement. Reopen an existing thread when the host requires it for continuation. Replace a reviewer only when the existing conversation cannot continue under the required conditions, following `subagent-review-flow`’s recovery and role-transfer procedure. Apply `cli-subagent`’s CLI mechanics only to CLI exchanges.

For a required replacement, follow [Select Reviewers](select-reviewers.md) before choosing an execution interface. For a cross-provider replacement, reapply `cli-subagent`’s interface selection with the same provider, model, and effort requirements. The previously selected interface is binding only when the user or governing host requires it. If the required reviewer selection or interface can’t be preserved, or the retained cross-provider participant conflicts with the coordinator’s current identity, identify the specific host restriction, capability gap, or reviewer mismatch and use `subagent-review-flow`’s limited-review endpoint rather than substituting self-review or another participant. Distinguish an observed tool failure from an interpretation of host instructions. Ask only for necessary user information, authority, or a material decision.

## Coordinate Resolution

Use `subagent-review-flow` for consolidated rounds, validated fix batches, evidence reuse, and affected re-review. Give both reviewers the same target. Run their read-only assessments in parallel when supported.

Provide only the context that workflow requires, pointing to available sources rather than copying them. On follow-up, send only the necessary delta when context retention is established, using `cli-subagent`’s retained-context checks for CLI exchanges. When a follow-up requires a fresh session, use `subagent-review-flow`’s continuation handoff.

Assign each reviewer the assessment, not this orchestration workflow. Reviewers must not start another full review pair. Preserve `cli-subagent`’s recursion boundary for the cross-provider participant and its delegates.

Use `subagent-review-flow`’s completion and reporting criteria for both reviewer roles.
