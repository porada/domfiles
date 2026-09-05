---
name: commit
description: |-
    Use whenever an agent considers or prepares to commit changes, from composing messages and grouping hunks through commit-related staging to creating a commit. Also use before invoking scripts or tests that create commits.

    Do not use for read-only inspection of existing commits when no commit is being prepared.

metadata:
    internal: true
---

# Commit

Neither automatic activation nor `/commit` invocation authorizes staging or committing. Apply the global **Commit gate** and **Index preservation** policies throughout.

For commits assembled from working-tree changes, prepare the proposal through [Inspect Changes](#inspect-changes), [Group Hunks](#group-hunks), and [Compose Subjects](#compose-subjects). For cherry-picks and merges that create commits, including continuation after a pause, prepare it through [Preserve Operation Messages](#preserve-operation-messages) instead. Both routes then follow [Confirm Commits](#confirm-commits), [Create Approved Commits](#create-approved-commits), and [Report the Result](#report-the-result), in that order.

Before invoking a script or test that creates commits, inspect its implementation and inputs to establish the target repositories and expected commit sequence. Select the applicable route for those commits rather than assuming the invoking repository’s diff represents them. Include the exact invocation in [confirmation](#confirm-commits), and obtain approval to create the proposed batch through that command. Invoke it only if it can satisfy every per-commit requirement, including inspection before creation and verification before advancing. Otherwise, report what cannot be established or satisfied and stop before invocation.

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

## Compose Subjects

Use this decision order for each group:

1. Identify the dominant intended change, using verified task context rather than patch mechanics alone. Distinguish the actual change from capabilities that can merely be inferred from it.
2. Choose the narrowest durable repository concept that captures that intent. Name a concrete artifact or surface when sufficient. Move up to a capability, maintenance class, or subsystem only when the narrower objects are supporting details. Reuse established compact vocabulary such as `config` and `README` when it fits.
3. Choose a semantic verb. Added lines do not necessarily mean `Add`, and deleted lines do not necessarily mean `Remove`. Use the role guide below as a vocabulary aid, not an exhaustive list or a rigid taxonomy.
4. Add a qualifier only when it distinguishes a material condition, mechanism, purpose, or scope. Describe the delta rather than inventorying the resulting state. A conjunction may join objects under one action, but should not combine unrelated changes.
5. Apply the [message form](#message-form). Check that the subject covers its assigned hunks without claiming an unverified motivation or outcome.

The role guide is alphabetized by editorial role:

| Role | Verb Choices |
| --- | --- |
| Adoption | Use `Install` for managed provisioning, `Set up` for integrated first-time configuration, and `Use` for adopting a selected mechanism. |
| Creation | Use `Add` for a concrete artifact or supported case, `Establish` for durable architecture, and `Introduce` for a named public option or substantial capability. |
| Maintenance | Use `Adjust` or `Tweak` for a bounded refinement and `Update` for an existing surface or recurring maintenance class. Do not force a distinction between near-synonyms that context cannot establish. |
| Organization | Use `Clean up` for heterogeneous pruning within one area and `Refactor` when structure or ownership is the organizing decision. `Refactor` does not guarantee behavior preservation. Prefer a direct operation such as `Extract`, `Move`, `Remove`, or `Rename` when that operation defines the change. |
| Outcomes | Use `Disable` or `Enable` for the resulting inactive or active state and `Warn on` for warning severity. Use `Fix` for an established defect. Choose a precise outcome verb such as `Ensure`, `Preserve`, `Prevent`, or `Reject` when it states the effect more clearly. |

### Recurring Forms

- **Dependencies:** Use `Update dependencies` for a dependency-maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Documentation:** Use `Update documentation` for a documentation follow-up in a contribution to another repository. A specific maintained surface may instead justify a narrower subject, such as “Update `README`.”
- **Special commits:** Use `Initial commit` for a repository root. Use a bare semantic version only for an actual release commit when that form is established in the repository. Do not choose or change a release version as part of composing its subject.

### Message Form

- **Grammar:** Write one compact, sentence-case imperative clause, normally `<verb> <object>`, subject to the special forms above. Omit articles that add no meaning, colons, Conventional Commit prefixes, scope labels, and terminal punctuation. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Literal names:** Put exact searchable tokens in backticks, including commands, configuration keys, domains, file labels, package selectors, paths, and rule IDs. Leave conceptual categories and canonically styled product names in prose. For example, `typescript@7` is a literal selector, while TypeScript is a product name.
- **Bodyless output:** The subject is the complete proposed message. Do not generate a body, explanatory prose, issue-reference paragraphs, testing checklists, or trailers. Preserve an existing hosted `(#<number>)` subject suffix when it belongs to the selected message, but never invent one.
- **Authorship:** Never make yourself or another AI agent a commit author or co-author. Do not append agent-attribution trailers or message signatures. Preserve established human authorship and do not silently discard supplied human co-author attribution to satisfy the bodyless format.
- **Conflicts:** Treat user-supplied wording and mandatory repository message requirements as constraints. If they conflict with this message form, resolve the conflict with the user before proceeding rather than silently rewriting the input, dropping attribution, or bypassing a requirement.

## Confirm Commits

Present the proposal in its own response before any staging or committing:

1. State the target repository, branch, and resolved scope briefly.
2. For inherited or generated messages, use the proposal defined in [Preserve Operation Messages](#preserve-operation-messages). Otherwise, show each proposed commit in execution order. Put its exact complete subject in a blockquote, followed by a concise description of its included changes. Identify the hunk boundaries when a file is shared between commits or only partly included. Do not substitute filenames alone for a change description.
3. Explain a split only when its rationale is not obvious. State material exclusions, validation limitations, and any required grants. Do not request access before its target and purpose are concrete.
4. Ask explicitly whether to execute the proposed batch, including its staging and commit creation, then stop. Approval of that request is the user’s command to execute the named batch, not merely approval of an editorial plan.

A correction to the proposal is not approval to execute it unless the user explicitly says so. Reconfirm any material change to the approved content, messages, order, or target. Authorization is limited to the proposed batch and does not waive a separate approval or security boundary.

## Create Approved Commits

Execute the approved batch in order:

1. Recheck the recorded refs, scoped diffs, and relevant index and working-tree state against the proposal. If concurrent work changes the approved content or invalidates its boundaries, stop and return the affected part to confirmation.
2. Apply the global **Index preservation** policy, including for partially staged files, and prepare only the next approved candidate through the selected route. Before invoking a commit-producing operation, establish how candidate validation will occur before commit creation, using a supported pause when necessary. If the mechanism cannot satisfy the required checks, stop before invoking it. For working-tree commits, stage only the approved group, using hunk-level staging when a path is shared with another group or unrelated work. Never substitute whole-file staging for an approved partial-file selection.
3. Inspect the complete effective candidate patch and confirm it contains exactly the approved changes. Run the required validation against that candidate rather than relying on later or unstaged changes.
4. Create the commit through the selected route’s execution mechanism and approved message policy, preserving established human authorship. Follow repository-required hook and signing behavior without bypassing checks. For working-tree commits, supply the exact approved subject through literal-safe, noninteractive input so backticks remain message characters rather than shell syntax.
5. Verify each recorded patch, complete message, and authorship against the proposal after hooks and commit tooling have run, including the selected route’s additional checks. Verify the remaining index and working-tree state before advancing to the next commit.

If execution encounters conflicts, errors, failed checks, or a result that differs from the approved proposal, preserve the resulting state and completed commits, report what happened, and stop. Continue only after required checks pass and the resolution remains within the approved scope. Obtain fresh confirmation for a material change. Do not add unapproved fixes, blindly retry, or automatically amend, reset, or replay the batch.

## Preserve Operation Messages

1. Resolve the repository, destination checkout and branch, source commits, and operation order. Record the destination’s starting `HEAD`, inspect the source patches and complete messages against the relevant destination history, and identify unrelated index and working-tree state to preserve. Do not substitute the destination’s uncommitted diff for the operation’s changes or proceed through an unrelated Git operation already in progress.
2. Preserve complete inherited cherry-pick messages and Git-generated merge messages by default, including bodies and trailers. Skip hunk grouping and subject composition, and do not impose the authored-message conventions. Retain the **Authorship** and **Conflicts** safeguards in [Message Form](#message-form). Resolve an explicit message-rewrite request before proposing the operation rather than silently normalizing the existing message.
3. Prepare a read-only proposal naming the exact operation and invocation, source commit IDs and order, destination and starting `HEAD`, intended changes, and expected parent relationships. Put each inherited message in a blockquote without rewriting it. For generated messages, show a read-only preview when available. Otherwise, state that Git will generate the message from the approved inputs instead of inventing exact text.
4. During [Create Approved Commits](#create-approved-commits), use the operation’s native preparation, command, and continuation flow rather than staging authored hunk groups or replacing it with freshly composed commits. For cherry-pick sequences, apply one source commit at a time. Add verification of expected parent relationships to the shared post-commit checks.

## Report the Result

Finish with the created commit references and subjects in execution order, plus any material remaining work or validation limitation. If execution stops partway, distinguish the commits actually created from the remaining proposal. Stop after local commits. Follow the global **Git publication** prohibition.
