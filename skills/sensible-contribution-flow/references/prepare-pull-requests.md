# Prepare Pull Requests

Use these checkpoints in the entrypoint’s [selected workflow](../SKILL.md#workflow).

## Enter Supplied Checkout

Begin active pull request preparation with only the read-only preflight needed for the first scoped upstream fetch, not a complete setup plan. Read the supplied checkout’s applicable instructions, confirm that active tools target the intended repository and checkout, and record `HEAD` and its branch or detached state. Assessment-only work remains on the entrypoint’s read-only route. Apply the shared [execution boundaries](execution-boundaries.md) to tools and mutations throughout this path.

Verify the intended upstream repository and target branch from task context and non-secret metadata. Do not assume that the default branch is named `main` or `master`, or that remote names establish repository roles. Reuse a remote that identifies the intended upstream. If none exists, include adding an upstream remote at its verified public destination as a fetch prerequisite. Do not overwrite a conflicting remote configuration or expose embedded credentials while resolving it.

Resolve authority for the fetch and any necessary remote setup through the entrypoint’s [authority boundary](../SKILL.md#resolve-scope-and-authority). When applicable instructions expressly delegate early setup, verify their conditions and retain the user’s exact instruction with the verified target, covered effects, and fetch boundaries. Otherwise present that target and those operations and obtain only the authorization still needed. Do not require a contribution branch choice or rewrite plan for this checkpoint.

Invoke the real scoped fetch as soon as this preflight is complete. Use the client’s declared permission model to identify required network and Git metadata write access, and request the narrowest supported grant when invoking the fetch or its necessary preceding remote creation. Do not attempt a write already known to be forbidden in the current sandbox or elevate a read-only probe merely to reserve access. Do not delay this checkpoint for branch selection, continuing implementation approval, fork or publication destination work, history inspection, or rewrite planning unless that work is necessary to make the fetch safe and authorized. Contribution research, proposal drafting, review, and validation tool inspection remain later.

When existing permissions cover the operation, proceed without another prompt. If required access is declined or unavailable, stop setup under the shared execution boundaries. An early permission grant does not authorize later phases or extend the client’s grant lifetime.

Record the fetched target tip, then identify the fork and plan branch setup under its applicable authority. Select or retain the [contribution branch](#select-branch-and-scope), recording its identity and authorized branch setup or replay effects before creating it or changing history. [Synchronize](#synchronize-with-upstream) using that fetched tip, supplying the completed authorization record to the resolved [commit workflow](../SKILL.md#compose-with-peers) for necessary history updates and retaining any limits on base-only replay.

Inspecting current upstream files remotely does not synchronize the contribution branch. Setup does not settle whether a contribution is justified.

## Select Branch and Scope

Create the contribution branch in the supplied worktree during the first synchronization, or retain an existing branch established as belonging to this contribution. Keep its name extremely concise, lowercase kebab-case, without prepositions or prefixes by default. The name identifies the distinguishing change rather than repeating the full title.

The branch name and pull request title remain provisional as the changeset develops and may be revised before submission. They freeze once the user publishes the pull request. Renaming the branch does not authorize renaming or moving the user/application-managed checkout. If a pre-submission rename affects an already published remote branch, establish the needed user-run publication or cleanup separately rather than silently changing remote state.

Keep one pull request focused enough for maintainers to assess on its own. When a larger contribution remains justified, plan a series of coherent, independently reviewable pull requests with explicit dependencies. By default, prepare and submit the first, then prepare the next only after the preceding pull request has merged. Verify the merged upstream state when the user resumes the series. Do not automatically submit stacked or concurrent dependent pull requests, and keep later work provisional rather than treating the initial plan as an obligation to complete the series.

## Synchronize With Upstream

Use this checkpoint during initial setup, before implementation if the proposal discussion has outlasted the verified baseline, and immediately before final readiness. Apply the pre-implementation and final checkpoints to each approved review fix round, including a fresh upstream check after its history updates.

1. Use the just-completed setup fetch for the first synchronization of each preparation or revision round rather than fetching again. At later checkpoints, or when a material delay or new evidence makes that baseline stale, fetch the intended upstream target through the verified remote under the applicable setup or execution authorization.
2. Create the contribution branch at the supplied checkout’s current `HEAD` if needed. If it is not current, ask the resolved commit workflow to synchronize that branch onto the refreshed upstream target. Supply the desired base and actual authorization record. That workflow establishes rewrite eligibility, records the concrete proposal, obtains approval only for uncovered effects, and performs the authorized update.
3. Record the fetched target and verified result, then confirm that the contribution branch contains that target and the intended contribution still fits. Revalidate any affected integration and review the resulting delta when synchronization changes the reviewed result. Do not repeat the history mechanics or validation owned by the resolved commit workflow.

Branch updates are confined to the contribution branch. Do not rebase, merge into, reset, or synchronize the fork’s primary branch as a substitute. If the resolved commit workflow cannot establish eligibility or complete the approved update, report the remaining synchronization requirement rather than claiming readiness or selecting another rewrite method.

## Plan and Implement Commits

### Agree on the Proposal

Identify the contribution’s central claim and the most direct appropriate validation. A regression test should distinguish the defect from the intended behavior, while a documentation correction may require checking the description against current behavior. Use the repository’s checks and the resolved commit workflow’s validation checkpoints to establish that claim rather than creating a parallel validation procedure.

Inspect the repository’s validation commands, tool versions, and acquisition path before the proposal. Establish whether validation uses available tooling, may acquire already prescribed dependencies or tools, or requires a new dependency choice. A `PATH` check alone does not settle package-runner availability or download needs. Do not download tools merely to investigate availability. Include known uncovered approvals in the proposal, preserving the [dependency and execution boundaries](execution-boundaries.md).

Ask the resolved commit workflow for a read-only, prospective breakdown based on the intended scope. Supply the contribution’s packaging constraints, then retain only a provisional plan until the actual diff exists.

Plan for at least two meaningful commits when the changes can be reliably separated. The usual minimum is the implementation followed by `Update documentation`. Do not invent documentation, empty commits, or artificial boundaries to reach a count, and keep documentation with the implementation when it is necessary for that commit’s correctness.

If a reliable split is not possible, use one commit whose complete message is the exact pull request title. Use the resolved writing workflow to shape that title, and supply it to the commit workflow as required wording rather than requesting a second subject.

Prepare an early title and body through [post content preparation](prepare-post-content.md). Use the draft to expose the intended outcome and scope, not to claim implementation or testing has already happened. Keep the wording provisional while discussing the solution.

When presenting that initial draft, add a brief source note outside the title and body. Identify the repository template followed by its repository-relative path, and cite any existing PR used as a structural model, distinguishing examples from repository requirements. If no template was found, say so and identify the fallback used. If template availability could not be verified, report that limitation instead of claiming absence or compliance. Attribute only sources actually inspected.

Before asking for implementation approval, run a quick adversarial design check using the [review method](#review-the-contribution). Challenge the problem’s current existence, the proposed solution, the best simpler alternative, and upstream fit. Resolve its conclusions with the user as part of the same scope discussion, rather than adding a separate approval ceremony when the check confirms the proposal.

Present the verified repository, checkout, contribution branch, upstream target, draft, intended behavior, exclusions, planned validation, and provisional commit breakdown together. When applicable governing instructions expressly delegate a continuing preparation mode, use their bounded checkpoint. Name only the effects that authority permits, including any implementation, validation, independent review, later validated corrections, local commits, provisional revisions, or unpublished-history synchronization it covers. Make its revision boundaries and lifetime explicit. Ask once for any execution grant still needed, wait for the user’s response, and retain its record before execution. When existing authorization already covers the concrete phase, present the proposal as a notice rather than requesting duplicate approval.

Unless the user requests a narrower deliverable, do not substitute an edit-only proposal for an applicable continuing preparation checkpoint. A narrower approval already given remains limited to its recorded effects.

Without that governing delegation, ask for implementation approval rather than offering a new broad continuing grant. Let the resolved commit workflow establish execution authorization for commits and history updates, including any applicable one-off request. Approval of a draft or plan alone never substitutes for execution authorization.

### Implement and Review

Implement only the approved contribution scope. Respect the execution boundary’s dependency and protected-content gates before dependent work. Run the repository’s applicable checks, inspect the actual changes, and complete the [implementation review](#review-the-contribution) before preparing commits. Correct validated in-scope findings within the existing mutation authority. A material change to the agreed outcome or approach returns to the proposal discussion, and separate approval gates remain applicable.

### Prepare Commits

After review, give the resolved commit workflow the actual changes, packaging constraints, and applicable authorization record. It still records and verifies each concrete execution plan. Continue through covered effects under an explicit scoped commit request, a valid continuing grant, or a one-off history-update request without duplicate confirmation. For uncovered effects, present only the approval still required, rather than ending with an uncommitted-work summary and waiting for “Proceed.” An implementation plan or accepted finding alone does not authorize creating or rewriting commits.

Keep the authored commits coherent on their own. For approved post-review fixes, ask the resolved commit workflow to map pending changes to existing commits or independently useful new commits. State explicitly when existing history needs an update, allowing separate new commits in the same plan. When every change belongs in a new commit, request only those new commits. Selecting an operation does not authorize its execution.

Continue from the commit result to [final editorial work](#finalize-the-pull-request). Do not mistake completion of the commit operation for completion of contribution preparation.

## Review the Contribution

Use a separate in-client agent for each review checkpoint when available, with a read-only assignment and bounded output. If no independent reviewer is available, perform a fresh adversarial self-review and disclose the lack of independent review. Do not silently omit the check or require an external handoff, new tool, or remote processing service as a substitute.

The design check examines the concrete proposal before implementation approval. The initial implementation review examines the complete contribution against its recorded baseline after validation, even for an apparently easy or mechanical change. Give the reviewer the agreed outcome, exclusions, decisive upstream context, and relevant evidence. Bound its sources, access, stopping conditions, and required output. The reviewer inherits the task’s authority and security limits, cannot supply user approval, and must return boundary questions instead of expanding its assignment. For implementation review, also provide the complete changeset and validation results. Look for claims unsupported by the implementation or tests, correctness problems, regressions, unnecessary changes, and upstream-fit problems. Passing tests are evidence, not a replacement for assessing whether every change belongs.

Keep one review record with the baseline first, followed by scope and exclusions, settled decisions, and current finding classifications. Validate findings against current evidence and those decisions before acting. Consolidate required corrections into one authorized batch, then review only the resulting delta and integration boundaries. If a second fix round exposes another issue in the same construct, stop extending it. Narrow, replace, or remove it within valid authority, or ask the user to choose among materially different options. Do not reopen settled classifications without changed relevant content or new evidence. Keep optional preferences and adjacent cleanup out of the required batch. User-supplied external findings return through the entrypoint’s [findings workflow](../SKILL.md#incorporate-feedback-and-findings).

## Finalize the Pull Request

Update the draft to reflect the reviewed, committed outcome and resolve the user’s editorial feedback through [content finalization](prepare-post-content.md#finalize-content). Reassess the provisional branch name against the actual change and record any authorized rename against the same task-owned branch. The first committed result is an intermediate checkpoint, not the final handoff.

For a single-commit contribution, a later title change also requires asking the resolved commit workflow to update the complete commit message. If its authorization or eligibility rules prevent the update, stop for the user’s decision rather than claiming the title and commit agree. Preserve exact wording the user has settled unless they authorize changing it.

After final code, history, and editorial changes, repeat [upstream synchronization](#synchronize-with-upstream) and the readiness check. A new integration change may require targeted validation or delta review, but not an automatic restart of the entire review.

## Check Submission Readiness

Recheck the relevant prior work and current upstream evidence from [contribution assessment](../SKILL.md#assess-the-contribution). Determine whether intervening work has resolved or narrowed the problem or invalidated the motivation. Return to assessment when that evidence changes the selected outcome or scope rather than claiming the original contribution is still ready.

Confirm that the final changeset still fits the selected contribution, its commits remain coherent, the applicable validation and adversarial reviews are complete, required findings are resolved, and the contribution branch includes the latest fetched upstream target. Required human checkpoints are also part of readiness, even after agent review and commits are complete. Recheck the title and branch name against the final outcome, including the single-commit title rule when applicable.

Reassess the body’s [reference relationships](prepare-post-content.md#select-references) against the final outcome. Report material evidence or validation limitations without requiring an external review. An amendment or rebase invalidates the earlier final readiness check, so reassess the affected result. When a published branch requires authorized replacement, have the resolved commit workflow recheck the publication destination immediately before handoff, reconcile changed remote work within authorization, and repeat affected validation. New scope requires a user decision, not merely a newer lease expectation. Then return to the entrypoint’s complete [handoff](../SKILL.md#hand-back-the-contribution).

## Revise Existing Pull Requests

Resume from the existing PR and contribution branch rather than restarting the initial proposal. Record the pre-revision state as the review baseline, resolve current [scope and authority](../SKILL.md#resolve-scope-and-authority), and apply the [checkout and setup access checkpoint](#enter-supplied-checkout) under that authority. Preserve the published branch name and title unless the current request covers changing them. A prior preparation grant that ended at handoff or publication stays expired.

Validate maintainer feedback and other supplied findings through [Incorporate Feedback and Findings](../SKILL.md#incorporate-feedback-and-findings). That workflow establishes applicable fix confirmation and returns to this revision checkpoint rather than treating maintainer comments as authorization.

After authorization, use [Implement and Review](#implement-and-review) for the selected revisions. The mandatory implementation review covers the complete revision delta and affected integration boundaries against the recorded baseline, not a fresh review of the entire PR. Supply the selected feedback and settled decisions to the reviewer, and retain the shared [review method](#review-the-contribution), including its independent-review requirement and disclosed self-review fallback. Complete validation and this review before committing or handing back working tree changes, even when no commit was requested.

Apply both [upstream synchronization checkpoints](#synchronize-with-upstream) within current authorization. Route any requested commit or history update through [Prepare Commits](#prepare-commits). For a working-tree-only request, report any synchronization requiring uncovered history changes rather than treating the request as authority to perform them. Complete the applicable [readiness check](#check-submission-readiness) and [handoff](../SKILL.md#hand-back-the-contribution) for the revised result. Do not reopen settled decisions without changed relevant content or materially new evidence, or turn the round into ongoing monitoring.
