---
name: commit
description: |-
    Use whenever an agent considers or prepares to commit changes, from composing messages and grouping hunks through commit-related staging to creating a commit. Also use for prospective planning before implementation and for requested history updates or rebases. Use before invoking scripts or tests that create commits.

    Do not use for read-only inspection of existing commits when no commit or history update is being prepared.

metadata:
    internal: true
---

# Commit

Neither automatic activation nor `/commit` invocation authorizes staging or committing. Apply the global **Commit gate** and **Index preservation** policies throughout.

For prospective commit planning before changes exist, follow [Plan Before Implementation](#plan-before-implementation) and return to the calling workflow without entering confirmation or execution.

A bare `/commit` prepares new commits. Select [Update Unpushed Commits](references/update-unpushed-commits.md) only when the user or an applicable calling workflow explicitly requests a history update or rebase. A request to fold changes into their original commits selects that route without requiring Git terminology.

For new commits assembled from working-tree changes, prepare the proposal through [Inspect Changes](#inspect-changes), [Group Hunks](#group-hunks), and [Compose Subjects](references/compose-subjects.md). For cherry-picks and merges that create commits, including continuation after a pause, prepare it through [Preserve Operation Messages](references/preserve-operation-messages.md) instead. Every execution route then follows [Confirm Commits](#confirm-commits), [Create Approved Commits](#create-approved-commits), and [Report the Result](#report-the-result), in that order.

Before invoking a script or test that creates commits, inspect its implementation and inputs to establish the target repositories and expected commit sequence. Select the applicable route for those commits rather than assuming the invoking repository’s diff represents them. Include the exact invocation in [confirmation](#confirm-commits), and obtain approval to create the proposed batch through that command. Invoke it only if it can satisfy the selected route’s inspection, preservation, validation, and verification requirements at that route’s defined checkpoints. Otherwise, report what cannot be established or satisfied and stop before invocation.

## Plan Before Implementation

Use the intended task scope and available repository evidence to apply [Group Hunks](#group-hunks) prospectively. Identify coherent change units, their dependency order, and assumptions that could alter the breakdown. Respect supplied packaging constraints without inventing changes to satisfy them. Identify a conflict when the proposed scope does not support a coherent requested split. If the plan includes provisional subjects, use [Compose Subjects](references/compose-subjects.md) for their wording.

Keep this pass read-only. Commit counts, boundaries, and any subjects are provisional, not a staging plan or authorization to create commits. Return the breakdown to the implementation workflow. Once the changes exist, start [Inspect Changes](#inspect-changes) against the actual diff rather than treating the earlier plan as a concrete commit proposal.

## Inspect Changes

1. Resolve the current repository, checkout, and user-requested scope. Use the current branch unless the user selects another target. Identify unresolved conflicts or an in-progress Git operation before proposing new commits. Do not infer permission to amend, rewrite history, or switch branches.
2. Record `HEAD` and inspect the scoped staged and unstaged diffs separately. Inspect relevant untracked files only when applicable policy permits them. Existing staging is evidence of selection, not authorization to consume it. Identify unrelated state that must remain untouched.
3. Read complete relevant hunks with enough surrounding context to understand their relationships. Account for additions, binary changes, deletions, file modes, and renames. Use the task context to establish intent and the diff to verify it. Ask only when an ambiguity would materially change the included work or its meaning.
4. Consult a bounded sample of recent relevant commit subjects for established vocabulary and repository conventions. Do not infer editorial rules from generated Git messages or let repeated maintenance and release commits dominate the sample. Do not fetch remote history merely to compose a message.
5. Identify the applicable validation and commit requirements. Keep proposal preparation read-only. Do not edit project files, alter the index, or create commits before [confirmation](#confirm-commits). If nothing eligible remains, report that and stop.

## Group Hunks

Treat each commit as one independently understandable change, not as a container for one file type. Use the smallest number of commits that preserves meaningful intent boundaries.

- **Cohesion:** Keep implementation, necessary integration, and regression tests together when they establish one behavior. Supporting documentation and generated changes may belong to the same unit. A single subject should explain why its hunks belong together without concealing an independent concern.
- **Splitting:** Separate changes that express distinct decisions or form a useful functional-change–then-documentation sequence. Group by changed hunks rather than filenames. One file may contribute to several commits, while one commit may span many files.
- **Documentation follow-ups:** When contributing a pull request to another repository, a documentation follow-up after the functional changes is a common valid split. Do not force that split merely because documentation files changed, especially when documentation is necessary for the functional commit’s correctness.
- **Order:** Put prerequisites before their consumers. Every intermediate commit must remain coherent and must not depend on a later commit to work. A passing final working tree does not establish that an earlier proposed commit is valid.
- **Feasibility:** Check that the proposed groups can be staged from the existing changes without duplicating or dropping hunks. Keep inseparable hunks together. Do not rewrite source, introduce temporary behavior, or fabricate changes merely to manufacture a split.

## Preserve Message Constraints

Apply these safeguards to every route, including supplied and inherited messages:

- **Authorship:** Never make yourself or another AI agent a commit author or co-author. Do not append agent-attribution trailers or message signatures. Preserve established human authorship and do not silently discard supplied human co-author attribution to satisfy the bodyless format.
- **Conflicts:** Treat user-supplied wording and mandatory repository message requirements as constraints. If they conflict with the selected route’s message rules, resolve the conflict with the user before proceeding rather than silently rewriting the input, dropping attribution, or bypassing a requirement.

## Confirm Commits

Present the proposal in its own response before any staging or committing:

1. State the target repository, branch, and resolved scope briefly.
2. For history updates, use the proposal defined in [Update Unpushed Commits](references/update-unpushed-commits.md#prepare-update-proposals). For cherry-picks and merges, use [Preserve Operation Messages](references/preserve-operation-messages.md). Otherwise, show each proposed commit in execution order. Put its exact complete subject in a blockquote, followed by a concise description of its included changes. Identify the hunk boundaries when a file is shared between commits or only partly included. Do not substitute filenames alone for a change description.
3. Explain a split only when its rationale is not obvious. State material exclusions, validation limitations, and any required grants. Do not request access before its target and purpose are concrete.
4. Ask explicitly whether to execute the proposed batch, including its staging, commit creation, and any history rewrite, then stop. Approval of that request is the user’s command to execute the named batch, not merely approval of an editorial plan.

A correction to the proposal is not approval to execute it unless the user explicitly says so. Reconfirm any material change to the approved content, messages, order, or target. Authorization is limited to the proposed batch and does not waive a separate approval or security boundary.

## Create Approved Commits

Use these safeguards for every execution route. Ordinary routes validate each candidate before creating its commit, using a supported pause when necessary, and verify the recorded result before advancing. The [unpushed-history route](references/update-unpushed-commits.md#execute-history-updates) inspects the complete inputs and update plan before execution. For that route, apply steps 3 and 5 to the resulting series and final working tree only after step 4 completes the rebase, not between temporary fixup or replay steps.

1. Recheck the recorded refs, scoped diffs, and relevant index and working-tree state against the proposal. If concurrent work changes the approved content or invalidates its boundaries, stop and return the affected part to confirmation.
2. Apply the global **Index preservation** policy, including for partially staged files, and prepare only approved inputs through the selected route. Establish how its required checkpoints will be satisfied before invoking a commit-producing operation. If the mechanism cannot satisfy them, stop before invoking it. For working-tree changes, stage only the approved group, using hunk-level staging when a path is shared with another group or unrelated work. Never substitute whole-file staging for an approved partial-file selection.
3. At the selected validation checkpoint, inspect the complete effective patches and confirm they contain exactly the approved changes. Run the required validation against that candidate or completed series, not unrelated or unapproved working-tree changes.
4. Create commits through the selected route’s execution mechanism and approved message policy, preserving established human authorship. Follow repository-required hook and signing behavior at every invocation without bypassing checks. For newly composed messages, supply the exact approved subject through literal-safe, noninteractive input so backticks remain message characters rather than shell syntax.
5. At the selected verification checkpoint, verify each recorded patch, complete message, and authorship against the proposal after hooks and commit tooling have run, including the selected route’s additional checks. Verify the remaining index and working-tree state.

If execution encounters conflicts, errors, failed checks, or a result that differs from the approved proposal, preserve the resulting state and completed commits, report what happened, and stop. Continue only when the resolution remains within the approved scope and the checks required at that checkpoint pass. Obtain fresh confirmation for a material change. Do not add unapproved fixes, blindly retry, or automatically amend, reset, or replay the batch.

## Report the Result

Finish with the created commit references and subjects in execution order, plus any material remaining work or validation limitation. If execution stops partway, distinguish the commits actually created from the remaining proposal. Stop after local commits. Follow the global **Git publication** prohibition.
