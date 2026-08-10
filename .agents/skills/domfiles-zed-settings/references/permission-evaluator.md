# Permission evaluator

## Inventory terminal patterns

Never read or emit the complete terminal permission arrays during a targeted task. Start with a command-filtered inventory that returns only a transient pattern ID, character count, case setting, and a 160-character preview around the requested command token:

```sh
jq --arg command '<command>' '
  ["always_allow", "always_confirm", "always_deny"] as $buckets
  | .agent.tool_permissions.tools.terminal as $terminal
  | [
      $buckets[] as $bucket
      | $terminal[$bucket]
      | to_entries[]
      | .value.pattern as $pattern
      | select($pattern | contains($command))
      | ($pattern | index($command)) as $offset
      | {
          id: "\($bucket)[\(.key)]",
          characters: ($pattern | length),
          case_sensitive: (.value.case_sensitive // false),
          preview: $pattern[$offset:($offset + 160)]
        }
    ]
' .config/zed/settings.json
```

Treat each ID as valid only for the inspected settings state. Read only the selected object, directly related precedence overrides, and mirrored patterns required by policy. Re-run the inventory after edits or concurrent changes. Extract a selected pattern directly into the task-specific temporary directory with `jq --join-output` rather than printing or copying its body through the conversation.

## Build and promote a permission candidate

1. Create a candidate settings file under the task-specific temporary directory from the latest live settings. Modify only the intended permission subtree. Do not install an uncompiled regex in live settings.
2. Parse the candidate and extract every changed pattern as exact bytes. Also extract every unchanged pattern from a participating settings layer or permission bucket that may match a representative decision input, and record the resolved effective default. Keep the baseline needed to compare the changed buckets, precedence decisions, and structural invariants without printing complete permission arrays.
3. Compile every extracted candidate pattern, run the complete pattern-case and configured-pattern decision suite, resolve the candidate through [Evaluate permission behavior](#evaluate-permission-behavior), and verify selected-branch invariants before promotion. Treat any failure as a candidate defect rather than testing it through live Zed permissions.
4. Immediately before promotion, reread live settings. If the intended subtree changed after candidate creation, reconcile that concurrent work and repeat candidate validation. Preserve changes elsewhere by replacing only the validated subtree rather than copying a stale whole-file candidate over live settings.
5. After promotion, parse and format the live settings, confirm that the promoted subtree matches the candidate, and inspect the final scoped diff.

## Compile permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/zed-regex-audit.rs` to compile a candidate or audited pattern set once against the pinned Zed-compatible regex engine. Extract each selected pattern as exact bytes into a separate file under the [task-specific temporary directory](../../../../.config/zed/AGENTS.md#temporary-files). Group pattern files by their resolved case setting, then run `cargo run --locked --quiet --bin domfiles-zed-settings-zed-regex-audit -- --case-sensitive --pattern-file <pattern-file> [--pattern-file <pattern-file> …]`. Omit `--case-sensitive` only for a group whose pattern objects resolve it to `false`.

Treat exit status `0` as successful compilation and any other status as a validation failure. The audit identifies an invalid pattern file and gives a concise engine reason without emitting the pattern body. Compilation does not establish whether a pattern accepts or rejects the intended inputs, so continue with the matching workflow below.

Run the focused contract test with `cargo test --locked --test domfiles-zed-settings-zed-regex-audit-test`.

## Match permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/zed-pattern-match.rs` to reproduce Zed’s `regex::RegexBuilder` compilation and matching semantics. Do not substitute `rg`, which changes anchor and byte-matching behavior and wraps configured patterns before parsing.

For one configured pattern:

1. Reject an empty configured pattern before matching. Treat it as a validation failure and, when evaluating permissions, as a denial of the affected tool.
2. Write the pattern as exact bytes under the [task-specific temporary directory](../../../../.config/zed/AGENTS.md#temporary-files), then choose the least expensive input mode.
    - For one input or data containing a line break, write the complete raw or normalized input as exact bytes without a line terminator. Run `cargo run --locked --quiet --bin domfiles-zed-settings-zed-pattern-match -- --input-file <input-file> --pattern-file <pattern-file>`.
    - For multiple single-line inputs, write an LF-delimited UTF-8 manifest whose every line is `match<TAB><input>` or `no-match<TAB><input>`. The input is everything after the first tab and may be empty. Run `cargo run --locked --quiet --bin domfiles-zed-settings-zed-pattern-match -- --cases-file <cases-file> --pattern-file <pattern-file>`.
    - Add `--case-sensitive` in either mode when the pattern object sets `case_sensitive` to `true`.
3. Treat exit status `0` as a single match or a successful batch, `1` as a single non-match or failed batch expectation, and any other status as a validation failure.

For a changed pattern set, use one suite manifest to validate every pattern-specific expectation and configured-pattern precedence decision. Define records in any order using these forms:

```text
decision-case<TAB>allow|confirm|deny<TAB><input>
decision-case-file<TAB>allow|confirm|deny<TAB><input-file>
default<TAB>allow|confirm|deny
pattern<TAB><id><TAB>always_allow|always_confirm|always_deny<TAB>case-sensitive|case-insensitive<TAB><pattern-file>
pattern-case<TAB><id><TAB>match|no-match<TAB><input>
pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>
```

Relative pattern and input paths resolve from the suite file’s parent. For an input containing a line break, write the complete input as exact bytes without an added line terminator and use the corresponding file-backed record. Keep inline records to single-line inputs.

Use one unique, nonempty pattern ID. Declare every changed candidate pattern. Also declare every unchanged candidate pattern from all participating effective settings layers and permission buckets that may match any decision-case input. Set `default` to the resolved effective default, include at least one pattern case for every declared pattern, and include at least one representative decision case. Cover required matches, hazardous forms, near misses, and precedence interactions. Run `cargo run --locked --quiet --bin domfiles-zed-settings-zed-pattern-match -- --suite-file <suite-file>`.

Suite decisions apply only configured pattern precedence to one normalized input: deny, then confirm, then allow, then the declared effective default. They do not reproduce pre-rule denial checks, input derivation, multi-input evaluation, or native-path normalization. Success prints pattern-case, decision-case, and pattern counts. Expectation failures return status `1` and report at most 10 manifest line numbers without pattern bodies or inputs. Invalid manifests, unreadable or invalid patterns, and I/O failures return status `2`.

Resolve any conflict with current Zed source or observed Zed behavior in favor of Zed. Run the focused contract test with `cargo test --locked --test domfiles-zed-settings-zed-pattern-match-test`.

## Audit Zed regex compatibility

During a documentation audit that includes [Zed permission regex compatibility](../../../PROJECT.md#zed-permission-regex-compatibility), obtain Zed’s current `main` `Cargo.lock` and short commit reference through one bounded official-source retrieval. Do not search Zed’s dependency changelog, release notes, or repository history. If the source cannot be retrieved, report the verification limitation instead of inferring compatibility.

Use `.agents/skills/domfiles-zed-settings/scripts/zed-regex-audit.rs` to compare the root `Cargo.toml` pin and its adjacent `Cargo.lock` dependency closure with the retrieved lockfile. For the local graph, the script identifies the source-less root package from the manifest, resolves `regex` through that package’s lockfile dependency reference, and then walks the locked dependency graph. It compares dependency edges plus package versions, sources, and checksums. It reads local files only and never retrieves upstream source or modifies repository state. Run it with `cargo run --locked --quiet --bin domfiles-zed-settings-zed-regex-audit -- --local-manifest Cargo.toml --upstream-lock <zed-checkout>/Cargo.lock --upstream-revision <short-zed-revision>`. Its focused contract test is defined in [Compile permission patterns](#compile-permission-patterns).

Treat a direct-version, dependency-edge, or dependency-package mismatch as a semantic-accuracy finding against the documented compatibility claim. Treat a source-less package in either dependency closure as a validation failure because the lockfile cannot establish its code identity. A matching locked dependency closure produces no finding. Neither result authorizes a dependency or documentation change.

When the user explicitly authorizes a compatibility repair:

1. Verify the reported Zed revision and its locked `regex` dependency closure against the same upstream checkout.
2. Update the exact `regex` pin in `Cargo.toml` and the root `Cargo.lock`, then update the short Zed commit reference in `.agents/PROJECT.md` to the verified revision.
3. Run the focused contract test, the audit against that upstream `Cargo.lock`, and the root Rust validation.

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

Use the [pattern inventory](#inventory-terminal-patterns) before reading terminal patterns, the [compilation workflow](#compile-permission-patterns) for configured pattern sets, and the [pattern-matching workflow](#match-permission-patterns) for independently evaluated inputs.

When decomposing or consolidating patterns, compare old and new match unions separately within `always_deny`, `always_confirm`, and `always_allow` over representative inputs. Then resolve the final decision through normal precedence and compare the old and new results. Require both per-bucket and precedence-resolved decision equivalence unless the current task explicitly authorizes a behavior change. For Git, use one root command, direct subcommand, or compound workflow as the comparison unit. Do not compare objects one-for-one when ownership moves between patterns.

When confirmation precedence and Rust-compatible regex limits make a narrow allowance require a fragile complement expression, leave the form confirmable and record the durable rationale in `.agents/PROJECT.md`.
