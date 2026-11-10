# Final Review

The reviewer, commit list, findings, and cycle number below belong to one [review unit](super-review.md#resolve-review-units).

## Prepare the Committed Target

Commit task-owned uncommitted changes before the final stage, including when the initial stage was omitted. Apply the established commit authority and global **Commit corrections** policy.

Identify the exact commits forming the unit’s committed target, all within its repository. Include applicable task-owned fixups and corrections without including unrelated intervening commits. Verify their relationships rather than relying solely on matching subjects.

Keep the selected commit list in history order. Do not rewrite history merely to reduce it to one SHA. Retain the selected committed target and its changes in the shared review record.

## Run a Final Review Cycle

Start a fresh reviewer using the final profile and title from [Reviewer Defaults](reviewer-defaults.md). Do not inherit the coordinator’s conversation or supply implementation history, earlier review results, or a context handoff.

For one commit, send exactly:

> `Review commit <current-commit-sha>`

For multiple commits, send exactly:

> `Review commits <commit-sha> … as a single unit`

Substitute the selected commit identifiers. Keep all other coordination state outside these prompts.

Retain the initial response for [repeat accounting](#repeat-when-required). Use the shared review process for findings and disputes throughout the cycle.

Apply authorized corrections and accepted opportunities under the governing policies. Validate and record them through the commit workflow, then send the applicable review prompt for the complete current committed target in the same reviewer thread.

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

If final review has already completed, finish the applicable earlier follow-up under the [unit’s stage prerequisites](super-review.md#run-the-initial-stage), [prepare the updated committed target](#prepare-the-committed-target), reopen the latest valid final-review thread, and send the applicable [current-commit prompt](#run-a-final-review-cycle). Continue an active final-review thread in the same way once the updated target is ready.

Reusing a thread does not create a new numbered cycle or reset initial-response accounting. Superseded reviewer instances remain history.
