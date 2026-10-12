---
name: dom-sensible-contribution-flow
description: |-
    Use whenever `sensible-contribution-flow` applies.

metadata:
    internal: true
---

# Contribution Flow

Load `sensible-contribution-flow` and apply these personal contribution defaults under the global **Explicit user direction** policy. The base owns the contribution lifecycle and its authorization checkpoints.

## Implementation Review

For the coordinator’s implementation review, use two independent reviewers against the same baseline:

- One fresh native subagent, without specifying a model.
- One fresh Claude CLI session through `claude-cli-subagent`, using that skill’s runtime defaults. Resume its readiness-only exchange if already established.

Apply this selection to initial contributions and implementation revisions, including documentation changes. Leave the base’s quick design review and standalone read-only review routes unchanged. A reviewer evaluates its assigned scope rather than starting another pair.

Use `subagent-review` for prompt context, consolidated findings, fixes and disputes, same-session follow-ups, and completion. `claude-cli-subagent` owns CLI session mechanics. If either required reviewer is unavailable, report the limitation and ask how to proceed rather than substituting another reviewer or self-review.

Under **External services**, include the Claude review exchange and disclosure of task-required repository material in the contribution’s existing execution checkpoint. Reuse explicit authorization that already covers that exchange. When no checkpoint is otherwise needed, resolve only the uncovered Claude authorization before implementation. Loading this overlay supplies no disclosure consent.

## Adjacent Opportunities

Apply the global **Contribution refinement** delegation to opportunities noticed during authorized work. After consolidating review reports, the coordinator selects a disposition from the available evidence rather than asking the user to decide each opportunity:

- **Current PR:** Include an opportunity when it strengthens the same contribution outcome, fits upstream expectations, and adds proportionate implementation and validation work. Sharing files or having a small diff does not establish that fit.
- **Separate PR idea:** Surface an independently useful opportunity whose distinct outcome, design decision, or review burden makes it a better separate contribution. Explain the separation without implementing it or opening another PR.
- **Do not pursue:** Retain a brief evidence-based disposition when the opportunity does not justify further work.

Keep the disposition and decisive reason in the shared review record, and report opportunities under **Findings**. During read-only work, report recommendations without accepting work for implementation.

For opportunities accepted into the current PR, return the expanded scope to `sensible-commit-flow` to reconsider the provisional commit breakdown before the next commit preparation. Preserve user-supplied packaging constraints and the base’s shared pull request history rules. Implement accepted opportunities separately from the required fix batch, then validate and re-review the complete edited scope in the same next round. Acceptance supplies no additional commit or history rewrite authority.
