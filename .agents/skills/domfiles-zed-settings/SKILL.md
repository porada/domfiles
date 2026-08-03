---
name: domfiles-zed-settings
description: Edit, review, audit, and diagnose `.config/zed/settings.json` and `.zed/settings.json`. Use this skill whenever the resolved task scope includes changing or evaluating either settings file—even when the user did not name it—including its agent permissions, terminal rules, network and fetch allowances, MCP servers, or language settings; do not use it for other Zed files.
---

# Zed settings

Use this skill whenever the resolved task scope includes editing, reviewing, auditing, or diagnosing `.config/zed/settings.json` or `.zed/settings.json`, even when the user did not name them explicitly. Do not use it for `.config/zed/AGENTS.md`, Zed documentation, or other files merely because they relate to Zed.

Use this skill as the canonical source for Zed settings policy and workflow. Continue to follow applicable `AGENTS.md` files for repository-wide instructions. Do not copy the current command, domain, or settings inventory into this skill.

## Apply the policy

- Always split Zed settings audits into multiple smaller steps because a single pass can easily exceed the available context window.
- Keep `.config/zed/settings.json` free of entries that only restate Zed defaults.
    - Exempt `"tab_size": 4` from this requirement.
    - Keep `.zed/settings.json` free of entries that only restate `.config/zed/settings.json` or Zed defaults.
- Keep order-independent arrays in Zed settings files alphabetized by value or, for object entries, by the value of their identifying field.
- Treat Zed agent permissions as layered security boundaries.
    - Preserve `agent.tool_permissions.default` as `allow`.
    - Treat ordinary package-manager workflows as intentional allowances. Continue to require confirmation for package runners that can download and execute arbitrary code.
    - Allow Docker inspection operations without confirmation. Require confirmation for operations that execute workloads or create, modify, or remove Docker state.
    - Keep `agent.sandbox_permissions.network_hosts` aligned with `agent.tool_permissions.tools.fetch.always_allow`.
    - Treat `*.domain.name` and `domain.name` as distinct `network_hosts` entries. Preserve both when access to the apex domain and its subdomains is intended.
    - Prefer wildcard domain allowances when subdomains are involved. Include the apex domain only when it is actually used.
    - Restrict automatically allowed fetch URL patterns to `https://` and anchor each pattern at the hostname boundary.
- Keep terminal permission patterns concise and consistent.
    - Set `"case_sensitive": true` on every terminal command pattern unless a verified command-specific requirement justifies case-insensitive matching. Prefer a scoped inline case-insensitive group for an exceptional token instead of making the entire pattern case-insensitive.
    - Keep the consolidated general `terminal.always_allow` pattern first, followed by the shared `(?:-[hv]|--(?:help|version))` pattern and then the shared `--(?:help|version)` pattern for commands whose short forms are not safely informational. List every executable absent from the general allowance in exactly one shared discovery pattern, even when the executable does not support one or more permitted flags. Alphabetize the remaining patterns by command family.
    - Alphabetize executable, command, and subcommand alternatives within consolidated patterns when their grammar permits.
    - Consolidate variants within the same command family. Keep unrelated command families separate.
        - Keep informational forms in a command-family pattern only when they use different syntax, such as short flags, help subcommands, command-specific prefixes or wrappers, global options, or subcommand help.
    - Prefer explicit alternatives over optional fragments when consolidating distinct executable names.
    - Keep command-specific prefixes and wrappers out of the consolidated general and shared discovery patterns. Define family-specific allowances separately. Account for approved prefixes and wrappers in applicable allowances and confirmation overrides.
        - Account for the optional `-C <path>`, `--no-optional-locks`, and `--no-pager` global options before every Git subcommand.
        - Apply the optional `GIT_EDITOR=true`, `GIT_PAGER=cat`, `MANPAGER=cat`, and `PAGER=cat` prefixes to every Git terminal pattern.
        - Apply optional `HOMEBREW_NO_*` prefixes with the fixed value `1` to every Homebrew terminal pattern.
    - Prefer literal spaces over whitespace character classes.
    - Treat signaling explicit numeric process IDs as an intentional allowance for polling and stopping processes associated with the current task. Do not extend this allowance to process names or patterns.
    - For mixed-purpose utilities and interpreters, prefer positive allowlists of non-mutating forms. Use a broad allowance with `terminal.always_confirm` only when every hazardous form can be matched reliably. Otherwise, preserve default confirmation.
    - Use `terminal.always_confirm` to override broader `terminal.always_allow` entries for hazardous argument forms, including code-execution hooks, package runners, destructive operations, force flags, and commands that uninstall the invoked tool itself. Account for global options, combined short flags, and accepted long-option abbreviations.
    - Do not report overlaps between `terminal.always_allow` and `terminal.always_confirm` when `terminal.always_confirm` acts as a safety override.

## Choose the workflow

- For a change, investigate the current behavior, plan the implementation, make the smallest applicable edit, and use the change-validation workflow below.
- For an audit or review, keep the task read-only and never execute arbitrary commands.
    - Treat terminal command candidates as inert strings and evaluate them only through permission-pattern matching.
    - Limit shell execution to the bounded, non-mutating inspection and validation utilities required by the read-only workflow below.
    - Skip change planning, implementation, formatting, and change validation; use the read-only validation workflow below.

## Investigate the task

1. Read `AGENTS.md`, inspect the relevant settings object, and review the existing diff.
2. For terminal permissions, inspect the locally installed executable’s help or manual. Record the exact forms the user needs and the forms that execute code, write data, alter state, or remove resources.
    - Run each local help or manual inspection with a short, bounded timeout. Prefer `MANPAGER=cat PAGER=cat man <command> | col -b` when a manual is available.
    - If the executable is unavailable or local help remains interactive, consult current official documentation or source.
3. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
4. Consult current official Zed documentation or source when parsing, regex support, schema, defaults, migration, or permission precedence is unclear.
5. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Plan a change

1. Apply the permission treatment and structural policy above to the observed behavior.
2. Identify the smallest existing object or command family that owns the change.
3. Enumerate the required syntactic variants before writing the regex.
4. Prefer a minimal edit over reorganizing unrelated patterns.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification. In an allowance pattern that accepts trailing arguments, end each executable, subcommand, or option token with `(?: |$)`. The weaker `\b` is a lexical boundary, not a shell token boundary; use it only when lexical matching is intentional, such as in a conservative confirmation override.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
- Zed matches permission patterns case-insensitively by default. Follow the explicit case-sensitivity policy above for terminal commands, including patterns whose current tokens happen to be unambiguous.
- Build positive branches for accepted grammar instead of trying to subtract cases with unsupported regex features.
- Test against Zed’s normalized permission input, not merely the original shell line.
- Permit exact `COMMAND --help` and `COMMAND --version` discovery forms for every allowed command family, even when the installed executable does not support one or both flags, so unsupported discovery attempts fail without prompting. Permit `COMMAND -h` and `COMMAND -v` only when both short forms have been verified to exit without executing input, reading commands from standard input, mutating state, or starting an interactive or workload mode. Keep every executable absent from the general allowance in the applicable shared discovery pattern, and keep all discovery forms end-anchored so flags cannot acquire operands.
- Apply optional repeated `MANPAGER=cat` and `PAGER=cat` prefixes to the general and both shared discovery patterns. Treat family-specific pager coverage independently from discovery-form ownership: preserve existing prefixes when they cover other approved forms, and do not remove them merely because exact discovery forms moved into a shared pattern.
- Do not execute a destructive command merely to test a permission pattern.

## Translate approved domains

After applying the domain-scope policy above, encode the corresponding fetch hostname with one of these shapes:

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
    - Every syntactic variant required by the policy above.
6. Verify every applicable Zed settings policy invariant and repository-wide `AGENTS.md` instruction against the final values.
7. Inspect the final diff and status. Remove only artifacts created by validation.

Do not run the entire repository formatter when a targeted formatting check is sufficient.

## Validate a read-only audit or review

Without editing or formatting files:

1. Parse relevant JSON with `jq -e`.
2. Compile relevant existing permission patterns with a Rust-compatible regex tool such as `rg`.
3. Use the permission-behavior workflow above to test representative intended inputs, hazardous forms, and near misses against the existing patterns.
4. Verify the applicable Zed settings policy invariants and repository-wide `AGENTS.md` instructions against the audited contents.
5. Inspect the relevant diff and status only to identify pre-existing or concurrent changes.

## Report the result

- For a change, lead with what changed. State which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable.
- For an audit or review, report evidence-backed findings rather than changes and follow the applicable reporting procedure in `AGENTS.md`.
- Summarize the validation performed and identify any behavior that could not be verified.
