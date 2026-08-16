# Fetch candidate

Use this reference to prepare, validate, and promote a fetch or sandbox-host allowance built by the fast path. Apply the [fetch and network permission policy](fetch-permissions.md) and its [fast-path selection rules](fetch-permissions.md#choose-the-fetch-fast-path) first.

## Prepare a fast-path candidate

Run the candidate binary with exact `--help` for current invocation syntax. Follow [CLI contract authority](permission-evaluator.md#resolve-cli-contract-authority) without copying its schema into this reference. Capture from the latest live settings with an empty terminal-pattern selection and these authorized scopes:

- For hostname coverage, select `/agent/tool_permissions/tools/fetch/always_allow` and `/agent/sandbox_permissions/network_hosts`.
- For path-qualified coverage, select only `/agent/tool_permissions/tools/fetch/always_allow`.

Verify the capture against live settings before applying the fast path. Keep `baseline-settings.json` and `state.json` immutable, and require `candidate-settings.json` to remain byte-identical to the baseline until the fast-path invocation.

For hostname coverage, run the applicable form:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-fetch-permissions -- \
    apply \
    --baseline '<capture-path>/baseline-settings.json' \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --output '<fetch-artifact-directory>' \
    --coverage exact-hostname \
    --hostname '<hostname>' \
    --write
```

Replace `exact-hostname` with `subdomains-only` or `exact-hostname-plus-subdomains` when that is the selected coverage.

For a supported path prefix, run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-fetch-permissions -- \
    apply \
    --baseline '<capture-path>/baseline-settings.json' \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --output '<fetch-artifact-directory>' \
    --coverage path-qualified-url \
    --url-prefix '<credential-free-https-url-prefix>' \
    --write
```

`apply` validates the complete current fetch allowance and sandbox-host arrays, writes each added regex as exact decoded UTF-8 bytes without a newline, binds the exact baseline, candidate, and opaque state bytes in `fetch-validation.json`, then atomically replaces only the captured candidate after a concurrent-write recheck. As with final candidate promotion, pathname replacement cannot provide compare-and-swap, so an uncooperative writer can still replace the candidate after the recheck and before the rename. Avoid concurrent candidate writers. The script never reads or promotes live settings and never contacts the destination.

Diagnostics for unknown modes and options omit the supplied values. Regex compilation diagnostics omit complete regex bodies.

## Validate and promote the candidate

Materialize a scope-only candidate catalog through the current permission-candidate contract with an empty terminal-pattern selection. The resulting empty catalog binds the candidate and state without inventing URL ownership in the terminal arrays.

Validate the fetch bundle:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-fetch-permissions -- \
    validate \
    --baseline '<capture-path>/baseline-settings.json' \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --bundle '<fetch-artifact-directory>/fetch-validation.json'
```

`validate` independently revalidates each artifact-supplied request against canonical fast-path hostname or URL-prefix syntax before making canonical URL assumptions. It then reconstructs the complete candidate deterministically, verifies exact hashes and artifact bytes, reruns duplicate, alignment, and ordering audits, and evaluates every case in the [standard fetch corpus](fetch-permissions.md#standard-fetch-corpus) through `always_deny`, `always_confirm`, `always_allow`, then the configured default. It refuses malformed or encoded-separator requests, added newlines, trailing bytes, candidate drift, state drift, scope mismatches, and stale or reordered output without mutating the candidate.

Resolve any extension, profile, release-channel, operating-system, or server settings that participate in the effective permission layers through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior). Use the generic fallback when those layers add an affected fetch rule or sandbox host that the candidate file cannot represent.

A scope-only candidate requires no generic terminal validation results. Create the candidate validation manifest with an empty `results` array, then seal the exact candidate, state, empty catalog, empty owner specification, and validation manifest into one bundle through [Build and promote a permission candidate](permission-candidate.md#build-and-promote-a-permission-candidate). Run bundle preflight and stop until the user explicitly authorizes promotion.

After approval, promote with the untouched sealed bundle. Promotion retains exact-byte binding, authorized-scope enforcement, the final live-byte recheck, and atomic replacement. When live settings have drifted but the reviewed scope-only transformation remains replayable, use the generic bundle refresh workflow instead of manually rebuilding or editing indexes.
