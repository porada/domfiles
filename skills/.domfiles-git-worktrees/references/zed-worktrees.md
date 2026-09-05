# Zed Worktrees

## Create and Enter the Checkout

Use the title-bar worktree picker (`git: worktree`) or an available native operation that preserves Zed’s managed lifecycle. If the required operation is unavailable to the agent, ask the user to perform it in Zed. Do not substitute raw Git creation, even beneath Zed’s managed directory, because the resulting checkout would lack Zed’s creation record.

Use Zed’s configured worktree location, generated name, and default starting ref unless the user specifies otherwise or the task requires a particular revision. The location comes from the global `git.worktree_directory` setting. Do not supply a custom name merely to impose a task slug or branch naming scheme. Confirm the starting ref when it matters to the task rather than assuming every creation interface has the same default.

New Zed worktrees start detached, so beginning edits or validation does not require creating a branch. A fresh checkout contains the selected committed state, not the original checkout’s staged, unstaged, or untracked files. Leave those files untouched unless the task authorizes a bounded transfer.

After creation, follow the entrypoint’s [existing-worktree entry checks](../SKILL.md#enter-an-existing-worktree) before editing. Creating a worktree does not by itself establish that the current conversation now operates there.

## Recognize Conversation Handoffs

Zed’s `create_thread` operation can create both a linked worktree and an independent sibling conversation. The sibling receives no parent conversation history, and its output does not return to the parent. Apply the entrypoint’s handoff boundary before selecting such an operation. Its availability does not make it a substitute for a subagent that returns results to a coordinator.

## Preserve Managed State

Keep managed archival, restoration, and removal user-controlled through Zed. Moving the last active thread using a managed worktree to Thread History does not guarantee checkout archival. Zed excludes worktree roots still referenced by sidebar terminals. If no eligible roots remain, the thread enters Thread History without a new worktree checkpoint or checkout removal.

When worktree archival completes, Zed saves the Git state and removes the checkout. Reopening a thread restores saved state only when a worktree archive exists. Without an archive, it reuses the existing checkout. Worktree archival writes internal Git checkpoint commits and is not integration or routine agent cleanup under the global **Commit gate**.

Those checkpoints preserve tracked and ordinary untracked files, but omit untracked Git-ignored files and empty directories. Before recommending archival or removal, account for any still-needed ignored state. Preserve it through an authorized operation or leave the worktree active. Do not force-stage ignored material merely to make archival retain it.

Before recommending permanent worktree or thread deletion, verify that the work is integrated or explicitly abandoned and that no needed local state depends on the checkout or archive. Branch cleanup is a separate task, not a consequence of removing a worktree.

Do not move or rename a Zed-managed checkout with raw Git or filesystem operations. Zed’s ownership and archive records depend on its path. If a different name or location becomes necessary, resolve an authorized transition rather than modifying those records or treating a filesystem move as a managed rename.
