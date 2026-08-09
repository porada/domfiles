# Permission evaluator

## Smoke-test permission patterns

Use `rg` only as a bounded, read-only smoke test. Ripgrep does not reproduce Zed’s whole-input `regex::RegexBuilder` semantics: it changes anchor behavior, uses a byte-oriented matcher, and wraps configured patterns before parsing. Never treat an `rg` result as authoritative for an engine-sensitive pattern or input.

1. Reject an empty configured pattern before invoking `rg`. Treat it as a validation failure and, when evaluating permissions, as a denial of the affected tool.
2. Use this smoke test only when both the pattern and input are nonempty, single-line ASCII without NUL bytes, and the pattern uses no inline regex flags other than scoped case-insensitivity. Consult current Zed source or verify behavior in a running Zed instance for every other case.
3. Write one pattern and one complete raw or normalized input to separate files in the [task-specific temporary directory](../../../../.config/zed/AGENTS.md#temporary-files). Do not append synthetic line terminators or combine multiple inputs in one file.
4. Invoke `rg --case-sensitive --file <pattern-file> --no-config --quiet --text <input-file>` when the pattern object sets `case_sensitive` to `true`. Otherwise, replace `--case-sensitive` with `--ignore-case` to reproduce Zed’s default.
5. Treat exit status `0` as supporting evidence of a match, `1` as supporting evidence of no match, and any other status as a validation failure. Resolve any conflict between `rg` and current Zed source or observed Zed behavior in favor of Zed.

## Evaluate permission behavior

Verify version-sensitive behavior against current official Zed documentation or source, then apply this sequence:

1. Build the effective permission settings.
    - Apply defaults, extension settings, the global-settings layer from `global_settings.json`, the conditional user-settings layer from `.config/zed/settings.json`, active profile settings, and server settings in that order. Within the user-settings layer, apply base user settings, user release-channel overrides, and user operating-system overrides in that order.
    - Project settings do not participate in agent permission evaluation. The user-settings layer is included when no profile is active or the active profile uses `base: "user"`, and omitted when it uses `base: "default"`.
    - Accumulate `always_deny`, `always_confirm`, and `always_allow` patterns across the participating layers. A later layer cannot remove an individual accumulated pattern.
    - Resolve the tool-specific `default`, when configured, and otherwise use `agent.tool_permissions.default`.
2. Apply pre-rule denial checks in order.
    - Apply terminal hardcoded security rules to each raw input and, for supported shells, its extracted subcommands.
    - Deny the selected tool when any configured regex pattern is empty or failed to compile.
    - Deny unsafe or unsupported terminal syntax before configured patterns unless the effective terminal default is `allow` and no `always_deny` or `always_confirm` restrictions exist.
3. Derive every independently evaluated input.
    - For terminal, extract normalized commands, nested commands, and non-`/dev/null` file redirections. If extraction or shell support is insufficient, disable `always_allow`. Deny an incompatible shell when allow patterns are configured.
    - For native path tools, retain every raw path. Include both source and destination for `copy_path` and `move_path`.
    - For other tools, use each original text input.
4. Evaluate all derived inputs together. Any `always_deny` match denies. Otherwise, any `always_confirm` match confirms. Otherwise, `always_allow` allows only when allow matching is enabled and every input matches an allowance. Fall back to the resolved tool-specific or global default.
5. For native path tools, repeat the configured-rule evaluation against the lexically normalized path list when normalization changes an input, then use the most restrictive result across raw and normalized decisions: deny, then confirm, then allow.

Use the [bounded smoke-test workflow](#smoke-test-permission-patterns) for eligible configured patterns and independently evaluated inputs.

When consolidating patterns, compare the union of the old family’s matches with the union of the new family’s matches over representative inputs. Do not compare objects one-for-one when ownership moved between patterns.

When confirmation precedence and Rust-compatible regex limits make a narrow allowance require a fragile complement expression, leave the form confirmable and record the durable rationale in `.agents/PROJECT.md`.
