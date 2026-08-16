# Permission candidate

Use this reference for the guarded workflow that builds, seals, and promotes a Zed permission change. Apply the parent [agent permission workflow](permissions.md), and keep read-only inventory, ownership, matching, and behavior evaluation in the [permission evaluator](permission-evaluator.md).

## Build and promote a permission candidate

Run the candidate binary with exact `--help` for current invocation syntax. Treat all task artifacts as current-only. Unknown fields are rejected, and there is no schema version, compatibility adapter, or migration path.

Use the [established graph root](permission-evaluator.md#establish-the-graph-root) for every path below.

A capture selection must contain at least one authorized non-root JSON pointer scope. Its terminal pattern selection may be empty for a scope-only candidate or an insertion-only owner with no baseline sources. For a terminal replacement or deletion, select every source in every complete baseline owner group, including known lexically invisible members identified through supplemental evidence.

Capture from the latest live settings:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    capture \
    --settings .config/zed/settings.json \
    --selection '<capture-selection-path>' \
    --output '<capture-path>'
```

Capture creates immutable exact-byte `baseline-settings.json`, editable byte-identical `candidate-settings.json`, one exact decoded UTF-8 file for each selected terminal pattern, and `state.json`. Edit only `candidate-settings.json`, and only inside authorized scopes. Do not modify the baseline, state, or captured pattern files.

Use `verify --settings .config/zed/settings.json --state '<state-path>'` as a bounded early freshness check while preparing the graph. It validates state artifacts and establishes source identity only. It never establishes promotion readiness or replaces bundle preflight.

After the candidate reaches its reviewed form, author one strict owner specification. It partitions every state member and catalog member independently:

```json
{
    "owners": [
        {
            "id": "<owner-operation-id>",
            "inventory_owner": "<top-level-executable>",
            "operation": "<insert-or-replace-or-delete>",
            "baseline_members": [
                "<state-member-id>"
            ],
            "candidate_members": [
                "<catalog-member-id>"
            ]
        }
    ],
    "overlaps": []
}
```

An `insert` has no baseline members, a `replace` has baseline and candidate members, and a `delete` has no candidate members. Every state member belongs to exactly one operation. Every catalog member belongs to exactly one operation or the overlap set. No owner’s addition or removal may cancel another owner’s undeclared change. Add `supplemental` and `visibility_rewrites` only when the [inventory workflow](permission-evaluator.md#inventory-terminal-patterns) requires them, following exact `--help` and the implemented strict schema.

Write a materialization selection containing only stable IDs and transient candidate bucket/index locators. Do not add `owner_replacement`:

```json
{
    "patterns": [
        {
            "id": "<catalog-member-id>",
            "bucket": "always_allow",
            "index": 0
        }
    ]
}
```

Materialize the candidate graph:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    materialize \
    --candidate '<candidate-settings-path>' \
    --state '<state-path>' \
    --selection '<materialization-selection-path>' \
    --output '<materialization-directory>'
```

Materialization validates the captured state and authorized candidate, compiles every selected candidate regex with the Zed-compatible engine, and writes exact pattern artifacts plus `artifact-catalog.json`. Pattern files contain exact decoded UTF-8 bytes with no added newline or reserialization. Create-new output uses complete preflight, safe names, symlink and overwrite refusal, and rollback. Candidate, baseline, state, and live settings remain untouched.

The owner specification is consumed by `seal`, not `materialize`. Keep it unchanged for the sealing step.

Validate every terminal-pattern graph through all three graph-wide evidence kinds:

- [`comparison`](permission-evaluator.md#compare-baseline-and-candidate-behavior) for baseline/candidate bucket states and representative decision transitions.
- [`layer_decision`](permission-evaluator.md#evaluate-a-configured-pattern-layer) for the complete supplied configured terminal arrays, precedence, default, settled normalized inputs, and aggregate cases.
- [`matcher_suite`](permission-evaluator.md#compile-and-match-permission-patterns) for pattern expectations and configured decisions.

Also satisfy the [owner evidence requirements](permission-evaluator.md#audit-permission-ownership): one exclusion-aware `candidate_inventory` result for each `inventory_owner` token with one or more delete operations, covering every such operation that shares the token, and `owner_audit` evidence covering every operation retaining candidate members. A scope-only graph with no terminal-pattern change requires no generic terminal evidence. Every result is hash-bound reviewed workflow evidence. It binds the complete file closure and exact binding, manifest, or overlay, but it is not authenticated proof that the validator ran and does not authorize promotion.

Create the strict validation manifest from those result paths, then seal one bundle:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    seal \
    --candidate '<candidate-settings-path>' \
    --state '<state-path>' \
    --catalog '<artifact-catalog-path>' \
    --owner-spec '<owner-spec-path>' \
    --validation '<validation-manifest-path>' \
    --output '<graph-root>/candidate-bundle.json'
```

`seal` binds the complete graph and evidence, derives each result’s covered owner operations, recomputes the full input closure, compiles every candidate regex, and refuses missing, stale, cross-owner, or mislabeled evidence. The bundle does not carry user approval.

Run a read-only rehearsal:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    preflight \
    --settings .config/zed/settings.json \
    --bundle '<bundle-path>'
```

`preflight` runs the complete structural, artifact, evidence-integrity, owner-accounting, live-freshness, merge, destination, and race-boundary checks without writing settings. It rehashes semantic evidence inputs but does not rerun matcher suites, representative comparisons, configured-layer cases, behavior research, or runtime experiments.

Stop after preparing and rehearsing the sealed bundle until the user explicitly instructs promotion. Neither the bundle, a passing preflight, `--write`, nor agent approval grants that authority.

When the user authorizes promotion and the graph remains current, promote directly. A separate preflight result is never authoritative because live settings may change after it. Promotion reruns the complete preflight in-process immediately before the mutation boundary:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    promote \
    --settings .config/zed/settings.json \
    --bundle '<bundle-path>' \
    --write
```

Promotion verifies exact source and candidate identities, independent owner membership, supplemental ownership, optional visibility rewrites, ordered outside-owner remainder equality, per-bucket count reconciliation, complete input closures, authorized-scope freshness, and the merged result. Byte-identical output leaves settings untouched. Changed output is written to a create-new same-directory sibling, receives the live file’s permissions, is synced, and is atomically renamed after a best-effort live-byte recheck. An uncooperative writer can still race after that recheck.

A delay alone does not invalidate reviewed evidence. When live settings and every bundle input remain unchanged, do not repeat semantic validation solely because approval arrived later.

When current settings have drifted but the reviewed transformation remains replayable, refresh the sealed graph instead of recapturing or writing a task-local replay script:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-permission-candidate -- \
    refresh \
    --settings .config/zed/settings.json \
    --bundle '<reviewed-bundle-path>' \
    --output '<graph-root>/refresh-<name>'
```

Refresh creates a new unsealed graph, path overlay, owner and zero-owner bindings, validation plan, ordered validation commands, and report. It preserves reviewed semantic manifests byte-for-byte. Candidate placement is replayed from reviewed candidate gaps. Start and end gaps use sentinels, while an interior gap requires both retained boundaries to relocate uniquely and remain ordered. Missing, duplicate, ambiguous, or reversed boundaries refuse refresh. Current outside-owner drift remains intact.

Inspect the refresh report, run its ordered fresh-validator commands, and seal the generated validation plan into a new bundle under the same graph root. The validators use `--artifact-root` or `--binding` without rewriting reviewed manifests. Each new result binds the refreshed graph and overlay. Then rerun preflight and promote the new bundle under the existing explicit semantic authorization. If refresh refuses because the reviewed transformation cannot be replayed, recapture from current settings and obtain renewed approval for the new candidate.

After promotion, audit every retaining owner against the promoted settings with a fresh exact manifest. Verify deleted operations through one `--delete-all-manifest` per `inventory_owner` token, covering every deleted operation that shares it. Report the hash observed or written without claiming that no later writer changed it.
