# Agent repository permissions

## Apply the agent-directory allowance policy

- Treat the project-relative `.agent-<name>` namespace defined by the [global temporary-file policy](../../../../.config/zed/AGENTS.md#temporary-files) as a standing user-approved scope for operation-specific namespace-bounded terminal allowances. This approval establishes eligibility for a scoped variant, not authorization for the command family itself or blanket trust in every command or directory entry.
- Keep each variant within the requested operation family. Treat it as eligible for automatic allowance only when every effect that makes the general form confirmable is a filesystem or repository mutation contained within the task-owned namespace and the normalized command exposes every behavior-bearing path for lexical validation. Require each such path to be an explicit traversal-free project-relative `.agent-<name>` root or descendant.
- Keep archive extraction and comparable input-directed mutation confirmable when archive members, links, manifests, other input data, or rules can determine an output path or effect absent from the normalized command. An explicit `.agent-<name>` destination does not qualify by itself. Require operation-specific evidence that every created or modified path and link is exposed or otherwise contained, or a more specific policy that explicitly accepts the residual input-controlled effect.
- The agent-directory namespace alone does not relax confirmation or denial for effects that path scoping does not contain, including arbitrary code execution, ambient credential or configuration access, authentication and signing, external helper selection, network or remote activity, process or system state, direct symbolic-link creation, and paths that are absolute, traversal-bearing, unresolved, or outside the namespace. Only a more specific policy may explicitly accept such a residual effect. Zed’s sandbox, sensitive-path, and symlink-escape checks remain additional boundaries.
- Classify plain task directories, top-level registered worktrees, and strict-descendant fixture repositories separately. Apply the more specific repository policies below when Git or worktree behavior is in scope.
- Do not infer agent-directory scope from the terminal’s current working directory because permission matching does not expose it. When a safe grammar cannot carry every required path explicitly, leave the operation confirmable.

## Maintain agent worktree permissions

- Allow `git worktree prune` automatically only in dry-run forms. Keep actual pruning, out-of-namespace paths or branches, remote operations, shell globs, path traversal, parent-removing `rmdir -p`, and broader deletion mechanisms confirmable.
- Keep forced worktree and branch operations constrained to their respective namespaces, and keep `--detach` confirmable.
- Allow commits inside agent worktrees to stage tracked changes with `-a` or `--all` and to amend the current commit through bounded noninteractive `-m` or `--no-edit` forms only with the exact `-c commit.gpgsign=false` guard. Preserve its supported placement before or after the worktree’s `-C` option. Keep editor-driven amendments and broader history rewriting confirmable.
- For terminal allowances whose safety depends on top-level agent-worktree scope, require the normalized command to carry an explicit project-relative `.agent-<name>` operand—for example, as Git’s `-C` path. Do not infer that scope from the terminal’s current working directory.
- Keep native-tool and terminal permission patterns synchronized with the [worktree convention](../../../../skills/git-worktrees/SKILL.md).
- Leave direct symbolic-link creation confirmable. Treat existing worktree-internal symlinks as user-managed repository state when native path operations are automatically allowed.
- See [Zed worktree permission coupling](../../../PROJECT.md#zed-worktree-permission-coupling) for rationale.
- Keep stored Git continuation operations subject to the [Git continuation policy](git-permissions.md#apply-the-git-permission-policy). An agent-worktree path contains repository mutation but does not contain hidden sequencer commands, hooks, or configuration-driven execution.
- Use native `move_path` for strict descendant moves within agent worktrees and `git worktree move` for top-level worktree moves. Leave terminal `mv` confirmable.

## Maintain disposable fixture repository permissions

- Within the [documented fixture repository scope](../../../PROJECT.md#zed-fixture-repository-permissions), permit audited local Git forms for fixture setup, history construction, ref management, teardown, and working-tree changes.
- Require an explicit traversal-free strict-descendant `-C` path and a positive command grammar. Leave blanket trailing-argument allowances confirmable.
- Keep cross-boundary path options, external-helper selection, network subcommands, signing requests, submodule-recursion options, and unrestricted configuration outside the scoped allowance. Preserve higher-precedence denials for credential-disclosing or authentication-capability forms, and leave other credential or configuration source selection confirmable. Permit remote metadata changes only when the form does not contact a remote.
- Apply the [documented residual trust boundary](../../../PROJECT.md#zed-fixture-repository-permissions).
- Keep commands whose `-C` operand is the top-level `.agent-<name>` worktree governed by the narrower worktree policy above. Descendant rules intentionally accept Git’s upward discovery as part of task-owned state.

## Validate agent repository permissions

Validate in-scope patterns against intended namespace-bounded task-directory, worktree, and fixture operations plus near misses involving ordinary, absolute, top-level, out-of-namespace, or traversal-bearing paths. Include archive- or input-controlled outputs and links, broader deletion or history-rewriting forms, credential or configuration access, detached worktrees, direct symbolic-link creation, effects not contained by path operands, external helpers, and network or remote operations.
