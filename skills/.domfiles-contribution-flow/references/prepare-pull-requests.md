# Prepare Pull Requests

Preparation progresses from checkout setup through proposal agreement, implementation and review, commits, and final editorial readiness. Upstream synchronization recurs at the checkpoints below.

## Enter the Supplied Checkout

The default is that the user has cloned the fork and initiated a worktree. Follow `git-worktrees` for entry checks rather than creating another checkout. Distinguish an active preparation request from read-only assessment before changing Git state.

Identify the fork, original upstream repository, and intended upstream target branch. Do not assume that the default branch is named `main` or `master`, or that remote names establish repository roles. Reuse a remote that identifies the intended upstream. If none exists, add the upstream remote using its verified public destination once that setup is authorized. Do not overwrite a conflicting remote configuration or expose embedded credentials while resolving it.

Resolve setup authorization through the entrypoint’s governing policy. When **Contribution Preparation Authorization** applies, retain its record of the user’s explicit preparation instruction and supply it to `commit-flow` for any necessary base-only replay. Otherwise establish the required setup authorization with the verified targets and operations before proceeding. Once authorized, select the [provisional branch](#select-the-branch-and-scope) and [synchronize](#synchronize-with-upstream) before researching the implementation or discussing the solution, not after the code is written.

## Select the Branch and Scope

Create the contribution branch in the supplied worktree during the first synchronization, or retain an existing branch established as belonging to this contribution. Keep its name extremely concise, lowercase kebab-case, without prepositions or prefixes by default. The name identifies the distinguishing change rather than repeating the full title.

The branch name and pull request title remain provisional as the changeset develops and may be revised before submission. They freeze once the user publishes the pull request. Renaming the branch does not authorize renaming or moving the Zed-managed checkout. If a pre-submission rename affects an already published remote branch, establish the needed user-run publication or cleanup separately rather than silently changing remote state.

Keep one pull request focused enough for maintainers to assess on its own. When a larger contribution remains justified, plan a series of coherent, independently reviewable pull requests with explicit dependencies. By default, prepare and submit the first, then prepare the next only after the preceding pull request has merged. Verify the merged upstream state when the user resumes the series. Do not automatically submit stacked or concurrent dependent pull requests, and keep later work provisional rather than treating the initial plan as an obligation to complete the series.

## Synchronize With Upstream

Use this checkpoint during initial setup, before implementation if the proposal discussion has outlasted the verified baseline, and immediately before final readiness. Apply the pre-implementation and final checkpoints to each approved review fix round, including a fresh upstream check after its history updates.

1. Fetch the intended upstream target through the verified remote under the applicable setup or execution authorization.
2. Create the contribution branch at the supplied checkout’s current `HEAD` if needed. If it is not current, explicitly request `commit-flow`’s unpushed history update route, supplying the contribution branch and refreshed upstream target as the desired base. Let `commit-flow` establish rewrite eligibility, prepare the proposal, verify coverage under the recorded authorization or obtain the required approval, and perform the synchronization.
3. Use the verified result to confirm that the contribution branch contains the fetched target and the intended contribution still fits. Revalidate any affected integration and review the resulting delta when synchronization changes the reviewed result. Do not repeat the history update mechanics or validation owned by `commit-flow`.

Branch updates are confined to the contribution branch. Do not rebase, merge into, reset, or synchronize the fork’s primary branch as a substitute. If `commit-flow` cannot establish eligibility or complete the approved update, report the remaining synchronization requirement rather than claiming readiness or selecting another rewrite method.

## Plan and Implement the Commits

### Agree on the Proposal

Identify the contribution’s central claim and the most direct appropriate validation. A regression test should distinguish the defect from the intended behavior, while a documentation correction may require checking the description against current behavior. Use the repository’s checks and `commit-flow`’s validation checkpoints to establish that claim rather than creating a parallel validation procedure.

Ask `commit-flow` for a read-only, prospective breakdown based on the intended scope. Supply the contribution’s packaging constraints, then retain only a provisional plan until the actual diff exists.

Plan for at least two meaningful commits when the changes can be reliably separated. The usual minimum is the implementation followed by `Update documentation`. Do not invent documentation, empty commits, or artificial boundaries to reach a count, and keep documentation with the implementation when it is necessary for that commit’s correctness.

If a reliable split is not possible, use one commit whose complete message is the exact pull request title. Have `human-facing-writing` shape that title, and supply it to `commit-flow` as required wording rather than requesting a second subject.

Prepare an early title and body through [post content preparation](prepare-post-content.md). Use the draft to expose the intended outcome and scope, not to claim implementation or testing has already happened. Keep the wording provisional while discussing the solution.

Before asking for implementation approval, run a quick adversarial design check using the [review method](#review-the-contribution). Challenge the problem’s current existence, the proposed solution, the best simpler alternative, and upstream fit. Resolve its conclusions with the user as part of the same scope discussion, rather than adding a separate approval ceremony when the check confirms the proposal.

Present the verified repository, checkout, contribution branch, upstream target, draft, intended behavior, exclusions, planned validation, and provisional commit breakdown together. When **Contribution Preparation Authorization** applies, ask once for its bounded execution grant: implementation, validation, independent in-client review, in-scope corrections including later validated findings, local commits, provisional branch and message revisions, and synchronization of task-owned unpublished history. Make the permitted revisions and end of the preparation cycle explicit, then wait for the user’s response and retain the approval record before execution.

Without that governing delegation, ask for implementation approval and retain `commit-flow`’s concrete-batch confirmation for commits and history updates. Approval of a draft or plan alone never substitutes for an execution grant.

### Implement and Review

Implement only the approved contribution scope. Run the repository’s applicable checks, inspect the actual changes, and complete the [implementation review](#review-the-contribution) before preparing commits. Correct validated in-scope findings within the existing mutation authority. A material change to the agreed outcome or approach returns to the proposal discussion, and separate approval gates remain applicable.

### Prepare the Commits

After review, give `commit-flow` the actual changes, packaging constraints, and applicable authorization record. It still prepares and verifies each concrete execution plan. Within a valid contribution grant, continue through covered commits and provisional revisions without another routine confirmation. Otherwise present its required approval request in the same continuation rather than ending with an uncommitted-work summary and waiting for “Proceed.” An implementation plan or accepted finding alone does not authorize creating or rewriting commits.

Keep the authored commits coherent on their own. For approved post-review fixes, ask `commit-flow` to map the pending changes to existing commits or independently useful new commits. Explicitly request its unpushed history update route when existing commits need changes, allowing separate new commits in the same plan. When every change belongs in a new commit, use its ordinary new commit route. Mode selection does not authorize execution.

Continue from the commit result to [final editorial work](#finalize-the-pull-request). Do not mistake completion of the commit operation for completion of contribution preparation.

## Review the Contribution

Use a separate in-client agent for each review checkpoint when available, with a read-only assignment and bounded output. If no independent reviewer is available, perform a fresh adversarial self-review and disclose the lack of independent review. Do not silently omit the check or require an external handoff, new tool, or remote processing service as a substitute.

The design check examines the concrete proposal before implementation approval. The implementation review examines the complete contribution against its recorded baseline after validation, even for an apparently easy or mechanical change. Give the reviewer the agreed outcome, exclusions, decisive upstream context, and relevant evidence. For implementation review, also provide the complete changeset and validation results. Look for claims unsupported by the implementation or tests, correctness problems, regressions, unnecessary changes, and upstream-fit problems. Passing tests are evidence, not a replacement for assessing whether every change belongs.

Validate findings against current evidence and settled decisions before acting. Consolidate required corrections into one fix batch, then review only the resulting delta and its integration boundaries. Follow the global **Review convergence** policy rather than reopening the complete review after every correction. Keep optional preferences and adjacent cleanup out of the required fix batch. User-supplied external findings return through the entrypoint’s [findings workflow](../SKILL.md#incorporate-findings-before-submission).

## Finalize the Pull Request

Update the draft to reflect the reviewed, committed outcome and resolve the user’s editorial feedback through [content finalization](prepare-post-content.md#finalize-the-content). Reassess the provisional branch name against the actual change. The first committed result is an intermediate checkpoint, not the final handoff.

For a single-commit contribution, a later title change also requires updating the complete commit message through `commit-flow`’s unpushed history update route. If its publication cutoff prevents the update, stop for the user’s decision rather than claiming the title and commit agree. Preserve exact wording the user has settled unless they authorize changing it.

After final code, history, and editorial changes, repeat [upstream synchronization](#synchronize-with-upstream) and the readiness check. A new integration change may require targeted validation or delta review, but not an automatic restart of the entire review.

## Check Submission Readiness

Recheck the relevant prior work and current upstream evidence from [contribution assessment](../SKILL.md#assess-the-contribution). Determine whether intervening work has resolved or narrowed the problem or invalidated the motivation. Return to assessment when that evidence changes the selected outcome or scope rather than claiming the original contribution is still ready.

Confirm that the final changeset still fits the selected contribution, its commits remain coherent, the applicable validation and adversarial reviews are complete, required findings are resolved, and the contribution branch includes the latest fetched upstream target. Recheck the title and branch name against the final outcome, including the single-commit title rule when applicable.

Reassess the body’s [reference relationships](prepare-post-content.md#select-references) against the final outcome. Report material evidence or validation limitations without requiring an external review. An amendment or rebase invalidates the earlier final readiness check, so reassess the affected result before returning to the entrypoint’s complete [handoff](../SKILL.md#hand-back-the-contribution).
