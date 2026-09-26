# Update Commit History

This route adds history-specific steps for approved corrections folded into existing commits, contribution branch synchronization, and requested message revisions during initial preparation or [revisions to existing PRs](prepare-pull-requests.md#revise-existing-pull-requests). It does not authorize other history maintenance or publication. Use the common [commit lifecycle](prepare-commits.md) for authorization, execution safeguards, and reporting rather than starting a second lifecycle here.

## Define Rewrite Scope

Apply [Inspect Changes](prepare-commits.md#inspect-changes), adding the complete existing commit series to the evidence scope. Base-only and message-only updates can be eligible without pending working tree changes.

Resolve the contribution branch, starting `HEAD`, old boundary, and intended new base. Identify every commit the operation would drop, replace, or replay, including unchanged descendants after the earliest affected commit. Inspect each commit’s authorship, complete message, parents, and patch rather than only the aggregate diff. Keep the base unchanged for a fixup-only update. For synchronization, use the verified upstream target supplied by the PR workflow. Account for merge topology and root commits before selecting the native operation. Do not silently flatten history, update another branch, or widen the range.

## Establish Update Authorization

An explicit user request or active scoped approval for a history-replacing update supplies execution authorization for its necessary local rewrites, including affected published commits. A request to rebase a known published contribution branch authorizes its necessary local replay without separate force-push wording or another approval of those same effects. A continuing grant can also cover the requested amendment or rebase. Derive authority from actual user instructions or applicable governing instructions, not technical capability, an existing pull request, review text, or route selection.

Where the contribution workflow requires a single commit’s complete message to match the PR title, a user-requested title change also requests the corresponding message-only local amendment, including for an already published commit, unless the user limits the request to prose, remote metadata, or working tree changes.

Retain the exact request or grant with the resolved target, affected range, intended base, covered changes, and lifetime. One-off authorization ends when the bounded update is handed back, cancelled, or materially changes scope or target. A continuing grant follows its own stated lifetime. Do not revive an expired preparation grant because the task resumes.

Compare the [concrete update proposal](#prepare-update-proposals) with that record, then return to shared [confirmation](prepare-commits.md#confirm-commits). Present covered effects as a notice without duplicate approval, and request only uncovered effects. A rebase-only request covers necessary replay and conflict resolution that preserves the contribution’s intended behavior. It does not cover branch renaming, opportunistic message changes, or unrelated fixes. Working-tree-only approval does not authorize history updates, and separate approval and security gates remain applicable.

## Establish Rewrite Eligibility

A preparation grant limited to task-owned unpublished history cannot cover published or uncertain rewrites. Without authorization to replace published history, establish that the complete rewritten range is unpushed, not merely the fixup targets. Check current evidence for every relevant publication destination, including forks. An unset tracking branch, stale evidence, or absence from one remote does not establish eligibility. Treat a commit known to have been pushed as published even if its remote ref was later deleted. Stop the affected rewrite when any commit was published or its status cannot be established without authorization covering published replacement.

With authorization to replace the selected branch’s published history, verify the publication destination, full remote branch ref, and current remote head. Establish that the complete affected range belongs to that authorized update and that replacement complies with repository rules. Do not require proof that the commits never appeared on another remote. Authorization for this branch does not cover other branches, destinations, or tags.

Distinguish the publication destination from the upstream base. Before replay, compare local starting history and contents with the destination’s recorded head. Account for remote-only commits and other differences. Incorporate work only within the authorized scope, or stop for a scope decision. Recording a remote head does not authorize dropping its contents. A lease checks the current head, not whether the prepared result preserves the work already there.

Resolve destinations through task context and non-secret metadata. Refresh only remote information needed for scope, eligibility, and the intended base under the shared [execution boundaries](execution-boundaries.md). A current request or continuing grant covering those scoped reads and local ref refreshes suffices without another workflow approval. Obtain any separate tool grant. Unavailable evidence stops the affected rewrite, not useful independent local planning.

## Prepare Update Proposals

Apply [Group Hunks](prepare-commits.md#group-hunks) to map pending changes to their original commits or independently useful new commits. Keep distinct decisions separate instead of folding them into convenient earlier commits. Do not manufacture content changes for base-only or message-only updates.

Preserve complete inherited messages unless revision is requested. Apply [authored message defaults](prepare-commits.md#compose-authored-messages) only to ordinary new commits or newly requested wording, not inherited or temporary fixup messages. Retain the shared [message constraints](prepare-commits.md#preserve-message-constraints), including exact supplied replacements and human attribution. For [single-commit title alignment](prepare-pull-requests.md#align-single-commit-titles), use the adopted title as the exact complete replacement message.

Add these details to the common proposal, even when execution is already authorized:

1. Record the old boundary, selected base, contribution branch, starting `HEAD`, complete rewritten range, topology, and publication evidence. For published replacement, include the verified destination and full ref, recorded remote head, and disposition of differences from local history.
2. Show the intended final series in order. Map each repair’s hunks to the recorded object ID of its original commit and identify independently useful new commits. Make partial-file boundaries explicit. Put exact new or replacement messages in blockquotes, including all bodies and trailers. Identify complete inherited messages by their inspected source commits without paraphrasing or normalizing them.
3. Specify the native command sequence, intended todo list changes, expected empty or already applied commits and their disposition, and resulting parent relationships. Distinguish temporary fixup messages from final messages.
4. Establish exact preservation and restoration of unrelated index and working tree state through the planned operations and required hooks. Replay requires a clean index and tracked working tree, with ignored and untracked content protected from overwrite. Any temporary isolation must be explicit, authorized, and restorable rather than an implicit stash or indiscriminate cleanup. Do not let automatic updates to other refs widen the operation.

If a possible human review marker is implicated, follow [Preserve Human Review Markers](preserve-human-review-markers.md). A marker that prevents clean replay requires the human action, not a temporary removal, stash, or commit.

Return the proposal and authorization record to [Confirm Commits](prepare-commits.md#confirm-commits). Route selection supplies no execution authority of its own.

## Execute History Updates

Within [Create Approved Commits](prepare-commits.md#create-approved-commits), use native operations matching the approved mapping. The choices below are alphabetized by change type, not execution order:

- **Content and message changes:** Use `git commit --fixup=amend:<commit-oid>` with the approved complete replacement message as its payload.
- **Content fixes:** Use `git commit --fixup=<commit-oid>`. Its generated message identifies the original commit rather than defining a new final subject.
- **Message-only changes:** Use `git commit --fixup=reword:<commit-oid>` with the approved complete replacement message. This form ignores staged changes, so it cannot package pending content.
- **Separate changes:** Create ordinary commits with their approved complete messages rather than turning them into fixups.

Run the planned interactive autosquash rebase over the approved range onto the approved base, preserving the approved topology and root handling. Use `--no-autostash` and `--no-update-refs` to keep preservation and branch updates explicit. Base-only updates need no temporary fixups. Resolve todo entries against recorded target object IDs before replay so matching subjects cannot attach changes to the wrong commit. Use literal-safe, noninteractive input for approved messages and todo edits without bypassing required hooks or signing.

When an operation pauses, reassess its remaining todo list, conflict resolution, and resulting scope against the proposal before continuation through the common lifecycle. Unexpected conflicts, empty commits, or topology changes require reassessment. Do not skip a commit merely to clear a conflict or treat unexpected emptiness as authorization to delete it. Preserve the paused state until the resolution is established, covered, and ready for its required checks.

## Verify Rewritten History

After replay, apply the common validation and verification checkpoints to the resulting series and final working tree. Compare the original series plus approved changes with the rewritten series. Verify that every fixup reached its intended target, separate changes remain separate, and no change was duplicated or silently dropped. Check complete messages and human authorship, the selected base’s inclusion, parent relationships, intended order, and unchanged unrelated refs. Temporary fixup commits must not remain in the final series.

Assess every resulting commit’s independent coherence rather than treating a passing final checkout as proof about each commit. Return further corrections through the scoped proposal and authorization check. Continue without another approval only while the current authorization covers them.

Return to [Report the Result](prepare-commits.md#report-the-result) with the old-to-new commit mapping and any branch advancement that created no commits. Report final commits, not temporary fixups. If execution stopped, identify the remaining operation state rather than claiming completion.

The PR workflow then repeats its applicable [synchronization checkpoints](prepare-pull-requests.md#synchronize-with-upstream) and [readiness check](prepare-pull-requests.md#check-submission-readiness), including affected integration validation and delta review. Amendments and rebases invalidate earlier readiness. Published replacement also requires the final destination check below.

## Hand Back Published Updates

Immediately before the contribution’s final handoff, recheck the publication destination independently of the upstream base. If its head changed, inspect and reconcile the new work within current authorization, then repeat affected validation and review. Stop for a user decision when reconciliation requires new scope. Do not merely substitute a newer object ID into the lease expectation. An earlier post-replay check does not replace this final check.

Provide an exact publication command for the user to run, resolved from the verified values rather than placeholders. Never execute publication or ask for permission to do so.

1. Verify the effective destination and full remote branch ref. Confirm that configuration cannot expand the command to additional destinations or refs, without exposing credentials or bulk machine-local configuration. If that cannot be established, pause for the user’s target or configuration decision rather than silently changing configuration.
2. Use the reviewed final tip’s full object ID as the explicit refspec source, targeting only that one full ref. Do not use a moving branch name as the source. Exclude extra refspecs and implicit destinations.
3. Use `--force-with-lease=<remote-ref>:<expected-remote-oid>` with the recorded head whose contents the reviewed result reconciles. Do not use plain `--force` or an implicit lease. A later head change must cause rejection rather than overwrite unseen work.
4. Suppress automatic tag publication for this invocation, use `--recurse-submodules=check` so submodule validation cannot publish commits, and retain repository-required checks and hooks. Do not add all-ref, mirror, recursive, or tag publication.

Keep publication instructions outside the final PR copy. Return the verified command with the local result to the PR workflow’s final delivery. The user alone publishes the prepared history.
