---
name: domfiles-zed-config
description: Edit and review Zed settings in this repository, especially agent terminal, tool, sandbox, network, fetch, MCP, skill, and language configuration. Use whenever changing or diagnosing `.config/zed/settings.json`, `.zed/settings.json`, Zed agent permissions, command allow/confirm patterns, or domain allowances.
---

# Zed configuration workflow

Use the applicable `AGENTS.md` files as the sole source of policy. This skill supplies only investigation, implementation, and validation mechanics. Do not restate those policies or copy the current command, domain, or settings inventory into this skill.

## Investigate the change

1. Read `AGENTS.md`, inspect the relevant settings object, and review the existing diff.
2. For terminal permissions, inspect the locally installed executable’s help or manual. Record the exact forms the user needs and the forms that execute code, write data, alter state, or remove resources.
3. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
4. Consult current official Zed documentation or source when parsing, regex support, schema, defaults, migration, or permission precedence is unclear.
5. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Plan the implementation

1. Apply the permission treatment and structural rules from `AGENTS.md` to the observed behavior.
2. Identify the smallest existing object or command family that owns the change.
3. Enumerate the required syntactic variants before writing the regex.
4. Prefer a minimal edit over reorganizing unrelated patterns.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification; use `\b` only when accepting further arguments is intentional.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
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

## Validate the result

1. Parse changed JSON with `jq -e`.
2. Check formatting through the repository’s existing `pnpm` formatter workflow.
3. Run `git diff --check`.
4. Compile new permission patterns with a Rust-compatible regex tool such as `rg`.
5. Test each new pattern against representative inputs:
    - Intended commands or URLs that should match.
    - Hazardous forms that must match an override or remain unmatched by the allowance.
    - Near misses that must not match.
    - Every syntactic variant required by `AGENTS.md`.
6. Verify every applicable `AGENTS.md` invariant against the final values.
7. Inspect the final diff and status. Remove only artifacts created by validation.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Report the outcome

Lead with what changed. State which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable. Summarize the validation performed and identify any behavior that could not be verified.
