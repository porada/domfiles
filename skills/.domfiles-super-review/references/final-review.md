# Final Review

The reviewer, commit list, findings, and cycle number below belong to one [final review group](super-review.md#choose-review-groups).

## Prepare the Committed Target

Commit task-owned uncommitted changes required by the group before its final review, including when the initial stage was omitted. Apply the established commit authority and global **Commit corrections** policy. Coordinate Git operations across groups sharing a repository so they preserve live review targets and unrelated work.

Identify the exact commits forming the group’s committed target, all within its unit’s repository. Include applicable task-owned fixups and corrections without including unrelated intervening commits. Verify their relationships rather than relying solely on matching subjects. Check these commits against the work assigned to this group under [Choose Review Groups](super-review.md#choose-review-groups). Other groups’ pending work remains in the review record until the [whole-run coverage check](super-review.md#complete-the-run).

Keep the selected commit list in history order. Do not rewrite history merely to reduce it to one SHA. Retain the selected committed target and its changes in the shared review record.

## Run a Final Review Cycle

Start a fresh reviewer through the [selected execution interface](super-review.md#establish-review-execution), using the final profile and title from [Reviewer Defaults](reviewer-defaults.md). Do not inherit the coordinator’s conversation or supply implementation history, earlier review results, or a context handoff.

For one commit, send exactly:

> `Review commit <current-commit-sha>`

For multiple commits, work through the selection in history order. Starting at the first unrepresented commit, take the longest run of two or more selected commits where each commit is a parent of the next and `git rev-list <first-sha>^..<last-sha>` returns exactly the commits in that run. Record the range as `<first-sha>^..<last-sha>`. If no such run starts there, record that commit’s SHA. Continue until every selected commit is represented.

For one range, send exactly:

> `Review commits <range> as a single unit`

For several entries, separate them with commas and send exactly:

> `Review commits <entry-list> as a single unit. Expand each range separately and include each standalone SHA as one commit.`

Keep all other coordination state outside these prompts.

Retain the initial response for [repeat accounting](#repeat-when-required). Use the shared review process for findings and disputes throughout the cycle.

Apply authorized corrections and accepted opportunities under the governing policies. Validate and record them through the commit workflow, then send the applicable review prompt for the complete current committed target through the shared reviewer-continuity process. Resume the same conversation when available and permitted. For a required fresh session, preserve the independent initial assessment and predecessor reconciliation under [Replace a Final Reviewer](#replace-a-final-reviewer), with successor counting governed by the shared continuity rules.

Continue until no findings remain standing and all required verification is complete. Discussion messages may contain the evidence necessary to resolve a dispute. Subsequent commit-review requests retain the prompt forms above.

The independent first assessment helps reveal context the repository may need to document. Assess resulting concerns under **Durable knowledge**, without changing the reviewer’s initial prompt.

## Repeat When Required

Count distinct findings from the cycle’s initial response that are validated as requiring a change. Exclude duplicates, rejected claims, and optional opportunities. A finding still counts after its correction.

When that count is three or more, finish resolving the current cycle before starting another fresh final-review cycle. Increment the cycle number and apply the same initial-response rule to the new cycle. Record when the required repeat has started so later continuation does not schedule it twice.

Findings first raised during follow-ups do not affect this count.

## Replace a Final Reviewer

Use the shared replacement process while preserving an independent initial assessment. Start the replacement with only the applicable [current-commit prompt](#run-a-final-review-cycle).

Retain the current cycle number, outstanding obligations, and established repeat requirement. A replacement does not count as a repeated final review.

If the predecessor produced no initial report, use the replacement’s initial response for repeat accounting. Otherwise, retain the predecessor’s initial-response finding set.

Keep predecessor evidence with the coordinator until the replacement has independently assessed the target. Reconcile unresolved predecessor findings afterward through focused discussion.

## Continue After Changes

If final review has already completed, finish the applicable earlier follow-up under the [group’s stage prerequisites](super-review.md#run-the-initial-stage), [prepare the updated committed target](#prepare-the-committed-target), resume the latest valid final reviewer through the shared continuity process, and send the applicable [current-commit prompt](#run-a-final-review-cycle). Continue an active final reviewer in the same way once the updated target is ready.

Continuing a reviewer role through the shared continuity process does not create a new numbered cycle or reset initial-response accounting. Superseded reviewer instances remain history.
