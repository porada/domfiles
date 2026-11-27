---
name: dom-sensible-contribution-flow
description: |-
    Use whenever `sensible-contribution-flow` applies.

metadata:
    internal: true
---

# Contribution Flow

Load `sensible-contribution-flow` and apply these personal contribution defaults under the global **Explicit user direction** policy. The base owns the contribution lifecycle and its authorization checkpoints.

## Start a Contribution in T3 Code

Prepare each new pull request in its own worktree. Stay in a supplied worktree or in a root checkout whose pending changes the task reviews, commits, or continues. Revise an existing pull request where its branch is checked out. Outside T3 Code, use the base workflow’s supplied checkout.

Before a handoff or launch, complete the base’s read-only preflight and verify the upstream repository and target branch. Record setup authority for the fetch, any required remote setup, and the provisional contribution branch. Set up any necessary upstream remote under the base’s **Configure Git Remotes** rules before creating the worktree. For independent work, use the verified `<remote>/<target-branch>` as `baseRef` with `startFromOrigin: true` to fetch the target. For work stacked on an unmerged contribution, use its recorded prerequisite as `baseRef` with `startFromOrigin: false`.

- **Current thread:** From the project root, make `t3_worktree_handoff` the last call in the turn, before creating the branch or editing. Give it the provisional name as `branch` and the selected base settings. Set `runSetupScript: false` unless the user authorized the project’s setup script. Pass the task, user instructions and limits, verified upstream target, and authorization record in `continuationPrompt`.
- **Requested contribution threads:** When the user asks to start contribution threads, check the project’s worktree setup before launch. If creation could run a configured setup script, obtain any outstanding authorization for that script before launch. Use `t3_thread_launch` for each thread with `workspaceStrategy` set to `type: "worktree"`, the provisional `branch`, and the same base settings. Include the verified target and setup record in the assignment.

If a handoff or launch starts a setup script, verify that it completed successfully from its exit status or equivalent evidence before doing work that depends on it or could overlap its writes. Continue the base workflow with the new worktree as the supplied checkout and the branch T3 created as the provisional contribution branch. Verify it starts at the selected base. From that worktree, perform the base’s scoped upstream fetch and synchronization, reusing a verified fetch from worktree creation for independent work. The new checkout starts without the original checkout’s installed dependencies. Apply the base’s installation rules when authorized work first needs them.

## Fork Remotes

When creating or reusing a fork for a contribution, keep `origin` pointing to the fork and `upstream` pointing to the verified upstream repository. Reconcile existing remote names during authorized local setup under the base’s **Configure Git Remotes** rules, preserving existing branch tracking relationships.

## Implementation Review

Read and apply `super-review`’s `SKILL.md` from the same installed skills root for the coordinator’s implementation review of initial contributions and implementation revisions, including documentation changes. Pass the base workflow’s review scope and established authority to it. Leave the base’s design review and standalone read-only review routes unchanged.

Unless the user selects different stages or waives review, use peer reviews to satisfy the base’s review before commit preparation. For requests limited to working tree changes, select only peer reviews. Otherwise, select the full sequence, using the base’s commit preparation to prepare the target for `cold-review`. Carry the base’s commit authorization, packaging, message, and shared pull request history constraints into that operation. After later changes, apply `super-review`’s continuation requirements before declaring readiness.

Apply global **Contribution Preparation Authorization**, **External services**, and **Agent communication** to the selected T3 Code reviewers, providers, task material, and required follow-ups, replacements, or repeat reviews. Check disclosure authority separately before sending a proposal to a design reviewer. Before implementation, obtain approval for any uncovered implementation-review exchange. Identify the recipients, providers, and material in the existing execution checkpoint when one is needed. When asking for preparation approval, say when the selected reviewers will start. Peer reviews follow validation of the working tree. Cold review follows commit preparation.

## Adjacent Opportunities

Apply global **Task refinement** and **Preserve Adjacent Opportunities** with upstream fit in mind:

- **Current PR:** Include accepted improvements when they fit upstream expectations for this contribution. Sharing files or having a small diff doesn’t establish that fit.
- **Separate PR idea:** Surface an independently useful opportunity whose distinct outcome, design decision, or review burden makes it a better separate contribution. Explain the separation without implementing it or opening another PR.
- **Do not pursue:** Set aside opportunities whose benefit doesn’t justify the work.

Include accepted work in the contribution’s commit planning and review coverage, preserving its packaging constraints and shared pull request history rules.
