---
name: domfiles-shell-integration
description: |-
    Use this skill whenever the resolved task scope includes shell code—including `domlib`, Fish configuration, `bin` scripts, and `.hooks`—or adds or reconsiders a command entrypoint in this repository, including whether a Git helper should be a plain alias or a `bin/git-*` script.

    Do not use it merely because the task runs terminal commands.

metadata:
    internal: true
---

# Domfiles Shell Integration

Use this skill as the canonical source for domfiles-specific shell integration, invariants, and validation.

For every Fish target, load `fish-shell-scripting` for portable language policy and workflow. For every POSIX shell target, load `posix-shell-scripting`. Apply this skill as the narrower domfiles layer when either language skill governs the same task.

## Choose the Workflow

For a standalone audit in either language, follow the applicable language skill’s audit workflow, keep the task read-only, and apply the domfiles-specific checks in this skill and its routed references.

## Investigate the Task

1. Classify each in-scope shell file from its hashbang and syntax rather than its extension alone, then apply `fish-shell-scripting` or `posix-shell-scripting`.
2. When changes to `.config/fish/config.fish` alter the interactive guard or which tracked alias and color files it sources, keep [Fish interactive configuration](../../PROJECT.md#fish-interactive-configuration) aligned.
3. When changes to `.config/fish/config.fish` alter machine-local sourcing, keep [Fish local configuration](../../PROJECT.md#fish-local-configuration) aligned.
4. Do not report `.config/fish/local.fish`’s [documented sourcing behavior](../../PROJECT.md#fish-local-configuration) as hidden diagnostics.
5. Evaluate `.config/fish/functions/clone.fish` against the [Fish `clone` argument contract](../../PROJECT.md#fish-clone-argument-contract). Do not report the absence of Git option parsing, option rejection, or reliable follow-up directory changes for unsupported option-bearing invocations.

For every non-Fish shell target and whenever the task touches `domlib`, a Fish `__domfiles_*` helper, shared `$DOMFILES_*` state, or command suppression, follow [`domlib` integration](references/domlib-integration.md).

## Check Supported-Environment Compatibility

- Evaluate every in-scope `domfiles` shell script’s interpreter, external commands, options, `PATH`, architecture, and default-shell assumptions against the [supported environment](../../PROJECT.md#supported-environment).
- Judge each requirement at its intended lifecycle stage—fresh bootstrap, synchronization, post-sync runtime, or development—and account for prerequisites provisioned earlier by `domfiles sync`.
- Treat `domfiles dependencies` as the user-facing readiness check defined by [dependency status labels](../../PROJECT.md#dependency-status-labels). Add a row only for an established user-facing synchronization or runtime contract. Agent-only use or installation by synchronization alone does not qualify a dependency.

## Apply Domfiles Shell Wording Constraints

Use the applicable language skill for semantic requirements and composition with `human-facing-writing`. Apply these domfiles-specific constraints after that workflow:

- Avoid first-person and subjective wording.
- Omit final punctuation from script comments and user-facing strings passed to `__print*`.
- Treat standalone headings and status labels as labels rather than sentences. Allow sentence case or title case, and do not require imperative voice.
- Use sentence-case imperative voice for action and section comments.

## Choose Command Form and Location

Do not report an existing command solely because another supported form could express it, except when [Git helper form](references/command-form-and-location.md#choose-git-helper-form) applies.

Before adding a command entrypoint or explicitly reconsidering an existing command’s form or location, follow [command form and location](references/command-form-and-location.md).

Before reviewing a `bin/git-*` entrypoint, follow [Git helper form](references/command-form-and-location.md#choose-git-helper-form).

## Evaluate Duplication and Reuse

- Do not report the language-specific `bin/domfiles-dev-lint-*` entrypoints as duplication merely because each retains its own default scope and lint command. Shared discovery and execution belong in `domlib`. See [development lint wrapper architecture](../../PROJECT.md#development-lint-wrapper-architecture) for rationale.
- Consolidate shell implementations when they duplicate a substantial, virtually identical behavior pipeline that must remain aligned.
- Do not report `__string_*` helpers or equivalent inline string operations as reimplementations. See [string helper reuse](../../PROJECT.md#string-helper-reuse) for rationale.

## Apply Domfiles POSIX Conventions

Report `find` commands that place `-maxdepth` anywhere other than immediately after the search path.

## Validate a Change

After editing, use the narrowest applicable validation scope:

1. Pass changed paths explicitly to the matching lint wrapper. Omit paths only when repository-wide validation is intended. Explicit paths bypass default discovery. With no paths, wrappers discover tracked files and non-ignored untracked files. The Fish, JSON, and TOML wrappers respectively restrict that inventory to `*.fish`, `*.json`, and `*.toml` files. The POSIX wrapper uses `*.sh`, `.hooks/*`, and `bin/*` to include extensionless entrypoints. Every wrapper skips non-files and symlinks.
2. For Fish, run `pnpm run lint:fish <changed-fish-files>`. Include `.config/fish/local.fish` explicitly when it exists. The wrapper already runs `fish --no-execute`, so do not repeat that check.
3. For POSIX shell, run `sh -n -- <file>` for each changed file and `pnpm run lint:sh <changed-posix-files>`. The wrapper supplies the complementary ShellCheck analysis.
4. For JSON or TOML, run `pnpm run lint:<format> <changed-format-files>`. The JSON wrapper requires exactly one parsed JSON value, and the TOML wrapper runs `taplo lint --no-schema`.
5. Check formatting for changed `.fish` and `.sh` files and extensionless Fish and POSIX shell scripts with `pnpm --config.verifyDepsBeforeRun=error exec prettier --check <changed-shell-files>`. The configured Fish plugin infers extensionless Fish scripts from their `fish` hashbang, so do not force `--parser fish`. Verify every applicable policy invariant in this skill and its routed references, including `domlib` ordering, usage, and `$DOMFILES_*` parity when relevant. Run `git --no-pager diff --check` and, when task-owned changes are staged, `git --no-pager diff --cached --check`. Inspect task-owned untracked files directly without staging them because Git diff does not include them.

## Validate a Shell Audit, Review, or Diagnosis

Complete the applicable language skill’s read-only validation, then verify every in-scope domfiles cross-file invariant in this skill and its routed references.
