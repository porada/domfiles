# Zed Worktrees

## Create and Enter the Checkout

Check the tools actually exposed in the current session, including `create_thread`, and use their schemas to determine which creation actions they support. Do not infer availability merely because Zed supports the feature. Follow [Recognize Conversation Handoffs](#recognize-conversation-handoffs) before selecting an operation that also starts a conversation.

Use an available native operation that preserves Zed’s managed lifecycle. If no exposed tool supports the authorized creation, identify the missing tool or capability as a limitation of this session and request the smallest necessary Zed UI action, using the title-bar worktree picker (`git: worktree`) for checkout creation. Do not substitute raw Git creation, even beneath Zed’s managed directory, because the resulting checkout would lack Zed’s creation record.

Use Zed’s configured worktree location, generated name, and default starting ref unless the user specifies otherwise or the task requires a particular revision. The location comes from the global `git.worktree_directory` setting. Do not supply a custom name merely to impose a task slug or branch naming scheme. Confirm the starting ref when it matters to the task rather than assuming every creation interface has the same default.

New Zed worktrees start detached, so beginning edits or validation does not require creating a branch. A fresh checkout contains the selected committed state, not the original checkout’s staged, unstaged, or untracked files. Leave those files untouched unless the task authorizes a bounded transfer.

After creation, follow the entrypoint’s [existing-worktree entry checks](../SKILL.md#enter-an-existing-worktree) before editing. Creating a worktree does not by itself establish that the current conversation now operates there.

## Recognize Conversation Handoffs

When exposed in the current session, `create_thread` can create both a linked worktree and an independent sibling conversation. The sibling receives no parent conversation history, and its output does not return to the parent. Apply the entrypoint’s [handoff boundary](../SKILL.md#select-the-lifecycle) before invoking such an operation. Its availability does not make it a substitute for a subagent that returns results to a coordinator.

When the confirmed flow selects native dispatch, use `agent-task-relay`’s workflow-owned delivery rule to obtain the composed assignment rather than emit a user-mediated relay. Supply that assignment through the exposed tool’s schema, preserving the confirmed flow and worktree choices. Report only the creation outcome established by the tool response, then stop rather than claiming the receiving task is complete.

Without a suitable native dispatch tool, use `agent-task-relay`’s normal user-mediated delivery and include any required Zed UI action in the proposed flow.

## Preserve Managed State

Keep managed archival, restoration, and removal user-controlled through Zed. Moving the last active thread using a managed worktree to Thread History does not guarantee checkout archival. Zed excludes worktree roots still referenced by sidebar terminals. If no eligible roots remain, the thread enters Thread History without a new worktree checkpoint or checkout removal.

When worktree archival completes, Zed saves the Git state and removes the checkout. Reopening a thread restores saved state only when a worktree archive exists. Without an archive, it reuses the existing checkout. Worktree archival writes internal Git checkpoint commits and is not integration or routine agent cleanup under the global **Commit gate**.

Those checkpoints preserve tracked and ordinary untracked files, but omit untracked Git-ignored files and empty directories. Before recommending archival or removal, account for any still-needed ignored state. Preserve it through an authorized operation or leave the worktree active. Do not force-stage ignored material merely to make archival retain it.

Before recommending permanent worktree or thread deletion, verify that the work is integrated or explicitly abandoned and that no needed local state depends on the checkout or archive. Branch cleanup is a separate task, not a consequence of removing a worktree.

Do not move or rename a Zed-managed checkout with raw Git or filesystem operations. Zed’s ownership and archive records depend on its path. If a different name or location becomes necessary, resolve an authorized transition rather than modifying those records or treating a filesystem move as a managed rename.
