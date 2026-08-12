# Fetch and network permissions

Apply this branch with the shared [agent permission workflow](permissions.md).

## Apply the fetch and network permission policy

- Keep `agent.sandbox_permissions.network_hosts` aligned with `agent.tool_permissions.tools.fetch.always_allow`, subject to the [documented host-scope exception](../../../PROJECT.md#zed-fetch-and-sandbox-host-scope).
- Preserve `agent.tool_permissions.tools.fetch.default` as `confirm`.
- Treat `*.domain.name` and `domain.name` as distinct `network_hosts` entries. Preserve both when access to the apex domain and its subdomains is intended.
- Prefer wildcard domain allowances when subdomains are involved. Include the apex domain only when it is actually used.
- Set `"case_sensitive": true` on every automatically allowed fetch pattern. Scope case-insensitivity with `(?i:...)` to only the scheme and hostname so explicit ports and later URL components remain case-sensitive.
- Restrict automatically allowed fetch patterns to `https://` and anchor each pattern at the hostname boundary. Reject an explicitly supplied URL using another scheme rather than translating it.

## Translate approved domains and URLs

When the user explicitly requests an allowance for a named domain or URL, apply the policy above before these scope-specific steps:

1. Parse the literal request without network access. Reject a URL that embeds literal authentication material, including passwords, secret-bearing query or fragment values, tokens, or userinfo. Never copy or normalize that material into settings. Ask for a credential-free URL or domain scope instead. Treat the request as authorization only for its supplied scope, not for unrequested redirect or subresource hosts.
2. For a domain or hostname request, require the requested scope to establish exact-hostname only, subdomains only, or the exact hostname plus subdomains. When it does not, ask one focused scope question before creating a candidate and do not infer subdomain access from the word “domain.” Then select the matching canonical fetch hostname shape, escape every literal hostname dot, and add the corresponding fetch and sandbox host scopes:
    - Exact hostname and subdomains: `^(?i:https://(?:[^./?#:@]+\.)*domain\.example)(?:[/?#]|$)`
    - Subdomains only: `^(?i:https://(?:[^./?#:@]+\.)+domain\.example)(?:[/?#]|$)`
    - Exact hostname only: `^(?i:https://domain\.example)(?:[/?#]|$)`
3. For a URL request, preserve the supplied hostname, port, path, query, and fragment constraints, escaping only what regex syntax requires. Allow descendants only when the requested URL or an established pattern clearly represents a subtree. Omit `agent.sandbox_permissions.network_hosts` when adding it would broaden the URL-specific fetch scope under the documented host-scope exception. Require explicit user authorization before widening the URL to hostname scope.
4. Reuse an equivalent existing pattern instead of adding a duplicate. For URL-specific patterns, follow the established path-qualified syntax for the selected exact or subtree scope rather than introducing an equivalent regex form. Preserve the fetch array’s hostname-scope groups in the order shown above and alphabetize each group by the represented hostname. Preserve the wildcard-host and exact-host groups in `agent.sandbox_permissions.network_hosts` and alphabetize each group by the represented hostname.
5. Complete static configuration and pattern validation before any network access to the destination. Access the destination afterward only when the task requires live verification or redirect or subresource discovery. Add a newly discovered hostname only when it remains within the user-approved scope. Otherwise ask before broadening the allowance. If the requested destination still prompts, stop and diagnose the permission configuration instead of accepting the prompt or widening the pattern speculatively.

## Validate fetch patterns

Validate every in-scope pattern against:

- Intended URLs, including scheme and hostname case variants.
- Alternate ports, broader host roots, non-HTTPS schemes, sibling paths, and URLs outside supplied query or fragment constraints.
- Case variants of path, query, and fragment components, which must remain unmatched by URL-specific allowances.
