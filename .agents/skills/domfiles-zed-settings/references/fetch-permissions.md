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
3. Apply the selected hostname coverage exactly:
    - Exact hostname: `^(?i:https://domain\.example)(?:[/?#]|$)` and `domain.example`.
    - Subdomains only: `^(?i:https://(?:[^./?#:@]+\.)+domain\.example)(?:[/?#]|$)` and `*.domain.example` only.
    - Exact hostname plus subdomains: `^(?i:https://(?:[^./?#:@]+\.)*domain\.example)(?:[/?#]|$)` plus both `*.domain.example` and `domain.example`.
4. For a URL request, preserve only the explicitly approved hostname, port, path, query, and fragment constraints. Allow descendants only when the request or an established pattern clearly selects a subtree. Omit `network_hosts` unless the user separately widens the request to hostname coverage.
5. Reuse an equivalent existing allowance rather than adding a duplicate. Order the complete fetch array by the parent skill’s [represented-hostname rule](../SKILL.md#apply-the-general-policy), without grouping by hostname coverage. Preserve wildcard and exact groups in `network_hosts`, alphabetizing each group by represented hostname.

An explicit-port URL falls through the canonical hostname fetch pattern to `confirm` even though the persistent sandbox grant covers that hostname and port.

Zed’s native fetch tool applies configured fetch patterns to the initial URL, then separately authorizes every redirect hostname. It does not re-evaluate redirect URLs against the original fetch regex. Treat redirects and subresources as outside the request unless their hosts and URL scopes were already approved. Do not make a live request merely to validate a settings change.

## Choose the fetch fast path

Use `.agents/skills/domfiles-zed-settings/scripts/fetch_permissions.rs` for one canonical addition whenever the requested coverage is one of these:

- Exact hostname.
- Exact hostname plus subdomains.
- Path-qualified URL, given a credential-free canonical ASCII HTTPS path prefix ending in `/`, with uppercase `%HH` escapes and no port, query, fragment, userinfo, encoded slash, or dot segment.
- Subdomains only.

The fast path owns canonical pattern generation, candidate insertion, represented-hostname ordering, duplicate and equivalent-coverage detection, fetch and sandbox alignment, exact pattern artifacts, and the standard decision corpus. For supported inputs, its complete-array audit and candidate comparison satisfy the fetch branch’s ownership, matching, comparison, and configured-decision checks. Do not construct a terminal owner manifest, add an unrelated terminal sentinel, inventory terminal indexes, or prepare a separate task-local matcher suite.

The fast path can reuse an existing factored hostname pattern only when it can structurally expand the hostname expression into a complete finite represented-host set through supported noncapturing alternatives and optional groups. Expansion is capped at 256 represented hosts. It validates every represented host, the canonical path-pattern tail, and sandbox alignment before accepting the array.

Use the generic [permission evaluator](permission-evaluator.md) instead when the request or existing affected grammar includes an exact path rather than a prefix, a port-qualified or non-ASCII URL, query or fragment constraints, regex factoring outside the bounded finite hostname-expression contract above, an unclassifiable pattern, unresolved effective settings layers, or another shape outside the fast-path contract. Reject secret-bearing inputs rather than routing them through either workflow.

When the fast path applies and the task authorizes mutation, follow the [fetch candidate](fetch-candidate.md) workflow to prepare, validate, and promote it. Every read-only fetch workflow skips that candidate workflow and still applies the [standard fetch corpus](#standard-fetch-corpus).

## Standard fetch corpus

The fast path always checks the selected pattern independently and reconstructs the complete configured fetch decision. Hostname coverage always includes:

- Intended HTTPS hosts, including scheme and hostname case variants and material hostname-boundary suffixes.
- HTTP, explicit ports, userinfo, lookalike broader hostnames, and every unapproved apex or subdomain form.

Exact-hostname validation includes the apex plus path, query, and fragment starts at the hostname boundary. Subdomain coverage includes one and multiple descendant levels, with the apex classified according to the selected coverage. Path-qualified validation adds the exact prefix, a descendant, a sibling path, a path case variant when the path contains letters, and the hostname boundary cases above.

For each boundary case, the complete baseline and candidate bucket states and final decision must remain equal. Every intended case must resolve to `allow` after deny and confirm precedence. This proves only the configured fetch-layer transition, subject to the independent execution boundaries above.

## Run focused contract tests

```sh
cargo test --locked --test domfiles-zed-settings-fetch-permissions-test
```
