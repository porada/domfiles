# Permission evaluator

## Inventory terminal patterns

Never read or emit complete terminal permission arrays during a targeted task. Start with the owner-audit tool’s bounded lexical inventory:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings .config/zed/settings.json \
    --owner '<top-level-executable>'
```

The inventory reports only bucket and index IDs, decoded Unicode-scalar counts, required boolean case settings, and bounded source-text previews. Its token-aware source search produces candidate evidence, not semantic ownership proof. Classify each result through the applicable terminal and domain policy before including it in an owner group. Exclude lexical hits owned by another command, such as a manager name inside a Corepack denial.

Treat every reported bucket and index ID as transient. It expires when the relevant arrays change, including after an edit, rebase, integration, or concurrent permission task. Rerun inventory before using an expired ID. Once a candidate capture exists, use its exact pattern identities and guarded reindexing rather than relying on the original indexes.

## Audit permission ownership

After identifying every entry in the complete semantic owner groups under review, run the owner-audit binary with exact `--help` and use the strict manifest schema it prints as the canonical field contract. Create that manifest under the task-specific temporary directory. Declare each entry’s unique ID, bucket and current index, semantic owner, owner and domain-section sort keys, role, stable role-local sort key, and one normalized witness. Declare `nohup` and `xargs` child witnesses as `wrapped`. When a selected pattern is case-insensitive under the terminal policy’s verified command-specific exception, also declare a nonempty `case_insensitive_reason`. Omit that field for case-sensitive patterns. Derive this audit manifest independently from the transformation that built the candidate. Do not let one unreviewed owner map both construct and certify its own order.

Resolve semantic ownership through the [terminal command-owner policy](terminal-permissions.md#apply-the-terminal-permission-policy) and each selected domain policy before encoding the manifest. Apply the [Git owner partition](git-permissions.md#apply-the-git-permission-policy) to Git owner and section sort keys, the [Node manager boundaries](node-package-manager-permissions.md#apply-manager-boundaries) to Corepack mediation, and the terminal policy’s wrapper ownership to `xargs`.

For a discovery entry, set `discovery_coverage` to one of these values and provide nonempty `discovery_inputs` containing the witness:

- `complete_finite` means the array is the complete finite normalized grammar represented by that entry. It enables finite duplicate-coverage findings against retained entries in the same `always_allow` manager group.
- `representative` records bounded examples for variable-operand discovery grammar without claiming completeness. Validate the complete variable grammar, hazardous operands, and near misses through matcher suites instead.

Direct and wrapped entries must omit both discovery fields. Encode every entry’s final bucket position after applying the [terminal owner-ordering policy](terminal-permissions.md#apply-the-terminal-permission-policy) and any selected domain order.

The audit requires complete occupancy for each `(bucket, independently inferred semantic owner, inferred Git repository scope)` span. Repository scope is one of:

- `general` for non-Git witnesses, absent or ambiguous Git `-C` paths, unsupported wrapper forms, multiple `-C` operands, absolute paths, malformed agent names, and paths containing empty, current-directory, or parent-directory components.
- `agent worktree` for one exact traversal-free project-relative `-C .agent-<name>` path.
- `fixture repository` for one traversal-free strict descendant `-C .agent-<name>/<path>`.

Assignments, supported `nohup`, and supported `xargs` child syntax participate in owner and repository-scope inference. Childless and discovery-only `xargs` forms remain in the general `xargs` span. The declared discovery, direct, or wrapped role and `section_sort_key` participate in ordering but never partition completeness.

Run the structural audit against the candidate settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings '<candidate-settings-path>' \
    --manifest '<owner-manifest-path>'
```

The audit independently checks supported wrapper-aware owner inference, witness matches, case sensitivity or a declared verified exception, decoded lengths, owner-group contiguity, declared bucket order, and complete finite discovery redundancy. It does not verify the semantic basis of `case_insensitive_reason`, prove formal regex-language equivalence, infer safety classifications, or replace matcher coverage and complete effective-permission evaluation.

Before adding a finite discovery object, test its complete normalized input set against every retained pattern in the resolved owner group and add only uncovered forms. Before removing one as redundant, require every member of its complete finite grammar to remain covered. Do not use provisional owner metadata alone as evidence of duplicate coverage.

## Build and promote a permission candidate

Run the candidate binary with exact `--help` and use its strict capture-selection schema as the canonical field contract. Write a selection document containing the authorized non-root JSON pointer scopes and every semantically owned pattern selected from the latest inventory. Capture from the latest live settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    capture \
    --settings .config/zed/settings.json \
    --selection '<selection-path>' \
    --output '<capture-path>'
```

Capture creates these artifacts without overwriting existing paths:

- Immutable exact-byte `baseline-settings.json`
- Editable exact-byte `candidate-settings.json`
- One exact decoded UTF-8 file for each selected pattern
- `state.json`, which records authorized scopes, source identities, artifact paths, and SHA-256 integrity values

Edit only `candidate-settings.json`, and only inside the authorized scopes. Do not modify the baseline, state, or captured pattern files.

After the candidate reaches its final validation form, write a strict version-1 materialization selection using transient candidate bucket/index locators and stable logical IDs:

```json
{
    "version": 1,
    "patterns": [
        {
            "id": "<logical-pattern-id>",
            "bucket": "always_allow",
            "index": 0
        }
    ]
}
```

Unknown fields, empty selections or IDs, duplicate IDs, and duplicate bucket/index pairs are invalid. Materialize every changed candidate pattern and unchanged participating overlap needed by matcher manifests:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    materialize \
    --candidate '<candidate-settings-path>' \
    --state '<state-path>' \
    --selection '<materialization-selection-path>' \
    --output '<materialization-directory>'
```

Materialization validates the complete captured state first, requires every candidate value outside the authorized scopes to equal the baseline, and derives each selected pattern’s bucket, source index, case setting, and exact decoded UTF-8 bytes from the candidate object. It writes exact pattern files without a newline, normalization, quoting, or reserialization plus `artifact-catalog.json`:

```json
{
    "version": 1,
    "candidate_sha256": "<64-lowercase-hex>",
    "state_sha256": "<64-lowercase-hex>",
    "patterns": [
        {
            "id": "<logical-pattern-id>",
            "bucket": "always_allow",
            "source_index": 0,
            "case_sensitive": true,
            "sha256": "<64-lowercase-hex>",
            "pattern_file": "<safe-relative-path>"
        }
    ]
}
```

The catalog requires a nonempty pattern array, unique IDs, bucket/source-index pairs, and safe relative artifact paths, plus valid lowercase SHA-256 values. Candidate and state hashes bind the complete exact source bytes, while artifact hashes bind exact pattern files. These hashes establish integrity and freshness, not authenticity. Materialization preflights every output, refuses symlink traversal and overwrites, uses create-new writes with safe generated names, and rolls back created artifacts after a later failure. It leaves candidate, baseline, state, and live settings untouched.

Verify the captured identities against current live settings before validation and again immediately before promotion:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    verify \
    --settings .config/zed/settings.json \
    --state '<capture-path>/state.json'
```

`verify` indexes each relevant bucket once and relocates a captured pattern only when its exact decoded bytes and case setting identify one unique current object. If a relevant rebase, integration, source-pattern edit, or same-scope concurrent change makes the candidate stale, rebuild it from the latest live settings and rerun every structural, matching, comparison, and effective-permission check. Do not reconcile stale arrays by transient index.

When concurrent owner work changed the same permission buckets, reapply only the authorized owner transformation to a new capture. Compare semantic ownership by removing the exact old owned entries from the latest baseline and the exact replacement entries from the rebuilt candidate, then require the remaining bucket objects and every out-of-scope setting to be equal. This comparison supplements, rather than bypasses, candidate validation.

Promote only after every required check passes:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    promote \
    --settings .config/zed/settings.json \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --write
```

Promotion refuses candidate changes outside authorized scopes and refuses when any live authorized scope differs from the captured baseline. It merges authorized candidate scope values into the live object read for promotion so that object’s out-of-scope values survive. Immediately before atomic pathname replacement, it rechecks the live bytes and refuses an observed change. This check is best-effort rather than synchronized compare-and-swap—a writer can still replace the destination after the recheck and before rename. Avoid concurrent writers during promotion. After promotion, confirm that each promoted scope equals the validated candidate.

## Compile and match permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/pattern-match.rs` for Zed-compatible regex compilation and matching. Do not use the dependency-audit tool as a pattern compiler, and do not substitute `rg`, which changes anchor and byte-matching behavior and wraps configured patterns before parsing.

For one pattern and input, write both as exact UTF-8 bytes under the task-specific temporary directory and run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --case-sensitive \
    --input-file '<input-path>' \
    --pattern-file '<pattern-path>'
```

Omit `--case-sensitive` only when the pattern object explicitly resolves it to `false`. For multiple single-line cases against one pattern, use `--cases-file` with LF-delimited `match<TAB><input>` and `no-match<TAB><input>` records.

For a changed or audited pattern set, prefer one suite invocation so shared patterns are read and compiled once. A suite manifest accepts these LF-delimited records in any order:

```text
decision-case<TAB>allow|confirm|deny<TAB><input>
decision-case-file<TAB>allow|confirm|deny<TAB><input-file>
default<TAB>allow|confirm|deny
catalog-pattern<TAB><catalog-id><TAB><pattern-id>
pattern<TAB><id><TAB>always_allow|always_confirm|always_deny<TAB>case-sensitive|case-insensitive<TAB><pattern-file>
pattern-catalog<TAB><catalog-id><TAB><catalog-file><TAB><candidate-file><TAB><state-file>
pattern-case<TAB><id><TAB>match|no-match<TAB><input>
pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>
```

Records may appear in any order. A suite requires exactly one `default`, at least one ordinary or catalog-backed pattern, at least one decision case, and at least one pattern case for every pattern ID. Suite declaration, candidate, state, ordinary-pattern, and input paths resolve from the suite file’s parent. Catalog artifact paths resolve from the catalog’s parent. A catalog-backed pattern uses the catalog entry as the sole source of its bucket, case setting, exact artifact path, and logical pattern ID. Catalog IDs and pattern IDs must remain unique, including across ordinary and catalog-backed patterns. Catalog declarations alone do not count as patterns, and catalog-backed patterns retain the ordinary pattern-case coverage requirement. Suites may use catalogs without comparison mode or transitions.

Use file-backed records for inputs containing line breaks. Declare every changed pattern and every unchanged pattern from participating effective settings layers and buckets that may match a decision input. Include at least one pattern case for every declared pattern. For every changed or audited pattern set, add representative intended inputs, hazardous forms, and near misses even when no narrower permission branch applies, plus the additional cases required by each selected branch and any precedence interactions.

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --suite-file '<suite-path>'
```

Suite decisions apply configured precedence to one normalized input only: deny, then confirm, then allow, then the declared default. They do not reproduce pre-rule denial checks, input derivation, multi-input aggregation, native-path normalization, or settings-layer resolution.

When supplied classifications are settled, derive representative cases from the current configured grammar. If an assumed case fails, inspect a bounded pattern slice or structural representation and correct the harness from repository evidence. Do not reopen prohibited executable or online behavioral research.

## Compare baseline and candidate behavior

Use one JSON comparison manifest to compile each baseline and candidate pattern set once and evaluate one normalized representative corpus against both:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --comparison-file '<comparison-path>'
```

Version 1 remains the strict equivalence-only contract. Each set declares its resolved default and pattern records containing a unique ID, bucket, case setting, and manifest-relative exact-byte pattern file. Cases are inline single-line inputs or file-backed inputs. The matcher preserves version-1 parsing, statuses, diagnostics, mismatch rendering, and success output.

Use strict version 2 when a corpus contains authorized behavior transitions or catalog-backed patterns. A minimal ordinary-pattern manifest has this shape:

```json
{
    "version": 2,
    "catalogs": [],
    "baseline": {
        "default": "deny",
        "patterns": [
            {
                "type": "file",
                "id": "baseline-example",
                "bucket": "always_confirm",
                "case_sensitive": true,
                "pattern_file": "baseline-example.regex"
            }
        ]
    },
    "candidate": {
        "default": "deny",
        "patterns": [
            {
                "type": "file",
                "id": "candidate-example",
                "bucket": "always_confirm",
                "case_sensitive": true,
                "pattern_file": "candidate-example.regex"
            }
        ]
    },
    "cases": [
        {
            "type": "inline",
            "input": "example"
        }
    ]
}
```

`catalogs` is required and may be empty for ordinary-only comparisons. Catalog declarations use this strict shape:

```json
{
    "id": "<catalog-id>",
    "catalog_file": "<manifest-relative-path>",
    "candidate_file": "<manifest-relative-path>",
    "state_file": "<manifest-relative-path>"
}
```

Version-2 patterns are explicitly tagged. Ordinary patterns declare matcher-owned metadata:

```json
{
    "type": "file",
    "id": "<pattern-id>",
    "bucket": "always_confirm",
    "case_sensitive": true,
    "pattern_file": "<manifest-relative-path>"
}
```

Catalog-backed patterns declare only stable references because the catalog supplies bucket, case setting, exact artifact path, and logical ID:

```json
{
    "type": "catalog",
    "catalog_id": "<catalog-id>",
    "pattern_id": "<catalog-pattern-id>"
}
```

A version-2 inline or file-backed case without `expected_transition` requires complete baseline/candidate equivalence across all three buckets and the final configured decision. An intentional change must declare both complete states:

```json
{
    "type": "inline",
    "input": "<single-line-input>",
    "expected_transition": {
        "baseline": {
            "always_allow": false,
            "always_confirm": true,
            "always_deny": false,
            "final_decision": "confirm"
        },
        "candidate": {
            "always_allow": true,
            "always_confirm": false,
            "always_deny": false,
            "final_decision": "allow"
        }
    }
}
```

All eight state fields are mandatory. The matcher recomputes each declared `final_decision` from that side’s bucket booleans and configured default using deny, confirm, allow, then default precedence. Incomplete or contradictory declarations, no-op transitions, malformed or stale catalogs, and artifact failures are invalid data with status `2`. Observed mismatches, undeclared drift, and transition-side mismatches use status `1`. Bucket drift remains visible even when precedence preserves the final decision.

Version-2 declarations and ordinary paths resolve from the comparison manifest’s parent, while artifact paths resolve from the catalog’s parent. Before compilation, the matcher loads each catalog once, verifies its candidate and state hashes, verifies every artifact hash and exact UTF-8 file, and then compiles with catalog-owned metadata. It never extracts settings or interprets candidate authorization.

Failure output retains at most 10 case details and reports only the case ordinal, affected side, and differing bucket or final-decision dimensions, without inputs or pattern bodies. A successful version-2 summary distinguishes equivalence cases from matched transitions and describes only a representative corpus.

For mechanical ordinary-pattern migration from version 1, change `version` to `2`, add `"catalogs": []`, add `"type": "file"` to each pattern, and leave unchanged cases without `expected_transition`.

For every version, comparison checks the matched state of `always_allow`, `always_confirm`, and `always_deny` separately before resolving the configured decision. A successful representative corpus comparison is not formal regex-language equivalence or complete Zed permission evaluation.

When factoring, consolidating, or reordering a finite pattern language, supplement representative comparison with the strongest bounded structural proof available. Examples include unchanged prefixes and suffixes with exact alternative-set equality, exact branch-set equality, or an unchanged pattern multiset. Compare match unions rather than pattern objects when ownership decomposition changes object counts.

## Audit Zed regex compatibility

During a documentation audit that includes [Zed permission regex compatibility](../../../PROJECT.md#zed-permission-regex-compatibility), obtain Zed’s current `main` `Cargo.lock` and short commit reference through one bounded official-source retrieval. Do not search Zed’s dependency changelog, release notes, or repository history. If the source cannot be retrieved, report the verification limitation instead of inferring compatibility.

Use `.agents/skills/domfiles-zed-settings/scripts/regex-dependency-audit.rs` only to compare the exact root `Cargo.toml` pin, the version resolved for the local root package through its adjacent `Cargo.lock`, and the direct `regex` version in the retrieved upstream lockfile:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-regex-dependency-audit -- \
    --local-manifest Cargo.toml \
    --upstream-lock '<zed-checkout-path>/Cargo.lock' \
    --upstream-revision '<short-zed-revision>'
```

The audit reads local files only and compares direct `regex` versions. Transitive versions, sources, checksums, and dependency edges may update independently. Treat a direct-version mismatch against current Zed `main` as compatibility drift from the documented maintenance baseline. Treat evidence that contradicts the recorded Zed revision or the local pin and lock state as a semantic-accuracy finding. Treat an unresolved, missing, ambiguous, or locally inconsistent direct version as a validation failure. Neither result authorizes a dependency or documentation change.

When the user explicitly authorizes a compatibility repair:

1. Verify the reported Zed revision and its locked `regex` version against the same upstream checkout.
2. Update the exact `regex` pin in `Cargo.toml` and the root `Cargo.lock`, then update the short Zed commit reference in `.agents/PROJECT.md`.
3. Run the focused contract test, the audit against that upstream lockfile, and root Rust validation.

## Run focused contract tests

```sh
cargo test --locked --test domfiles-zed-settings-permission-patterns-test
cargo test --locked --test domfiles-zed-settings-pattern-match-test
cargo test --locked --test domfiles-zed-settings-permission-candidate-test
cargo test --locked --test domfiles-zed-settings-permission-owner-audit-test
cargo test --locked --test domfiles-zed-settings-regex-dependency-audit-test
```

Select the applicable commands above and remaining focused and root checks through the [skill-owned script validation policy](../../agent-documentation/references/skill-owned-scripts.md#test-the-contracts).

## Evaluate permission behavior

After resolving version-sensitive behavior through the parent [investigation workflow](../SKILL.md#investigate-and-plan), apply this sequence:

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

For Git, use one root command, direct subcommand, or compound workflow as the semantic comparison unit. Do not compare objects one-for-one when ownership moves between patterns.

When confirmation precedence and Rust-compatible regex limits make a narrow allowance require a fragile complement expression, leave the form confirmable and record the durable rationale in `.agents/PROJECT.md`.
