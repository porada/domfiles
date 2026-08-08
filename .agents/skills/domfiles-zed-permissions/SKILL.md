---
name: domfiles-zed-permissions
description: Edit, review, audit, and diagnose Zed agent permission behavior in `.config/zed/settings.json` and `.zed/settings.json`. Use this skill with `domfiles-zed-settings` whenever the resolved task scope includes `agent.tool_permissions`, `agent.sandbox_permissions`, terminal rules, native path-tool permissions, fetch or network allowances, worktree permissions, or a tool or command unexpectedly allowing, confirming, or denying. Do not use it for unrelated settings or documentation-only tasks.
---

# Zed agent permissions

Use this skill as the canonical source for Zed agent permission policy and permission-specific workflow. Follow `domfiles-zed-settings` for general settings policy, workflow, validation, and reporting. Continue to follow applicable `AGENTS.md` files for repository-wide instructions. Do not copy the current command, domain, or settings inventory into this skill.

## Apply the permission policy

- Treat Zed agent permissions as layered security boundaries.
    - Preserve `agent.tool_permissions.default` as `allow`.
    - Treat ordinary package-manager workflows as intentional allowances. Continue to require confirmation for package runners that can download and execute arbitrary code.
    - Allow Docker inspection operations without confirmation. Require confirmation for operations that execute workloads or create, modify, or remove Docker state.
    - Keep `agent.sandbox_permissions.network_hosts` aligned with `agent.tool_permissions.tools.fetch.always_allow`, subject to the [documented host-scope exception](../../PROJECT.md#zed-fetch-and-sandbox-host-scope).
    - Treat `*.domain.name` and `domain.name` as distinct `network_hosts` entries. Preserve both when access to the apex domain and its subdomains is intended.
    - Prefer wildcard domain allowances when subdomains are involved. Include the apex domain only when it is actually used.
    - Restrict automatically allowed fetch URL patterns to `https://` and anchor each pattern at the hostname boundary.
- Keep terminal permission patterns concise and consistent.
    - Set `"case_sensitive": true` on every terminal command pattern unless a verified command-specific requirement justifies case-insensitive matching. Prefer a scoped inline case-insensitive group for an exceptional token instead of making the entire pattern case-insensitive.
    - Keep the consolidated general `terminal.always_allow` pattern first, followed by the shared `(?:-[hv]|--(?:help|version))` pattern and then the shared `--(?:help|version)` pattern for commands whose short forms are not safely informational. List every executable absent from the general allowance in exactly one shared discovery pattern, even when the executable does not support one or more permitted flags. Alphabetize the remaining patterns by command family.
    - Keep the child-executable alternatives in the dedicated `xargs` allowance identical to the consolidated general terminal allowance and update both in the same change.
        - Limit `xargs`’s own options to bounded, noninteractive argument splitting and batching controls.
        - Require confirmation for the complete nested command family whenever standard input could activate a code-execution hook, file-writing option, destructive operation, or other hazardous form. See [Zed xargs permission mirroring](../../PROJECT.md#zed-xargs-permission-mirroring) for rationale.
    - Alphabetize executable, command, and subcommand alternatives within consolidated patterns when their grammar permits.
    - Consolidate variants within the same command family. Keep unrelated command families separate.
        - Keep informational forms in a command-family pattern only when they use different syntax, such as short flags, help subcommands, command-specific prefixes or wrappers, global options, or subcommand help.
    - Prefer explicit alternatives over optional fragments when consolidating distinct executable names.
    - Keep command-specific prefixes and wrappers out of the consolidated general and shared discovery patterns. Define family-specific allowances separately. Account for approved prefixes and wrappers in applicable allowances and confirmation overrides.
        - Account for the optional `-C <path>`, `--no-optional-locks`, and `--no-pager` global options before every Git subcommand.
        - Apply the same optional repeated fixed-value `GIT_*`, `MANPAGER=cat`, and `PAGER=cat` prefix grammar from `.config/zed/settings.json` to every dedicated Git terminal allowance and matching confirmation override. Exempt shared discovery patterns, which retain only their common pager prefixes. Use dedicated Git discovery patterns for approved fixed-value `GIT_*` assignments. Treat the settings grammar as canonical, keep its variable and value alternatives alphabetized, and keep every copy byte-identical.
        - Never allow wildcard or unknown `GIT_*` assignments. Treat each variable and value as behavior-bearing, audit it against the hazard classes documented in [Zed terminal permission limitations](../../PROJECT.md#zed-terminal-permission-limitations), and update that rationale when the safety boundary changes.
        - Keep the fixed-value Git assignment list minimal and evidence-driven. Include a name and value only after recurring approved use demonstrates that automatic permission is useful. Documented safety alone is insufficient. Prefer disabling or noninteractive values over default-restoring or enabling values, and re-audit retained semantics whenever Git changes.
        - Apply optional `HOMEBREW_NO_*` prefixes with the fixed value `1` to every Homebrew terminal pattern.
    - Prefer literal spaces over whitespace character classes.
    - Treat signaling explicit numeric process IDs as an intentional allowance for polling and stopping processes associated with the current task. Do not extend this allowance to process names or patterns.
    - For mixed-purpose utilities and interpreters, prefer positive allowlists of non-mutating forms. Use a broad allowance with `terminal.always_confirm` only when every hazardous form can be matched reliably. Otherwise, preserve default confirmation.
    - Use `terminal.always_confirm` to override broader `terminal.always_allow` entries for hazardous argument forms, including code-execution hooks, package runners, destructive operations, force flags, and commands that uninstall the invoked tool itself. Account for global options, combined short flags, and accepted long-option abbreviations.
    - Do not report overlaps between `terminal.always_allow` and `terminal.always_confirm` when `terminal.always_confirm` acts as a safety override.

## Maintain agent worktree permissions

- Keep native-tool and terminal permission patterns synchronized with the [global worktree convention](../../../.config/zed/AGENTS.md#git-worktrees).
- Use native `move_path` for strict descendant moves within agent worktrees and `git worktree move` for top-level worktree moves. Leave terminal `mv` confirmable.
- Keep forced worktree and branch operations constrained to their respective namespaces, and keep `--detach` confirmable.
- Allow `git worktree prune` automatically only in dry-run forms. Keep actual pruning, out-of-namespace paths or branches, remote operations, shell globs, path traversal, parent-removing `rmdir -p`, and broader deletion mechanisms confirmable.
- See [Zed worktree permission coupling](../../PROJECT.md#zed-worktree-permission-coupling) for rationale.

## Extend the workflow

- For an explicit permission change, including a request that also uses review or audit language, follow the change workflow in `domfiles-zed-settings` and apply the [permission policy](#apply-the-permission-policy).
- For a standalone permission audit, keep the task read-only, follow the [repository audit process](../domfiles-repository-audit/SKILL.md), and use the permission audit validation below.
- For a standalone permission review, keep the task read-only and use the permission review validation below without change planning, implementation, or formatting.

For either read-only workflow:

- Treat terminal command candidates as inert strings and evaluate them only through permission-pattern matching.
- Limit shell execution to the bounded, non-mutating inspection and validation utilities required by the read-only validation workflow below.

## Investigate permission behavior

1. For terminal permissions, inspect the locally installed executable’s help or manual. Record the exact forms the user needs and the forms that execute code, write data, alter state, or remove resources.
    - Run each local help or manual inspection with a short, bounded timeout. Prefer `MANPAGER=cat PAGER=cat man <command> | col -b` when a manual is available.
    - If the executable is unavailable or local help remains interactive, consult current official documentation or source.
2. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
3. Consult current official Zed documentation or source when parsing, regex support, or permission precedence is unclear.
4. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Plan a permission change

1. Apply the permission treatment and structural policy above to the observed behavior.
2. Identify the smallest existing permission object or command family that owns the change.
3. Enumerate the required syntactic variants before writing the regex.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification. In an allowance pattern that accepts trailing arguments, end each executable, subcommand, or option token with `(?: |$)`. The weaker `\b` is a lexical boundary, not a shell token boundary. Use it only when lexical matching is intentional, such as in a conservative confirmation override.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
- Zed matches permission patterns case-insensitively by default. Follow the explicit case-sensitivity policy above for terminal commands, including patterns whose current tokens happen to be unambiguous.
- Build positive branches for accepted grammar instead of trying to subtract cases with unsupported regex features.
- Test against Zed’s normalized permission input, not merely the original shell line.
- Permit exact `COMMAND --help` and `COMMAND --version` discovery forms for every allowed command family, even when the installed executable does not support one or both flags, so unsupported discovery attempts fail without prompting. Permit `COMMAND -h` and `COMMAND -v` only when both short forms have been verified to exit without executing input, reading commands from standard input, mutating state, or starting an interactive or workload mode. Keep every executable absent from the general allowance in the applicable shared discovery pattern, and keep all discovery forms end-anchored so flags cannot acquire operands.
- Apply optional repeated `MANPAGER=cat` and `PAGER=cat` prefixes to the general and both shared discovery patterns. Treat family-specific pager coverage independently from discovery-form ownership: preserve existing prefixes when they cover other approved forms, and do not remove them merely because exact discovery forms moved into a shared pattern.
- Do not execute a destructive command merely to test a permission pattern.

## Translate approved domains

After applying the domain-scope policy above:

1. Select the matching fetch hostname shape:
    - Apex only: `^https://domain\.example(?:[/?#]|$)`
    - Subdomains only: `^https://(?:[^./?#:@]+\.)+domain\.example(?:[/?#]|$)`
    - Apex and subdomains: `^https://(?:[^./?#:@]+\.)*domain\.example(?:[/?#]|$)`
2. Escape every literal hostname dot.
3. Inspect redirects and required subresources before deciding whether the approved scope is sufficient.

## Evaluate permission behavior

Read the [permission evaluator reference](references/permission-evaluator.md) when testing configured patterns, reconstructing a permission decision, or comparing consolidated pattern families. It defines the bounded `rg` smoke test, evaluator precedence, normalized-input handling, and pattern-family comparison.

## Validate a permission change

At the domain-specific step of the change-validation workflow in `domfiles-zed-settings`:

1. Validate every changed pattern against representative inputs, using the [bounded smoke-test workflow](references/permission-evaluator.md#smoke-test-permission-patterns) only for eligible cases and current Zed source or observed Zed behavior for every other case:
    - Intended commands or URLs that should match.
    - Hazardous forms that must match an override or remain unmatched by the allowance.
    - Near misses that must not match.
    - Every syntactic variant required by the policy above.
2. Verify every applicable Zed permission policy invariant against the final values.

## Validate a permission audit or review

At the domain-specific step of the read-only validation workflow in `domfiles-zed-settings`:

1. Validate relevant existing patterns against representative intended inputs, hazardous forms, and near misses, using the [bounded smoke-test workflow](references/permission-evaluator.md#smoke-test-permission-patterns) only for eligible cases and current Zed source or observed Zed behavior for every other case.
2. Verify the applicable Zed permission policy invariants against the audited contents.

## Extend the report

For a permission change, state which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable.
