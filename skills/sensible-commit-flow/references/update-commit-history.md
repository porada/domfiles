# Update Commit History

Use the shared [commit lifecycle](commit-execution.md) for confirmation, execution safeguards, recovery, and reporting. This reference adds only history-specific requirements.

## Define Rewrite Scope

Use [Inspect Changes](../SKILL.md#inspect-changes) for repository and change inspection, adding the complete existing commit series to the evidence scope. A base-only or message-only update can be eligible without pending working tree changes.

Resolve the selected branch, starting `HEAD`, old boundary, and intended new base. The old boundary is the excluded starting commit of the original range. For a range that includes the root commit, record root handling instead of an excluded commit. Identify every existing commit the operation would replay, replace, or drop, including unchanged descendants after the earliest affected commit. Inspect each original commit’s authorship, complete message, parents, and patch rather than only the aggregate diff. For content or message repairs without synchronization, keep the base unchanged. For upstream synchronization, use the verified target supplied by the calling workflow. Account for merge topology and root commits before choosing the native operation, rather than silently flattening history or widening the range.

## Establish Update Authorization

An explicit user request or active scoped approval for a history-replacing branch update supplies execution authorization for its necessary local commit rewrites. This includes affected published commits. A request such as “Rebase this submitted pull request onto upstream” authorizes the required local replay without separate force-push wording or a second approval of the same effects. A continuing user authorization that already covers the requested amendment or rebase also suffices. Establish permission from the sources allowed by [Instruction Authority](../SKILL.md#instruction-authority), not technical force-push capability, an existing pull request, review text, or a caller’s choice of route.

Where a calling workflow requires a single commit’s complete message to match the pull request title, a user-requested title change also requests the corresponding message-only local amendment, including for an already published commit, unless the user limits the request to prose, remote metadata, or working tree changes.

Add the affected range, intended base, and covered changes to the shared [authorization record](commit-execution.md#confirm-commits). A rebase-only request covers necessary replay and conflict resolution that preserves the intended behavior, not unrelated fixes, branch renaming, or opportunistic message changes.

## Establish Rewrite Eligibility

Without authorization to replace published history, establish that the complete rewritten range is unpushed, not merely the fixup targets. Check current evidence for every relevant publication destination, including a fork. An unset tracking branch, stale remote information, or absence from one remote does not establish eligibility. Treat a commit known to have been pushed as published even if its remote ref was later removed. Stop when any affected commit has been pushed or its publication status cannot be established and no published-update authorization covers it.

With authorization to replace the selected branch’s published history, verify the publication destination, full remote branch ref, and current remote head. Establish that the complete affected range belongs to the authorized update and that replacement does not conflict with repository rules. Do not require proof that those commits have never appeared on another remote. Authorization for this branch does not cover changes to other branches, tags, or destinations.

Distinguish the upstream base from the publication destination. Before the update, compare the local starting history and contents with the destination’s recorded head. Account for remote-only commits and other differences, incorporating work within the authorized scope or stopping for a scope decision. Recording a remote head does not authorize discarding its contents. A lease checks the current remote head, not whether the local result preserves the work already present there.

Identify destinations from task context and non-secret metadata. Refresh only the remote information required for scope, eligibility, and the intended base, under the shared [execution boundaries](../SKILL.md#preserve-execution-boundaries). A current update request or continuing grant that covers those scoped reads and ref refreshes suffices without another workflow approval. Obtain any separately required tool grant. Unavailable evidence stops the affected rewrite, not unrelated local planning.

## Prepare Update Details

Apply [Group Hunks](../SKILL.md#group-hunks) to map each pending change to its appropriate original commit or to an independently useful new commit. Keep changes separate when they express a distinct decision rather than repairing an earlier one. Do not manufacture content changes for a base-only or message-only update.

Preserve each existing commit’s complete message unless a message change is requested. Use [Compose Messages](compose-messages.md) for ordinary new commits and newly requested wording, not for inherited messages or Git’s temporary fixup messages. Treat a supplied exact replacement as a constraint, retaining the entrypoint’s [message safeguards](../SKILL.md#preserve-message-constraints).

Add these details to the [execution baseline](commit-execution.md#confirm-commits). Shared confirmation determines whether to present them:

1. Record the old boundary, intended base, complete rewritten range, topology, and publication evidence. For a published update, include the destination ref, recorded remote head, and disposition of differences from local history.
2. Map each repair’s hunks to its original commit’s recorded object ID in the intended final series, identifying separate new commits and partial-file boundaries. Retain exact new or replacement messages and inspected sources for complete inherited messages.
3. Select the [native operation](#execute-history-updates) and command sequence. Record any todo list changes, expected empty or already applied commits and their disposition, and resulting parent relationships. Distinguish temporary fixup messages from final messages.

Replay requires a clean index and tracked working tree, with ignored and untracked state protected from overwrite. Any temporary isolation must be explicit, authorized, and exactly restorable. Do not let implicit autostashing or automatic updates to other branches widen the operation.

A pending [human review marker](preserve-human-review-markers.md) may prevent clean replay. In that case, pause for the user’s required action instead of committing or temporarily hiding it.

## Execute History Updates

During [Create Approved Commits](commit-execution.md#create-approved-commits), choose the smallest native operation that implements the approved transformation while preserving topology, hooks, signing, authorship, and message constraints:

- **Base-only updates:** Use native rebase to move the original series onto the approved base without temporary fixups or autosquash.
- **Current-tip repairs:** When only the current tip needs replacement and no base or series change requires replay, amend directly. Use `git commit --amend --no-edit` for content-only repairs, or `git commit --amend` with the exact replacement message when wording also changes. For a message-only amendment, use `git commit --amend --only` without pathspecs to exclude staged content. Do not stage content for that operation, and retain the shared exact unrelated-index preservation and restoration checks even though `--only` excludes it.
- **Repairs requiring replay:** When earlier commits change or repairs accompany a base or series change, create target-linked fixups, then run an interactive autosquash rebase over the approved range onto the approved base. Use `git commit --fixup=amend:<commit-oid>` for content and message changes, `git commit --fixup=<commit-oid>` for content-only repairs, or `git commit --fixup=reword:<commit-oid>` for message-only changes. Supply the exact replacement message for amend and reword fixups. Reword fixups ignore staged content. Resolve the todo list against recorded target IDs before replay so matching subjects cannot attach a fixup to the wrong commit.
- **Separate changes:** Create ordinary commits with their approved complete messages instead of converting them into fixups.

For every rebase, use `--no-autostash` and `--no-update-refs` so preservation and branch updates remain explicit, and preserve the selected topology and root handling. Use literal-safe, non-interactive input for approved message and todo edits.

For a paused operation, establish that its remaining todo list and conflict resolution still match the execution baseline before continuing through the shared lifecycle. Do not skip a commit merely to clear a conflict or accept an unexpected empty commit as an authorized deletion.

## Verify Rewritten History

At the shared verification checkpoint, compare the original series plus approved changes with the resulting series. For direct amendments, verify unchanged parents and, for message-only changes, an identical tree. For fixup replay, check that every fixup reached its intended target and no temporary fixups remain. Separate new commits must remain separate, with no change duplicated or silently dropped. Verify the selected base, parent relationships, intended order, and unchanged unrelated refs.

Assess each resulting commit’s independent coherence rather than treating a passing final checkout as proof that every commit is coherent.

Return to [Report Results](commit-execution.md#report-results), including the old-to-new commit mapping and any branch advancement that created no commits. Report final commits rather than temporary fixups. If execution stopped, identify the remaining operation state instead of claiming the series is complete. When a published branch needs replacement, complete the handoff below.

## Hand Back Published Updates

Immediately before publication or handing back its command, recheck the publication destination. If its head has changed, inspect and reconcile the new work within the current authorization, then repeat affected validation. Stop when reconciliation requires new scope. Do not merely substitute a newer object ID into the lease expectation. An earlier post-update check does not replace this final check.

Under the shared [publication authorization](commit-execution.md#report-results), target only the single verified publication destination and full branch ref, use `--force-with-lease=<remote-ref>:<expected-remote-oid>` with the recorded expected object ID, and use the reviewed new tip’s object ID as the explicit refspec source. Do not use implicit destinations or plain `--force`. A later remote-head change must make the lease reject the command rather than replace unseen work.

Suppress automatic tag publication for that invocation, and use `--recurse-submodules=check` so submodule validation cannot publish extra commits. Preserve repository-required checks and confirm that configuration cannot expand the invocation to other destinations or refs. If the publication scope cannot be established, pause for the user’s target or configuration decision rather than guessing or silently changing configuration.
