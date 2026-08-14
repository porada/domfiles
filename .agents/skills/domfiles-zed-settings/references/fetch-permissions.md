# Fetch and network permissions

Apply this branch with the shared [agent permission workflow](permissions.md).

## Apply the fetch and network permission policy

- Preserve `agent.tool_permissions.tools.fetch.default` as `confirm`.
- Restrict automatically allowed fetch patterns to `https://` and anchor hostname-wide patterns at the hostname boundary. Set `"case_sensitive": true` on every automatically allowed pattern, and scope `(?i:...)` to only the scheme and hostname so ports and later URL components remain case-sensitive.
- Treat `*.domain.name` and `domain.name` as distinct `agent.sandbox_permissions.network_hosts` entries. A wildcard matches strict subdomains at any depth, not the apex.
- Treat an explicit domain or hostname allowance as authorization for its corresponding persistent sandbox host scope. Each `network_hosts` entry covers every port and becomes part of the sandbox network floor available to later sandboxed terminal processes. The terminal command remains subject to independent terminal permission evaluation.
- Keep hostname-wide fetch and sandbox scopes aligned, subject to the [path-qualified exception](../../../PROJECT.md#zed-fetch-and-sandbox-host-scope). A path-qualified URL allowance must not add hostname-wide sandbox access.

## Translate approved domains and URLs

When the user explicitly requests an allowance for a named domain or URL, apply the policy above before these scope-specific steps:

1. Parse the literal request without network access. Reject non-HTTPS URLs and URLs containing credentials, passwords, secret-bearing path, query, or fragment values, tokens, or userinfo. Never copy or normalize such material into settings or task artifacts. Ask for a credential-free URL or domain scope instead.
2. For a domain or hostname request, require the request to select exact hostname, subdomains only, or exact hostname plus subdomains. Do not infer subdomain access from the word “domain.” The established authorization includes the corresponding persistent, all-port sandbox scope, so do not ask the user to reselect that boundary for each hostname.
3. Apply the selected hostname scope exactly:
    - Exact hostname: `^(?i:https://domain\.example)(?:[/?#]|$)` and `domain.example`.
    - Subdomains only: `^(?i:https://(?:[^./?#:@]+\.)+domain\.example)(?:[/?#]|$)` and `*.domain.example` only.
    - Exact hostname plus subdomains: `^(?i:https://(?:[^./?#:@]+\.)*domain\.example)(?:[/?#]|$)` plus both `*.domain.example` and `domain.example`.
4. For a URL request, preserve only the explicitly approved hostname, port, path, query, and fragment constraints. Allow descendants only when the request or an established pattern clearly selects a subtree. Omit `network_hosts` unless the user separately widens the request to hostname scope.
5. Reuse an equivalent existing allowance rather than adding a duplicate. Preserve the fetch array’s hostname-scope groups in the order above and alphabetize each group by represented hostname. Preserve wildcard and exact groups in `network_hosts`, alphabetizing each group by represented hostname.

A fetch-tool allowance and a sandbox hostname grant remain independent. An explicit-port URL falls through the canonical hostname fetch pattern to `confirm` even though the persistent sandbox grant already covers that hostname and port. A sandbox grant neither authorizes a terminal command nor bypasses terminal permission evaluation.

Zed’s native fetch tool applies configured fetch patterns to the initial URL, then separately authorizes every redirect hostname. It does not re-evaluate redirect URLs against the original fetch regex. Treat redirects and subresources as outside the request unless their hosts and URL scopes were already approved. Do not make a live request merely to validate a settings change.

## Choose the fetch fast path

Use `scripts/fetch_permissions.rs` for one canonical addition in any of these bounded scopes:

- Exact hostname.
- Exact hostname plus subdomains.
- Subdomains only.
- A credential-free canonical ASCII HTTPS path prefix ending in `/`, with uppercase `%HH` escapes and no port, query, fragment, userinfo, encoded slash, or dot segment.

The fast path owns canonical pattern generation, candidate insertion, hostname-scope grouping, lexical ordering, duplicate and equivalent-coverage detection, fetch and sandbox alignment, exact pattern artifacts, and the standard decision corpus. For supported inputs, its complete-array audit and candidate comparison satisfy the fetch branch’s ownership, matching, comparison, and configured-decision checks. Do not construct a terminal owner manifest, add an unrelated terminal sentinel, inventory terminal indexes, or prepare a separate task-local matcher suite.

The fast path can reuse an existing factored hostname pattern only when it can structurally expand the hostname expression into a complete finite represented-host set through supported noncapturing alternatives and optional groups. Expansion is capped at 256 represented hosts. It validates every represented host, the canonical path-pattern tail, and sandbox alignment before accepting the array.

Use the generic [permission evaluator](permission-evaluator.md) instead when the request or existing affected grammar includes an exact path rather than a prefix, a port-qualified or non-ASCII URL, query or fragment constraints, regex factoring outside the bounded finite hostname-expression contract above, an unclassifiable pattern, unresolved effective settings layers, or another shape outside the fast-path contract. Reject secret-bearing inputs rather than routing them through either workflow.

## Prepare a fast-path candidate

Run the candidate binary with `--help` and follow its current schema without copying that schema into this reference. Capture from the latest live settings with an empty terminal-pattern selection and these authorized scopes:

- For a hostname scope, select `/agent/tool_permissions/tools/fetch/always_allow` and `/agent/sandbox_permissions/network_hosts`.
- For a path-qualified scope, select only `/agent/tool_permissions/tools/fetch/always_allow`.

Verify the capture against live settings before applying the fast path. Keep `baseline-settings.json` and `state.json` immutable, and require `candidate-settings.json` to remain byte-identical to the baseline until the fast-path invocation.

For a hostname scope, run the applicable form:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-fetch-permissions -- \
    apply \
    --baseline '<capture-path>/baseline-settings.json' \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --output '<fetch-artifact-directory>' \
    --scope exact-hostname \
    --hostname '<hostname>' \
    --write
```

Replace `exact-hostname` with `subdomains-only` or `exact-hostname-plus-subdomains` when that is the selected scope.

For a supported path prefix, run:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-fetch-permissions -- \
    apply \
    --baseline '<capture-path>/baseline-settings.json' \
    --candidate '<capture-path>/candidate-settings.json' \
    --state '<capture-path>/state.json' \
    --output '<fetch-artifact-directory>' \
    --scope path-qualified-url \
    --url-prefix '<credential-free-https-url-prefix>' \
    --write
```

`apply` validates the complete current fetch allowance and sandbox-host arrays, writes each added regex as exact decoded UTF-8 bytes without a newline, binds the exact baseline, candidate, and opaque state bytes in `fetch-validation.json`, then atomically replaces only the captured candidate after a concurrent-write recheck. As with final candidate promotion, pathname replacement cannot provide compare-and-swap, so an uncooperative writer can still replace the candidate after the recheck and before the rename. Avoid concurrent candidate writers. The script never reads or promotes live settings and never contacts the destination.

Diagnostics for unknown modes and options omit the supplied values. Regex compilation diagnostics omit complete regex bodies.

## Validate and promote the candidate

Materialize a scope-only candidate catalog through the current permission-candidate contract with an empty terminal-pattern selection. This catalog binds the same candidate and state without inventing URL ownership in the terminal arrays.

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

`validate` independently revalidates each artifact-supplied request against canonical fast-path hostname or URL-prefix syntax before making canonical URL assumptions. It then reconstructs the complete candidate deterministically, verifies exact hashes and artifact bytes, reruns duplicate, alignment, and ordering audits, and evaluates every standard case through `always_deny`, `always_confirm`, `always_allow`, then the configured default. It refuses malformed or encoded-separator requests, added newlines, trailing bytes, candidate drift, state drift, scope mismatches, and stale or reordered output without mutating the candidate.

Resolve any extension, profile, release-channel, operating-system, or server settings that participate in the effective permission layers through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior). Use the generic fallback when those layers add an affected fetch rule or sandbox host that the candidate file cannot represent.

Immediately before promotion, rerun candidate freshness verification against live settings. Promote only with the untouched scope-only catalog, candidate, and state through the guarded permission-candidate workflow. Candidate promotion retains exact-byte binding, authorized-scope enforcement, the final live-byte recheck, and atomic replacement.

## Standard fetch corpus

The fast path always checks the selected pattern independently and reconstructs the complete configured fetch decision. Every hostname scope includes:

- Intended HTTPS hosts, including scheme and hostname case variants and material hostname-boundary suffixes.
- HTTP, explicit ports, userinfo, lookalike broader hostnames, and every unapproved apex or subdomain form.

Exact-hostname validation includes the apex plus path, query, and fragment starts at the hostname boundary. Subdomain scopes include one and multiple descendant levels, with the apex classified according to the selected scope. Path-qualified validation adds the exact prefix, a descendant, a sibling path, a path case variant when the path contains letters, and the hostname boundary cases above.

For each boundary case, the complete baseline and candidate bucket states and final decision must remain equal. Every intended case must resolve to `allow` after deny and confirm precedence. This proves the configured fetch-layer transition only. Sandbox authorization, terminal permission, DNS filtering, platform support, and runtime settings layers remain independent execution boundaries.

## Run focused contract tests

```sh
cargo test --locked --test domfiles-zed-settings-fetch-permissions-test
```
