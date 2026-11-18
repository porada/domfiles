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

Resolve the operation through the global **Automatic commits**, **Commit corrections**, and **Commit offers** policies. Pass the actual governing grant, task scope, applicable review evidence or reviewed findings and their bounded corrections, and current target into the base’s **Alternative Approval Modes** route. Include any separate **Checkpoint quality** permission and its remaining hard gates. This overlay supplies neither authorization nor a review result.

Use the base’s history-update route for amendments and replay under **Commit corrections**, carrying the verified affected range and concurrent-work constraints. Use its new-commit route for retained fixups and independent changes. Before a retained fixup, inspect the target’s patch and complete message, apply the base’s **Establish Rewrite Eligibility** checks to that target, and confirm its ancestry. Use the fixup subject under **Recurring Subjects**, then verify the recorded target and contents. For subject-based fixups, use `git commit --fixup=<commit-oid>` to copy the target subject exactly.

Leave target message corrections to separately authorized history updates through the base’s history route. For every **Rewrite eligibility** check, establish the user’s authorship from task-confirmed author identity or reliable repository evidence, not the committer field or an assumed email alias. Identify upstream or vendor imports from their provenance, not just their paths.

Before a history update involving existing fixups or their targets, follow [Preserve Pending Fixups](references/preserve-pending-fixups.md).

## Contribution Preparation Authorization

Apply the global **Contribution Preparation Authorization** policy only when its managed-installation and task conditions hold. That policy alone delegates this mode. Supply its recorded setup authorization or checkpoint grant to `sensible-commit-flow` through its **Alternative Approval Modes** reference. Neither setup authorization nor a checkpoint grant supplies a waiver.

For each call, supply the verified target, actual change scope, and requested operation. Carry forward the grant’s express request to revise provisional messages without assuming that supplied exact wording or all inherited messages may be revised. Return each commit result to `sensible-contribution-flow` while the recorded lifetime continues. Other calls follow the public skill’s applicable authorization path, including already authorized history updates.

Apply **Commit corrections** during contribution preparation too. Consolidate pending fixups only when the active authority covers their targets, contents, and affected history. Resolve uncovered history updates before declaring readiness.

## Message Form

For newly authored wording, replace the conditional lookup in [Compose Messages](../sensible-commit-flow/references/compose-messages.md) with one sample per batch, even when repository conventions are clear. Select up to 20 recent non-merge commits by the user from the current repository’s local history. Match the Git author using the user’s configured Git author name or another task-confirmed author identity without exposing private author details. Apply the public guidance on Git-generated, maintenance, and release messages when selecting examples. Favor comparable changes for vocabulary and phrasing, and infer conventions only from recurring usage. If no examples qualify or the author cannot be identified safely, rely on the written guidance alone.

- **Subject:** Prefer the shortest clear statement of the shared purpose, using the public skill’s grammar except for the recurring subjects below. Omit colons, Conventional Commit prefixes, and scope labels. Do not use `a`, `an`, or `the` as articles in newly authored subjects. Rephrase to preserve meaning without articles rather than retaining them for fluency. Preserve exact names and code tokens.
- **Body:** Use the subject as the complete newly authored message by default. Add an explanatory body, issue reference paragraphs, testing checklists, or trailers only when explicitly requested or required by the applicable message constraints. Route a remaining conflict with supplied attribution or another required message element through the public skill’s message safeguards rather than discarding it.
- **Issue references:** When including issue references in a newly authored message for a commit not intended for a pull request, put each reference in a separate body paragraph before any trailers. Use `Closes #<number>` when the commit resolves the issue, and bare `#<number>` for a reference without closure. For another repository, use `<owner>/<repository>#<number>`, retaining `Closes` when applicable. Use no terminal punctuation.

## Recurring Subjects

- **Dependencies:** Use `Update dependencies` for a dependency maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Fixups:** Prefer `fixup! <target-subject>`, preserving the target commit’s complete subject without applying **Message Form**’s wording rules to the copied subject. Use `fixup! <full-commit-oid>` when another ancestor of `HEAD` has the same subject. Select and record the target by its full object ID in either case.
- **Root commits:** Use `Initial commit` for a commit with no parents.
- **Releases:** Use a bare semantic version only for an actual release commit when that form is established in the repository.
