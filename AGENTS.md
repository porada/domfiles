# Agent Instructions

## Overview

- This repository is the home of all my dotfiles actively used across multiple Apple Silicon–based Macs (also referred to as `domfiles`).

### Code

- Always consider `.config/fish/local.fish` an active part of domfiles if it exists.
    - Always include `local.fish` in any analysis or execution.
    - Do not report `.gitignore` including `local.fish`.
    - Do not suggest adding additional documentation for `local.fish`.
- Do not analyze the contents of `bin/git-diff-highlight` (it’s a symlink).
- Do not read or analyze `.config/npm/user.npmrc` (it contains secrets).
- Do not report empty config files.
- Report any cases that would tie this repository to a fixed filesystem location.
    - Do not report `$HOME/*` paths, system paths, or vendor paths.
    - Do not report symlinks created via `domfiles sync`.
    - Do not report `.config/fish/fish_variables`.
    - Do not report documentation.

### Shell Scripting

- Always assume that `fish` is the default shell.
    - Run all shell terminal commands using `fish` syntax when `fish` is available.
- Always ensure that any shell scripts not written in `fish` strictly conform to POSIX `sh`.
    - Apply strict mode when applicable.
- Ensure all POSIX shell scripts source `domlib`.
    - Exempt `.husky` scripts from this requirement.
    - Always keep all functions defined in `domlib` alphabetized in natural order.
    - Always keep the set of `$DOMFILES_*` variables defined in `domlib` and `.config/fish/config.fish` in sync.
        - Variable names must match exactly in both locations.
        - Exempt `$DOMFILES_DEFAULT_IFS`, `$DOMFILES_SSH_KEY`, and `$DOMFILES_VIM_PLUG` from this requirement.
    - Always report unused functions or variables defined in `domlib`.
        - Do not treat variables as unused when they exist solely to maintain parity with `.config/fish/config.fish`.
    - Always report any POSIX shell functions prefixed with `__` when they are defined outside of `domlib`.
- Always ensure strings are quoted appropriately:
    - Use double quotes for any string where expansion may occur.
    - Use single quotes for literal strings containing characters that would otherwise require escaping.
    - Never quote `$#` when used in a condition.
    - Never quote `$?` when passed to `exit`.
- Always set `IFS` locally when iterating over filenames or command output.
- Avoid bare pipelines when feeding command output into a loop. Use command substitution for better detection of potential upstream failures.
    - Exempt `printf` output piped into `while` from this requirement.
    - Exempt any `domlib` command output piped into `while` from this requirement.
- Always prefer the variable name `param` over `arg`.
    - Exempt `fish`’s built-in `$argv` variable from this rule.
- Do not report use of `eval` unless it poses a security risk.
- Always report when `find` uses `-maxdepth` in any position other than immediately after the search path.

### `domfiles` Scripts

- Always assume the setup instructions in the `README` run on a fresh macOS install (version 26 or newer) with Command Line Tools and Homebrew installed.
    - Always report any commands that may cause issues in that environment.
- Always assume this repository is updated via `domfiles sync`.
    - Do not report `domfiles sync` overwriting initial state.

## General

- Never read, analyze, or report files that do not belong to this repository.
    - Exempt skill files from this rule.
- Always stay alert to any issues outlined in this document, even when performing unrelated tasks.
    - Report all discovered issues, but do not fix them without my confirmation.
- Never edit this file.
- Never override or alter my input unless explicitly asked.
- If you cannot verify something directly, admit that you cannot verify it, that you do not have access to the information, or that your knowledge base does not contain it.
- Enforce strict consistency across all modules, configuration files, and naming conventions.
- Do not report inconsistencies automatically handled by the formatter or linter.

## Style

- Always report when `AGENTS.md` contains typos or any inconsistencies with the rules defined in this section.
- Enclose all tokens and code fragments in `backticks` when quoting them in strings or comments.
- Follow the format below:

```ts
/**
 * Tests for the `Icon` component
 */
describe('`Icon` component with a custom `ASSET_PATH`', () => {
    process.env.ASSET_PATH = '/assets';

    test('accepts `true` as the `name` prop', () => {
        // …
    });

    test('returns `undefined` if the `name` prop isn’t provided', () => {
        // …
    });
});
```

## Shorthand Commands

- Shorthand commands are high-level task macros that define complete, self-contained procedures.
- Always execute the task exactly as defined below whenever I say its name.

### Audit

- Review the entire repository for redundancies, inconsistencies, typos, and potential structural or type issues.
    - Perform the review without making edits.
- Ensure there is no dead or unused code.
- Treat the audit as one continuous task.
    - Do not wait for my confirmation between steps.
    - Report all findings at the end.
    - Report only issues that may require fixing.

### Review

- Review the commit matching the provided hash.
    - If no hash is given, review the most recent commit on the current branch.
    - Perform the review without making edits.
- Report any regressions the changes may introduce.
- Report any cases where new code reimplements behavior already available in the language, standard library, or existing shared utilities in this repository.
- Ensure all changes comply with the latest version of `AGENTS.md`.

### Verify

- Re-read `AGENTS.md` and all reported files to confirm whether reported issues remain relevant.
    - Ensure that all findings align with the latest version of `AGENTS.md`.
- Mark resolved issues as addressed and exclude them from future reports.
- Highlight any issues that persist after the latest edits.
