---
name: git-worktrees
description: |-
    Use this skill when beginning a task in an existing linked Git worktree. Also use it before deciding whether a task warrants an isolated worktree, before administering, archiving, creating, moving, removing, or restoring a worktree, before integrating changes from a worktree into another branch, before creating, deleting, or renaming a branch as part of worktree management, and before materializing a historical revision.

    Do not use it for branch operations unrelated to worktree management, ordinary work in the primary checkout that involves neither worktree integration nor an isolation decision, or temporary directories that are not worktrees. After the current task’s entry checks, ordinary Git commands and edits alone do not require reloading it.

metadata:
    internal: true
---

# Git Worktrees

Keep worktree paths independent of branch names. A worktree may use a branch or detached `HEAD`. The global **Concurrent work** rule remains responsible for preserving existing changes and coordinating write scopes.

## Enter an Existing Worktree

Use this path for an existing worktree rather than repeating its creation or original isolation decision. Do not investigate or ask about lifecycle ownership solely for ordinary edits, inspection, or validation.

1. Confirm that active tools target the intended repository and linked worktree, not the primary checkout or another worktree. Verify registration with `git --no-pager worktree list --porcelain`.
2. Read this checkout’s applicable instructions and inspect its state under the global **Concurrent work** rule. Record `HEAD` and its branch or detached state, and confirm the starting revision when the task depends on it. Do not assume the checkout is untouched.
3. Keep the existing branch or detached state, location, and name unless the task authorizes a change. Detached `HEAD` alone is not a reason to create a branch. Do not transfer state from another checkout without bounded task authorization.
4. Before commit-related staging or creating commits, follow `commit`, which applies the global **Commit gate**.

## Decide Whether to Isolate

- **Create only for:**
    - An explicit user request for an isolated worktree.
    - Another active agent with an overlapping write scope.
    - A task requiring isolated branch, dependency, build, or test state.
    - A broad or high-risk change that materially benefits from independent rollback and has a clear integration plan.
- **Do not isolate merely for:** A dirty repository, possible concurrent activity, or any task that modifies repository files. Keep follow-up edits to the same uncommitted task in its existing checkout.
- **Historical inspection:** Inspect revisions through Git without materializing them. Materialize a revision only when a filesystem-based tool requires it, applying the criteria above when isolation is necessary.

## Select the Lifecycle

Before a worktree lifecycle operation, resolve who manages or will manage the checkout from established task context or available evidence. If ownership remains unclear, ask before that operation. Neither a directory name nor its presence in an editor proves which tool manages it. Recheck Git registration when an administration decision depends on it.

When Zed owns or will create the worktree, follow [Zed Worktrees](references/zed-worktrees.md) for every lifecycle operation. Prefer this workflow for tasks using Zed. The creation, administration, and dismantling sections below apply only to explicitly selected direct Git management, not as a fallback for an unavailable native operation. [Integration](#integrate-changes) applies to both direct-Git and Zed-managed worktrees.

Checkout isolation does not authorize starting another conversation. If proceeding requires an independent conversation handoff, use `agent-task-relay` for confirmation and assignment composition, carrying the resolved isolation decision into that workflow. This skill does not dispatch threads.

## Create a Worktree

- **Destination:** Resolve the destination and start point from the user’s request or applicable project policy. Do not invent a directory convention when neither provides one.
- **Branch state:** Choose detached mode or an explicitly selected branch independently of the destination. Creating a new branch requires its own task authorization.
- **Creation:** Use `git worktree add` with the resolved destination and start point, explicitly selecting the branch or detached mode. Before beginning work there, follow [Enter an Existing Worktree](#enter-an-existing-worktree).

## Administer a Worktree

- **Repository targeting:** Run direct Git worktree-administration commands from the primary checkout unless the command requires another location.
- **Relocation:** For a task-authorized move, apply the global **Concurrent work** and **Recoverability** policies to the source and destination. Preserve the checkout’s `HEAD`, index, and working-tree state, including ignored and untracked files. Use `git worktree move` without replacing existing destination state, then verify the preserved state and updated Git registration. A state-preserving relocation does not require integration or abandonment.

## Integrate Changes

Use this workflow for task-authorized integration into another branch. Integrating commits does not administer the worktree or require resolving its lifecycle owner.

1. Identify the source checkout, intended changes, and destination branch and checkout. Inspect both checkouts for existing changes and unfinished Git operations. Preserve unrelated state under the global **Concurrent work** policy. Do not mix integration with a Git operation already in progress.
2. Identify the exact source commits and confirm that their changes fit the authorized scope. Detached `HEAD` does not require creating a branch, and linked worktrees share commit objects. Follow `commit` before any commit-producing operation, including committing source changes, cherry-picking, or a non-fast-forward merge. A fast-forward creates no new commits.
3. Run integration from the destination checkout on the selected branch. Prefer a fast-forward when the destination tip is an ancestor of the source tip and every intervening commit is in scope. Otherwise, resolve whether to merge the source history or cherry-pick selected commits in dependency order. Ask when the requested history outcome does not settle that choice.
4. Resolve integration conflicts within the authorized scope, then validate the combined result and verify that every intended change reached the destination. Keep the source worktree available until verification is complete. Integration alone does not authorize branch or worktree cleanup, and the global **Git publication** prohibition still applies. When cleanup is separately requested, follow [Select the Lifecycle](#select-the-lifecycle).

## Dismantle a Worktree

- **Destructive-operation gate:** Before removing a worktree, or force-renaming or deleting an associated branch, inspect the affected worktree status and verify that the affected changes are integrated or explicitly abandoned.
    - Remove worktrees with `git worktree remove`. Use one or two exact `-f` or `--force` options only after verification when an unclean or locked worktree requires them. Verify afterward that the corresponding directory is gone, and inspect it rather than deleting it recursively if it remains.
    - Delete a branch only when the task explicitly includes branch cleanup. Worktree removal alone does not authorize it. First use `git branch -d <branch-name>`. Use `git branch -D <branch-name>` only when Git refuses because the branch is unmerged and the preceding verification established that its changes are integrated or explicitly abandoned.
