---
name: domfiles-shell-scripts
description: Edit, review, audit, and diagnose Fish and POSIX shell scripts in domfiles. Use this skill whenever the resolved task scope includes shell code—including `domlib`, Fish configuration, `bin` scripts, and `.vite-hooks`—but not merely because the task runs terminal commands.
---

# Domfiles shell scripts

Use this skill whenever the resolved task scope includes Fish or POSIX shell code, including extensionless files identified by their shebang or syntax. Do not infer the language from the filename alone.

Use this skill as the canonical source for shell-script policy and workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions.

## Choose the workflow

- For a change, investigate the affected scripts and cross-file invariants, make the smallest applicable edit, and use the change-validation workflow below.
- For an audit or review, keep the task read-only. Apply the policy below, then follow the applicable reporting workflow without formatting or modifying files.

## Investigate the task

1. Read every applicable `AGENTS.md` file, inspect the relevant shell code, and review the existing diff.
2. Identify whether each in-scope file uses Fish or POSIX `sh` from its shebang and syntax rather than its extension alone.
3. When `domlib` or `.config/fish/config.fish` is relevant, inspect both files before evaluating shared variables or functions.
4. Search all in-scope call sites before reporting a `domlib` function or variable as unused.

## Check supported-environment compatibility

- Evaluate every in-scope `domfiles` shell script’s interpreter, external commands, options, `PATH`, architecture, and default-shell assumptions against the supported environment documented in `.agents/PROJECT.md`.
- Judge each requirement at its intended lifecycle stage—fresh bootstrap, synchronization, post-sync runtime, or development—and account for prerequisites provisioned earlier by `domfiles sync`.

## Write concise shell prose

- Keep script comments and user-facing strings passed to `__print*` concise, neutral, and consistent.
    - Use sentence-case imperative voice for action and section comments.
    - In explanatory comments, describe stable intent or policy rather than restating control flow; do not require comments to enumerate conditional behavior that is clear from adjacent code.
    - Phrase diagnostics as direct descriptions of the outcome or constraint, using consistent terminology for the same condition.
    - Avoid first-person and subjective wording.
    - Omit final punctuation from prose.
    - Treat standalone headings and status labels as labels rather than sentences; allow sentence case or title case, and do not require imperative voice.

## Keep POSIX scripts portable

- Ensure every shell script not written in Fish strictly conforms to POSIX `sh`, applying strict mode when applicable.
- Ensure every POSIX shell entrypoint sources `domlib`; exempt `.vite-hooks` scripts. Treat `bin/domlib` as the shared library rather than an entrypoint, and keep strict mode there so sourced scripts inherit it.

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

## Write robust shell control flow

- Set `IFS` locally when iterating over filenames or command output; exempt loops over a fixed list of literal filenames.
- Avoid bare pipelines when feeding command output into a loop. Use command substitution for better detection of potential upstream failures.
    - Exempt `printf` output piped into `while`.
    - Exempt `domlib` command output piped into `while`.
- In POSIX `sh` strict mode, when an optional command emits either a usable nonempty value or no output on failure, scope `|| true` inside the command substitution before testing the quoted result. This keeps the expected failure from triggering `set -e` while limiting suppression to that command:

    ```sh
    value="$(optional-command || true)"
    [ -z "$value" ] && value="$(fallback-command)"
    ```

    - Use exit-status control flow instead when successful empty output or partial output on failure must remain distinguishable.

- Prefer the variable name `param` over `arg`; exempt Fish’s built-in `$argv` variable.
- Do not report `eval` unless it poses a security risk.
- Report `find` commands that place `-maxdepth` anywhere other than immediately after the search path.

## Validate a change

After editing:

1. Syntax-check each changed Fish file with `fish --no-execute <file>` and each changed POSIX shell file with `sh -n <file>`.
2. Run the applicable existing `pnpm run lint:fish` or `pnpm run lint:sh` workflow; inspect its script first if mutating behavior is unclear.
3. Check changed-file formatting with `pnpm exec prettier --check <changed-files>`.
4. Verify every applicable policy invariant above, including `domlib` ordering, usage, and `$DOMFILES_*` parity when relevant.
5. Run `git --no-pager diff --check` and, when task-owned changes are staged, `git --no-pager diff --cached --check`. Inspect task-owned unstaged and staged diffs, inspect task-owned untracked files directly without staging them, and review the final status without altering concurrent changes.

## Validate a read-only audit or review

Without editing, formatting, or running a prohibited linter:

1. Inspect every in-scope shell file and applicable cross-file invariant.
2. Verify the policy above against the current contents.
3. Follow the applicable audit or review reporting procedure and identify anything that could not be verified.
