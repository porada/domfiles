# Permission evaluator

## Inventory terminal patterns

Never read or emit complete terminal permission arrays during a targeted task. Start with the owner-audit tool’s bounded lexical inventory:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings .config/zed/settings.json \
    --owner '<top-level-executable>'
```

Each inventory page reports the exact settings SHA-256, at most 100 bucket and index IDs, decoded Unicode-scalar counts, required boolean case settings, and bounded source-text previews. Its token-aware source search produces candidate evidence, not semantic ownership proof. When another page remains, rerun the same command with the reported opaque `--after '<inventory-cursor>'`. The cursor is bound to the exact settings bytes and inventory owner. Any settings change or different owner invalidates it and requires restarting from the first page.

Classify each lexical candidate through the applicable terminal and domain policy. Include candidates owned by the inventory owner in its complete semantic owner groups. Classify lexical hits owned by another command, such as a manager name inside a Corepack denial, as explicit exclusions. Treat every reported bucket and index ID as transient. It expires after an edit, rebase, integration, concurrent permission task, or other relevant-array change. Once a candidate capture exists, use its exact pattern identities and guarded reindexing rather than relying on the original indexes.

## Audit permission ownership

After classifying every lexical candidate, run the owner-audit binary with exact `--help` and use its single strict manifest schema as the canonical field contract. Create that manifest under the task-specific temporary directory. Bind `settings_sha256` to the exact candidate settings bytes and set `inventory_owner` to the inventoried top-level executable. Declare each inventory-owned entry’s unique ID, bucket and current index, semantic owner, owner and domain-section sort keys, role, stable role-local sort key, and one normalized witness. Declare every excluded lexical candidate’s bucket, index, semantic owner outside `inventory_owner`, normalized matching witness, and nonempty semantic reason. The entry and exclusion positions must be disjoint and together classify the complete recomputed lexical inventory.

Declare `nohup` and `xargs` child entry witnesses as `wrapped`. When a selected pattern is case-insensitive under the terminal policy’s verified command-specific exception, also declare a nonempty `case_insensitive_reason`. Omit that field for case-sensitive patterns. Derive the audit manifest independently from the transformation that built the candidate. Do not let one unreviewed owner map both construct and certify its own order.

Resolve semantic ownership through the [terminal command-owner policy](terminal-permissions.md#apply-the-terminal-permission-policy) and each selected domain policy before encoding the manifest. Apply the [Git owner partition](git-permissions.md#apply-the-git-permission-policy) to Git owner and section sort keys, the [Node manager boundaries](node-package-manager-permissions.md#apply-manager-boundaries) to Corepack mediation, and the terminal policy’s wrapper ownership to `xargs`.

For a discovery entry, set `discovery_coverage` to one of these values and provide nonempty `discovery_inputs` containing the witness:

- `complete_finite` means the array is the complete finite normalized grammar represented by that entry. It enables finite duplicate-coverage findings against retained entries in the same `always_allow` manager group.
- `representative` records bounded examples for variable-operand discovery grammar without claiming completeness. Validate the complete variable grammar, hazardous operands, and near misses through matcher suites instead.

Direct and wrapped entries must omit both discovery fields. Encode every entry’s final bucket position after applying the [terminal owner-ordering policy](terminal-permissions.md#apply-the-terminal-permission-policy) and any selected domain order.

The audit requires complete occupancy for each `(bucket, independently inferred semantic owner, inferred Git repository scope)` span. Repository scope is one of:

- `general` for non-Git witnesses, absent or ambiguous Git `-C` paths, unsupported wrapper forms, multiple `-C` operands, absolute paths, malformed agent names, and paths containing empty, current-directory, or parent-directory components.
- `agent worktree` for one exact traversal-free project-relative `-C .agent-<name>` path.
- `fixture repository` for one traversal-free strict descendant `-C .agent-<name>/<path>`.

One Git ordering exception permits a discovery entry to be separated from a later direct or wrapped entry when both have the same independently inferred Git owner and repository scope and every intervening manifest entry independently infers to a Git owner. Non-Git owner groups remain contiguous. This exception affects ordering separation only and does not partition completeness.

Assignments, supported `nohup`, and supported `xargs` child syntax participate in owner and repository-scope inference. Childless and discovery-only `xargs` forms remain in the general `xargs` span. The declared discovery, direct, or wrapped role and `section_sort_key` participate in ordering but never partition completeness.

Run the structural audit against the candidate settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings '<candidate-settings-path>' \
    --manifest '<owner-manifest-path>'
```

The audit verifies the exact settings snapshot, complete lexical-candidate classification, outside-owner exclusion witnesses, supported wrapper-aware owner inference, witness matches, case sensitivity or a declared verified exception, decoded lengths, owner-group contiguity subject to the Git ordering exception above, declared bucket order, and complete finite discovery redundancy. It does not verify exclusion reasons or the semantic basis of `case_insensitive_reason`, prove formal regex-language equivalence, infer safety classifications, or replace matcher coverage and complete effective-permission evaluation.

Before adding a finite discovery object, test its complete normalized input set against every retained pattern in the resolved owner group and add only uncovered forms. Before removing one as redundant, require every member of its complete finite grammar to remain covered. Do not use provisional owner metadata alone as evidence of duplicate coverage.

## Build and promote a permission candidate

Run the candidate binary with exact `--help` and use its strict schemas as the canonical field contracts. A capture selection must contain at least one authorized non-root JSON pointer scope. Its terminal pattern selection may be empty for a scope-only candidate or an insertion-only owner with no baseline sources. For a terminal owner replacement, select every source in the complete baseline owner group from the latest inventory. Capture from the latest live settings:

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
- One exact decoded UTF-8 file for each selected terminal pattern
- `state.json`, which records authorized scopes, source identities, artifact paths, and SHA-256 integrity values

The state’s pattern array may be empty when authorized scopes are nonempty. Capture selections and state manifests reject unknown fields, including `version`. Edit only `candidate-settings.json`, and only inside the authorized scopes. Do not modify the baseline, state, or captured pattern files.

After the candidate reaches its final validation form, write a strict materialization selection using transient candidate bucket/index locators, stable logical IDs, and an explicit owner role:

```json
{
    "patterns": [
        {
            "id": "<logical-pattern-id>",
            "bucket": "always_allow",
            "index": 0,
            "owner_replacement": true
        }
    ]
}
```

`owner_replacement: true` marks a candidate source as part of the complete replacement owner. `false` marks a validation-only overlap that must remain in the candidate remainder. The pattern array may be empty for a scope-only candidate or a delete-all owner replacement. Unknown fields, including `version`, are invalid, as are missing or nonboolean owner roles, empty IDs, duplicate IDs, and duplicate bucket/index pairs.

Materialize every candidate replacement-owner source and every unchanged participating overlap needed by matcher manifests:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    materialize \
    --candidate '<candidate-settings-path>' \
    --state '<state-path>' \
    --selection '<materialization-selection-path>' \
    --output '<materialization-directory>'
```

Materialization validates the complete captured state first, requires every candidate value outside the authorized scopes to equal the baseline, and derives each selected pattern’s bucket, source index, case setting, exact decoded UTF-8 bytes, and owner role from the candidate and selection. It writes exact pattern files without a newline, normalization, quoting, or reserialization plus a strict `artifact-catalog.json`:

```json
{
    "candidate_sha256": "<64-lowercase-hex>",
    "state_sha256": "<64-lowercase-hex>",
    "patterns": [
        {
            "id": "<logical-pattern-id>",
            "bucket": "always_allow",
            "source_index": 0,
            "case_sensitive": true,
            "owner_replacement": true,
            "sha256": "<64-lowercase-hex>",
            "pattern_file": "<safe-relative-path>"
        }
    ]
}
```

The catalog’s pattern array may be empty. Every entry requires `owner_replacement`, and unknown fields—including `version`—are rejected. IDs, bucket/source-index pairs, and safe relative artifact paths must be unique where required, and every hash must be lowercase SHA-256. Candidate and state hashes bind their complete exact source bytes, while artifact hashes bind exact pattern files. These hashes establish integrity and freshness, not authenticity. Use the same untouched catalog, candidate, and state for matcher validation and promotion. Accepting a catalog does not prove that matcher validation ran.

Materialization preflights every output, refuses symlink traversal and overwrites, uses create-new writes with safe generated names, and rolls back created artifacts after a later failure. It leaves candidate, baseline, state, and live settings untouched.

Verify the captured state against current live settings before validation and again immediately before promotion:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    verify \
    --settings .config/zed/settings.json \
    --state '<capture-path>/state.json'
```

When the state contains terminal patterns, `verify` indexes each relevant bucket once and relocates a captured pattern only when its exact decoded bytes and case setting identify one unique current object. A scope-only state instead requires every current authorized scope to equal its captured baseline scope. If a relevant rebase, integration, source-pattern edit, or same-scope concurrent change makes the candidate stale, rebuild it from the latest live settings and rerun every structural, matching, comparison, and effective-permission check. Do not reconcile stale arrays by transient index.

For a terminal owner replacement, the state patterns define the complete baseline owner and the catalog entries marked `owner_replacement: true` define the complete candidate replacement owner. Promotion removes those exact sources in one pass per bucket, preserving every remaining object and its relative order, then requires the complete baseline and candidate remainders to be equal. Catalog entries marked `false` remain in the candidate remainder as validation-only overlaps. This contract supports one-to-many replacement, cross-bucket movement, duplicate decoded identities selected by exact source index, and delete-all replacement. It rejects undeclared additions, removals, object edits, reorders, and omitted retained-owner entries.

For an insertion-only owner, capture no baseline patterns and mark every new owner source as `owner_replacement: true` in the materialization selection. Promotion removes those exact candidate sources and requires each remaining terminal pattern array to equal its captured baseline. Validation-only catalog entries remain in the candidate remainder. With no insertion sources, every terminal pattern array must remain unchanged. A remainder mismatch refuses promotion, while a missing or malformed required array is invalid data.

Promote only after every required owner, matcher, comparison, effective-permission, and freshness check passes:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    promote \
    --settings .config/zed/settings.json \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --catalog '<materialization-directory>/artifact-catalog.json' \
    --write
```

Promotion requires exactly one `--catalog` and the exact `--write` guard. Before inspecting live settings, it validates the complete state and its artifacts, reads and parses the exact candidate bytes once, verifies the strict catalog’s exact candidate and state hashes and artifacts, and verifies every catalog entry’s authorized candidate pointer, decoded bytes, and case setting. Catalog schema, binding, artifact, and source-identity failures are invalid data with status `2`.

Promotion then validates the live destination and authorized-scope freshness, refuses candidate changes outside authorized scopes, and applies the mandatory terminal-array or owner-remainder comparison above. Live drift, candidate authorization, terminal-array or insertion-only remainder drift without captured owner patterns, owner-remainder, and observed replacement-drift refusals use status `1`. Authorized candidate scope values are merged into the live object read for promotion so its out-of-scope values survive.

Byte-identical output leaves live settings untouched. Changed output is written to a create-new same-directory sibling, assigned the live file’s permissions, synced, and atomically renamed. Immediately before rename, promotion rechecks the live bytes once and refuses an observed change. This remains a best-effort check rather than synchronized compare-and-swap: an uncooperative writer can still replace the destination after the recheck and before rename. Avoid concurrent writers during promotion. After promotion, confirm that every promoted scope equals the validated candidate.

## Compile and match permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/pattern_match.rs` for Zed-compatible regex compilation and matching. Do not use the dependency-audit tool as a pattern compiler, and do not substitute `rg`, which changes anchor and byte-matching behavior and wraps configured patterns before parsing.

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

Records may appear in any order. A suite requires exactly one `default`, at least one ordinary or catalog-backed pattern, at least one decision case, and at least one pattern case for every pattern ID. Suite declaration, candidate, state, ordinary-pattern, and input paths resolve from the suite file’s parent. Catalog artifact paths resolve from the catalog’s parent. Pattern catalogs use the strict artifact catalog schema, every catalog entry requires boolean `owner_replacement`, and unknown fields—including `version`—are rejected. A catalog may contain no patterns, but its declaration does not count as a matcher pattern. A catalog-backed pattern uses the catalog entry as the sole source of its bucket, case setting, exact artifact path, and logical pattern ID. Catalog IDs and pattern IDs must remain unique, including across ordinary and catalog-backed patterns. Catalog-backed patterns retain the ordinary pattern-case coverage requirement. Suites may use catalogs without comparison mode or transitions.

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

The strict comparison schema supports equivalence cases, authorized transitions, catalog-backed patterns, and separate defaults for each side. Either pattern set may be empty, but at least one comparison case is mandatory. An empty side has no matches in `always_allow`, `always_confirm`, or `always_deny`, so its final decision comes from that side’s configured default. A minimal ordinary-pattern manifest has this shape:

```json
{
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

Patterns are explicitly tagged. Ordinary patterns declare matcher-owned metadata:

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

An inline or file-backed case without `expected_transition` requires complete baseline/candidate equivalence across all three buckets and the final configured decision. An intentional change must declare both complete states:

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

Comparison declarations and ordinary paths resolve from the comparison manifest’s parent, while artifact paths resolve from the catalog’s parent. Before compilation, the matcher loads each strict catalog once, verifies its candidate and state hashes, verifies every artifact hash and exact UTF-8 file, and then compiles with catalog-owned metadata. Empty catalog declarations add no patterns to either set. Unknown fields—including `version`—and the former untagged equivalence-only shape are invalid. The matcher never extracts settings, interprets candidate authorization, or proves that a later promotion used the validated catalog.

Failure output retains at most 10 case details and reports only the case ordinal, affected side, and differing bucket or final-decision dimensions, without inputs or pattern bodies. A successful summary distinguishes equivalence cases from matched transitions and describes only a representative corpus.

Every comparison checks the matched state of `always_allow`, `always_confirm`, and `always_deny` separately before resolving the configured decision. A successful representative corpus comparison is not formal regex-language equivalence or complete Zed permission evaluation.

When factoring, consolidating, or reordering a finite pattern language, supplement representative comparison with the strongest bounded structural proof available. Examples include unchanged prefixes and suffixes with exact alternative-set equality, exact branch-set equality, or an unchanged pattern multiset. Compare match unions rather than pattern objects when ownership decomposition changes object counts.

## Audit Zed regex compatibility

During a documentation audit that includes [Zed permission regex compatibility](../../../PROJECT.md#zed-permission-regex-compatibility), obtain Zed’s current `main` `Cargo.lock` and short commit reference through one bounded official-source retrieval. Do not search Zed’s dependency changelog, release notes, or repository history. If the source cannot be retrieved, report the verification limitation instead of inferring compatibility.

Use `.agents/skills/domfiles-zed-settings/scripts/regex_dependency_audit.rs` only to compare the exact root `Cargo.toml` pin, the version resolved for the local root package through its adjacent `Cargo.lock`, and the direct `regex` version in the retrieved upstream lockfile:

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
