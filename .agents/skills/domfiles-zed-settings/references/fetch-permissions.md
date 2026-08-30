# Fetch and network permissions

Apply this branch with the shared [agent permission workflow](permissions.md).

## Apply the fetch and network permission policy

- Preserve `agent.tool_permissions.tools.fetch.default` as `confirm`.
- Keep exactly one generic `agent.tool_permissions.tools.fetch.always_allow` rule at `"case_sensitive": true` with this pattern:

```regex
^(?i:https://(?:[^./?#:@]+\.)*[^./?#:@]+)(?:[/?#]|$)
```

- Treat the generic rule as an initial-URL syntax gate, not a host trust inventory. It matches HTTPS authorities made from nonempty dot-separated labels without URL userinfo, an explicit port, a trailing dot, or bracketed IPv6 syntax. Paths, queries, and fragments remain unrestricted by this rule. It cannot determine whether those components contain secret material.
- Treat a path-filtered fetch allowance as two independently approved scopes: persistent whole-host authorization shared by native fetch and sandboxed terminal actions, and the direct initial URL prefixes that remain prompt-free. Add an exact `network_hosts` entry only after the user explicitly accepts all-port access to that hostname for both uses.
- Guard a path-qualified hostname with one same-host `always_confirm` complement. The generic rule already allows every approved initial path, so do not add redundant path-prefix allow rules. The confirmation rule must match the hostname boundary and every other initial path while excluding every approved case-sensitive prefix and its descendants. Confirmation precedence then overrides the generic allowance everywhere outside those paths.
- Group every approved prefix for one hostname under the same confirmation complement. Adding or removing a prefix requires rebuilding and revalidating that complete hostname guard.
- Treat `agent.sandbox_permissions.network_hosts` as the canonical persistent host-grant inventory shared by native fetch and sandboxed terminal actions. Each entry covers every port, authorizes native fetch to that hostname, and becomes part of the sandbox network floor available to later terminal actions. Terminal commands remain subject to independent task authorization and sandbox evaluation.
- Use exact `network_hosts` entries by default. Add or retain a wildcard only when a named recurring workflow depends on provider-selected or materially unstable subdomains, exact enumeration would materially impair that workflow, and the wildcard suffix is accepted as the shared all-port authorization boundary. Record the wildcard and its rationale under [Zed fetch and sandbox host scope](../../../PROJECT.md#zed-fetch-and-sandbox-host-scope) without reproducing the surrounding host inventory. Common ownership, convenience, or unverified possible use is insufficient.
- Do not remove an existing host grant solely because current repository or local-project evidence does not demonstrate use. Require replacement coverage, positive evidence that the grant is obsolete or redundant, or an explicit user decision.
- Treat `*.domain.name` and `domain.name` as distinct `network_hosts` entries. A wildcard matches strict subdomains at any depth, not the apex.

Automatic fetch execution requires both the fetch-tool layer and host-grant authorization to allow the action. An initial HTTP URL, explicit-port URL, URL containing userinfo, trailing-dot hostname, or bracketed IPv6 literal remains `confirm` at the fetch-tool layer even when its hostname is trusted. A matching HTTPS URL for an untrusted hostname remains `confirm` at the host-grant layer. For a path-filtered hostname with persistent host authorization, approved initial prefixes can run without either prompt, while every other direct initial path confirms at the fetch-tool layer. Without that host authorization, an approved prefix still reaches host-grant confirmation. Loopback and IP-literal destinations require unsandboxed access rather than a persistent `network_hosts` entry.

Zed applies the fetch rules only to the initial URL, then separately authorizes the initial hostname and every redirect hostname. It does not re-evaluate redirect schemes, ports, paths, queries, or fragments against the fetch regexes. The confirmation complement is therefore an initial-fetch prompt filter, not path-scoped network containment. Same-host redirect paths and sandboxed terminal traffic remain outside it. Treat redirects and subresources as outside the request unless their hosts were already approved. Do not make a live request merely to validate a settings change.

## Translate approved domains and URLs

When the user explicitly requests an allowance for a named domain or URL, apply the policy above before these scope-specific steps:

1. Parse the literal request without network access. Reject non-HTTPS URLs and URLs containing credentials, passwords, secret-bearing path, query, or fragment values, tokens, or userinfo. Never copy or normalize such material into settings or task artifacts. Ask for a credential-free URL or domain scope instead.
2. Before settings mutation, recommend a bounded workflow-complete host review in the plan. Unless the user limits the request to the literal host, inspect current first-party endpoint documentation and published network allowlists, relevant local-project evidence, and credential-free redirect behavior for the named workflow. Do not enumerate arbitrary subdomains or claim that a crawl proves completeness. Classify required exact hosts separately from account, authentication, executable-artifact, registry, and user-content surfaces. Present every discovered host outside the approved scope as one batched proposal requiring explicit user approval. Batch any required network grant through the global “Execution checkpoints” policy.
3. For a domain or hostname request, identify the exact hostname or named exact subdomains required. If the request instead selects subdomains only or the exact hostname plus subdomains, apply the wildcard exception policy before accepting that scope. Do not infer subdomain access from the word “domain.” The established authorization includes the corresponding persistent, all-port host-grant scope, so do not ask the user to reselect that boundary for each hostname.
4. For a path-qualified URL request, require a credential-free canonical ASCII HTTPS path prefix ending in `/`, with uppercase `%HH` escapes and no port, query, fragment, userinfo, encoded slash, backslash, or dot segment. Resolve both decisions together: authorization for the exact hostname and path subtree at the fetch-tool layer, and whether the exact hostname also receives persistent all-port host-grant authorization. Do not infer the second decision from the first.
5. Apply the approved scope exactly:
    - Exact hostname: add `domain.example` to `network_hosts`.
    - Named exact subdomains: add each hostname separately.
    - Justified wildcard subdomains only: add `*.domain.example` to `network_hosts`.
    - Justified exact hostname plus wildcard subdomains: add both entries.
    - Path subtree: build or update the same-host confirmation complement against the complete approved-prefix set. Add the exact hostname to `network_hosts` only when the separate whole-host decision approved it.
6. Reuse equivalent existing coverage rather than adding a duplicate. Preserve wildcard and exact groups in `network_hosts`, alphabetizing each group by represented hostname. Order fetch arrays by the parent skill’s [represented-hostname rule](../SKILL.md#apply-the-general-policy).

Do not add or modify a fetch regex for a hostname allowance. A path-qualified allowance does not by itself add `network_hosts`. If the user does not separately authorize the persistent hostname scope, its approved path remains confirmable at the host-grant layer.

Rust regex does not support look-around. Build each same-host confirmation complement from anchored prefix alternatives that match the first differing path byte, including every truncated prefix. Validate every alternative through the [fetch rule corpus](#fetch-rule-corpus), and do not use an unverified hand-written negation as a permission boundary.

Before editing a candidate, build the complete guard for the hostname’s full approved-prefix set and apply the repository [permission pattern length bound](../../../PROJECT.md#permission-pattern-length-bound). If the decoded pattern exceeds `1,000` Unicode scalars, report that the requested path scope is unsupported by the one-guard model and stop before settings mutation. Do not split the guard or weaken the approved scope to fit the bound.

## Validate a change

For a `network_hosts`-only change, do not invoke the pattern matcher.

For a fetch pattern or default change, validate the candidate settings through the evaluator’s [configured fetch layer](permission-evaluator.md#validate-a-configured-fetch-layer). Include every applicable input from the [fetch rule corpus](#fetch-rule-corpus), satisfy the matching and nonmatching case requirement for every configured pattern, and provide one deciding-source witness for the configured default and every nonempty bucket.

For every fetch pattern or default change, [compare the baseline and candidate settings](permission-evaluator.md#compare-fetch-permission-states). Declare every intended matched-bucket or final-decision transition and retain representative unchanged boundary cases. A representative comparison is not formal regex-language equivalence.

After a `network_hosts`-only change or a fetch pattern or default change, verify the approved exact and wildcard host-grant coverage, every wildcard’s documented exception rationale, complete-array ordering, participating settings layers, and effective fetch and terminal boundaries.

## Fetch rule corpus

The generic rule must match credential-free HTTPS URLs using an ordinary hostname, including scheme and hostname case variants plus path, query, and fragment starts at the hostname boundary. It must not match HTTP, explicit ports, URL userinfo, trailing-dot hostnames, bracketed IPv6 literals, or empty authorities.

For each guarded hostname, independently verify the complete same-host confirmation complement. For every approved prefix, include the exact prefix, a descendant, every truncation, and one nonapproved path for each byte position after the leading `/`, with that position as the path’s first differing byte. Also include the hostname boundary, root path, a sibling path, a path case variant, and query and fragment starts. The confirmation rule must match every nonapproved case and none of the approved prefixes or descendants.

Evaluate the complete configured fetch layer with confirmation precedence. Approved prefixes must resolve to `allow`, while every other initial path on a guarded hostname must resolve to `confirm`. Include IPv4-like authorities as generic-rule matches, then resolve their effective behavior through the independent host-authorization rule that requires unsandboxed access for IP literals.
