---
name: domfiles-zed-settings
description: Edit, review, audit, and diagnose `.config/zed/settings.json` and `.zed/settings.json` plus project-authored maintainer assets, policy, and skill scripts for those settings. Use this skill whenever the resolved scope includes either settings file—even when the user did not name it—including agent permissions, fetch or network allowances, formatter settings, language settings, MCP settings, scan settings, terminal rules, tool or sandbox permissions, unexpected permission outcomes, or worktree permissions. Do not use it for other Zed files alone.
metadata:
    internal: true
---

# Zed settings

Use this skill as the entrypoint and canonical owner of settings-wide Zed policy and workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions. Do not copy the current command, domain, permission-pattern, or settings inventories into agent documentation.

When agent repository permissions, agent tool or sandbox permissions, fetch or network allowances, native path-tool permissions, terminal rules, or unexpected permission outcomes are in scope, follow the conditional [agent permission branch](references/permissions.md) before investigation or planning. Read only the branch references it selects.

## Apply the general policy

- Always split Zed settings audits into multiple smaller steps because a single pass can easily exceed the available context window.
- Follow the repository [skill-script language and filename policy](../../../AGENTS.md#skills) and the [skill-owned script policy](../../../skills/agent-documentation/references/skill-owned-scripts.md) for artifacts, layout, root toolchain ownership, staging, and tests.
- Keep `.config/zed/settings.json` free of entries that only restate Zed defaults.
    - Exempt `"tab_size": 4` from this requirement.
- Keep `.zed/settings.json` free of entries that only restate `.config/zed/settings.json` or Zed defaults.
    - Exempt `file_scan_exclusions`. Preserve its repository-specific override without adding installed Zed defaults, following the [documented rationale](../../PROJECT.md#zed-project-scan-exclusions).
- Keep every order-independent list introduced or modified in this scope alphabetized, including prose enumerations, regex alternatives, and Zed settings arrays. Sort object arrays by the value of their identifying field.
    - Within URL-pattern arrays, alphabetize the complete array by each pattern’s first represented hostname rather than its raw escaped regex text. Do not group patterns by hostname coverage.

## Choose the workflow

- For an explicit change, including a request that also uses review or audit language, complete the shared investigation, then follow every selected conditional branch’s change workflow. When no branch defines a mutation route, make a minimal edit to the selected settings object. Use the change-validation workflow below.
- For a standalone audit, follow the [repository audit process](../domfiles-repository-audit/SKILL.md).
- During commit review, do not analyze or validate permission patterns in `.config/zed/settings.json` unless the user explicitly includes that analysis. Review surrounding non-pattern changes normally. If evaluating the patterns is necessary to complete the review, stop before that analysis and ask for permission.
- For a standalone review, keep the task read-only and skip change planning, implementation, formatting, and change validation.
- For a standalone diagnosis, keep the task read-only. Reproduce the behavior with the narrowest non-mutating check, trace the relevant settings resolution, and use the read-only validation workflow below.

## Investigate and plan

1. Inspect the relevant settings object.
2. Resolve version-sensitive behavior, defaults, migrations, property names, or schema through current official Zed documentation or source.
3. Identify the smallest existing settings object that would own a change without reorganizing unrelated settings.

Do not mutate settings during this shared investigation. Mutation begins only through the explicit change workflow above.

## Resolve settings behavior

- Treat an editor deprecation banner as a lead, not proof that a specific property is deprecated.
- When Zed produces a migrated backup, compare the parsed values and relevant migration code before removing a setting.

## Validate a change

After editing:

1. Run every applicable conditional-branch change-validation workflow.
2. Parse each changed settings JSON file with `jq -e 'type == "object"' <path>`.
3. Check formatting with `pnpm --config.verifyDepsBeforeRun=error exec prettier --check <changed-files>`, following the [repository command rationale](../../PROJECT.md#repository-scoped-commands). If dependencies are unavailable, report the limitation unless the current task separately authorizes reconciliation.
4. Verify every applicable general and selected-branch Zed settings policy invariant and repository-wide `AGENTS.md` instruction against the final values.
5. Run `git --no-pager diff --check` and inspect the final scoped diff and status.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Validate a Zed settings audit, review, or diagnosis

1. Parse each relevant settings JSON file with `jq -e 'type == "object"' <path>`.
2. Run every applicable conditional-branch read-only validation workflow.
3. Verify the applicable general and selected-branch Zed settings policy invariants and repository-wide `AGENTS.md` instructions against the audited contents.
