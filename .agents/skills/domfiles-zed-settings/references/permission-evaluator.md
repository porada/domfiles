# Permission evaluator

This reference owns the observable contracts for the retained fetch-pattern matcher and regex compatibility audit. It does not own fetch policy, runtime network behavior, host-grant approval, or settings mutation.

## Apply the matcher contract

Retain `.agents/skills/domfiles-zed-settings/scripts/pattern_match.rs` and the Cargo target `domfiles-zed-settings-pattern-match` as the read-only implementation. It supports exactly these routes:

- `--baseline-settings <baseline-settings-path> --candidate-settings <candidate-settings-path> --comparison-file <comparison-manifest-path>`
- `--help`
- `--layer-file <layer-manifest-path> --settings <settings-path>`

Treat the source and configuration as the authority for implemented behavior, the exact `--help` output as the CLI projection, and adjacent `pattern_match.test.rs` tests as corroborating contract evidence. Before using either non-help route, require the focused contract test to pass. Stop and report contract drift when it fails.

Require exactly one route. Reject combinations from different routes, missing values, positional arguments, repeated singleton options, and unknown options. Keep each help option list alphabetized.

The matcher reads only caller-selected regular UTF-8 files. It does not execute case inputs, load environment-selected configuration, make network requests, search for settings, or write files.

Both manifests are strict JSON objects. Reject duplicate keys, invalid enum values, non-string inputs, and unknown fields. Reject inputs containing U+000A, U+000B, U+000C, U+000D, U+001C, U+001D, U+001E, U+0085, U+2028, or U+2029 as line breaks. Treat every case input as inert text.

The `--help` route exits with status `0`, writes only the exact help text to standard output, and leaves standard error empty. A successful layer or comparison route writes one bounded summary to standard output. For exit status `1`, count every validation finding while retaining only the first 100 reportable details, write the exact total and retained findings to standard error, then write the omitted count. Apply one retention budget across the complete invocation, including baseline and candidate settings in the comparison route. Count later findings without materializing their bodies. For exit status `2`, write one bounded diagnostic to standard error when that stream accepts output. If any required standard-output or standard-error write fails, return status `2` even when the computed route result was status `0` or `1`.

For every failure, identify files by role, manifest cases by array and zero-based index, and settings patterns by bucket and zero-based index. For a duplicate key, identify the containing object through a safe structural location formed only from schema-owned field names, an array or bucket name, and a zero-based index when applicable. Do not emit the duplicated key or any value. Do not emit caller-selected paths, case inputs, manifest contents, pattern text, settings contents, or upstream diagnostic excerpts that could reproduce those values. Limit each rendered finding or diagnostic to `512` UTF-8 bytes and complete standard error to `64` KiB, truncating only at a Unicode-scalar boundary. Keep total and omitted counts within that bound. Select status-`1` findings in the deterministic route-specific order defined below.

Select the single status-`2` diagnostic by this phase order: argument validation, file type and readability in each route’s displayed option order, UTF-8 decoding in that option order, JSON parsing and duplicate-key detection in that option order, manifest structural validation, settings projection in settings-file order, cross-file reference or coverage validation, and required output writing. Stop at the first failing phase and the first error in its defined order.

Use these exit statuses:

- `0` when every pattern compiles and every declared expectation passes.
- `1` for a well-formed invocation that finds a configured decision, pattern case, pattern length, or regex failure.
- `2` for contract-invalid input, invalid arguments, malformed input, operational output failure, or unreadable files.

An exit status is not a recovery classification. A configured-pattern finding identifies a settings defect. A pattern-case, decision-case, or comparison disagreement establishes only that a well-formed declaration and observed behavior differ. The caller must determine whether the declaration is wrong or the selected settings must change. Status `2` identifies invalid input or an operational failure rather than a settings defect.

## Parse settings inputs

Parse every settings file as strict UTF-8 JSON, not JSONC, and require its root to be an object. Reject duplicate keys in every object. The `agent`, `tool_permissions`, `tools`, and `fetch` objects along the selected `agent.tool_permissions.tools.fetch` path must exist and have object values. Allow and ignore unrelated fields outside the selected fetch object.

The fetch object permits exactly these fields:

- `always_allow`, `always_confirm`, and `always_deny` are optional arrays. An absent array is empty.
- `default` is required and must be `allow`, `confirm`, or `deny`.

Every pattern-array entry must be an object containing exactly a Boolean `case_sensitive` field and a string `pattern` field. Reject missing required fields, null values, unknown fields within the fetch or pattern objects, and wrong types as contract-invalid input with exit status `2`.

Apply the repository [permission pattern length bound](../../../PROJECT.md#permission-pattern-length-bound) before compilation. Count each decoded `pattern` value in Unicode scalars. An empty pattern or a pattern over `1,000` scalars is a validation finding with exit status `1` and must not be compiled. Compile every remaining pattern exactly once with its configured case setting. An invalid regex is a validation finding with exit status `1`.

Within each settings file, process buckets in `always_allow`, `always_confirm`, then `always_deny` order and each bucket by ascending array index. If any pattern is empty or fails the length or compilation check, report all such configuration findings and skip manifest expectation evaluation for that route.

## Validate a configured fetch layer

Run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --layer-file '<layer-manifest-path>' \
    --settings '<settings-path>'
```

The layer manifest has this schema:

```json
{
    "decision_cases": [
        {
            "expected": {
                "always_allow": true,
                "always_confirm": false,
                "always_deny": false,
                "decision": "allow"
            },
            "input": "https://example.com/"
        }
    ],
    "pattern_cases": [
        {
            "bucket": "always_allow",
            "expected_match": true,
            "index": 0,
            "input": "https://example.com/"
        }
    ]
}
```

Require both arrays and at least one `decision_cases` entry. Permit an empty `pattern_cases` array only when the selected settings contain no configured patterns. An empty array with any configured pattern is missing coverage and contract-invalid input with exit status `2`. A pattern case identifies one configured fetch pattern by `bucket` and zero-based `index`, then declares whether that pattern must match the input. Every configured pattern requires at least one matching and one nonmatching single-line case. A manifest missing either polarity is contract-invalid input with exit status `2`. The matcher does not attempt to prove that a missing witness cannot exist. A configuration whose pattern cannot supply both polarities is unsupported by this workflow and must change before approval.

A decision case declares the complete matched-bucket state and final configured decision for one input. Accept `allow`, `confirm`, or `deny` as the decision. Use the patterns compiled with their configured case settings to resolve `always_deny`, `always_confirm`, `always_allow`, and the configured fetch default in that precedence order. Reject an expected state whose declared decision does not follow that precedence as contract-invalid input with exit status `2`.

Decision-source coverage depends on the complete expected state rather than the final decision value alone:

- `always_allow` requires `always_allow: true`, `always_confirm: false`, `always_deny: false`, and `decision: "allow"`.
- `always_confirm` requires `always_confirm: true`, `always_deny: false`, and `decision: "confirm"`. `always_allow` may be either value.
- `always_deny` requires `always_deny: true` and `decision: "deny"`. The other bucket flags may be either value.
- The configured default requires all three bucket flags to be `false` and `decision` to equal that default.

Require at least one declared decision case for the configured default and for every nonempty bucket. Missing declared source coverage is contract-invalid input with exit status `2`. A valid declaration that disagrees with observed matches or the observed decision is a validation finding with exit status `1`.

The planning workflow must identify a deciding-source witness before candidate mutation. A fully shadowed bucket or unreachable default is unsupported by the ordinary change workflow. Stop and report that state rather than changing other patterns without authorization. The matcher evaluates only declared cases and does not attempt a formal reachability proof.

Apply the [settings-input contract](#parse-settings-inputs) to the selected settings file. Reject an out-of-range pattern index or unknown bucket in the manifest as contract-invalid input with exit status `2`.

Order status-`1` findings from source validity to local expectations to final decisions: configured settings patterns in the settings-input order, pattern cases in manifest order, then decision cases in manifest order. On success, report counts of configured patterns, decision cases, and pattern cases. This route establishes configured fetch-pattern matches and decisions for the selected settings file only. It does not resolve displayed prompts, other settings layers, redirect behavior, runtime network access, or host grants.

## Compare fetch permission states

Run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    --baseline-settings '<baseline-settings-path>' \
    --candidate-settings '<candidate-settings-path>' \
    --comparison-file '<comparison-manifest-path>'
```

The comparison manifest has this schema:

```json
{
    "cases": [
        {
            "baseline": {
                "always_allow": false,
                "always_confirm": false,
                "always_deny": false,
                "decision": "confirm"
            },
            "candidate": {
                "always_allow": true,
                "always_confirm": false,
                "always_deny": false,
                "decision": "allow"
            },
            "input": "https://example.com/"
        }
    ]
}
```

Require a nonempty `cases` array. Each `baseline` and `candidate` state is complete and must declare a decision consistent with deny, confirm, allow, then the corresponding settings file’s default. An inconsistent state is contract-invalid input with exit status `2`.

Apply the [settings-input contract](#parse-settings-inputs) to the baseline settings and then the candidate settings. Compile every valid fetch pattern once per file. Baseline and candidate compilation share the invocation-wide retained-detail budget. After the first 100 reportable details, count every remaining finding without constructing its body. If either settings file has pattern-length or compilation findings, report baseline findings before candidate findings and skip comparison-case evaluation. Otherwise, evaluate cases in manifest order. For each input, evaluate the complete matched-bucket state and final decision against both files, then require exact agreement with the declared `baseline` and `candidate` states.

This route does not validate a repair from baseline configuration findings. The ordinary candidate workflow requires status `0` from baseline-layer validation before mutation. A baseline expectation disagreement does not by itself establish invalid baseline settings.

Include every intentional bucket or final-decision transition, each changed boundary, and representative unchanged near misses. On success, report counts of baseline patterns, candidate patterns, and comparison cases. A successful comparison establishes only the declared corpus. It is not formal regex-language equivalence and does not establish runtime network access or host-grant behavior.

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

The audit reads local files only and compares direct `regex` versions. Transitive versions, sources, checksums, and dependency edges may update independently. Treat a direct-version mismatch against current Zed `main` as compatibility drift from the documented maintenance baseline. Treat evidence that the retrieved lockfile does not belong to the reported upstream revision or that the local pin and lock state disagree as a semantic-accuracy finding. Treat an unresolved, missing, ambiguous, or locally inconsistent direct version as a validation failure. Neither result authorizes a dependency or documentation change.

The compatibility audit owns no repair route. Do not mutate `Cargo.toml` or `Cargo.lock` through this workflow. Treat a requested repair as a separate dependency change governed by the global “Dependencies” policy, including approval for the complete manifest and lockfile transition before mutation. After that separately authorized change, rerun the focused contract test, the audit against the same upstream lockfile, and root Rust validation.

## Resolve effective permission behavior

After resolving version-sensitive behavior through the parent [investigation workflow](../SKILL.md#investigate-and-plan):

1. Identify the selected tool’s implementation and whether it invokes configured permission evaluation. Only when it does, identify every participating Zed settings layer and resolve the tool’s effective default and accumulated pattern arrays.
2. For fetch, treat any empty or regex-invalid effective pattern as denying the tool before pattern precedence.
3. When every effective fetch pattern is valid, apply deny, confirm, allow, then default precedence to the initial URL. Evaluate shared host-grant authorization for the initial hostname and every redirect hostname independently.
4. For `terminal`, establish that this repository contributes no tool-specific override, then evaluate Zed’s built-in behavior, the inherited global default, task authorization, and operating-system sandbox effects separately.
5. For a native path tool, establish that this repository contributes no tool-specific override. When its implementation invokes configured permission evaluation, evaluate the inherited global default, task authorization, and applicable built-in path, privacy, sensitive-settings, and symlink-escape checks. When it does not, omit the configured-permission layer and evaluate task authorization plus those built-in checks. Do not attribute native path behavior to the operating-system sandbox.

The matcher evaluates one selected settings file rather than constructing Zed’s complete effective configuration. When another participating layer contributes fetch rules or a different default, inspect and account for that layer separately.

## Run focused contract tests

```sh
cargo test --locked --test domfiles-zed-settings-pattern-match-test
cargo test --locked --test domfiles-zed-settings-regex-dependency-audit-test
```

Use the pattern-matcher test as contract evidence for help-to-parser agreement, bounded finding retention and output, schemas, and status behavior. Select remaining root checks through the [skill-owned script validation policy](../../../../skills/domfiles-agent-documentation/references/skill-owned-scripts.md#test-the-contracts).
