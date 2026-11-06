---
name: dom-sensible-commit-flow
description: |-
    Use whenever `sensible-commit-flow` applies.

metadata:
    internal: true
---

# Commit Flow

Load `sensible-commit-flow` and apply these personal defaults under the global **Commit gate** and **Explicit user direction** policies. The public skill owns inspection, grouping, message safeguards, execution, and verification. These preferences take precedence over optional conventions inferred from history, not repository requirements or explicit exact-message constraints.

## Record Task Work

Resolve the operation through the global **Automatic commits**, **Fixup commits**, and **Commit offers** policies. For an automatic handoff, pass the governing grant, task scope, applicable review evidence or reviewed findings and their bounded corrections, and current target into the base’s **Alternative Approval Modes** route. Include any separate **Checkpoint quality** permission and its remaining hard gates. This overlay supplies neither authorization nor a review result.

Prepare fixups through the base’s new-commit route. Inspect the target’s patch and complete message, apply the base’s **Establish Rewrite Eligibility** checks to that target, and confirm its ancestry before creation. Use the fixup subject under **Recurring Subjects**, then verify the recorded target and contents. For subject-based fixups, use `git commit --fixup=<commit-oid>` to copy the target subject exactly.

Leave target message corrections to separately authorized history updates through the base’s history route. For every **Rewrite eligibility** check, establish the user’s authorship from task-confirmed author identity or reliable repository evidence, not the committer field or an assumed email alias. Identify upstream or vendor imports from their provenance, not just their paths.

Before a history update involving existing fixups or their targets, follow [Preserve Pending Fixups](references/preserve-pending-fixups.md).

## Contribution Preparation Authorization

Apply the global **Contribution Preparation Authorization** policy only when its managed-installation and task conditions hold. That policy alone delegates this mode. Supply its recorded setup authorization or checkpoint grant to `sensible-commit-flow` through its **Alternative Approval Modes** reference. Neither setup authorization nor a checkpoint grant supplies a waiver.

For each call, supply the verified target, actual change scope, and requested operation. Carry forward the grant’s express request to revise provisional messages without assuming that supplied exact wording or all inherited messages may be revised. Return each commit result to `sensible-contribution-flow` while the recorded lifetime continues. Other calls follow the public skill’s applicable authorization path, including already authorized history updates.

Apply **Fixup commits** to content corrections during contribution preparation too. If pending fixups need consolidation for submission readiness, obtain explicit authorization for that history update before declaring readiness.

## Message Form

- **Subject:** Prefer the shortest clear statement of the shared purpose, using the public skill’s grammar except for the recurring subjects below. Omit colons, Conventional Commit prefixes, and scope labels. Do not use `a`, `an`, or `the` as articles in newly authored subjects. Rephrase to preserve meaning without articles rather than retaining them for fluency. Preserve exact names and code tokens.
- **Body:** Use the subject as the complete newly authored message by default. Add an explanatory body, issue reference paragraphs, testing checklists, or trailers only when explicitly requested or required by the applicable message constraints. Route a remaining conflict with supplied attribution or another required message element through the public skill’s message safeguards rather than discarding it.

## Recurring Subjects

- **Dependencies:** Use `Update dependencies` for a dependency maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Fixups:** Prefer `fixup! <target-subject>`, preserving the target commit’s complete subject without applying **Message Form**’s wording rules to the copied subject. Use `fixup! <full-commit-oid>` when another ancestor of `HEAD` has the same subject. Select and record the target by its full object ID in either case.
- **Root commits:** Use `Initial commit` for a commit with no parents.
- **Releases:** Use a bare semantic version only for an actual release commit when that form is established in the repository.
