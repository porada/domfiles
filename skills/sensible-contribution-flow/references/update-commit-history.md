# Update Commit History

This route adds history-specific steps for approved corrections folded into existing commits, contribution branch synchronization, and requested message revisions during initial preparation or [revisions to existing pull requests](revise-existing-pull-requests.md). It does not authorize other history maintenance or publication. Use the common [commit lifecycle](prepare-commits.md) for confirmation, execution safeguards, recovery, and reporting rather than starting a second lifecycle here.

## Define Rewrite Scope

Apply [Inspect Changes](prepare-commits.md#inspect-changes), adding the complete existing commit series to the evidence scope. Base-only and message-only updates can be eligible without pending working tree changes.

Resolve the contribution branch, starting `HEAD`, old boundary, and intended new base. The old boundary is the excluded starting commit of the original range. For a range that includes the root commit, record root handling instead of an excluded commit. Identify every commit the operation would drop, replace, or replay, including unchanged descendants after the earliest affected commit. Inspect each commit’s authorship, complete message, parents, and patch rather than only the aggregate diff. For content or message repairs without synchronization, keep the base unchanged. For synchronization, use the verified upstream target supplied by the pull request workflow. Account for merge topology and root commits before selecting the native operation. Don’t silently flatten history, update another branch, or widen the range.

## Establish Update Authorization

An explicit user request or active scoped approval for a history-replacing update supplies execution authorization for its necessary local rewrites, including affected published commits. A request to rebase a known published contribution branch authorizes its necessary local replay without separate force-push wording or another approval of those same effects. A continuing grant can also cover the requested amendment or rebase. Derive authority from actual user instructions or applicable governing instructions, not technical capability, an existing pull request, review text, or route selection.

Where the contribution workflow requires a single commit’s complete message to match the pull request title, a user-requested title change also requests the corresponding message-only local amendment, including for an already published commit, unless the user limits the request to prose, remote metadata, or working tree changes.

Add the affected range, intended base, and covered changes to the shared [authorization record](prepare-commits.md#apply-recorded-authorization). A rebase-only request covers necessary replay and conflict resolution that preserves the contribution’s intended behavior, not branch renaming, opportunistic message changes, or unrelated fixes.

## Establish Rewrite Eligibility

A preparation grant limited to task-owned unpublished history can’t cover published or uncertain rewrites. Without authorization to replace published history, establish that the complete rewritten range is unpushed, not merely the fixup targets. Check current evidence for every relevant publication destination, including forks. An unset tracking branch, stale evidence, or absence from one remote doesn’t establish eligibility. Treat a commit known to have been pushed as published even if its remote ref was later deleted. Stop the affected rewrite when any commit was published or its status can’t be established without authorization covering published replacement.

With authorization to replace the selected branch’s published history, verify the publication destination, full remote branch ref, and current remote head. Establish that the complete affected range belongs to that authorized update and that replacement complies with repository rules. Don’t require proof that the commits never appeared on another remote. Authorization for this branch doesn’t cover other branches, destinations, or tags.

Distinguish the publication destination from the upstream base. Before the update, compare local starting history and contents with the destination’s recorded head. Account for remote-only commits and other differences. Incorporate work only within the authorized scope, or stop for a scope decision. Recording a remote head doesn’t authorize dropping its contents. A lease checks the current head, not whether the prepared result preserves the work already there.

Resolve destinations through task context and non-secret metadata. Refresh only remote information needed for scope, eligibility, and the intended base under the shared [execution boundaries](execution-boundaries.md). A current request or continuing grant that covers those scoped reads and local ref refreshes is sufficient. No further workflow approval is needed. Obtain any separate tool grant. Unavailable evidence stops the affected rewrite, not useful independent local planning.

## Prepare Update Details

Apply [Group Hunks](prepare-commits.md#group-hunks) to map pending changes to their original commits or independently useful new commits. Keep distinct decisions separate instead of folding them into convenient earlier commits. Don’t manufacture content changes for base-only or message-only updates.

Preserve complete inherited messages unless revision is requested. Apply [authored message defaults](prepare-commits.md#compose-authored-messages) only to ordinary new commits or newly requested wording, not inherited or temporary fixup messages. Retain the shared [message constraints](prepare-commits.md#preserve-message-constraints), including exact supplied replacements and human attribution. For [single-commit title alignment](prepare-pull-requests.md#align-single-commit-titles), use the adopted title as the exact complete replacement message.

Add these details to the [execution baseline](prepare-commits.md#confirm-commits). Shared confirmation determines whether to present them:

1. Record the old boundary, intended base, complete rewritten range, topology, and publication evidence. For published replacement, include the verified destination and full ref, recorded remote head, and disposition of differences from local history.
2. Map each repair’s hunks to its original commit’s recorded object ID in the intended final series, identifying separate new commits and partial-file boundaries. Retain exact new or replacement messages and inspected sources for complete inherited messages.
3. Select the [native operation](#execute-history-updates) and command sequence. Record any todo list changes, expected empty or already applied commits and their disposition, and resulting parent relationships. Distinguish temporary fixup messages from final messages.

Replay requires a clean index and tracked working tree, with ignored and untracked content protected from overwrite. Any temporary isolation must be explicit, authorized, and exactly restorable. Don’t let implicit autostashing or automatic updates to other refs widen the operation.

If a possible human review marker is implicated, follow [Preserve Human Review Markers](preserve-human-review-markers.md). A marker that prevents clean replay requires the human action, not a temporary removal, stash, or commit.

## Execute History Updates

Within [Create Approved Commits](prepare-commits.md#create-approved-commits), choose the smallest native operation that implements the approved transformation while preserving topology, hooks, signing, authorship, and message constraints:

- **Base-only updates:** Use native rebase to move the original series onto the approved base without temporary fixups or autosquash.
- **Current-tip repairs:** When only the current tip needs replacement and no base or series change requires replay, amend directly. Use `git commit --amend --no-edit` for content-only repairs, or `git commit --amend` with the exact replacement message when wording also changes. For a message-only amendment, use `git commit --amend --only` without pathspecs to exclude staged content. Don’t stage content for that operation, and retain the shared exact unrelated-index preservation and restoration checks even though `--only` excludes it.
- **Repairs requiring replay:** When earlier commits change or repairs accompany a base or series change, create target-linked fixups, then run an interactive autosquash rebase over the approved range onto the approved base. Use `git commit --fixup=amend:<commit-oid>` for content and message changes, `git commit --fixup=<commit-oid>` for content-only repairs, or `git commit --fixup=reword:<commit-oid>` for message-only changes. Supply the exact replacement message for amend and reword fixups. Reword fixups ignore staged content. Resolve the todo list against recorded target IDs before replay so matching subjects can’t attach a fixup to the wrong commit.
- **Separate changes:** Create ordinary commits with their approved complete messages rather than turning them into fixups.

For every rebase, use `--no-autostash` and `--no-update-refs` so preservation and branch updates remain explicit, and preserve the selected topology and root handling. Use literal-safe, non-interactive input for approved message and todo edits.

For a paused operation, establish that its remaining todo list and conflict resolution still match the execution baseline before continuing through the shared lifecycle. Don’t skip a commit merely to clear a conflict or accept an unexpected empty commit as an authorized deletion.

## Verify Rewritten History

At the shared verification checkpoint, compare the original series plus approved changes with the resulting series. For direct amendments, verify unchanged parents and, for message-only changes, an identical tree. For fixup replay, check that every fixup reached its intended target and no temporary fixups remain. Separate new commits must remain separate, with no change duplicated or silently dropped. Verify the selected base, parent relationships, intended order, and unchanged unrelated refs.

Assess every resulting commit’s independent coherence rather than treating a passing final checkout as proof about each commit.

Return to [Report Results](prepare-commits.md#report-results) with the old-to-new commit mapping and any branch advancement that created no commits. Report final commits, not temporary fixups. If execution stopped, identify the remaining operation state rather than claiming completion.

The pull request workflow then reassesses the resulting state through its applicable [synchronization checkpoints](prepare-pull-requests.md#synchronize-with-upstream) and [readiness check](prepare-pull-requests.md#check-submission-readiness), reusing applicable evidence under those checkpoints and completing affected integration validation and delta review. Published replacement also requires the final destination check below.

## Hand Back Published Updates

Immediately before publication or handing back its command, recheck the publication destination independently of the upstream base. If its head changed, inspect and reconcile the new work within current authorization, then repeat affected validation and review. Stop for a user decision when reconciliation requires new scope. Don’t merely substitute a newer object ID into the lease expectation. An earlier post-update check doesn’t replace this final check.

Resolve the exact publication command from verified values rather than placeholders. Execute it only under the entrypoint’s [direct, scoped publication authorization](../SKILL.md#hand-off-contributions). Otherwise, provide it for the user to run. In either case, preserve these safeguards:

1. Verify the effective destination and full remote branch ref. Confirm that configuration can’t expand the command to additional destinations or refs, without exposing credentials or bulk machine-local configuration. If that can’t be established, pause for the user’s target or configuration decision rather than silently changing configuration.
2. Use the reviewed final tip’s full object ID as the explicit refspec source, targeting only that one full ref. Don’t use a moving branch name as the source. Exclude extra refspecs and implicit destinations.
3. Use `--force-with-lease=<remote-ref>:<expected-remote-oid>` with the recorded head whose contents the reviewed result reconciles. Don’t use plain `--force` or an implicit lease. A later head change must cause rejection rather than overwrite unseen work.
4. Suppress automatic tag publication for this invocation, use `--recurse-submodules=check` so submodule validation can’t publish commits, and retain repository-required checks and hooks. Don’t add all-ref, mirror, recursive, or tag publication.

Keep publication instructions outside the final pull request copy. Return the actual publication result or the verified user-run command with the local result to the pull request workflow’s final delivery. Don’t claim publication when only the command was prepared.
