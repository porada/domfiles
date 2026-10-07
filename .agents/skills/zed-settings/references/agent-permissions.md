# Agent Permissions

Agent permissions are configured through `home/.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for general validation, investigation, and mutation boundaries.

Do not read every permission reference by default. Select only the branches required by the resolved scope, and within each branch read the sections the task needs rather than the complete file.

## Apply Shared Permission Policy

- Keep `fetch` as the only tool with repository-configured overrides unless the user expressly requests another configuration under the parent [general policy](../SKILL.md#apply-general-policy). Use the parent change workflow for that departure rather than treating the absence of a specialized validation tool as a categorical prohibition.
- Preserve `agent.tool_permissions.default` as `allow` by default.
- Treat the always-loaded global agent policy as the canonical owner of authentication handling, command intent, security boundary restrictions, and task authorization. Do not encode those policies as terminal command patterns.
- Treat configured tool permissions, the operating system sandbox for terminal processes, and native fetch network checks as distinct layers. The operating system sandbox applies to `terminal`, not native `fetch` or native path tools. A tool permission `allow` does not grant a `terminal` effect outside that sandbox or let `fetch` bypass Zed’s separate network checks. Native path tools use their own built-in checks.

## Select Permission Branches

- For domains, fetch patterns, network hosts, redirects, or URLs, read [Fetch and Network Permissions](fetch-and-network-permissions.md).
- For effective authorization, an observed permission outcome, or settings behavior involving `fetch`, a native path tool, or `terminal`, read [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior).
- For regex compilation, string matching, or Zed regex compatibility, read the [pattern matcher and compatibility guidance](fetch-pattern-matching-and-regex-compatibility.md).

## Extend Parent Workflow

Apply the shared policy and every selected branch throughout the workflow chosen in the parent skill. For an unexpected native path or terminal permission outcome, first establish whether any repository-configured override participates rather than assuming the default configuration remains unchanged. Then follow [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior) for the tool’s distinct authorization layers.

For every read-only workflow, treat configured regexes and proposed cases as inert strings.

## Resolve Effective Permission Behavior

After resolving version-sensitive behavior through the parent [investigation workflow](../SKILL.md#investigate-and-plan):

1. Identify the selected tool’s implementation and whether it invokes configured permission evaluation. Only when it does, identify every participating Zed settings layer and any tool-specific overrides, then resolve the tool’s effective default and accumulated pattern arrays.
2. For `fetch`, treat any empty or regex-invalid effective pattern as denying the tool before pattern precedence.
3. When every effective fetch pattern is valid, apply deny, confirm, allow, then default precedence to the initial URL. Separately resolve the effective network mode, shared host grants, and native URL and DNS checks for the initial hostname and every redirect hostname under the [network policy](fetch-and-network-permissions.md#apply-fetch-and-network-permission-policy).
4. For `terminal`, evaluate Zed’s built-in behavior, the effective configured decision, task authorization, and operating system sandbox effects separately.
5. For a native path tool, when its implementation invokes configured permission evaluation, evaluate the effective configured decision, task authorization, and applicable built-in path, privacy, sensitive-settings, and symlink escape checks. When it does not, omit the configured permission layer and evaluate task authorization plus those built-in checks. Do not attribute native path behavior to the operating system sandbox.

Account for every participating settings layer. The pattern matcher evaluates only the supplied pattern and string, not the complete configured fetch layer.

## Apply Fetch Pattern or Default Changes

1. Inspect the actual fetch settings and participating layers. Identify expected configured decisions for the relevant [URL cases](fetch-and-network-permissions.md#build-fetch-rule-corpus), including intended changes and representative unchanged cases.
2. Check proposed patterns through the [pattern matcher](fetch-pattern-matching-and-regex-compatibility.md#check-a-pattern), using each pattern’s explicit case-sensitivity value. Resolve configured decisions separately through [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior).
3. Apply only authorized field-level edits with a native file editing tool under the parent mutation rules, then validate the final settings below.

## Validate Permission Changes

At the branch-specific step of the parent change validation workflow, follow [Validate Changes](fetch-and-network-permissions.md#validate-changes).

## Validate Permission Audits, Diagnoses, and Reviews

Apply the relevant fetch and network policy to the in-scope patterns or sandbox network settings. Follow [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior) only when the audit, review, or diagnosis includes effective authorization, an observed permission outcome, or settings behavior. Use configured pattern matching as one input to that analysis, not as evidence of network access or runtime behavior.

## Extend Reports

For a fetch permission change, state which initial URLs become allowed, confirmable, or denied at the configured tool layer. Distinguish those decisions from the effective network mode and native fetch checks rather than describing them as unconditional network access.

For a permission diagnosis, state the observed result, participating settings layer, configured decision, applicable operating system sandbox or native network check result, root cause, and corrective action without applying it.
