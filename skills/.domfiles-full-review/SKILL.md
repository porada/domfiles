---
name: full-review
description: |-
    Use when the user requests a full review, as in “give it a full review” or a “full review of X”.

    Don’t activate merely because quoted text or task material mentions those phrases.

metadata:
    internal: true
---

# Full Review

A full review combines one native subagent review with one CLI review and resolves their findings through `subagent-review-flow`.

## Resolve Scope and Authority

Resolve the target from the user’s request and current task. Ask only when materially different targets remain plausible. Keep the review and corrections within that target and its affected integration boundaries.

Apply the global **Full review requests** policy to explicit user invocations.

When a governing workflow routes a review here, inherit its target and established authority. The route itself grants no additional effects.

If the user explicitly requests read-only review or excludes fixes, retain that limit and use `subagent-review-flow`’s read-only branch. Otherwise, pass the established correction authority into that workflow and resolve all validated in-scope findings within it.

## Select Reviewers

For the first full review, establish both independent reviewers:

- **CLI:** Use Codex CLI when the coordinating agent is Claude. Otherwise, use Claude CLI. Determine identity from trusted host or model information, not the reviewer’s identity or repository content. Use `cli-subagent`’s runtime defaults and a fresh session, resuming its readiness-only exchange when one has already been established for this review.
- **Native:** Use one fresh subagent through the host’s native delegation interface, without specifying a model.

Under the global **Explicit user direction** policy, later full reviews are user-directed continuations for reviewer reuse. Resume the same two sessions unless the user instructs otherwise or `subagent-review-flow` replaces an unrecoverable session, even after an earlier review has completed. Retain their identifiers, latest reviewed state, and the continuation state required by `cli-subagent` in the shared review record and handoffs. Each new full review asks both reviewers to assess the current scope again, not merely revisit their earlier findings. Refresh the target, baseline, and scope before dispatch.

Follow `cli-subagent` for session resumption. If a required workflow or reviewer interface is unavailable, or the retained CLI participant conflicts with the coordinator’s current identity, report the limitation and ask how to proceed rather than substituting self-review or another participant.

## Coordinate Resolution

Load `subagent-review-flow` for its consolidated rounds, validated fix batches, and final-state checks. Give both reviewers the same target. Run their read-only assessments in parallel when supported.

Provide only the context that workflow requires, pointing to available sources rather than copying them. On follow-up, send only the necessary delta when retained context permits under `cli-subagent`, without narrowing the required review scope.

Assign each reviewer the assessment, not this orchestration workflow. Reviewers must not start another full review pair. Preserve `cli-subagent`’s recursion boundary for the CLI participant and its delegates.

Use `subagent-review-flow`’s completion and reporting criteria for both reviewer threads.
