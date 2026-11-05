# Apply Approved Proposal

After approval, recheck the approved proposal’s refs, selected changes, exclusions, and preservation assumptions against the execution baseline. If they materially change, [revise the affected proposal and obtain approval](prepare-commit-proposal.md) rather than applying a stale plan. Do not silently change approved messages or grouping during execution.

## Apply and Verify

Pass the approved series and its authority to `sensible-commit-flow`’s **Commit Execution** and **Update Commit History** routes. Complete their execution, verification, and reporting procedures, including **Verify Rewritten History**, rather than implementing a separate lifecycle here.

## Report the Local Result

Report the final commits in order with a compact old-to-new mapping and any material remaining work or validation limitation. For a content-preserving rewrite, confirm that the final contents match the approved result. Keep routine verification and unchanged-state details internal.

This workflow ends at the local result. Do not offer to push or prepare a publication command unless the user requests publication help. A separate publication request follows `sensible-commit-flow`’s publication authorization and safeguards.
