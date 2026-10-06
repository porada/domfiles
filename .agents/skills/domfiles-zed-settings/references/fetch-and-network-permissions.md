# Fetch and Network Permissions

Apply this branch with the shared [agent permission workflow](agent-permissions.md). The parent skill’s user direction route governs departures from these configuration conventions and validation procedures. The described outcomes assume this default model. A departure still needs explicit access scope and supported tool behavior, and a waived check must not be reported as completed.

## Apply Fetch and Network Permission Policy

Preserve `agent.sandbox_permissions.allow_all_hosts` as `true` and keep `network_hosts` absent. This satisfies native fetch’s shared host grants and gives sandboxed terminal commands unrestricted networking, including access to loopback and private networks. It does not lift filesystem write restrictions or protected Git metadata denials. Keep existing write grants unchanged and leave `allow_fs_write_all` and `allow_unsandboxed` disabled when changing network permissions alone.

Native fetch still validates the initial and redirect hostnames and performs DNS safety checks. It rejects IP literals, `localhost`, and `.localhost` names before host grant evaluation. These checks remain active with `allow_all_hosts`, unlike with unsandboxed access. A DNS safety check does not pin the subsequent connection to the checked address. Do not treat unrestricted host access as disabling these checks or as authorization for an operation or data transfer. The global agent policy still governs task scope, disclosure, and treatment of remote content.

- Preserve `agent.tool_permissions.tools.fetch.default` as `deny`.
- Keep exactly one generic `agent.tool_permissions.tools.fetch.always_allow` rule at `"case_sensitive": true` with this pattern:

    ```regex
    ^(?i:https://(?:[^./?#:@]+\.)*[^./?#:@]+)(?:[/?#]|$)
    ```

- Keep `always_confirm` and `always_deny` absent under this model.
- Treat the generic rule as an initial URL syntax gate, not a host trust inventory. It matches HTTPS authorities made from nonempty dot-separated labels without bracketed IPv6 syntax, an explicit port, a trailing dot, or URL userinfo. Paths, queries, and fragments remain unrestricted by this rule. It cannot determine whether those components contain secret material.

Matching HTTPS URLs pass the configured tool and host grant layers without confirmation. Nonmatching initial URLs are denied rather than confirmed, including HTTP URLs and HTTPS URLs with explicit ports. Native URL and DNS checks still apply after the configured decision. An allowance pattern cannot make native fetch reach a destination rejected by those checks.

Zed applies the fetch rules only to the initial URL, then separately checks the initial hostname and every redirect hostname. It does not re-evaluate redirect schemes, ports, paths, queries, or fragments against the fetch regexes. The HTTPS allowance is therefore not an end-to-end HTTPS requirement or a restriction on terminal traffic. Redirects do not require destination grants under this network model, but network availability does not expand the task’s authorized effects. Do not make a live request merely to validate a settings change.

## Translate Approved Domains and URLs

When the user explicitly requests an allowance for a named domain or URL, apply the policy above before these scope-specific steps:

1. Parse the literal request without network access. Reject URLs containing credentials, passwords, tokens, or userinfo, or containing secret-bearing values in any path, query, or fragment. Never copy or normalize such material into settings or task artifacts. Ask for a credential-free URL or domain scope instead.
2. For a domain or hostname request, no host grant or settings change is needed under the default network model. Do not perform a host review, add a host inventory, or add a fetch regex merely to allow that hostname. If the request follows an unexpected permission outcome, [resolve the effective permission behavior](agent-permissions.md#resolve-effective-permission-behavior) before proposing a change.
3. For a URL request, distinguish an existing generic allowance from a requested departure. Matching URLs need no additional rule, and a path-qualified request does not imply path containment. For a URL outside that allowance, establish the intended scope and supported native fetch behavior before following the parent [fetch permission change workflow](agent-permissions.md#plan-permission-changes). Do not infer authorization to broaden the rule from a denied fetch.

For an explicitly requested departure, reuse equivalent existing coverage rather than adding a duplicate, and order fetch arrays by the parent skill’s [represented hostname rule](../SKILL.md#apply-general-policy). Apply the repository [permission pattern length bound](../../../PROJECT.md#permission-pattern-length-bound) before editing a candidate. If a decoded pattern exceeds that bound, report that the pattern matcher does not support it rather than silently splitting the pattern or weakening the requested scope. Rust regex does not support look-around.

## Validate Changes

For a network-only change, do not invoke the pattern matcher. Verify that the fetch settings and filesystem permissions remain unchanged unless separately in scope.

For a fetch pattern or default change, validate the candidate settings through the pattern matcher’s [configured fetch layer](fetch-pattern-matching-and-regex-compatibility.md#validate-configured-fetch-layers). Include every applicable input from the [fetch rule corpus](#build-fetch-rule-corpus), satisfy the matching and nonmatching case requirement for every configured pattern, and provide one deciding-source witness for the configured default and every nonempty bucket.

For every fetch pattern or default change, [compare the baseline and candidate settings](fetch-pattern-matching-and-regex-compatibility.md#compare-fetch-permission-states).

After a network or fetch permission change, verify the participating settings layers, effective network mode, applicable complete-array ordering, and distinct fetch and terminal boundaries. Check the final configuration against the default model above or the user’s explicitly requested departure. Pattern matching does not validate runtime network access or prompt behavior.

## Build Fetch Rule Corpus

The generic rule must match credential-free HTTPS URLs using an ordinary hostname, including scheme and hostname case variants plus path, query, and fragment starts at the hostname boundary. It must not match bracketed IPv6 literals, empty authorities, explicit ports, HTTP, trailing-dot hostnames, or URL userinfo.

Evaluate the complete configured fetch layer. Under the default model, matching cases must resolve to `allow`, and nonmatching cases must resolve to `deny`. Include IPv4-like authorities as generic rule matches, then distinguish that configured result from native fetch’s rejection of IP literals even with unrestricted host grants. For an explicitly requested departure, add cases for every changed boundary and preserve representative unchanged cases.
