# Fetch and Network Permissions

Apply this branch with the shared [agent permission workflow](agent-permissions.md). The described outcomes assume the default model below.

## Apply Fetch and Network Permission Policy

Preserve `agent.sandbox_permissions.allow_all_hosts` as `true` and keep `network_hosts` absent. This satisfies native fetch’s shared host grants and gives sandboxed terminal commands unrestricted networking, including access to loopback and private networks. It does not lift filesystem write restrictions or protected Git metadata denials. Keep existing write grants unchanged and leave `allow_fs_write_all` and `allow_unsandboxed` disabled when changing network permissions alone.

Native fetch still validates the initial and redirect hostnames and performs DNS safety checks. It rejects IP literals, `localhost`, and `.localhost` names before host grant evaluation. These checks remain active with `allow_all_hosts`, unlike with unsandboxed access. A DNS safety check does not pin the subsequent connection to the checked address. Do not treat unrestricted host access as disabling these checks or as authorization for an operation or data transfer. The global agent policy still governs task scope, disclosure, and treatment of remote content.

Keep native fetch deny-by-default with one generic HTTPS syntax allowance. Read exact permission values and patterns from the [user settings](../../../../home/.config/zed/settings.json).

Treat the generic rule as an initial URL syntax gate, not a host trust inventory. It matches HTTPS authorities made from nonempty dot-separated labels without bracketed IPv6 syntax, an explicit port, a trailing dot, or URL userinfo. Paths, queries, and fragments remain unrestricted by this rule. It cannot determine whether those components contain secret material.

Matching HTTPS URLs pass the configured tool and host grant layers without confirmation. Nonmatching initial URLs are denied rather than confirmed, including HTTP URLs and HTTPS URLs with explicit ports. Native URL and DNS checks still apply after the configured decision. An allowance pattern cannot make native fetch reach a destination rejected by those checks.

Zed applies the fetch rules only to the initial URL, then separately checks the initial hostname and every redirect hostname. It does not re-evaluate redirect schemes, ports, paths, queries, or fragments against the fetch regexes. The HTTPS allowance is therefore not an end-to-end HTTPS requirement or a restriction on terminal traffic. Redirects do not require destination grants under this network model, but network availability does not expand the task’s authorized effects. Do not make a live request merely to validate a settings change.

## Validate Changes

For a network-only change, do not invoke the pattern matcher. Verify that the fetch settings and filesystem permissions remain unchanged unless separately in scope.

For a fetch pattern or default change, check the final stored patterns and their `case_sensitive` values with the [pattern matcher](fetch-pattern-matching-and-regex-compatibility.md#check-patterns). Include the applicable [URL cases](#build-fetch-rule-corpus), then separately [resolve configured decisions](agent-permissions.md#resolve-effective-permission-behavior) from the actual rules and default. Verify both intended decision changes and representative unchanged cases.

After a network or fetch permission change, verify the participating settings layers, effective network mode, applicable complete-array ordering, and distinct fetch and terminal boundaries. Check the final configuration against the default model above or the user’s explicitly requested departure. Pattern matching does not validate runtime network access or prompt behavior.

## Build Fetch Rule Corpus

The generic rule must match credential-free HTTPS URLs using an ordinary hostname, including scheme and hostname case variants plus path, query, and fragment starts at the hostname boundary. It must not match bracketed IPv6 literals, empty authorities, explicit ports, HTTP, trailing-dot hostnames, or URL userinfo.

Evaluate the complete configured fetch layer. Under the default model, matching cases must resolve to `allow`, and nonmatching cases must resolve to `deny`. Include IPv4-like authorities as generic rule matches, then distinguish that configured result from native fetch’s rejection of IP literals even with unrestricted host grants. For an explicitly requested departure, add cases for every changed boundary and preserve representative unchanged cases.
