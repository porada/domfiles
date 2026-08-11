---
name: domfiles-zed-settings
description: Edit, review, audit, and diagnose `.config/zed/settings.json` and `.zed/settings.json` plus project-authored maintainer assets, policy, and skill scripts for those settings. Use this skill whenever the resolved scope includes either settings file—even when the user did not name it—including agent permissions, fetch or network allowances, formatter settings, language settings, MCP settings, scan settings, terminal rules, tool or sandbox permissions, unexpected permission outcomes, or worktree permissions. Do not use it for other Zed files alone.
---

# Zed settings

Use this skill as the canonical source for Zed settings policy and workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions. Do not copy the current command, domain, permission-pattern, or settings inventories into agent documentation.

When agent tool or sandbox permissions, terminal rules, native path-tool permissions, fetch or network allowances, agent repository permissions, or unexpected permission outcomes are in scope, follow the conditional [agent permission branch](references/permissions.md) before investigation or planning. Read only the branch references it selects.

## Apply the general policy

- Always split Zed settings audits into multiple smaller steps because a single pass can easily exceed the available context window.
- Keep every script owned by this skill in Rust. Store executable entrypoints and their adjacent tests directly under `scripts`, shared implementation helpers and their adjacent tests under `scripts/helpers`, and Cargo target and dependency declarations at the repository root, following the [skill-owned script policy](../agent-documentation/references/skill-owned-scripts.md).
- Keep `.config/zed/settings.json` free of entries that only restate Zed defaults.
    - Exempt `"tab_size": 4` from this requirement.
- Keep `.zed/settings.json` free of entries that only restate `.config/zed/settings.json` or Zed defaults.
    - Exempt `file_scan_exclusions`. Preserve its repository-specific override without adding installed Zed defaults, following the [documented rationale](../../PROJECT.md#zed-project-scan-exclusions).
- Keep every order-independent list introduced or modified in this scope alphabetized, including prose enumerations, regex alternatives, and Zed settings arrays. Sort object arrays by the value of their identifying field.
    - Within URL-pattern arrays, preserve hostname-scope groupings and alphabetize each group by the represented hostname rather than the raw escaped regex text.

## Choose the workflow

- For an explicit change, including a request that also uses review or audit language, investigate the current behavior, plan the implementation, make the smallest applicable edit, and use the change-validation workflow below.
- For a standalone audit, follow the [repository audit process](../domfiles-repository-audit/SKILL.md).
- For a standalone review, keep the task read-only and skip change planning, implementation, formatting, and change validation.
- For a standalone diagnosis, keep the task read-only. Reproduce the behavior with the narrowest non-mutating check, trace the relevant settings resolution, use the read-only validation workflow below, and report the root cause, evidence, and corrective action.

## Investigate and plan

1. Read `AGENTS.md`, inspect the relevant settings object, and review the existing diff.
2. Consult current official Zed documentation or source when schema, defaults, migration, or settings behavior is unclear.
3. Identify the smallest existing settings object that owns the change.
4. Prefer a minimal edit over reorganizing unrelated settings.

## Resolve settings behavior

- Check the current Zed schema, documentation, or source for version-sensitive defaults and property names.
- Treat an editor deprecation banner as a lead, not proof that a specific property is deprecated.
- When Zed produces a migrated backup, compare the parsed values and relevant migration code before removing a setting.

## Validate a change

After editing:

1. Parse each changed settings JSON file with `jq -e 'type == "object"' <path>`.
2. Check formatting with `pnpm --config.verifyDepsBeforeRun=error exec prettier --check <changed-files>` so a fresh worktree cannot silently reconcile dependencies or run lifecycle scripts. If dependencies are unavailable, report the formatting limitation unless the current task separately authorizes reconciliation, following the [repository command rationale](../../PROJECT.md#repository-scoped-commands).
3. Run `git --no-pager diff --check`.
4. Run every applicable conditional-branch change-validation workflow.
5. Verify every applicable general and selected-branch Zed settings policy invariant and repository-wide `AGENTS.md` instruction against the final values.
6. Inspect the final diff and status. Remove only artifacts created by validation.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Validate a Zed settings audit, review, or diagnosis

1. Parse each relevant settings JSON file with `jq -e 'type == "object"' <path>`.
2. Run every applicable conditional-branch read-only validation workflow.
3. Verify the applicable general and selected-branch Zed settings policy invariants and repository-wide `AGENTS.md` instructions against the audited contents.

## Report a change, review, or diagnosis

- For a change, lead with what changed.
- For a review, report evidence-backed findings and identify any behavior that could not be verified.
- For a diagnosis, report the reproduced behavior, root cause, evidence boundary, and corrective action.
