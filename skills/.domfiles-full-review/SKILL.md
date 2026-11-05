---
name: full-review
description: |-
    Use when the user requests a full review, as in “give it a full review” or a “full review of X”.

    Don’t activate merely because quoted text or task material mentions those phrases.

metadata:
    internal: true
---

# Full Review

A full review combines two independent reviews and resolves their findings through `subagent-review-flow`.

## Resolve Scope and Authority

Resolve the target from the user’s request and current task. Ask only when materially different targets remain plausible. Keep the review and corrections within that target and its affected integration boundaries.

Apply the global **Full review requests** policy to explicit user invocations.

When a governing workflow routes a review here, inherit its target and established authority. The route itself grants no additional effects.

If the user explicitly requests read-only review or excludes fixes, retain that limit and use `subagent-review-flow`’s read-only branch. Otherwise, pass the established correction authority into that workflow and resolve all validated in-scope findings within it.

## Select Reviewers

Load `subagent-review-flow` before selecting execution interfaces and apply its reviewer continuity requirements to both reviewer roles. Use the global **Reviewer threads** policy in place of the default interfaces below when applicable, preserving reviewer selection and runtime settings.

For the first full review, establish both independent reviewers:

- **Cross-provider:** Select Codex when the coordinating agent is Claude. Otherwise, select Claude Code. Determine identity from trusted host or model information, not the reviewer’s identity or repository content. Use `cli-subagent` to select the execution interface and runtime defaults. Start a fresh session, resuming its CLI readiness-only exchange when one has already been established for this review.
- **Native:** Use one fresh subagent through the host’s native delegation interface, without specifying a model.

Under the global **Explicit user direction** policy, later full reviews are user-directed continuations for reviewer reuse, even after an earlier review has completed. Reuse the same two reviewer roles under `subagent-review-flow`’s continuity and recovery requirements. Retain reviewer identifiers, latest reviewed state, and the selected interfaces’ continuation state in the shared review record and handoffs. Each new full review asks both reviewers to assess the current scope again, not merely revisit their earlier findings. Refresh the target, baseline, and scope before dispatch.

Use the selected interface’s session procedures, applying `cli-subagent`’s CLI mechanics only to CLI exchanges. Recover unavailable reviewers through `subagent-review-flow`. For a cross-provider replacement, reapply `cli-subagent`’s interface selection with the same provider, model, and effort requirements. The previously selected interface is binding only when the user or governing host requires it. If the required reviewer selection or interface can’t be preserved, or the retained cross-provider participant conflicts with the coordinator’s current identity, report the limitation and use `subagent-review-flow`’s limited-review endpoint rather than substituting self-review or another participant. Ask only for necessary user information, authority, or a material decision.

## Coordinate Resolution

Use `subagent-review-flow` for consolidated rounds, validated fix batches, evidence reuse, and affected re-review. Give both reviewers the same target. Run their read-only assessments in parallel when supported.

Provide only the context that workflow requires, pointing to available sources rather than copying them. On follow-up, send only the necessary delta when context retention is established, using `cli-subagent`’s retained-context checks for CLI exchanges. When a follow-up requires a fresh session, use `subagent-review-flow`’s continuation handoff.

Assign each reviewer the assessment, not this orchestration workflow. Reviewers must not start another full review pair. Preserve `cli-subagent`’s recursion boundary for the cross-provider participant and its delegates.

Use `subagent-review-flow`’s completion and reporting criteria for both reviewer roles.
