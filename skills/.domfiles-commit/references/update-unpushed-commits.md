# Update Unpushed Commits

## Define Rewrite Scope

Use [Inspect Changes](../SKILL.md#inspect-changes) for repository and change inspection, adding the complete existing commit series to the evidence scope. A base-only or message-only update can be eligible without pending working-tree changes.

Resolve the selected branch, starting `HEAD`, old boundary, and intended new base. Identify every existing commit the operation would replay, replace, or drop, including unchanged descendants after the earliest affected commit. For a fixup-only update, keep the base unchanged. For upstream synchronization, use the verified target supplied by the calling workflow. Account for merge topology and root commits before choosing the native operation, rather than silently flattening history or widening the range.

Establish that the complete rewritten range is unpushed, not merely the fixup targets. Check current evidence for every relevant publication destination, including a fork, and refresh scoped remote information when necessary. An unset tracking branch, stale remote information, or absence from one remote does not establish eligibility. Treat a commit known to have been pushed as published even if its remote ref was later removed. Stop when any affected commit has been pushed or its publication status cannot be established.

## Prepare Update Proposals

Apply [Group Hunks](../SKILL.md#group-hunks) to map each pending change to its appropriate original commit or to an independently useful new commit. Keep changes separate when they express a distinct decision rather than repairing an earlier one. Do not manufacture content changes for a base-only or message-only update.

Preserve each existing commit’s complete message unless a message change is requested. Use [Compose Subjects](compose-subjects.md) for ordinary new commits and newly requested wording, not for inherited messages or Git’s temporary fixup messages. Treat a supplied exact replacement as a constraint, retaining the entrypoint’s [message safeguards](../SKILL.md#preserve-message-constraints).

Prepare these additions to the shared [confirmation](../SKILL.md#confirm-commits):

1. Name the old boundary, new base, selected branch, starting `HEAD`, complete rewritten range, and publication evidence.
2. Show the intended final series in order, mapping each fixup’s hunks to its original commit and identifying separate new commits. Make partial-file boundaries explicit. Put exact new or replacement messages in blockquotes, and identify complete inherited messages without rewriting them.
3. Specify the native command sequence, intended todo-list changes, any expected empty or already-applied commits, and the resulting parent relationships. Distinguish temporary fixup messages from final messages.
4. Establish how unrelated index and working-tree state will remain outside the batch and be restored exactly. The index and tracked working-tree state must be clean before replay begins, with ignored and untracked state protected from overwrite. Do not let implicit autostashing or automatic updates to other branches widen the operation.

Return to the shared lifecycle for execution approval. Selecting this mode is not permission to create fixups or rewrite commits.

## Execute History Updates

During [Create Approved Commits](../SKILL.md#create-approved-commits), use native Git operations for the approved mapping:

- **Content fixes:** Create target-linked commits with `git commit --fixup=<commit>`. Git’s generated message identifies the target rather than serving as a new final subject.
- **Content and message changes:** Use `git commit --fixup=amend:<commit>` with the approved replacement message as its payload.
- **Message-only changes:** Use `git commit --fixup=reword:<commit>` with the approved replacement message. This form ignores staged changes, so it does not package pending content.
- **Separate changes:** Create ordinary commits with their approved subjects instead of converting them into fixups.

Run an interactive autosquash rebase over the approved range onto the approved base, with `--no-autostash` and `--no-update-refs` so preservation and branch updates remain explicit. A base-only update needs no temporary fixup commits. Resolve the todo list against the recorded target IDs before replay so matching subjects cannot attach a fixup to the wrong commit. Use literal-safe, noninteractive input for approved message and todo edits.

For a paused operation, establish that its remaining todo list and conflict resolution still match the approved batch before continuing through the shared lifecycle. Do not skip a commit merely to clear a conflict or accept an unexpected empty commit as an authorized deletion.

## Verify Rewritten History

At the shared post-rebase checkpoint, compare the original series plus approved changes with the rewritten series. Check that every fixup reached its intended target, separate new commits remain separate, and no change was duplicated or silently dropped. Verify the selected base and parent relationships, the intended commit order, and that unrelated refs remain unchanged. Temporary fixup commits must not remain in the final series.

Assess each resulting commit’s independent coherence rather than treating a passing final checkout as proof that every commit is coherent. Any further corrections return through a scoped proposal and the shared confirmation and execution lifecycle.

Return to [Report the Result](../SKILL.md#report-the-result), including the old-to-new commit mapping and any branch advancement that created no commits. Report final commits rather than temporary fixups. If execution stopped, identify the remaining rebase state instead of claiming the series is complete.
