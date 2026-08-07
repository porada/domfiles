---
name: domfiles-zed-settings
description: Edit, review, audit, and diagnose `.config/zed/settings.json` and `.zed/settings.json`. Use this skill whenever the resolved task scope includes either settings file—even when the user did not name it—including language, formatter, scan, agent, or MCP settings. Also use `domfiles-zed-permissions` when agent tool or sandbox permissions, terminal rules, fetch or network allowances, worktree permissions, or permission outcomes are in scope. Do not use it for other Zed files alone.
---

# Zed settings

Use this skill as the canonical source for general Zed settings policy and workflow. `domfiles-zed-permissions` owns agent permission policy and permission-specific workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions. Do not copy the current command, domain, or settings inventory into either skill.

## Apply the general policy

- Always split Zed settings audits into multiple smaller steps because a single pass can easily exceed the available context window.
- Keep `.config/zed/settings.json` free of entries that only restate Zed defaults.
    - Exempt `"tab_size": 4` from this requirement.
- Keep `.zed/settings.json` free of entries that only restate `.config/zed/settings.json` or Zed defaults.
    - Exempt `file_scan_exclusions`. Preserve its repository-specific override without adding installed Zed defaults, following the [documented rationale](../../PROJECT.md#zed-project-scan-exclusions).
- Keep every order-independent list introduced or modified in this scope alphabetized, including prose enumerations, regex alternatives, and Zed settings arrays. Sort object arrays by the value of their identifying field.

## Choose the workflow

- For a change, investigate the current behavior, plan the implementation, make the smallest applicable edit, and use the change-validation workflow below.
- For an audit, follow the [repository audit process](../domfiles-repository-audit/SKILL.md).
- For a review, keep the task read-only and skip change planning, implementation, formatting, and change validation.

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

1. Parse changed JSON with `jq -e`.
2. Check formatting through the repository’s existing `pnpm` formatter workflow.
3. Run `git --no-pager diff --check`.
4. Run every applicable domain-specific change-validation workflow.
5. Verify every applicable general Zed settings policy invariant and repository-wide `AGENTS.md` instruction against the final values.
6. Inspect the final diff and status. Remove only artifacts created by validation.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Validate a Zed settings audit or review

1. Parse relevant JSON with `jq -e`.
2. Run every applicable domain-specific read-only validation workflow.
3. Verify the applicable general Zed settings policy invariants and repository-wide `AGENTS.md` instructions against the audited contents.

## Report a change or review

- For a change, lead with what changed.
- For a review, report evidence-backed findings and identify any behavior that could not be verified.
