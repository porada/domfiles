---
name: git-worktrees
description: Apply agent worktree isolation policy for Git repositories. Use this skill before deciding whether a task warrants an isolated worktree, before creating one or its paired branch, before running Git commands inside a registered worktree, before moving or removing either, and before materializing a historical revision. Covers isolation criteria, the paired worktree and branch namespaces, command location, and the destructive-operation gate. Do not use it for ordinary work in the current checkout, for task-specific temporary directories that are not worktrees, or for branch operations that involve no worktree.
metadata:
    internal: true
---

# Git worktrees

Apply every applicable global and project instruction. The global “Temporary files” policy owns the `.agent-<name>` namespace that worktrees share with ordinary task directories, and the global “Concurrent work” rule owns preserving existing changes and avoiding another agent’s write scope. This skill owns deciding, creating, operating, and dismantling isolation.

## Decide whether to isolate

- **Create only for:**
    - An explicit user request for an isolated worktree.
    - Another active agent with an overlapping write scope.
    - A task requiring isolated branch, dependency, build, or test state.
    - A broad or high-risk change that materially benefits from independent rollback and has a clear integration plan.
- **Do not isolate merely for:** A dirty repository, possible concurrent activity, or any task that modifies repository files. Keep follow-up edits to the same uncommitted task in its existing checkout.
- **Historical inspection:** Inspect revisions through Git without materializing them. Materialize a revision only when a filesystem-based tool requires it, applying the criteria above when isolation is necessary.

## Create a worktree

- **Pairing:** When a worktree is required, use a unique, filesystem-safe `<name>` containing a task slug and short unique suffix without path separators. Create it with `git worktree add -b agent/<name> .agent-<name> <start-point>`. Do not use `--detach`. Keep every worktree paired with its `agent/<name>` branch, and move the worktree and rename the branch together when changing `<name>`.

## Operate inside a worktree

- **Command location:** For Git commands intended to operate inside a registered `.agent-<name>` worktree, use `git -C .agent-<name> …` rather than relying only on process or tool working directory. Verify registration with `git --no-pager worktree list --porcelain`. Do not treat an ordinary task-specific `.agent-<name>` directory as a worktree. Run worktree-administration commands from the primary checkout unless the command requires another location.

## Dismantle a worktree

- **Destructive-operation gate:** Before moving or removing a worktree, or force-renaming or deleting its branch, inspect the affected worktree status and verify that its changes are integrated or explicitly abandoned.
    - Remove worktrees with `git worktree remove`. Use one or two exact `-f` or `--force` options only after verification when an unclean or locked worktree requires them. Verify afterward that the corresponding directory is gone, and inspect it rather than deleting it recursively if it remains.
    - After removing the worktree, first delete its branch with `git branch -d agent/<name>`. Use `git branch -D agent/<name>` only when Git refuses because the branch is unmerged and the preceding verification established that its changes are integrated or explicitly abandoned.
