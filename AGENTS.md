# Agent Instructions

## Overview

- This repository is the home of all my dotfiles actively used across multiple Apple Silicon–based Macs (also referred to as `domfiles`).

## Code

- Always consider `.config/fish/local.fish` an active part of domfiles if it exists.
    - Always include `local.fish` in any analysis or execution.
    - Do not report `.gitignore` including `local.fish`.
    - Do not suggest adding additional documentation for `local.fish`.
- Do not analyze the contents of `bin/git-diff-highlight` (it’s a symlink).
- Do not read or analyze `.config/npm/user.npmrc` (it contains secrets).
- Do not report empty config files.
- Always use `git mv` when renaming files already committed to this repository.
- Report any cases that would tie this repository to a fixed filesystem location.
    - Do not report `$HOME/*` paths, system paths, or vendor paths.
    - Do not report symlinks created via `domfiles sync`.
    - Do not report `.config/fish/fish_variables`.
    - Do not report documentation.
- Run project-local commands through `pnpm exec`, `pnpm`’s implicit executable form (`pnpm <executable> ...`), or an existing `pnpm` script instead of invoking executables from `node_modules/.bin` directly.
    - Exempt the external formatter command in `.zed/settings.json` from this requirement.

### Skills

- Treat every skill maintained in this repository as a living document.
    - After executing a task using a skill, suggest a concrete edit when the execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
    - Do not edit the skill unless explicitly asked.

### Shell Scripts

- Always assume that `fish` is the default shell.
- Always ensure that any shell scripts not written in `fish` strictly conform to POSIX `sh`.
    - Apply strict mode when applicable.
- Ensure all POSIX shell scripts source `domlib`.
    - Exempt `.vite-hooks` scripts from this requirement.
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
    - Exempt loops that iterate over a fixed list of literal filenames.
- Avoid bare pipelines when feeding command output into a loop. Use command substitution for better detection of potential upstream failures.
    - Exempt `printf` output piped into `while` from this requirement.
    - Exempt any `domlib` command output piped into `while` from this requirement.
- Always prefer the variable name `param` over `arg`.
    - Exempt `fish`’s built-in `$argv` variable from this rule.
- Do not report use of `eval` unless it poses a security risk.
- Always report when `find` uses `-maxdepth` in any position other than immediately after the search path.

### `domfiles` Scripts

- Evaluate the setup instructions in the `README` and all `domfiles` scripts against the supported environment documented in `.agents/PROJECT.md`.
    - Always report any commands that may cause issues in that environment.
- Always assume this repository is updated via `domfiles sync`.
    - Do not report `domfiles sync` overwriting initial state.

### Zed Config

- Keep `.config/zed/settings.json` free of entries that only restate Zed defaults.
    - Exempt `"tab_size": 4` from this requirement.
    - Keep `.zed/settings.json` free of entries that only restate `.config/zed/settings.json` or Zed defaults.
- Keep order-independent arrays in Zed configuration alphabetized by value or, for object entries, by the value of their identifying field.
- Treat Zed agent permissions as layered security boundaries.
    - Preserve `agent.tool_permissions.default` as `allow`.
    - Treat ordinary package-manager workflows as intentional allowances. Continue to require confirmation for package runners that can download and execute arbitrary code.
    - Allow Docker inspection operations without confirmation. Require confirmation for operations that execute workloads or create, modify, or remove Docker state.
    - Keep `agent.sandbox_permissions.network_hosts` aligned with `agent.tool_permissions.tools.fetch.always_allow`.
    - Treat `*.domain.name` and `domain.name` as distinct `network_hosts` entries. Preserve both when access to the apex domain and its subdomains is intended.
    - Prefer wildcard domain allowances when subdomains are involved. Include the apex domain only when it is actually used.
    - Restrict automatically allowed fetch URL patterns to `https://` and anchor each pattern at the hostname boundary.
- Keep terminal permission patterns concise and consistent.
    - Keep the consolidated general `terminal.always_allow` pattern first, followed by the shared `--(?:help|version)` pattern. Alphabetize the remaining patterns by command family.
    - Alphabetize executable, command, and subcommand alternatives within consolidated patterns when their grammar permits.
    - Consolidate variants within the same command family. Keep unrelated command families separate.
        - Keep informational forms in an existing command-family pattern instead of duplicating the executable in the shared `--(?:help|version)` pattern.
    - Prefer explicit alternatives over optional fragments when consolidating distinct executable names.
    - Keep command-specific prefixes and wrappers out of the consolidated general and shared `--(?:help|version)` patterns. Define family-specific allowances separately. Account for approved prefixes and wrappers in applicable allowances and confirmation overrides.
        - Account for the optional `-C <path>`, `--no-optional-locks`, and `--no-pager` global options before every Git subcommand.
        - Apply the optional `GIT_EDITOR=true`, `GIT_PAGER=cat`, `MANPAGER=cat`, and `PAGER=cat` prefixes to every Git terminal pattern.
        - Apply optional `HOMEBREW_NO_*` prefixes with the fixed value `1` to every Homebrew terminal pattern.
    - Prefer literal spaces over whitespace character classes.
    - Treat signaling explicit numeric process IDs as an intentional allowance for polling and stopping processes associated with the current task. Do not extend this allowance to process names or patterns.
    - For mixed-purpose utilities and interpreters, prefer positive allowlists of non-mutating forms. Use a broad allowance with `terminal.always_confirm` only when every hazardous form can be matched reliably. Otherwise, preserve default confirmation.
    - Use `terminal.always_confirm` to override broader `terminal.always_allow` entries for hazardous argument forms, including code-execution hooks, package runners, destructive operations, force flags, and commands that uninstall the invoked tool itself. Account for global options, combined short flags, and accepted long-option abbreviations.
    - Do not report overlaps between `terminal.always_allow` and `terminal.always_confirm` when `terminal.always_confirm` acts as a safety override.

## General

- Always reference the relevant `AGENTS.md` line number when reporting a violation.
- Never edit this file unless explicitly asked.
- Never override or alter my input unless explicitly asked.
- Consult `.agents/PROJECT.md` for non-obvious project rationale, compatibility constraints, and maintenance decisions.
    - Document newly discovered durable project knowledge there.
    - Keep entries alphabetized when their order is irrelevant.

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
- Always execute shorthand commands exactly as defined below.
- Report findings only when supported by concrete repository evidence.
    - Do not report speculative findings or preference-only alternatives.
    - Assign a unique number to each finding when it is first reported.
    - Preserve finding numbers in all subsequent reports.

### Audit

- Unless the user explicitly modifies the scope, review the entire repository for redundancies, inconsistencies, typos, and potential structural or type issues.
    - After reading all applicable `AGENTS.md` files, determine the complete audit scope before reviewing any other repository contents.
    - Start with `git`-tracked paths, then apply explicit user scope modifiers and any inclusions, exclusions, and exemptions defined by `AGENTS.md`.
    - Exclude symbolic links without reading or resolving their targets.
    - Do not report findings outside the resulting audit scope.
    - Do not run linters or formatters as part of the analysis.
- Ensure there is no dead or unused code.
- Treat the audit as one continuous task.
    - Do not stop after reporting individual findings.
    - Continue until the entire audit scope has been reviewed.
    - Report all findings only after completing the audit.
- Do not modify any files.
    - Defer any required `.agents/PROJECT.md` updates and report them as follow-up work.
- Base all findings solely on the current repository contents.
    - Never speculate about intent or hypothetical implementations.
- Number every finding and keep the numbering stable across follow-up requests.
- Ignore paths outside the audit scope.
- Include comments in the analysis.

### Review

- Review the commit matching the provided hash.
    - If no hash is given, review the most recent commit on the current branch.
- Report any regressions the changes may introduce.
- Report any cases where new code reimplements behavior already available in the language, standard library, or existing shared utilities in this repository.
- Ensure all changes align with the latest version of `AGENTS.md`.
- Perform this task without making edits.

### Verify

- Re-read `AGENTS.md` and all reported files to confirm whether reported issues remain relevant.
    - Ensure that all findings align with the latest version of `AGENTS.md`.
- Classify each previously reported finding as resolved, intentional, or unresolved.
    - Exclude resolved findings from future reports.
    - Exclude intentional findings from future reports unless the relevant code or `AGENTS.md` changes.
- Report only unresolved findings that still apply.
