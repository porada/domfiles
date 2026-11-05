# Prepush

Prepare the current branch’s local work for publication through a user-approved commit plan. This is an explicitly invoked agent workflow, not a Git hook. Invocation authorizes inspection and a proposal, including scoped fetches needed to refresh remote-tracking refs for the destination and base. It does not authorize staging, committing, rewriting history, or pushing.

Approval must cover the concrete plan, including new commits, amendments, broader replay, any permitted local rewrite of published commits, and necessary state isolation. An invocation, a request to revise the recommendation, or general commit authority is not approval of the resulting proposal.

Load `sensible-commit-flow` and `dom-sensible-commit-flow` for the commit lifecycle.

## Resolve the Work

Default to all unpushed commits reachable from the current branch’s `HEAD` and eligible staged, unstaged, and untracked changes in the current repository. Apply explicit user scope limits. Exclude secret-bearing and otherwise prohibited material before reading contents, and do not force-add ignored files. Existing staging does not itself select or authorize a change.

Establish the base, complete affected range, publication evidence, and protected commits through `sensible-commit-flow`’s history route and the global **Rewrite eligibility** and **History update authorization** policies. An absent upstream or stale tracking refs do not establish an unpushed range. Keep the base unchanged unless the user separately requests synchronization. Identify selected commits also reachable from other local refs, expose the implications in the proposal, and leave those other refs untouched.

This command packages the existing work. Do not fix source defects or alter final contents to simplify the history. Apply the global **Checkpoint quality** policy without claiming publication readiness or bypassing required gates. If there is no eligible change to record or useful rewrite to recommend, report that rather than manufacturing a commit or cosmetic rewrite.
