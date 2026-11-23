# Prepare Commit Proposal

Use `sensible-commit-flow`’s **Inspect Changes**, **Group Hunks**, **Compose Messages**, and **Update Commit History** routes. Do not invoke **Automatic commits** during proposal preparation or add a second commit offer alongside the proposal.

## Recommend the Series

Inspect the diff of every commit eligible for regrouping in the selected range, even when the working tree is clean. Look for whole commits or individual hunks that serve the same logical change and recommend grouping them, even across intervening commits or when the commit count stays the same. A mixed commit may contribute to several proposed commits. Pending changes may join an eligible commit or form a new one.

Check the proposed series in order. When moving hunks, assess both the destination and what remains in each source commit. Each resulting commit should have one clear purpose, a message that describes its diff, and its prerequisites in place at that point in the series.

Establish that the grouping and order are feasible. If the proposal would rewrite history, identify other local refs containing any commit it would replace, replay, or drop. Exclude tool-managed checkpoint refs that the operation leaves untouched. Preserve all other refs and report shared history only when it affects the decision.

Retain the internal execution baseline from **Confirm Commits** and **Prepare Update Details**, including the affected refs, exact selected changes and their placement, exclusions, and preservation assumptions. Defer the exact command sequence and rebase todo edits to the application phase unless a detail is needed to establish feasibility or affects the decision.

## Present the Proposal When Needed

Show the initial or revised proposal when the user requests a preview or needs to approve the series or make another decision. Otherwise, keep it with the coordinator and proceed to [Obtain Proposal Approval](#obtain-proposal-approval).

Briefly identify the base and selected range, then show the proposed series in order. Use the following format in place of the per-commit descriptions in **Confirm Commits**.

- For each new or changed commit, show its complete proposed message in a blockquote. Beside it, list each contributing commit’s short SHA, adding its subject when combining multiple commits. For a partial contribution, name the change taken from that source. Identify any assigned pending changes.
- For a commit kept or replayed with its changes and message intact, use its original short SHA and subject.
- Identify each dropped commit by its original short SHA and explain the omission.

Identify the hunk boundaries when a file is split between commits or only partly included. Mark protected commits and any authorized published rewrite. Briefly explain the reason for a split, grouping, or placement when it would otherwise be unclear.

Disclose necessary temporary isolation and restoration, material exclusions, and any review or validation gaps that affect approval or the claimed result. Omit routine preservation details, command recipes, verification checklists, and empty caveat sections. On revisions, show the updated series and explain only the changed decisions.

## Obtain Proposal Approval

If the caller carries the user’s waiver and authority to apply the prepared series, proceed directly to [Apply Approved Proposal](apply-approved-proposal.md). Otherwise, ask the user to approve applying the proposed series or request revisions, then stop.

Revise the proposal when requested and return to this checkpoint until the user agrees.
