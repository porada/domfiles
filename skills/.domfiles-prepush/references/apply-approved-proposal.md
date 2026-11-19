# Apply Approved Proposal

For a history rewrite, derive the deferred command sequence and any rebase todo edits from the recorded proposal. If planning requires a change to the series or reveals an uncovered effect, [revise the affected proposal](prepare-commit-proposal.md). Do the same if the **Create Approved Commits** recheck finds a material change from the recorded baseline. Obtain the user’s approval for the revision unless the caller carries the user’s waiver and authority to apply the revised series. Apply the recorded grouping, messages, and order.

## Apply and Verify

Pass the prepared series to `sensible-commit-flow`’s **Commit Execution** and **Update Commit History** routes with the user’s proposal approval or the waiver and execution authority the user granted through the calling workflow. Complete their execution, verification, and reporting procedures, including **Verify Rewritten History**, rather than implementing a separate lifecycle here.

## Report the Local Result

Report the final commits in order with a compact old-to-new mapping and any material remaining work or validation limitation. For a content-preserving rewrite, confirm that the final contents match the recorded proposal. Keep routine verification and unchanged-state details internal.

This workflow ends at the local result. Do not offer to push or prepare a publication command unless the user requests publication help. A separate publication request follows `sensible-commit-flow`’s publication authorization and safeguards.
