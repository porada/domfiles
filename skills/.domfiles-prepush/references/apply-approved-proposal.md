# Apply Approved Proposal

After approval, recheck the approved proposal’s refs, selected changes, exclusions, and preservation assumptions against the execution baseline. If they materially change, [revise the affected proposal and obtain approval](prepare-commit-proposal.md) rather than applying a stale plan. Do not silently change approved messages or grouping during execution.

## Apply and Verify

Pass the approved series and its authority to `sensible-commit-flow`’s **Commit Execution** and **Update Commit History** routes. Complete their execution, verification, and reporting procedures, including **Verify Rewritten History**, rather than implementing a separate lifecycle here.

## Offer Publication

At the end, offer to push the verified result if publication remains appropriate under applicable gates. Approval to apply the commit plan does not authorize pushing. Resolve the destination and any required publication checks before requesting permission to execute that push. If the user approves, use `sensible-commit-flow`’s publication safeguards, including its explicit expected-head lease for a published replacement. Otherwise leave publication to the user. Explain a remaining publication gate rather than offering to bypass it.
