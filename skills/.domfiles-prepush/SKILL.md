---
name: prepush
description: |-
    Prepare commits for publication.

disable-model-invocation: true
metadata:
    internal: true
---

# Prepush

Prepare the current branch’s local work for publication through a user-approved commit plan. This is an explicitly invoked agent workflow, not a Git hook. Invocation authorizes inspection and a proposal, including scoped fetches needed to refresh remote-tracking refs for the destination and base. It does not authorize staging, committing, rewriting history, or pushing.

Load `sensible-commit-flow` and `dom-sensible-commit-flow` for the commit lifecycle. Use the base’s **Inspect Changes**, **Group Hunks**, **Compose Messages**, and **Update Commit History** routes, returning to its execution and verification procedures after the approval below. Keep this workflow’s proposal checkpoint even when a standing grant would otherwise permit committing. Do not invoke **Automatic commits** during proposal preparation or add a second commit offer alongside the proposal.

## Resolve the Work

Default to all unpushed commits reachable from the current branch’s `HEAD` and eligible staged, unstaged, and untracked changes in the current repository. Apply explicit user scope limits. Exclude secret-bearing and otherwise prohibited material before reading contents, and do not force-add ignored files. Existing staging does not itself select or authorize a change.

Establish the base, complete affected range, publication evidence, and protected commits through the base’s history route and the global **Rewrite eligibility** and **History update authorization** policies. An absent upstream or stale tracking refs do not establish an unpushed range. Keep the base unchanged unless the user separately requests synchronization. Identify selected commits also reachable from other local refs, expose the implications in the proposal, and leave those other refs untouched.

## Recommend the Series

Pending changes may become new commits or join eligible rewritten commits through the base’s grouping and history routes. This explicit planning route can target older eligible commits without extending ordinary **HEAD amendments** authority.

Present the base and selected range, then the final series in order. For each proposed commit, show its complete message and purpose, the existing commits or hunks it incorporates, and the pending changes assigned to it. Identify new commits, unchanged or replay-only protected commits, grouping and ordering changes, and any local rewrite of published commits covered by a specific exception.

Include the base’s **Prepare Update Details** in the proposal, alongside exclusions, shared-history implications, review coverage, later review-driven corrections, and known validation or quality limitations.

This command packages the existing work. Do not fix source defects or alter final contents to simplify the history. Apply the global **Checkpoint quality** policy without claiming publication readiness or bypassing required gates. If there is no eligible change to record or useful rewrite to recommend, report that rather than manufacturing a commit or cosmetic rewrite.

## Obtain Proposal Approval

Ask the user to approve applying the proposed series or request revisions, then stop. Approval must cover the concrete plan, including new commits, amendments, broader replay, any permitted local rewrite of published commits, and necessary state isolation. An invocation, a request to revise the recommendation, or general commit authority is not approval of the resulting proposal.

Revise the proposal when requested and return to this checkpoint until the user agrees. After approval, recheck its refs, selected changes, exclusions, and preservation assumptions against the execution baseline. If they materially change, revise the affected proposal and obtain approval rather than applying a stale plan. Do not silently change approved messages or grouping during execution.

## Apply and Verify

Pass the approved series and its authority to the base’s **Commit Execution** and **Update Commit History** routes. Complete their execution, verification, and reporting procedures, including **Verify Rewritten History**, rather than implementing a separate lifecycle here.

## Offer Publication

At the end, offer to push the verified result if publication remains appropriate under applicable gates. Approval to apply the commit plan does not authorize pushing. Resolve the destination and any required publication checks before requesting permission to execute that push. If the user approves, use the base workflow’s publication safeguards, including its explicit expected-head lease for a published replacement. Otherwise leave publication to the user. Explain a remaining publication gate rather than offering to bypass it.
