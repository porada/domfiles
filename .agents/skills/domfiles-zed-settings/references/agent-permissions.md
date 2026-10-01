# Agent Permissions

Agent permissions are configured through `home/.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for general validation, investigation, and mutation boundaries.

Do not read every permission reference by default. Select only the branches required by the resolved scope, and within each branch read the sections the task needs rather than the complete file.

## Apply Shared Permission Policy

- Keep `fetch` as the only tool with repository-configured overrides unless the user expressly requests another configuration under the parent [general policy](../SKILL.md#apply-general-policy). Use the parent change workflow for that departure rather than treating the absence of a specialized validation tool as a categorical prohibition.
- Preserve `agent.tool_permissions.default` as `allow` by default.
- Treat the always-loaded global agent policy as the canonical owner of authentication handling, command intent, security boundary restrictions, and task authorization. Do not encode those policies as terminal command patterns.
- Treat configured tool permissions, the operating system sandbox for terminal processes, and native fetch host grant authorization as distinct layers. The operating system sandbox applies to `terminal`, not native `fetch` or native path tools. A tool permission `allow` does not grant a `terminal` effect outside that sandbox or let `fetch` bypass Zed’s separate host grant authorization. For a native path tool, first establish whether its implementation invokes configured permission evaluation. When it does, combine that decision with the applicable built-in checks. When it does not, omit only the configured permission layer and evaluate the built-in path, privacy, sensitive-settings, and symlink escape checks that its implementation applies. Task authorization under the global agent policy remains required in either case.

## Select Permission Branches

- For before-and-after comparison, configured decisions, fetch pattern compilation, or Zed regex compatibility, read the [pattern matcher and regex compatibility contracts](fetch-pattern-matching-and-regex-compatibility.md).
- For domains, fetch patterns, network hosts, redirects, or URLs, read [Fetch and Network Permissions](fetch-and-network-permissions.md).
- For effective authorization, an observed permission outcome, or settings behavior involving `fetch`, a native path tool, or `terminal`, read [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior).

## Extend Parent Workflow

Apply the shared policy and every selected branch throughout the workflow chosen in the parent skill, with these additions:

- For an explicitly requested domain or URL allowance, follow [Translate Approved Domains and URLs](fetch-and-network-permissions.md#translate-approved-domains-and-urls) before any network access to the requested destination other than the bounded workflow-complete host review that procedure defines.
- For a standalone documentation audit that includes Zed permission regex compatibility, follow [Audit Zed Regex Compatibility](fetch-pattern-matching-and-regex-compatibility.md#audit-zed-regex-compatibility) read-only.
- For an unexpected native path or terminal permission outcome, first establish whether any repository-configured override participates rather than assuming the default configuration remains unchanged. Then follow [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior) for the tool’s distinct authorization layers.

For every read-only workflow, treat configured regexes and proposed cases as inert strings.

## Resolve Effective Permission Behavior

After resolving version-sensitive behavior through the parent [investigation workflow](../SKILL.md#investigate-and-plan):

1. Identify the selected tool’s implementation and whether it invokes configured permission evaluation. Only when it does, identify every participating Zed settings layer and any tool-specific overrides, then resolve the tool’s effective default and accumulated pattern arrays.
2. For `fetch`, treat any empty or regex-invalid effective pattern as denying the tool before pattern precedence.
3. When every effective fetch pattern is valid, apply deny, confirm, allow, then default precedence to the initial URL. Evaluate shared host grant authorization for the initial hostname and every redirect hostname independently.
4. For `terminal`, evaluate Zed’s built-in behavior, the effective configured decision, task authorization, and operating system sandbox effects separately.
5. For a native path tool, when its implementation invokes configured permission evaluation, evaluate the effective configured decision, task authorization, and applicable built-in path, privacy, sensitive-settings, and symlink escape checks. When it does not, omit the configured permission layer and evaluate task authorization plus those built-in checks. Do not attribute native path behavior to the operating system sandbox.

When another participating layer contributes fetch rules or a different default, inspect and account for that layer separately from the matcher’s [single-file result](fetch-pattern-matching-and-regex-compatibility.md#validate-configured-fetch-layers).

## Plan Permission Changes

For a fetch pattern or fetch default change, enumerate the required URL cases and expected configured decisions, including one deciding-source witness for the default and every nonempty bucket in both states. If a source is fully shadowed and no witness can be identified, report that the ordinary change workflow does not support the configuration and stop before creating a candidate. Do not change unrelated patterns merely to make a source reachable.

## Apply Fetch Pattern or Default Changes

1. Run the pattern matcher’s focused [contract test](fetch-pattern-matching-and-regex-compatibility.md#run-focused-contract-tests). Stop before candidate creation when it fails.
2. In a task directory established through `agent-task-directories`, copy the complete current `home/.config/zed/settings.json` into separate baseline and candidate files. Do not intentionally modify the baseline.
3. Build a baseline layer manifest and run the [configured layer route](fetch-pattern-matching-and-regex-compatibility.md#validate-configured-fetch-layers) against the baseline. Require status `0` before candidate mutation. Correct an authored manifest disagreement, but treat an empty, overlong, or invalid baseline pattern as a settings repair that requires separate authorization.
4. Apply only the authorized change to the candidate. Build the candidate layer and comparison manifests, then require status `0` from the [configured layer](fetch-pattern-matching-and-regex-compatibility.md#validate-configured-fetch-layers) and [comparison](fetch-pattern-matching-and-regex-compatibility.md#compare-fetch-permission-states) routes.
5. Review the complete baseline-to-candidate diff and both layer manifests plus the comparison manifest. Confirm that every candidate difference belongs to the authorized fetch fields.
6. Immediately before editing canonical settings, require their complete bytes to remain identical to the baseline. On drift, preserve the current file, rebuild the baseline and candidate from it, and repeat the applicable validation rather than overwriting the concurrent change.
7. Apply only the reviewed field-level delta to canonical settings with a native file editing tool. Do not replace the complete canonical file with the candidate. Inspect the complete baseline-to-canonical and scoped repository diffs immediately afterward, and stop for reconciliation if either contains an unexpected change.

Candidate validation, the fresh live-file recheck, the field-level edit, and post-edit inspection reduce accidental overwrite but do not make the update atomic. Another writer can still act between those steps, which remains an ordinary concurrent work risk rather than a separate security boundary.

## Validate Permission Changes

At the branch-specific step of the parent change validation workflow, follow [Validate Changes](fetch-and-network-permissions.md#validate-changes). After a fetch pattern or default edit, rerun the candidate layer manifest against canonical settings and rerun the comparison manifest with the preserved baseline and canonical settings. Require status `0` from both pattern matcher routes before completing the parent validation.

## Validate Permission Audits, Diagnoses, and Reviews

Apply the relevant fetch and network policy to the in-scope patterns or network hosts. Follow [Resolve Effective Permission Behavior](#resolve-effective-permission-behavior) only when the audit, review, or diagnosis includes effective authorization, an observed permission outcome, or settings behavior. Use configured pattern matching as one input to that analysis, not as evidence of network access or runtime behavior.

## Extend Reports

For a fetch permission change, state the direct initial URLs that become prompt-free, the URLs that remain confirmable at the fetch tool layer, and the independently approved host grants. Do not describe either layer as unconditional network access.

For a permission diagnosis, state the observed result, participating settings layer, configured decision, applicable operating system sandbox or host grant decision, root cause, and corrective action without applying it.
