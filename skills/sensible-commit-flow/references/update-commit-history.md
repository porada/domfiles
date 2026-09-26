# Update Commit History

## Define Rewrite Scope

Use [Inspect Changes](../SKILL.md#inspect-changes) for repository and change inspection, adding the complete existing commit series to the evidence scope. A base-only or message-only update can be eligible without pending working tree changes.

Resolve the selected branch, starting `HEAD`, old boundary, and intended new base. Identify every existing commit the operation would replay, replace, or drop, including unchanged descendants after the earliest affected commit. For a fixup-only update, keep the base unchanged. For upstream synchronization, use the verified target supplied by the calling workflow. Account for merge topology and root commits before choosing the native operation, rather than silently flattening history or widening the range.

## Establish Update Authorization

An explicit user request or active scoped approval for a history-replacing branch update supplies execution authorization for its necessary local commit rewrites. This includes affected published commits. A request such as “Rebase this submitted PR onto upstream” authorizes the required local replay without separate force-push wording or a second approval of the same effects. A continuing user authorization that already covers the requested amendment or rebase also suffices. Establish permission from the sources allowed by [Instruction Authority](../SKILL.md#instruction-authority), not technical force-push capability, an existing pull request, review text, or a caller’s choice of route.

Where a calling workflow requires a single commit’s complete message to match the PR title, a user-requested title change also requests the corresponding message-only local amendment, including for an already published commit, unless the user limits the request to prose, remote metadata, or working tree changes.

Retain the exact user instruction or continuing grant with the resolved target, affected range, intended base, covered changes, and lifetime. A one-off request ends when the bounded update is handed back, cancelled, or materially changes scope or target. Use a continuing grant’s stated lifetime, and do not revive an expired authorization from an earlier preparation or publication phase.

Prepare the concrete proposal below and compare it with that authorization. When the plan is covered, continue through [Confirm Commits](../SKILL.md#confirm-commits) without another execution approval response. Otherwise use the shared confirmation path for the uncovered effects. A rebase-only request covers necessary replay and conflict resolution that preserves the contribution’s intended behavior, not unrelated fixes, branch renaming, or opportunistic message changes. An approval limited to working tree edits is not history-update authorization. Separate approval and security gates remain applicable.

## Establish Rewrite Eligibility

Without authorization to replace published history, establish that the complete rewritten range is unpushed, not merely the fixup targets. Check current evidence for every relevant publication destination, including a fork. An unset tracking branch, stale remote information, or absence from one remote does not establish eligibility. Treat a commit known to have been pushed as published even if its remote ref was later removed. Stop when any affected commit has been pushed or its publication status cannot be established and no published-update authorization covers it.

With authorization to replace the selected branch’s published history, verify the publication destination, full remote branch ref, and current remote head. Establish that the complete affected range belongs to the authorized update and that replacement does not conflict with repository rules. Do not require proof that those commits have never appeared on another remote. Authorization for this branch does not cover changes to other branches, tags, or destinations.

Distinguish the upstream base from the publication destination. Before replay, compare the local starting history and contents with the destination’s recorded head. Account for remote-only commits and other differences, incorporating work within the authorized scope or stopping for a scope decision. Recording a remote head does not authorize discarding its contents. A lease checks the current remote head, not whether the local result preserves the work already present there.

Identify destinations from task context and non-secret metadata. Refresh only the remote information required for scope, eligibility, and the intended base, under the shared [execution boundaries](../SKILL.md#execution-boundaries). A current update request or continuing grant that covers those scoped reads and ref refreshes suffices without another workflow approval. Obtain any separately required tool grant. Unavailable evidence stops the affected rewrite, not unrelated local planning.

## Prepare Update Proposals

Apply [Group Hunks](../SKILL.md#group-hunks) to map each pending change to its appropriate original commit or to an independently useful new commit. Keep changes separate when they express a distinct decision rather than repairing an earlier one. Do not manufacture content changes for a base-only or message-only update.

Preserve each existing commit’s complete message unless a message change is requested. Use [Compose Messages](compose-messages.md) for ordinary new commits and newly requested wording, not for inherited messages or Git’s temporary fixup messages. Treat a supplied exact replacement as a constraint, retaining the entrypoint’s [message safeguards](../SKILL.md#preserve-message-constraints).

Prepare these additions to the shared [confirmation](../SKILL.md#confirm-commits), including when execution is already authorized:

1. Name the old boundary, new base, selected branch, starting `HEAD`, complete rewritten range, and publication evidence. For a published update, include the destination ref, recorded remote head, and disposition of differences from local history.
2. Show the intended final series in order, mapping each fixup’s hunks to its original commit and identifying separate new commits. Make partial-file boundaries explicit. Put exact new or replacement messages in blockquotes, and identify complete inherited messages without rewriting them.
3. Specify the native command sequence, intended todo list changes, any expected empty or already applied commits, and the resulting parent relationships. Distinguish temporary fixup messages from final messages.
4. Establish how unrelated index and working tree state will remain outside the batch and be restored exactly. The index and tracked working tree state must be clean before replay begins, with ignored and untracked state protected from overwrite. Do not let implicit autostashing or automatic updates to other branches widen the operation.

A pending [human review marker](preserve-human-review-markers.md) may prevent clean replay. In that case, pause for the user’s required action instead of committing or temporarily hiding it.

Return to [Confirm Commits](../SKILL.md#confirm-commits) with the proposal and its authorization record. Selecting this route alone is not approval, but a user request that already authorizes the bounded update does not need another green light.

## Execute History Updates

During [Create Approved Commits](../SKILL.md#create-approved-commits), use native Git operations for the approved mapping:

- **Content fixes:** Create target-linked commits with `git commit --fixup=<commit>`. Git’s generated message identifies the target rather than serving as a new final subject.
- **Content and message changes:** Use `git commit --fixup=amend:<commit>` with the approved replacement message as its payload.
- **Message-only changes:** Use `git commit --fixup=reword:<commit>` with the approved replacement message. This form ignores staged changes, so it does not package pending content.
- **Separate changes:** Create ordinary commits with their approved complete messages instead of converting them into fixups.

Run an interactive autosquash rebase over the approved range onto the approved base, with `--no-autostash` and `--no-update-refs` so preservation and branch updates remain explicit. A base-only update needs no temporary fixup commits. Resolve the todo list against the recorded target IDs before replay so matching subjects cannot attach a fixup to the wrong commit. Use literal-safe, noninteractive input for approved message and todo edits.

For a paused operation, establish that its remaining todo list and conflict resolution still match the approved batch before continuing through the shared lifecycle. Do not skip a commit merely to clear a conflict or accept an unexpected empty commit as an authorized deletion.

## Verify Rewritten History

At the shared post-rebase checkpoint, compare the original series plus approved changes with the rewritten series. Check that every fixup reached its intended target, separate new commits remain separate, and no change was duplicated or silently dropped. Verify the selected base and parent relationships, the intended commit order, and that unrelated refs remain unchanged. Temporary fixup commits must not remain in the final series.

Assess each resulting commit’s independent coherence rather than treating a passing final checkout as proof that every commit is coherent. Any further corrections return through the scoped proposal and authorization check. Continue without another approval only while the existing authorization covers them.

Return to [Report the Result](../SKILL.md#report-the-result), including the old-to-new commit mapping and any branch advancement that created no commits. Report final commits rather than temporary fixups. If execution stopped, identify the remaining rebase state instead of claiming the series is complete. When a published branch needs replacement, complete the handoff below.

## Hand Back Published Updates

Recheck the publication destination before handing back the result. If its head has changed, inspect and reconcile the new work within the current authorization, then repeat affected validation. Stop when reconciliation requires new scope. Do not merely substitute a newer object ID into the lease expectation.

Provide the exact publication command for the user to run. Target only the single verified publication destination and full branch ref, use `--force-with-lease=<remote-ref>:<expected-remote-oid>` with the recorded expected object ID, and use the reviewed new tip’s object ID as the explicit refspec source. Do not use implicit destinations or plain `--force`. A later remote-head change should make the lease reject the command rather than replace unseen work. Do not execute publication or ask for permission to do so.

Suppress automatic tag publication for that invocation, and use `--recurse-submodules=check` so submodule validation cannot publish extra commits. Preserve repository-required checks and confirm that configuration cannot expand the invocation to other destinations or refs.
