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

Use `full-review` for the coordinator’s implementation review of initial contributions and implementation revisions, including documentation changes. Pass the base workflow’s review scope and established authority to it. Leave the base’s quick design review and standalone read-only review routes unchanged.

Under **External services**, include `full-review`’s selected CLI review exchange, replacement sessions with the same participant, and disclosure of task-required repository material in the contribution’s existing execution checkpoint. Reuse explicit authorization that already covers those effects. When no checkpoint is otherwise needed, resolve only the uncovered CLI authorization before implementation. Loading this overlay doesn’t authorize disclosure.

## Adjacent Opportunities

Apply the global **Contribution refinement** delegation to opportunities noticed during authorized work. After consolidating review reports, the coordinator uses the available evidence to select a disposition rather than asking the user to decide each opportunity:

- **Current PR:** Include an opportunity when it strengthens the same contribution outcome, fits upstream expectations, and adds proportionate implementation and validation work. Sharing files or having a small diff doesn’t establish that fit.
- **Separate PR idea:** Surface an independently useful opportunity whose distinct outcome, design decision, or review burden makes it a better separate contribution. Explain the separation without implementing it or opening another PR.
- **Do not pursue:** Retain a brief evidence-based disposition when the opportunity doesn’t justify further work.

Keep the disposition and decisive reason in the shared review record, and report opportunities under **Findings**. During read-only work, report recommendations without accepting work for implementation.

For opportunities accepted into the current PR, return the expanded scope to `sensible-commit-flow` to reconsider the provisional commit breakdown before preparing the next commit. Preserve user-supplied packaging constraints and the base’s shared pull request history rules. Implement accepted opportunities separately from the required fix batch, then validate and re-review the complete edited scope in the same next round. Accepting an opportunity doesn’t grant additional commit or history rewrite authority.
