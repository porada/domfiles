---
name: shell-integration
description: |-
    Use this skill whenever the resolved task scope includes shell code—including `domlib`, Fish configuration, `home/.local/bin` scripts, and `.hooks`—or adds or reconsiders a command entrypoint in this repository, including whether a Git helper should be a plain alias or a `home/.local/bin/git-*` script.

    Do not use it merely because the task runs terminal commands.

metadata:
    internal: true
---

# Domfiles Shell Integration

Use this skill as the canonical source for domfiles-specific shell integration, invariants, and validation.

For every Fish target, load `fish-shell-scripting` for portable language policy and workflow. For every POSIX shell target, load `posix-shell-scripting`. Apply this skill as the narrower domfiles layer when either language skill governs the same task.

## Investigate Tasks

1. Classify each in-scope shell file from its hashbang and syntax rather than its extension alone, then apply `fish-shell-scripting` or `posix-shell-scripting`.
2. Do not report `home/.config/fish/local.fish`’s [documented sourcing behavior](../../PROJECT.md#fish-local-configuration) as hidden diagnostics.
3. Evaluate `home/.config/fish/functions/clone.fish` against the [Fish `clone` argument contract](../../PROJECT.md#fish-clone-argument-contract). Do not report the absence of Git option parsing, option rejection, or reliable follow-up directory changes for unsupported option-bearing invocations.

For every non-Fish shell target and whenever the task touches `domlib`, a Fish `__domfiles_*` helper, shared `$DOMFILES_*` state, or command suppression, follow [`domlib` Integration](references/domlib-integration.md).

## Check Supported Environment Compatibility

- Evaluate every in-scope domfiles shell script’s interpreter, external commands, options, `PATH`, architecture, and default-shell assumptions against the [supported environment](../../PROJECT.md#supported-environment).
- Judge each requirement at its intended lifecycle stage—fresh bootstrap, synchronization, post-sync runtime, or development—and account for prerequisites provisioned earlier by `domfiles sync`.
- Treat `domfiles dependencies` as the user-facing readiness check defined by [dependency status labels](../../PROJECT.md#dependency-status-labels). Add a row only for an established user-facing synchronization or runtime contract. Agent-only use or installation by synchronization alone does not qualify a dependency.

## Apply Domfiles Shell Wording Constraints

Use the applicable language skill for semantic requirements and composition with `human-facing-writing`. Apply these domfiles-specific constraints after that workflow:

- Avoid first-person and subjective wording.
- Omit final punctuation from script comments and user-facing strings passed to `__print*`.
- Treat standalone headings and status labels as labels rather than sentences. Allow sentence case or title case, and do not require imperative voice.
- Use sentence case imperative voice for action and section comments.

## Choose Command Form and Location

Do not report an existing command solely because another supported form could express it, except when [Git helper form](references/command-form-and-location.md#choose-git-helper-form) applies.

Before adding a command entrypoint or explicitly reconsidering an existing command’s form or location, follow [Command Form and Location](references/command-form-and-location.md).

Before reviewing a `home/.local/bin/git-*` entrypoint, follow [Git helper form](references/command-form-and-location.md#choose-git-helper-form).

## Evaluate Duplication and Reuse

- Do not report the language-specific `home/.local/bin/domfiles-dev-lint-*` entrypoints as duplication merely because each retains its own default scope and lint command. Shared discovery and execution belong in `domlib`. See [development lint wrapper architecture](../../PROJECT.md#development-lint-wrapper-architecture) for rationale.
- Consolidate shell implementations when they duplicate a substantial, virtually identical behavior pipeline that must remain aligned.
- Do not report `__string_*` helpers or equivalent inline string operations as reimplementations. See [string helper reuse](../../PROJECT.md#string-helper-reuse) for rationale.

## Apply Domfiles POSIX Conventions

- Allow argumentless `echo` to print a blank line as an exception to `posix-shell-scripting`’s `printf` requirement.
- Report `find` commands that place `-maxdepth` anywhere other than immediately after the search path.

## Validate Changes

When changes to `home/.config/fish/config.fish` alter machine-local sourcing, keep [Fish local configuration](../../PROJECT.md#fish-local-configuration) aligned.

After editing, apply the [repository validation policy](../../../AGENTS.md#validation), then follow [Validate Shell Changes](references/validate-shell-changes.md) for lint and formatting commands and complete invariant checks.

## Validate Shell Audits, Diagnoses, and Reviews

Apply the [repository validation policy](../../../AGENTS.md#validation) before running the applicable language skill’s read-only validation. When invoked through `audit`, preserve its [ban on linters and formatters](../audit/SKILL.md#keep-audits-read-only) and use only the remaining permitted checks. Verify every in-scope domfiles cross-file invariant in this skill and its routed references.
