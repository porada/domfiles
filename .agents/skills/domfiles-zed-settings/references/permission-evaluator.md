# Permission evaluator

## Establish the graph root

Establish one graph root in the task-specific [`.agent-<name>` directory](../../../../.config/zed/AGENTS.md#temporary-files) before using any permission workflow. Pass this path as `--graph-root` wherever a validator requires it. Keep the complete workflow inside this root, including every bundle input, candidate artifact, file-backed matcher input, manifest, overlay, and result, so validators can bind the complete input closure without path or symlink escape.

## Resolve CLI contract authority

Apply the [CLI contract authority rule](../../../../skills/agent-documentation/references/skill-owned-scripts.md#make-the-interface-discoverable) to every permission binary. Use exact `--help` for ordinary invocation syntax. Inspect the exact implementation and relevant adjacent tests only when changing or reconciling a CLI projection, and limit that inspection to each affected combination, mode, option, or schema.

## Inventory terminal patterns

Never read or emit complete terminal permission arrays during a targeted task. Start with the owner-audit tool’s bounded lexical inventory:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings .config/zed/settings.json \
    --owner '<top-level-executable>'
```

Each inventory page reports the exact settings SHA-256, at most 100 bucket and index IDs, decoded Unicode-scalar counts, required boolean case settings, and bounded source-text previews. Its token-aware source search produces candidate evidence, not semantic ownership proof. When another page remains, rerun the same command with the reported opaque `--after '<inventory-cursor>'`. The cursor is bound to the exact settings bytes and inventory owner. Any settings change or different owner invalidates it and requires restarting from the first page.

Classify each lexical candidate through the applicable terminal and domain policy. Include candidates owned by the inventory owner in its complete semantic owner groups. Classify lexical hits owned by another command, such as a manager name inside a Corepack denial, as explicit exclusions. Treat every reported bucket and index as transient. It expires after an edit, rebase, integration, concurrent permission task, or other relevant-array change.

Regex source text can hide an owner token that the compiled regex matches. Treat a known omission as supplemental ownership, not permission to rewrite live settings. Use bounded read-only evidence to identify and capture the exact source. Declare its side, member ID, semantic owner, repository scope, invisibility reason, matching witness, and bound validation entry in the owner specification. The candidate and owner-audit tools share one wrapper-aware semantic owner and repository-scope inference implementation, and promotion refuses supplemental evidence that does not infer exactly as declared.

A behavior-preserving candidate-only rewrite may make a hidden owner lexically discoverable. Declare it separately as an optional visibility rewrite only when the tool’s bounded finite-literal expansion proof accepts the exact baseline and candidate forms. This is not general regex-language equivalence. When the proof does not apply, leave the source byte-identical and declare the candidate member supplemental too. A delete operation never needs a candidate rewrite.

## Audit permission ownership

Run the owner-audit binary with exact `--help` for current invocation syntax. Keep every manifest and referenced file under the [graph root](#establish-the-graph-root).

For an owner retaining candidate entries, bind the canonical audit manifest to the exact candidate settings. Declare every inventory-owned entry’s stable ID, bucket and current index, semantic owner, owner and domain-section sort keys, role, stable role-local sort key, and one normalized witness. Declare every excluded lexical candidate’s bucket, index, semantic outside owner, matching witness, and nonempty semantic reason. Derive this manifest independently from the candidate transformation.

Derive each witness from its own pattern and confirm the pattern accepts it before declaring it. Hand-authoring is impractical for a large owner such as `git`, and these failure modes are not evident in the reported finding text:

- A witness must match its pattern. Inferring the intended owner is not sufficient.
- A witness that ends in a space is not normalized and fails owner inference. Prefer an alternative branch when the pattern accepts one.
- An execution allowance witnessed through its `-h` or `--help` branch infers the discovery role and sorts ahead of its own group. Prefer a branch that shows the execution form.
- An entry whose witness fails owner inference leaves the position map, which breaks the Git ordering-separation exception for every span crossing its index. One unparseable witness can surface as occupancy findings against unrelated owners.
- An agent-namespace entry that binds its path positionally still infers `general` scope. Witness it through a `-C` worktree path when the pattern accepts one, so its span matches the run it belongs to.

Only `-C <path>`, `--no-optional-locks`, `--no-pager`, and the exact `-c commit.gpgsign=false` precede the subcommand during owner inference. Any other leading token ends the prefix and makes the inferred owner `git:root`, so declare that owner while keeping the entry’s domain-section key with the subcommand its pattern governs.

Resolve semantic ownership through the [terminal command-owner policy](terminal-permissions.md#apply-the-terminal-permission-policy) and each selected domain policy. Apply the [Git owner partition](git-permissions.md#apply-the-git-permission-policy) to Git owner and section keys, the [Node manager boundaries](node-package-manager-permissions.md#apply-manager-boundaries) to Corepack mediation, and the terminal policy’s wrapper ownership to `xargs`.

For a discovery entry, set `discovery_coverage` to one of these values and provide nonempty `discovery_inputs` containing the witness:

- `complete_finite` means the inputs are the complete finite normalized grammar represented by that entry. It enables finite duplicate-coverage findings against retained entries in the same `always_allow` manager group.
- `representative` records bounded examples for variable-operand discovery grammar without claiming completeness. Validate the complete variable grammar, hazardous operands, and near misses through matcher suites instead.

Direct and wrapped entries omit both discovery fields. Declare `nohup` and `xargs` child witnesses as `wrapped`. When a pattern is case-insensitive under the terminal policy’s verified command-specific exception, declare a nonempty `case_insensitive_reason`. Encode every entry’s final bucket position after applying the terminal owner-ordering policy and any selected domain order.

The audit requires complete occupancy for each `(bucket, independently inferred semantic owner, inferred Git repository scope)` span. Repository scope is one of:

- `general` for non-Git witnesses, absent or ambiguous Git `-C` paths, unsupported wrapper forms, multiple `-C` operands, absolute paths, malformed agent names, and paths containing empty, current-directory, or parent-directory components.
- `agent worktree` for one exact traversal-free project-relative `-C .agent-<name>` path.
- `fixture repository` for one traversal-free strict descendant `-C .agent-<name>/<path>`.

One Git ordering exception permits a discovery entry to be separated from a later direct or wrapped entry when both have the same inferred Git owner and repository scope and every intervening manifest entry independently infers to a Git owner. Non-Git owner groups remain contiguous.

Run a structural candidate audit and write bundle-ready evidence with:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-owner-audit -- \
    --settings '<candidate-settings-path>' \
    --manifest '<owner-manifest-path>' \
    --graph-root '<graph-root>' \
    --result-out '<owner-audit-result-path>'
```

For each `inventory_owner` token with one or more delete operations, use one strict `--zero-owner-manifest` result covering every delete operation that shares that token. Its complete recomputed lexical inventory must classify every hit exactly once as an outside-owner exclusion or a retained entry of a same-token owner operation. Raw `--owner` inventory produces only `inventory_query` evidence and cannot satisfy the required `candidate_inventory` result.

After refresh, keep reviewed semantic manifests byte-identical and pass the generated `--binding` overlay. The result binds that overlay and the complete post-binding input closure. Any changed binding or referenced file makes the evidence stale.

After promotion, use one `--delete-all-manifest` for each `inventory_owner` token with one or more delete operations, covering every delete operation that shares that token. This mode binds the sealed bundle and promoted scopes, requires byte-exact absence of every declared and supplemental baseline member, and rechecks complete exclusion classification. Do not create an empty canonical owner manifest.

The audit verifies the exact settings snapshot, complete lexical-candidate classification, wrapper-aware semantic owner and repository-scope inference, witnesses, case settings, decoded lengths, owner-group contiguity, declared bucket order, and complete finite discovery redundancy. It does not establish safety classifications, prove general regex-language equivalence, reproduce matcher coverage, or replace complete effective-permission evaluation.

When a complete audit reports a structural issue outside the intended owner set, compare the exact baseline and candidate objects before changing anything. Classify it as baseline state, candidate-caused drift, or manifest error. Expanding the change to repair baseline state requires explicit user approval.

Before adding a finite discovery object, test its complete normalized input set against every retained pattern in the owner group and add only uncovered forms. Before removing one as redundant, require every member of its complete finite grammar to remain covered. Do not use provisional owner metadata alone as duplicate-coverage evidence.

## Compile and match permission patterns

Use `.agents/skills/domfiles-zed-settings/scripts/pattern_match.rs` for Zed-compatible regex compilation and matching. Do not use the dependency-audit tool as a pattern compiler, and do not substitute `rg`, which changes anchor and byte-matching behavior and wraps configured patterns before parsing.

For one pattern and input, write both as exact UTF-8 bytes under the [graph root](#establish-the-graph-root) and run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --case-sensitive \
    --input-file '<input-path>' \
    --pattern-file '<pattern-path>'
```

Omit `--case-sensitive` only when the pattern object resolves it to `false`. For multiple single-line cases against one pattern, use `--cases-file` with LF-delimited `match<TAB><input>` and `no-match<TAB><input>` records.

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

A suite requires exactly one default, at least one ordinary or catalog-backed pattern, at least one decision case, and at least one pattern case for every pattern ID. Catalog-backed patterns obtain bucket, case setting, artifact path, and logical ID from the strict catalog. Ownership remains in the owner specification rather than the catalog.

For every user-supplied command intended to become automatic, preserve the raw shell line and its input-derivation evidence outside the suite, then include each derived normalized input as a decision case. Resolve aggregate operations separately when one shell line produces multiple permission inputs. Include every changed pattern and unchanged participating pattern that may match a decision input, plus representative intended, hazardous, near-miss, and precedence cases. Use file-backed records for line-break-bearing inputs.

Write bundle-ready suite evidence with:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --suite-file '<suite-path>' \
    --graph-root '<graph-root>' \
    --result-out '<suite-result-path>'
```

After refresh, add `--artifact-root '<refresh-directory>'`. The validator reads `<refresh-directory>/path-overlay.json` and redirects only the listed graph paths. It does not rewrite the reviewed suite.

Suite decisions apply configured precedence to one normalized input only: deny, then confirm, then allow, then the declared default. They do not reproduce pre-rule denial checks, input derivation, settings-layer resolution, or complete multi-input permission evaluation. Keep matcher success, aggregate effective-permission reasoning, observed command behavior, observed prompt behavior, and explicit user acceptance as separate claims.

## Evaluate a configured pattern layer

Use one strict layer manifest to evaluate settled normalized inputs against complete configured terminal arrays from a supplied settings snapshot:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --layer-file '<layer-manifest-path>' \
    --graph-root '<graph-root>' \
    --result-out '<layer-result-path>'
```

The manifest separates review-only `raw_provenance` from evaluated `settled_inputs` and explicit aggregate cases. The mode compiles each configured pattern once and applies the supplied default plus deny, confirm, and allow precedence. After refresh, add `--artifact-root '<refresh-directory>'` so the reviewed manifest resolves the refreshed settings snapshot through the generated path overlay.

This mode establishes configured-pattern-layer decisions only. It does not establish shell parsing, command decomposition, input derivation, pre-rule checks, participating settings layers, sandbox or Git-metadata permissions, displayed prompts, runtime behavior, or user acceptance. Complete effective-permission reasoning remains a separate workflow below.

## Compare baseline and candidate behavior

Use one strict JSON comparison manifest to compile each baseline and candidate pattern set once and evaluate one normalized representative corpus against both:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --comparison-file '<comparison-path>' \
    --graph-root '<graph-root>' \
    --result-out '<comparison-result-path>'
```

`--help` summarizes the current strict schema. Resolve any discrepancy through [CLI contract authority](#resolve-cli-contract-authority). The manifest supports ordinary and catalog-backed patterns, separate defaults, inline and file-backed cases, complete equivalence cases, and intentional `expected_transition` declarations. Either pattern set may be empty. Every case checks the matched state of `always_allow`, `always_confirm`, and `always_deny` separately before resolving the final configured decision.

A case without `expected_transition` requires complete baseline/candidate equivalence across all buckets and the final decision. An intentional transition declares both complete observed states. Declared final decisions must agree with deny, confirm, allow, then default precedence. Bucket drift remains visible even when precedence preserves the final decision.

After refresh, add `--artifact-root '<refresh-directory>'`. The comparison, reviewed manifest, file-backed cases, graph artifacts, and overlay are included in the result’s complete input closure. Any changed, added, removed, or redirected input makes the result stale.

A successful representative comparison is not formal regex-language equivalence or complete Zed permission evaluation. When a behavior-preserving visibility rewrite is needed, use the owner specification’s bounded literal-expansion proof rather than treating representative cases as equivalence proof.

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

Select the applicable commands above and remaining focused and root checks through the [skill-owned script validation policy](../../../../skills/agent-documentation/references/skill-owned-scripts.md#test-the-contracts).

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
