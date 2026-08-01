---
name: domfiles-zed-settings
description: Edit, review, audit, and diagnose only `.config/zed/settings.json` and `.zed/settings.json`. Use this skill whenever a task changes or evaluates either settings file, including its agent permissions, terminal rules, network and fetch allowances, MCP servers, or language settings; do not use it for other Zed files.
---

# Zed settings workflow

Use this skill only for tasks that directly edit, review, audit, or diagnose `.config/zed/settings.json` or `.zed/settings.json`. Do not use it for `.config/zed/AGENTS.md`, Zed documentation, or other files merely because they relate to Zed.

Use the applicable `AGENTS.md` files as the sole source of policy. This skill supplies only investigation, implementation, and validation mechanics. Do not restate those policies or copy the current command, domain, or settings inventory into this skill.

## Choose the workflow

- For a change, investigate the current behavior, plan the implementation, make the smallest applicable edit, and use the change-validation workflow below.
- For an audit or review, keep the task read-only. Skip change planning, implementation, formatting, and change validation; use the read-only validation workflow below.

## Investigate the task

1. Read `AGENTS.md`, inspect the relevant settings object, and review the existing diff.
2. For terminal permissions, inspect the locally installed executable’s help or manual. Record the exact forms the user needs and the forms that execute code, write data, alter state, or remove resources.
    - Run each local help or manual inspection with a short, bounded timeout. Prefer `MANPAGER=cat PAGER=cat man <command> | col -b` when a manual is available.
    - If the executable is unavailable or local help remains interactive, consult current official documentation or source.
3. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
4. Consult current official Zed documentation or source when parsing, regex support, schema, defaults, migration, or permission precedence is unclear.
5. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Plan a change

1. Apply the permission treatment and structural rules from `AGENTS.md` to the observed behavior.
2. Identify the smallest existing object or command family that owns the change.
3. Enumerate the required syntactic variants before writing the regex.
4. Prefer a minimal edit over reorganizing unrelated patterns.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification. In an allowance pattern that accepts trailing arguments, end each executable, subcommand, or option token with `(?: |$)`. The weaker `\b` is a lexical boundary, not a shell token boundary; use it only when lexical matching is intentional, such as in a conservative confirmation override.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
- Zed matches permission patterns case-insensitively by default. Set `case_sensitive` to `true` when shell semantics depend on case.
- Build positive branches for accepted grammar instead of trying to subtract cases with unsupported regex features.
- Test against Zed’s normalized permission input, not merely the original shell line.
- Add an executable to the shared `--(?:help|version)` rule only when the locally installed command treats those exact forms as informational.
- Do not execute a destructive command merely to test a permission pattern.

## Translate approved domains

After applying the domain-scope policy in `AGENTS.md`, encode the corresponding fetch hostname with one of these shapes:

1. Use one of these fetch hostname shapes:
    - Apex only: `^https://domain\.example(?:[/?#]|$)`
    - Subdomains only: `^https://(?:[^./?#:@]+\.)+domain\.example(?:[/?#]|$)`
    - Apex and subdomains: `^https://(?:[^./?#:@]+\.)*domain\.example(?:[/?#]|$)`
2. Escape every literal hostname dot.
3. Inspect redirects and required subresources before deciding whether the approved scope is sufficient.

## Resolve settings behavior

- Check the current Zed schema, documentation, or source for version-sensitive defaults and property names.
- Treat an editor deprecation banner as a lead, not proof that a specific property is deprecated.
- When Zed produces a migrated backup, compare the parsed values and relevant migration code before removing a setting.

## Evaluate permission behavior

1. Build an in-memory matrix that records matches from `always_allow` and `always_confirm` separately, then derive the effective result using Zed’s precedence: confirmation overrides allowance, followed by the configured default.
2. Match each pattern with `rg --no-config --case-sensitive` when its object sets `case_sensitive` to `true`; otherwise use `rg --no-config --ignore-case` to simulate Zed’s default. Treat exit status `0` as a match, `1` as no match, and any other status as a validation failure.
3. When consolidating patterns, compare the union of the old family’s matches with the union of the new family’s matches over representative inputs. Do not compare objects one-for-one when ownership moved between patterns.
4. When confirmation precedence and Rust-compatible regex limits make a narrow allowance require a fragile complement expression, leave the form confirmable and record the durable rationale in `.agents/PROJECT.md`.

## Validate a change

After editing:

1. Parse changed JSON with `jq -e`.
2. Check formatting through the repository’s existing `pnpm` formatter workflow.
3. Run `git diff --check`.
4. Compile changed permission patterns with a Rust-compatible regex tool such as `rg`.
5. Use the permission-behavior workflow above to test representative inputs:
    - Intended commands or URLs that should match.
    - Hazardous forms that must match an override or remain unmatched by the allowance.
    - Near misses that must not match.
    - Every syntactic variant required by `AGENTS.md`.
6. Verify every applicable `AGENTS.md` invariant against the final values.
7. Inspect the final diff and status. Remove only artifacts created by validation.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Validate a read-only audit or review

Without editing or formatting files:

1. Parse relevant JSON with `jq -e`.
2. Compile relevant existing permission patterns with a Rust-compatible regex tool such as `rg`.
3. Use the permission-behavior workflow above to test representative intended inputs, hazardous forms, and near misses against the existing patterns.
4. Verify the applicable `AGENTS.md` invariants against the audited contents.
5. Inspect the relevant diff and status only to identify pre-existing or concurrent changes.

## Report the result

- For a change, lead with what changed. State which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable.
- For an audit or review, report evidence-backed findings rather than changes and follow the applicable reporting procedure in `AGENTS.md`.
- Summarize the validation performed and identify any behavior that could not be verified.
