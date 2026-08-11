# Permission evaluator

## Inventory terminal patterns

Never read or emit complete terminal permission arrays during a targeted task. Start with the owner-audit tool’s bounded lexical inventory:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-permission-owner-audit -- \
    --settings .config/zed/settings.json \
    --owner TOP_LEVEL_EXECUTABLE
```

The inventory reports only bucket and index IDs, decoded character counts, required boolean case settings, and bounded source-text previews. Its token-aware source search produces candidate evidence, not semantic ownership proof. Classify each result through the applicable terminal and domain policy before including it in an owner group. Exclude lexical hits owned by another command, such as a manager name inside a Corepack denial.

Treat every reported bucket and index ID as transient. It expires when the relevant arrays change, including after an edit, rebase, integration, or concurrent permission task. Rerun inventory before using an expired ID. Once a candidate capture exists, use its exact pattern identities and guarded reindexing rather than relying on the original indexes.

## Audit permission ownership

After identifying every entry in the complete semantic owner groups under review, create a version-1 owner manifest under the task-specific temporary directory. Declare each entry’s unique ID, bucket and current index, semantic owner, owner and domain-section sort keys, role, stable role-local sort key, and one normalized witness. Declare `nohup` and `xargs` child witnesses as `wrapped`. When a selected pattern is case-insensitive under the terminal policy’s verified command-specific exception, also declare a nonempty `case_insensitive_reason`. Omit that field for case-sensitive patterns. Derive this audit manifest independently from the transformation that built the candidate. Do not let one unreviewed owner map both construct and certify its own order.

Resolve semantic ownership after approved fixed assignments and wrappers:

- An approved wrapper does not become the owner unless the selected domain policy says it does. For example, optional Corepack mediation retains the actual npm, pnpm, or Yarn manager as owner, while Corepack’s own selector operations remain Corepack-owned.
- A bounded `xargs … <child>` form belongs to the child command. Root and discovery forms of `xargs` remain `xargs`-owned.
- Git requires domain-aware owners and section sort keys for root or discovery forms, each direct subcommand, and each compound workflow. A flat `git` owner is insufficient.

For a discovery entry, set `discovery_coverage` to one of these values and provide nonempty `discovery_inputs` containing the witness:

- `complete_finite` means the array is the complete finite normalized grammar represented by that entry. It enables finite duplicate-coverage findings against retained entries in the same `always_allow` manager group.
- `representative` records bounded examples for variable-operand discovery grammar without claiming completeness. Validate the complete variable grammar, hazardous operands, and near misses through matcher suites instead.

Direct and wrapped entries must omit both discovery fields. Declare complete owner groups, not selected favorable entries. When splitting a pooled pattern, determine every resulting entry’s position from the complete bucket order rather than inserting every split at the pooled object’s former index.

Run the structural audit against the candidate settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-permission-owner-audit -- \
    --settings PATH_TO_CANDIDATE_SETTINGS \
    --manifest PATH_TO_OWNER_MANIFEST
```

The audit independently checks supported wrapper-aware owner inference, witness matches, case sensitivity or a declared verified exception, decoded lengths, owner-group contiguity, declared bucket order, and complete finite discovery redundancy. It does not verify the semantic basis of `case_insensitive_reason`, prove formal regex-language equivalence, infer safety classifications, or replace matcher coverage and complete effective-permission evaluation.

Before adding a finite discovery object, test its complete normalized input set against every retained pattern in the resolved owner group and add only uncovered forms. Before removing one as redundant, require every member of its complete finite grammar to remain covered. Do not use provisional owner metadata alone as evidence of duplicate coverage.

## Build and promote a permission candidate

Write a version-1 selection document containing the authorized non-root JSON pointer scopes and every semantically owned pattern selected from the latest inventory. Capture from the latest live settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-permission-candidate -- \
    capture \
    --settings .config/zed/settings.json \
    --selection PATH_TO_SELECTION \
    --output PATH_TO_CAPTURE
```

Capture creates these artifacts without overwriting existing paths:

- Immutable exact-byte `baseline-settings.json`
- Editable exact-byte `candidate-settings.json`
- One exact decoded UTF-8 file for each selected pattern
- `state.json`, which records authorized scopes, source identities, artifact paths, and SHA-256 integrity values

Edit only `candidate-settings.json`, and only inside the authorized scopes. Do not modify the baseline, state, or captured pattern files. Extract changed candidate patterns and unchanged overlaps as exact bytes for matcher manifests without printing complete arrays or long regex bodies.

Verify the captured identities against current live settings before validation and again immediately before promotion:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-permission-candidate -- \
    verify \
    --settings .config/zed/settings.json \
    --state PATH_TO_CAPTURE/state.json
```

`verify` indexes each relevant bucket once and relocates a captured pattern only when its exact decoded bytes and case setting identify one unique current object. If a relevant rebase, integration, source-pattern edit, or same-scope concurrent change makes the candidate stale, rebuild it from the latest live settings and rerun every structural, matching, comparison, and effective-permission check. Do not reconcile stale arrays by transient index.

When concurrent owner work changed the same permission buckets, reapply only the authorized owner transformation to a new capture. Compare semantic ownership by removing the exact old owned entries from the latest baseline and the exact replacement entries from the rebuilt candidate, then require the remaining bucket objects and every out-of-scope setting to be equal. This comparison supplements, rather than bypasses, candidate validation.

Promote only after every required check passes:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-permission-candidate -- \
    promote \
    --settings .config/zed/settings.json \
    --candidate PATH_TO_CAPTURE/candidate-settings.json \
    --state PATH_TO_CAPTURE/state.json \
    --write
```

Promotion refuses candidate changes outside authorized scopes and refuses when any live authorized scope differs from the captured baseline. It merges authorized candidate scope values into the live object read for promotion so that object’s out-of-scope values survive. Immediately before atomic pathname replacement, it rechecks the live bytes and refuses an observed change. This check is best-effort rather than synchronized compare-and-swap—a writer can still replace the destination after the recheck and before rename. Avoid concurrent writers during promotion. After promotion, parse and format live settings, confirm that each promoted scope equals the validated candidate, and inspect the final scoped diff.

## Compile and match permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/zed-pattern-match.rs` for Zed-compatible regex compilation and matching. Do not use the dependency-audit tool as a pattern compiler, and do not substitute `rg`, which changes anchor and byte-matching behavior and wraps configured patterns before parsing.

For one pattern and input, write both as exact UTF-8 bytes under the task-specific temporary directory and run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-pattern-match -- \
    --case-sensitive \
    --input-file PATH_TO_INPUT \
    --pattern-file PATH_TO_PATTERN
```

Omit `--case-sensitive` only when the pattern object explicitly resolves it to `false`. For multiple single-line cases against one pattern, use `--cases-file` with LF-delimited `match<TAB><input>` and `no-match<TAB><input>` records.

For a changed or audited pattern set, prefer one suite invocation so shared patterns are read and compiled once. A suite manifest accepts these LF-delimited records in any order:

```text
decision-case<TAB>allow|confirm|deny<TAB><input>
decision-case-file<TAB>allow|confirm|deny<TAB><input-file>
default<TAB>allow|confirm|deny
pattern<TAB><id><TAB>always_allow|always_confirm|always_deny<TAB>case-sensitive|case-insensitive<TAB><pattern-file>
pattern-case<TAB><id><TAB>match|no-match<TAB><input>
pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>
```

Relative paths resolve from the suite file’s parent. Use file-backed records for inputs containing line breaks. Declare every changed pattern and every unchanged pattern from participating effective settings layers and buckets that may match a decision input. Include at least one pattern case for every declared pattern and representative required matches, hazardous forms, near misses, and precedence interactions.

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-pattern-match -- \
    --suite-file PATH_TO_SUITE
```

Suite decisions apply configured precedence to one normalized input only: deny, then confirm, then allow, then the declared default. They do not reproduce pre-rule denial checks, input derivation, multi-input aggregation, native-path normalization, or settings-layer resolution.

When supplied classifications are settled, derive representative cases from the current configured grammar. If an assumed case fails, inspect a bounded pattern slice or structural representation and correct the harness from repository evidence. Do not reopen prohibited executable or online behavioral research.

## Compare baseline and candidate behavior

Use one version-1 JSON comparison manifest to compile each baseline and candidate pattern set once and evaluate one normalized representative corpus against both. Each set declares its resolved default and pattern records containing a unique ID, bucket, case setting, and manifest-relative exact-byte pattern file. Cases are either inline single-line inputs or file-backed inputs.

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-pattern-match -- \
    --comparison-file PATH_TO_COMPARISON
```

For every case, comparison mode checks the matched state of `always_allow`, `always_confirm`, and `always_deny` separately, then compares the precedence-resolved configured decision. Expect both per-bucket and final-decision equivalence unless the task explicitly authorizes a behavioral change. A successful representative corpus comparison is not formal regex-language equivalence or complete Zed permission evaluation.

When factoring, consolidating, or reordering a finite pattern language, supplement representative comparison with the strongest bounded structural proof available. Examples include unchanged prefixes and suffixes with exact alternative-set equality, exact branch-set equality, or an unchanged pattern multiset. Compare match unions rather than pattern objects when ownership decomposition changes object counts.

## Audit Zed regex compatibility

During a documentation audit that includes [Zed permission regex compatibility](../../../PROJECT.md#zed-permission-regex-compatibility), obtain Zed’s current `main` `Cargo.lock` and short commit reference through one bounded official-source retrieval. Do not search Zed’s dependency changelog, release notes, or repository history. If the source cannot be retrieved, report the verification limitation instead of inferring compatibility.

Use `.agents/skills/domfiles-zed-settings/scripts/zed-regex-dependency-audit.rs` only to compare the exact root `Cargo.toml` pin, the version resolved for the local root package through its adjacent `Cargo.lock`, and the direct `regex` version in the retrieved upstream lockfile:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-zed-regex-dependency-audit -- \
    --local-manifest Cargo.toml \
    --upstream-lock PATH_TO_ZED_CHECKOUT/Cargo.lock \
    --upstream-revision SHORT_ZED_REVISION
```

The audit reads local files only and compares direct `regex` versions. Transitive versions, sources, checksums, and dependency edges may update independently. Treat a direct-version mismatch as a semantic-accuracy finding against the documented compatibility claim. Treat an unresolved, missing, ambiguous, or locally inconsistent direct version as a validation failure. Neither result authorizes a dependency or documentation change.

When the user explicitly authorizes a compatibility repair:

1. Verify the reported Zed revision and its locked `regex` version against the same upstream checkout.
2. Update the exact `regex` pin in `Cargo.toml` and the root `Cargo.lock`, then update the short Zed commit reference in `.agents/PROJECT.md`.
3. Run the focused contract test, the audit against that upstream lockfile, and root Rust validation.

## Run focused contract tests

```sh
cargo test --locked --test domfiles-zed-settings-permission-patterns-test
cargo test --locked --test domfiles-zed-settings-zed-pattern-match-test
cargo test --locked --test domfiles-zed-settings-zed-permission-candidate-test
cargo test --locked --test domfiles-zed-settings-zed-permission-owner-audit-test
cargo test --locked --test domfiles-zed-settings-zed-regex-dependency-audit-test
```

Run the tests for every changed script or helper and every script contract used materially by the task. These focused tests do not replace applicable root Rust validation.

## Evaluate permission behavior

Verify version-sensitive behavior against current official Zed documentation or source, then apply this sequence:

1. Build the effective permission settings.
    - Apply defaults, extension settings, the global-settings layer from `global_settings.json`, the conditional user-settings layer from `.config/zed/settings.json`, active profile settings, and server settings in that order. Within the user-settings layer, apply base user settings, user release-channel overrides, and user operating-system overrides in that order.
    - Project settings do not participate in agent permission evaluation. The user-settings layer is included when no profile is active or the active profile uses `base: "user"`, and omitted when it uses `base: "default"`.
    - Accumulate `always_deny`, `always_confirm`, and `always_allow` patterns across participating layers. A later layer cannot remove an accumulated pattern.
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
5. For native path tools, repeat configured-rule evaluation against the lexically normalized path list when normalization changes an input, then use the most restrictive result across raw and normalized decisions: deny, then confirm, then allow.

Use the bounded [pattern inventory](#inventory-terminal-patterns) before reading terminal patterns, the [owner audit](#audit-permission-ownership) for structural invariants, the [candidate workflow](#build-and-promote-a-permission-candidate) for mutation, and the [matcher](#compile-and-match-permission-patterns) for independently evaluated inputs. Use [baseline and candidate comparison](#compare-baseline-and-candidate-behavior) whenever decomposition, consolidation, deduplication, or reordering can change a bucket’s match union.

For Git, use one root command, direct subcommand, or compound workflow as the semantic comparison unit. Do not compare objects one-for-one when ownership moves between patterns.

When confirmation precedence and Rust-compatible regex limits make a narrow allowance require a fragile complement expression, leave the form confirmable and record the durable rationale in `.agents/PROJECT.md`.
