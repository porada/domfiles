# Prepush

Prepare the current branch’s local work for publication when the user invokes `prepush` directly or a calling workflow explicitly loads its instructions. Invocation authorizes inspection and a proposal, including scoped fetches needed to refresh remote-tracking refs for the destination and base. This workflow never authorizes pushing.

For every direct invocation, obtain the user’s approval of the concrete proposal before staging, committing, or rewriting history. A calling workflow follows the same checkpoint unless it carries the user’s explicit waiver from a standing grant or a user-approved execution checkpoint. With that waiver, the coordinator prepares the series and may apply it within the waiver’s limits. On either path, the user’s authorization must cover the proposed effects, including new commits, amendments, broader replay, any permitted local rewrite of published commits, and necessary state isolation. A request to revise the proposal or general commit authority supplies neither proposal approval nor a waiver.

Load `sensible-commit-flow` and `dom-sensible-commit-flow` for the commit lifecycle.

## Resolve the Work

Default to all unpushed commits reachable from the current branch’s `HEAD` and eligible staged, unstaged, and untracked changes in the current repository. Apply the user’s scope limits and the scope passed by the calling workflow. Exclude secret-bearing and otherwise prohibited material before reading contents, and do not force-add ignored files. Existing staging does not itself select or authorize a change.

Establish the base, complete affected range, publication evidence, and protected commits through `sensible-commit-flow`’s history route and the global **Rewrite eligibility** and **History update authorization** policies. An absent upstream or stale tracking refs do not establish an unpushed range. Reuse applicable current evidence or fetch only the refs needed to establish that range and rewrite eligibility. Keep the base unchanged unless the user separately requests synchronization.

This command packages the existing work. Do not fix source defects or alter final contents to simplify the history. Apply the global **Checkpoint quality** policy without claiming publication readiness or bypassing required gates. If there is no eligible change to record or useful rewrite to recommend, report that rather than manufacturing a commit or cosmetic rewrite.
