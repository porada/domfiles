# Agent repository permissions

## Maintain agent worktree permissions

- Allow `git worktree prune` automatically only in dry-run forms. Keep actual pruning, out-of-namespace paths or branches, remote operations, shell globs, path traversal, parent-removing `rmdir -p`, and broader deletion mechanisms confirmable.
- Keep forced worktree and branch operations constrained to their respective namespaces, and keep `--detach` confirmable.
- Allow commits inside agent worktrees to stage tracked changes with `-a` or `--all` and to amend the current commit through bounded noninteractive `-m` or `--no-edit` forms only with the exact `-c commit.gpgsign=false` guard. Preserve its supported placement before or after the worktree’s `-C` option. Keep editor-driven amendments and broader history rewriting confirmable.
- For terminal allowances whose safety depends on top-level agent-worktree scope, require the normalized command to carry an explicit project-relative `.agent-<name>` operand—for example, as Git’s `-C` path. Do not infer that scope from the terminal’s current working directory.
- Keep native-tool and terminal permission patterns synchronized with the [global worktree convention](../../../../.config/zed/AGENTS.md#git-worktrees).
- Leave direct symbolic-link creation confirmable. Treat existing worktree-internal symlinks as user-managed repository state when native path operations are automatically allowed.
- See [Zed worktree permission coupling](../../../PROJECT.md#zed-worktree-permission-coupling) for rationale.
- Use native `move_path` for strict descendant moves within agent worktrees and `git worktree move` for top-level worktree moves. Leave terminal `mv` confirmable.

## Maintain disposable fixture repository permissions

- Within the [documented fixture repository scope](../../../PROJECT.md#zed-fixture-repository-permissions), permit audited local Git forms for fixture setup, history construction, ref management, teardown, and working-tree changes.
- Require an explicit traversal-free strict-descendant `-C` path and a positive command grammar. Leave blanket trailing-argument allowances confirmable.
- Keep cross-boundary path options, explicit credential access, external-helper selection, network subcommands, signing requests, submodule-recursion options, and unrestricted configuration confirmable. Permit remote metadata changes only when the form does not contact a remote.
- Apply the [documented residual trust boundary](../../../PROJECT.md#zed-fixture-repository-permissions).
- Keep commands whose `-C` operand is the top-level `.agent-<name>` worktree governed by the narrower worktree policy above. Descendant rules intentionally accept Git’s upward discovery as part of task-owned state.

## Validate agent repository permissions

Validate in-scope patterns against intended namespace-bounded worktree and fixture operations plus near misses involving top-level or out-of-namespace paths, traversal, detached worktrees, remote operations, direct symbolic-link creation, and broader deletion or history-rewriting forms.
