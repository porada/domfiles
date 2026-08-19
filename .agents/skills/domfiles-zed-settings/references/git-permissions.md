# Git permissions

This branch specializes the [terminal permission policy](terminal-permissions.md) for Git.

## Apply the Git permission policy

- Partition the terminal policy’s `git` executable family into separate command-owner groups for root Git forms, each direct Git subcommand, and each compound multi-invocation workflow.
    - Keep one direct Git subcommand per regex pattern.
    - Keep discovery, direct, wrapped, confirmation, and denial forms with the owning root command, subcommand, or compound workflow.
    - Do not use repeated Git prefix grammar as a reason to combine owner groups.
- Apply the terminal policy’s [decoded-length limit](terminal-permissions.md#apply-the-terminal-permission-policy) independently to each Git owner group. Split an oversized group by one coherent fixed-prefix form, syntax role, or wrapper form.
- Order root and discovery forms first, direct subcommand groups alphabetically, then compound workflow groups alphabetically within each permission bucket.
- Account for the optional `-C <path>`, `--no-optional-locks`, and `--no-pager` global options before every Git subcommand.
- Treat exact root-level Git options that print an installation path and exit as informational only after verifying the local manual. Keep value-taking variants that change helper lookup confirmable.
- Classify terminating Git help and discovery forms by output provenance as well as side effects. Keep forms that expose user-configured alias bodies, external helper names, or comparable user-specific configuration confirmable unless evidence establishes a narrower automatic output boundary. Read-only behavior and termination alone do not make that output automatically safe.
- Apply the same optional repeated fixed-value `GIT_*`, `MANPAGER=cat`, and `PAGER=cat` prefix grammar from `.config/zed/settings.json` to every applicable Git command-owned allowance, confirmation override, and denial. Treat the settings grammar as canonical, keep its variable and value alternatives alphabetized, and keep every copy byte-identical. An authentication or credential denial may accept a broader assignment value than the corresponding allowance when that prevents an unsupported prefix from weakening the protected decision.
- Never allow wildcard or unknown `GIT_*` assignments. Treat each variable and value as behavior-bearing, audit it against the hazard classes documented in [Zed terminal permission limitations](../../../PROJECT.md#zed-terminal-permission-limitations), and update that rationale when the safety boundary changes.
- Keep the fixed-value Git assignment list minimal and evidence-driven. Include a name and value only after recurring approved use demonstrates that automatic permission is useful. Documented safety alone is insufficient. Prefer disabling or noninteractive values over default-restoring or enabling values, and re-audit retained semantics whenever Git changes.
- Keep Git subcommand discovery restricted to compiled command names. Treat aliases and external `git-*` helpers as separate executable owners rather than Git subcommands. Derive the allowed names from `git --list-cmds=builtins` and `git --list-cmds=parseopt` rather than the global Git inventory form. See [Zed terminal permission limitations](../../../PROJECT.md#zed-terminal-permission-limitations) for the resulting lists, why that form cannot supply them, and their refresh requirement.
- Take an automatic commit message outside agent worktrees only from a traversal-free `.agent-<name>` descendant supplied through `-F` or `--file`. Keep `-m` message words confirmable there because normalization leaves them indistinguishable from pathspec tokens, and keep standard-input `-F -` and the editor-opening `-e` combination confirmable everywhere. See [Zed terminal permission limitations](../../../PROJECT.md#zed-terminal-permission-limitations) for the rationale.
- Classify autosquash commit forms by editor behavior rather than by their later effect on history. Allow the plain fixup form and keep every editor-opening variant confirmable, following the [autosquash rationale](../../../PROJECT.md#zed-terminal-permission-limitations).
- Keep `git rebase --continue` and `git rebase --skip` confirmable even when `-C` names an exact agent worktree and fixed assignments disable editor interaction. The stored sequencer can contain executable instructions, while hooks and repository configuration can activate behavior absent from the normalized command. Agent-worktree scope contains repository mutation, not that hidden execution, and terminal permission does not grant Git-metadata sandbox access.

## Validate Git patterns

Validate every in-scope Git pattern against:

- Alphabetic owner-group ordering and byte-identical repeated prefix grammar.
- Applicable global options, fixed assignments, pager prefixes, and wrapper forms across every participating permission bucket.
- Git help and discovery cases that distinguish bounded installation or compiled-command output from aliases, external helpers, and user-specific configuration output.
- Rebase continuation and skip forms with agent-worktree paths, fixed assignments, hooks, separate Git-metadata sandbox outcomes, and sequencer state.
- The root, direct-subcommand, or compound-workflow owner it declares.
