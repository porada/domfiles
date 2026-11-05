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

Read and apply `full-review`’s `SKILL.md` from the same installed skills root for the coordinator’s implementation review of initial contributions and implementation revisions, including documentation changes. Pass the base workflow’s review scope and established authority to it. Leave the base’s design review and standalone read-only review routes unchanged.

Under **External services**, include `full-review`’s selected provider review exchange through the interface selected by `cli-subagent`, follow-up exchanges in the same reviewer conversations and replacement sessions preserving its required interfaces and reviewer selection constraints, and disclosure of task-required repository material in the contribution’s existing execution checkpoint. Reuse explicit authorization that already covers those effects. When no checkpoint is otherwise needed, resolve only the uncovered provider exchange authorization before implementation. Loading this overlay doesn’t authorize disclosure.

## Adjacent Opportunities

Apply global **Task refinement** and **Preserve Adjacent Opportunities** with upstream fit in mind:

- **Current PR:** Include accepted improvements when they fit upstream expectations for this contribution. Sharing files or having a small diff doesn’t establish that fit.
- **Separate PR idea:** Surface an independently useful opportunity whose distinct outcome, design decision, or review burden makes it a better separate contribution. Explain the separation without implementing it or opening another PR.
- **Do not pursue:** Set aside opportunities whose benefit doesn’t justify the work.

Include accepted work in the contribution’s commit planning and review coverage, preserving its packaging constraints and shared pull request history rules.
