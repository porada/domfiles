---
name: dom-sensible-contribution-flow
description: |-
    Use whenever `sensible-contribution-flow` applies.

metadata:
    internal: true
---

# Contribution Flow

Load `sensible-contribution-flow` and apply these personal contribution defaults under the global **Explicit user direction** policy. The base owns the contribution lifecycle and its authorization checkpoints.

## Fork Remotes

When creating or reusing a fork for a contribution, keep `origin` pointing to the fork and `upstream` pointing to the verified upstream repository. Reconcile existing remote names during authorized local setup under the base’s **Configure Git Remotes** rules, preserving existing branch tracking relationships.

## Implementation Review

Read and apply `super-review`’s `SKILL.md` from the same installed skills root for the coordinator’s implementation review of initial contributions and implementation revisions, including documentation changes. Pass the base workflow’s review scope and established authority to it. Leave the base’s design review and standalone read-only review routes unchanged.

Unless the user selects different stages or waives review, use the initial stage to satisfy the base’s review before commit preparation. For requests limited to working tree changes, select only that stage. Otherwise, select the full sequence, with the base’s commit preparation serving as `super-review`’s committed-target preparation. Carry the base’s commit authorization, packaging, message, and shared pull request history constraints into that operation. After later changes, apply `super-review`’s continuation requirements before declaring readiness.

Apply global **Contribution Preparation Authorization**, **External services**, and **Agent communication** to the selected T3 Code reviewers, providers, task material, and required follow-ups, replacements, or repeat reviews. Check disclosure authority separately before sending a proposal to a design reviewer. Before implementation, obtain approval for any uncovered implementation-review exchange. Identify the recipients, providers, and material in the existing execution checkpoint when one is needed. When asking for preparation approval, say when the selected reviewers will start. Initial implementation review follows validation of the working tree. Final review follows commit preparation.

## Adjacent Opportunities

Apply global **Task refinement** and **Preserve Adjacent Opportunities** with upstream fit in mind:

- **Current PR:** Include accepted improvements when they fit upstream expectations for this contribution. Sharing files or having a small diff doesn’t establish that fit.
- **Separate PR idea:** Surface an independently useful opportunity whose distinct outcome, design decision, or review burden makes it a better separate contribution. Explain the separation without implementing it or opening another PR.
- **Do not pursue:** Set aside opportunities whose benefit doesn’t justify the work.

Include accepted work in the contribution’s commit planning and review coverage, preserving its packaging constraints and shared pull request history rules.
