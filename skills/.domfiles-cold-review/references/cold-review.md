# Cold Review

## Resolve the Review

Resolve the target and overrides from the invocation and conversation. A direct invocation defaults to the current task’s commits and pending changes. Ask for the intended target if the conversation does not establish it. On direct invocation, use global **Review workflow requests** and show the user the target’s complete commit messages before starting. Quote existing messages verbatim and label drafts for pending work as proposed. When another workflow calls this skill, retain its authority, title, review groups, dependencies, and validation requirements. Coordinate the review in the invoking thread. Use `subagent-review-flow` for shared coordination and both `sensible-commit-flow` and `dom-sensible-commit-flow` for commit operations.

Honor read-only and no-commit limits. For a read-only review, resolve disputes and return the validated findings that remain standing. Apply correction and repeat steps only when authorized. If the target includes work that must be committed and the invocation prohibits that, identify the affected work and continue with any independent groups the invocation permits.

Each repository and target checkout pair forms one review unit. Default to one group of whole commits per unit. Honor explicit grouping. Split independent work only when each group can cover its interactions and the split saves time. Keep comparison sources separate from review targets. Account for all selected changes, including pending work to prepare, and preserve dependencies and checks across groups and repositories. Complete each group’s prerequisite validation before dispatch. Each group owns its reviewer, commit list, findings, and cycle count.

## Prepare the Committed Target

Commit task-owned uncommitted changes required by the group before its cold review. Apply the established commit authority and global **Commit corrections** policy. Coordinate Git operations across groups sharing a repository so they preserve live review targets and unrelated work.

Identify the exact commits forming the group’s committed target, all within the group’s repository. Include applicable task-owned fixups and corrections without including unrelated intervening commits. Verify their relationships rather than relying solely on matching subjects. Check these commits against the work assigned to this group. Keep other groups’ pending work in the review record until [completion](#complete-the-review).

Keep the selected commit list in history order. Do not rewrite history merely to reduce it to one SHA. Retain the selected committed target and its changes in the shared review record.

## Run a Cold Review Cycle

Start a fresh reviewer through the selected execution interface, using the cold review profile and title from `super-review`’s reviewer defaults. Do not inherit the coordinator’s conversation or supply implementation history, earlier review results, or a context handoff.

In every commit-review prompt, enclose each standalone SHA and each complete range in literal backticks. For one commit, send exactly:

```text
Review commit `<current-commit-sha>`
```

For multiple commits, work through the selection in history order. Starting at the first unrepresented commit, take the longest run of two or more selected commits where each commit is a parent of the next and `git rev-list <first-sha>^..<last-sha>` returns exactly the commits in that run. Record the range as `<first-sha>^..<last-sha>`. If no such run starts there, record that commit’s SHA. Continue until every selected commit is represented.

For one range, send exactly:

```text
Review the commits in `<range>`. Consider each commit and how the changes work together.
```

For several entries, name each range as “the commits in `<range>`” and each standalone SHA as “the single commit `<sha>`”. Keep the entries in history order. Join two with “and”. For three or more, separate them with commas and use “and” before the last entry. Use this form, shown for a range followed by one commit:

```text
Review the commits in `8a31d7c2^..b74e290f` and the single commit `d0526ac1`. Consider each commit and how the changes work together.
```

Keep all other coordination state outside these prompts.

Retain the initial response for [repeat accounting](#repeat-when-required). Use the shared review process for findings and disputes throughout the cycle.

Apply authorized corrections and accepted opportunities under the governing policies. Validate and record them through the commit workflow, then send the applicable review prompt for the complete current committed target through the shared reviewer-continuity process. Resume the same conversation when available and permitted. For a required fresh session, preserve the independent initial assessment and predecessor reconciliation under [Replace a Cold Reviewer](#replace-a-cold-reviewer), with successor counting governed by the shared continuity rules.

Continue until no findings remain standing and all required verification is complete. Discussion messages may contain the evidence necessary to resolve a dispute. Subsequent commit-review requests retain the prompt forms above.

The independent first assessment helps reveal context the repository may need to document. Assess resulting concerns under **Durable knowledge**, without changing the reviewer’s initial prompt.

## Repeat When Required

Count distinct findings from the cycle’s initial response that are validated as requiring a change. Exclude duplicates, rejected claims, and optional opportunities. A finding still counts after its correction.

When that count is three or more, finish resolving the current cycle before starting another cold review cycle with a fresh reviewer. Increment the cycle number and apply the same initial-response rule to the new cycle. Record when the required repeat has started so later continuation does not schedule it twice.

Findings first raised during follow-ups do not affect this count.

## Replace a Cold Reviewer

Use the shared replacement process while preserving an independent initial assessment. Start the replacement with only the applicable [current-commit prompt](#run-a-cold-review-cycle).

Retain the current cycle number, outstanding obligations, and established repeat requirement. A replacement does not count as a repeated cold review.

If the predecessor produced no initial report, use the replacement’s initial response for repeat accounting. Otherwise, retain the predecessor’s initial-response finding set.

Keep predecessor evidence with the coordinator until the replacement has independently assessed the target. Reconcile unresolved predecessor findings afterward through focused discussion.

## Continue After Changes

Once a group starts, retain its identity through corrections and follow-ups. Update its target for affected work and new interactions, allowing overlap with other groups when needed. Add groups for independently added scope. Keep existing groups’ reviewer continuity, cycle counts, and repeat obligations.

When a change affects a group’s reviewed result or integration boundaries, complete any prerequisite follow-ups from the caller, [prepare the updated committed target](#prepare-the-committed-target), and resume the latest valid reviewer through the shared continuity process with the applicable [current-commit prompt](#run-a-cold-review-cycle). This also applies after a group has cleared.

Continuing a reviewer role through the shared continuity process does not create a new numbered cycle or reset initial-response accounting. Superseded reviewer instances remain history.

## Complete the Review

Account for every selected change in the cleared committed targets and their integration boundaries, every required repeat, and validation across groups and repositories. Report the resulting commits, any deferrals the caller permits, and material limitations. Claim clearance only for the work it covers.

Use the shared lifecycle in `super-review`’s `references/review-execution.md` to settle each reviewer and probe. Return the result in the invoking thread and leave it open for the user or calling workflow.
