# Git permissions

Follow the shared [agent permission workflow](permissions.md), the [terminal permission policy](terminal-permissions.md), and this branch whenever Git commands or permission patterns are in scope.

## Apply the Git permission policy

- Partition the terminal policy’s `git` executable family into separate command-owner groups for root Git forms, each direct Git subcommand, and each compound multi-invocation workflow.
    - Keep one direct Git subcommand per regex pattern.
    - Keep discovery, direct, wrapped, confirmation, and denial forms with the owning root command, subcommand, or compound workflow.
    - Do not use repeated Git prefix grammar as a reason to combine owner groups.
- Keep every Git regex under the terminal policy’s decoded `1,000`-character limit.
    - Split an oversized owner group by one coherent syntax role, wrapper form, or fixed-prefix form without combining another direct subcommand.
- Order root and discovery forms first, direct subcommand groups alphabetically, then compound workflow groups alphabetically within each permission bucket.
- Account for the optional `-C <path>`, `--no-optional-locks`, and `--no-pager` global options before every Git subcommand.
- Treat exact root-level Git options that print an installation path and exit as informational only after verifying the local manual. Keep value-taking variants that change helper lookup confirmable.
- Apply the same optional repeated fixed-value `GIT_*`, `MANPAGER=cat`, and `PAGER=cat` prefix grammar from `.config/zed/settings.json` to every applicable Git command-owned allowance and confirmation override. Treat the settings grammar as canonical, keep its variable and value alternatives alphabetized, and keep every copy byte-identical.
- Never allow wildcard or unknown `GIT_*` assignments. Treat each variable and value as behavior-bearing, audit it against the hazard classes documented in [Zed terminal permission limitations](../../../PROJECT.md#zed-terminal-permission-limitations), and update that rationale when the safety boundary changes.
- Keep the fixed-value Git assignment list minimal and evidence-driven. Include a name and value only after recurring approved use demonstrates that automatic permission is useful. Documented safety alone is insufficient. Prefer disabling or noninteractive values over default-restoring or enabling values, and re-audit retained semantics whenever Git changes.
- Keep Git subcommand discovery restricted to compiled command names. Treat aliases and external `git-*` helpers as separate executable owners rather than Git subcommands.

## Validate Git patterns

Validate every in-scope Git pattern against:

- The root, direct-subcommand, or compound-workflow owner it declares.
- Applicable global options, fixed assignments, pager prefixes, and wrapper forms.
- Alphabetic owner-group ordering and byte-identical repeated prefix grammar.
- The decoded-length limit without combining another direct subcommand during repair.
