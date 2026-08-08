---
name: domfiles-shell-scripts
description: Edit, review, audit, and diagnose Fish and POSIX shell scripts in domfiles. Use this skill whenever the resolved task scope includes shell code—including `domlib`, Fish configuration, `bin` scripts, and `.vite-hooks`—or evaluates whether a Git helper should be a plain alias or a `bin/git-*` script. Do not use it merely because the task runs terminal commands.
---

# Domfiles shell scripts

Use this skill as the canonical source for shell-script policy and workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions.

## Choose the workflow

- For an explicit change, including a request that also uses review or audit language, investigate the affected scripts and cross-file invariants, make the smallest applicable edit, and use the change-validation workflow below.
- For a standalone audit, follow the [repository audit process](../domfiles-repository-audit/SKILL.md) and apply the shell-specific checks below.
- For a standalone review, keep the task read-only. Apply the policy below and use the read-only validation workflow without formatting or modifying files.

## Investigate the task

1. Read every applicable `AGENTS.md` file, inspect the relevant shell code, and review the existing diff.
2. Identify whether each in-scope file uses Fish or POSIX `sh` from its shebang and syntax rather than its extension alone.
3. When `domlib` or `.config/fish/config.fish` is relevant, inspect both files before evaluating shared variables or functions.
4. Search repository-wide call sites before reporting a `domlib` function or variable as unused. More than one call site is sufficient reuse and must not be reported on usage-count grounds.
5. When `.config/fish/local.fish` is in the resolved scope, inspect or validate it directly. Do not report its [documented sourcing behavior](../../PROJECT.md#fish-local-configuration) as hidden diagnostics.

## Check supported-environment compatibility

- Evaluate every in-scope `domfiles` shell script’s interpreter, external commands, options, `PATH`, architecture, and default-shell assumptions against the supported environment documented in `.agents/PROJECT.md`.
- Judge each requirement at its intended lifecycle stage—fresh bootstrap, synchronization, post-sync runtime, or development—and account for prerequisites provisioned earlier by `domfiles sync`.

## Write concise shell prose

- Keep script comments and user-facing strings passed to `__print*` concise, neutral, and consistent.
    - Use sentence-case imperative voice for action and section comments.
    - In explanatory comments, describe stable intent or policy rather than restating control flow. Do not require comments to enumerate conditional behavior that is clear from adjacent code.
    - Phrase diagnostics as direct descriptions of the outcome or constraint, using consistent terminology for the same condition.
    - Avoid first-person and subjective wording.
    - Omit final punctuation from prose.
    - Treat standalone headings and status labels as labels rather than sentences. Allow sentence case or title case, and do not require imperative voice.

## Keep POSIX scripts portable

- Ensure every shell script not written in Fish strictly conforms to POSIX `sh`, applying strict mode when applicable.
- Ensure every POSIX shell entrypoint sources `domlib`. Exempt `.vite-hooks` scripts. Treat `bin/domlib` as the shared library rather than an entrypoint, and keep strict mode there so sourced scripts inherit it.

## Quote shell strings appropriately

- Use double quotes for strings where expansion may occur.
- Use single quotes for literal strings containing characters that would otherwise require escaping.
- In POSIX `sh`, never quote `$#` when used in a condition or `$?` when passed to `exit`.

## Maintain `domlib`

- Keep all functions defined in `domlib` alphabetized in natural order.
- Keep the set of `$DOMFILES_*` variables defined in `domlib` and `.config/fish/config.fish` in sync, with exactly matching names.
    - Exempt `$DOMFILES_DEFAULT_IFS`, `$DOMFILES_SSH_KEY`, and `$DOMFILES_VIM_PLUG`.
- Report unused functions or variables defined in `domlib`.
    - Do not treat variables as unused when they exist solely to maintain parity with `.config/fish/config.fish`.
- Report every POSIX shell function prefixed with `__` when it is defined outside `domlib`.

## Choose Git helper form

- Before adding or reviewing a `bin/git-*` entrypoint, inspect `.config/.gitconfig` and determine whether a plain Git alias preserves the required behavior.
- Prefer a plain Git alias when the helper invokes one Git subcommand with fixed options and relies on Git’s normal argument forwarding.
- Keep a script when the behavior requires shell control flow, dynamic values, safety checks, external commands, or shared `domlib` behavior.
- If a Git alias would require the `!` shell-command form, implement it as a `bin/git-*` script instead. Never define shell commands inside `.config/.gitconfig` aliases.
- Do not retain a script solely for custom argument-count validation unless strict arity is required behavior.

## Evaluate duplication and reuse

- Keep the Fish and POSIX discovery and lint wrappers separate. Do not report their shared traversal, argument handling, or heading setup as duplication. See [shell wrapper duplication](../../PROJECT.md#shell-wrapper-duplication) for rationale.
- Consolidate shell implementations when they duplicate a substantial, virtually identical behavior pipeline that must remain aligned.
- Do not report `__string_*` helpers or equivalent inline string operations as reimplementations. See [string helper reuse](../../PROJECT.md#string-helper-reuse) for rationale.

## Write robust shell control flow

- Set `IFS` locally when iterating over filenames or command output. Exempt loops over a fixed list of literal filenames.
- Avoid bare pipelines when feeding command output into a loop. Use command substitution for better detection of potential upstream failures.
    - Exempt `printf` output piped into `while`.
    - Exempt `domlib` command output piped into `while`.
- In POSIX `sh` strict mode, when an optional command emits either a usable nonempty value or no output on failure, scope `|| true` inside the command substitution before testing the quoted result. This keeps the expected failure from triggering `set -e` while limiting suppression to that command:

```sh
value="$(optional-command || true)"
[ -z "$value" ] && value="$(fallback-command)"
```

- Use exit-status control flow instead when successful empty output or partial output on failure must remain distinguishable.

- Prefer the variable name `param` over `arg`. Exempt Fish’s built-in `$argv` variable.
- Do not report `eval` unless it poses a security risk.
- Report `find` commands that place `-maxdepth` anywhere other than immediately after the search path.

## Validate a change

After editing, use the narrowest applicable validation scope:

1. Pass changed shell paths explicitly to the lint wrappers. Omit paths only when repository-wide validation is intended because no-argument discovery includes tracked and untracked non-ignored shell files. Fish discovery also includes `.config/fish/local.fish` when it exists.
2. For Fish, run `pnpm run lint:fish <changed-fish-files>`. The wrapper already runs `fish --no-execute`, so do not repeat that check.
3. For POSIX shell, run `sh -n <file>` for each changed file and `pnpm run lint:sh <changed-posix-files>`. The wrapper supplies the complementary ShellCheck analysis.
4. Check formatting only for changed `.fish` files with `pnpm exec prettier --check <changed-fish-files>`. Do not pass POSIX or extensionless Fish files because the configured plugin registers only the `.fish` extension.
5. Verify every applicable policy invariant above, including `domlib` ordering, usage, and `$DOMFILES_*` parity when relevant. Run `git --no-pager diff --check` and, when task-owned changes are staged, `git --no-pager diff --cached --check`. Inspect task-owned unstaged and staged diffs, inspect task-owned untracked files directly without staging them, and review the final status without altering concurrent changes.

## Validate a shell audit or review

1. Inspect every in-scope shell file and applicable cross-file invariant.
2. Verify the policy above against the current contents.
3. Identify anything that could not be verified.
