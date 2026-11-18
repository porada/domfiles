# Prepare Commit Proposal

Use `sensible-commit-flow`’s **Inspect Changes**, **Group Hunks**, **Compose Messages**, and **Update Commit History** routes, returning to its execution and verification procedures after the approval below. Keep this workflow’s proposal checkpoint even when a standing grant would otherwise permit committing. Do not invoke **Automatic commits** during proposal preparation or add a second commit offer alongside the proposal.

## Recommend the Series

Review the eligible unpushed commits as a series, even when the working tree is clean. When several commits are steps toward one logical change, propose combining them even if other commits fall between them. Keep independently useful changes separate and order the resulting commits so prerequisites come first. Pending changes may join an eligible commit or form a new one.

Establish that the grouping and order are feasible. If the proposal would rewrite history, identify other local refs containing any commit it would replace, replay, or drop. Exclude tool-managed checkpoint refs that the operation leaves untouched. Preserve all other refs and report shared history only when it affects the decision.

Briefly identify the base and selected range, then show the proposed series in order. Use the following format in place of the per-commit descriptions in **Confirm Commits**.

- For each new or changed commit, show its complete proposed message in a blockquote. Beside it, list each contributing commit’s short SHA, adding its subject when combining multiple commits. Identify any assigned pending changes.
- For a commit kept or replayed with its changes and message intact, use its original short SHA and subject.
- Identify each dropped commit by its original short SHA and explain the omission.

Mark protected commits and any authorized published rewrite. Briefly explain a split, grouping, placement, or hunk boundary when the reason or assignment would otherwise be unclear.

Before asking for approval, retain the internal execution baseline from **Confirm Commits** and **Prepare Update Details**, including the affected refs, exact selected changes and their placement, exclusions, and preservation assumptions. Defer the exact command sequence and rebase todo edits until approval unless a detail is needed to establish feasibility or affects the decision.

Disclose necessary temporary isolation and restoration, material exclusions, and any review or validation gaps that affect approval or the claimed result. Omit routine preservation details, command recipes, verification checklists, and empty caveat sections. On revisions, show the updated series and explain only the changed decisions.

## Obtain Proposal Approval

Ask the user to approve applying the proposed series or request revisions, then stop.

Revise the proposal when requested and return to this checkpoint until the user agrees.
