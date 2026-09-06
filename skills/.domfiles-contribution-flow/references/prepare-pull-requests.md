# Prepare Pull Requests

## Enter the Supplied Checkout

The default is that the user has cloned the fork and initiated a worktree. Follow `git-worktrees` for entry checks rather than creating another checkout. Leave its detached or existing branch state alone until a pull request is established as the desired outcome.

Identify the fork, original upstream repository, and intended upstream target branch. Do not assume that the default branch is named `main` or `master`, or that remote names establish repository roles. Reuse a remote that identifies the intended upstream. If none exists, add the upstream remote using its verified public destination. Do not overwrite a conflicting remote configuration or expose embedded credentials while resolving it.

## Select the Branch and Scope

Once the pull-request path is established, select the contribution branch name and create it in the supplied worktree during the first [upstream synchronization](#synchronize-with-upstream). Keep its name extremely concise, lowercase kebab-case, without prepositions or prefixes by default. The name identifies the distinguishing change rather than repeating the full title.

The branch name and pull-request title remain provisional as the changeset develops and may be revised before submission. They freeze once the user publishes the pull request. Renaming the branch does not authorize renaming or moving the Zed-managed checkout. If a pre-submission rename affects an already-published remote branch, establish the needed user-run publication or cleanup separately rather than silently changing remote state.

Keep one pull request focused enough for maintainers to assess on its own. When a larger contribution remains justified, plan a series of coherent, independently reviewable pull requests with explicit dependencies. By default, prepare and submit the first, then prepare the next only after the preceding pull request has merged. Verify the merged upstream state when the user resumes the series. Do not automatically submit stacked or concurrent dependent pull requests, and keep later work provisional rather than treating the initial plan as an obligation to complete the series.

## Synchronize With Upstream

Use this checkpoint before implementation and again immediately before final readiness. Apply both checkpoints to each approved review-fix round, including a fresh upstream check after its history updates.

1. Fetch the intended upstream target through the verified remote.
2. Create the contribution branch at the supplied checkout’s current `HEAD` if needed. If it is not current, explicitly request `commit`’s unpushed-history update route, supplying the contribution branch and refreshed upstream target as the desired base. Let `commit` establish rewrite eligibility, prepare the proposal, obtain execution approval, and perform the synchronization.
3. Use the verified result to confirm that the contribution branch contains the fetched target and the intended contribution still fits. Do not repeat the history-update mechanics or validation owned by `commit`.

Branch updates are confined to the contribution branch. Do not rebase, merge into, reset, or synchronize the fork’s primary branch as a substitute. If `commit` cannot establish eligibility or complete the approved update, report the remaining synchronization requirement rather than claiming readiness or selecting another rewrite method.

## Plan and Implement the Commits

Before implementation, identify the contribution’s central claim and the most direct appropriate validation. A regression test should distinguish the defect from the intended behavior, while a documentation correction may require checking the description against current behavior. Use the repository’s checks and `commit`’s validation checkpoints to establish that claim rather than creating a parallel validation procedure.

Ask `commit` for a read-only, prospective breakdown based on the intended scope. Supply the contribution’s packaging constraints, then retain only a provisional plan until the actual diff exists.

Plan for at least two meaningful commits when the changes can be reliably separated. The usual minimum is the implementation followed by `Update documentation`. Do not invent documentation, empty commits, or artificial boundaries to reach a count, and keep documentation with the implementation when it is necessary for that commit’s correctness.

If a reliable split is not possible, use one commit whose complete message is the exact pull-request title. Have `human-facing-writing` shape that title, and supply it to `commit` as required wording rather than requesting a second subject. Before submission, explicitly request `commit`’s unpushed-history update route for any later message change so the single commit and title agree. If its publication cutoff prevents the update, stop for the user’s decision rather than claiming they agree.

Implement only the approved contribution scope. Run the repository’s applicable checks and inspect the actual changes before `commit` prepares its concrete proposal. A prospective plan, approved code edit, or accepted finding does not itself authorize creating or rewriting commits.

Keep the authored commits coherent on their own. For approved post-review fixes, ask `commit` to map the pending changes to existing commits or independently useful new commits. Explicitly request its unpushed-history update route when existing commits need changes, allowing separate new commits in the same plan. When every change belongs in a new commit, use its ordinary new-commit route. Mode selection does not authorize execution. Then repeat [Synchronize With Upstream](#synchronize-with-upstream) and the final readiness check.

## Check Submission Readiness

Recheck the relevant prior work and current upstream evidence from [contribution assessment](../SKILL.md#assess-the-contribution). Determine whether intervening work has resolved or narrowed the problem or invalidated the motivation. Return to assessment when that evidence changes the selected outcome or scope rather than claiming the original contribution is still ready.

Confirm that the final changeset still fits the selected contribution, its commits remain coherent, the applicable validation has run, and the contribution branch includes the latest fetched upstream target. Recheck the title and branch name against the final outcome, including the single-commit title rule when applicable.

Use the completed [post-content preparation](prepare-post-content.md) for the body, and reassess its [reference relationships](prepare-post-content.md#select-references) against the final outcome. Apply any approved findings, but do not require an external or adversarial review to call the contribution ready. An amendment or rebase invalidates the earlier final readiness check, so reassess the affected result before returning to the entrypoint’s [handoff](../SKILL.md#hand-back-the-contribution).
