# Agent permissions

Agent permissions are configured through `.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for general validation, investigation, and mutation boundaries.

Do not read every permission reference by default. Select only the branches required by the resolved scope, and within each branch read the sections the task needs rather than the complete file.

## Apply the shared permission policy

- Preserve `agent.tool_permissions.default` as `allow`.
- Preserve `fetch` as the only tool with repository-configured overrides. Do not add or modify an override for any non-fetch tool. When a task requests one, stop before mutation and report that a dedicated workflow and validation contract are required.
- Treat the always-loaded global agent policy as the canonical owner of authentication handling, command intent, security-boundary restrictions, and task authorization. Do not encode those policies as terminal command patterns.
- Treat configured tool permissions, the operating-system sandbox for terminal processes, and native fetch host-grant authorization as distinct layers. The operating-system sandbox applies to `terminal`, not native `fetch` or native path tools. A tool-permission `allow` does not grant a `terminal` effect outside that sandbox or let `fetch` bypass Zed’s separate host-grant authorization. For a native path tool, first establish whether its implementation invokes configured permission evaluation. When it does, combine that decision with the applicable built-in checks. When it does not, omit the configured-permission layer and evaluate only the built-in path, privacy, sensitive-settings, and symlink-escape checks that its implementation applies.

## Select a permission branch

- For before-and-after comparison, configured decisions, fetch pattern compilation, or Zed regex compatibility, read the [Permission evaluator](permission-evaluator.md).
- For domains, fetch patterns, network hosts, redirects, or URLs, read [Fetch and network permissions](fetch-permissions.md).
- For effective authorization, an observed permission outcome, or settings behavior involving `fetch`, a native path tool, or `terminal`, read [Resolve effective permission behavior](permission-evaluator.md#resolve-effective-permission-behavior).

## Extend the parent workflow

Apply the shared policy and every selected branch throughout the workflow chosen in the parent skill, with these additions:

- For an explicitly requested domain or URL allowance, follow [Translate approved domains and URLs](fetch-permissions.md#translate-approved-domains-and-urls) before any network access to the requested destination other than the bounded workflow-complete host review that procedure defines.
- For a standalone documentation audit that includes the Zed permission regex compatibility rationale, follow [Audit Zed regex compatibility](permission-evaluator.md#audit-zed-regex-compatibility) read-only. Treat any requested dependency change as a separate task under the global “Dependencies” policy.
- For an unexpected native path or terminal permission outcome, first establish that no repository-configured override participates. Then follow [Resolve effective permission behavior](permission-evaluator.md#resolve-effective-permission-behavior) for the tool’s distinct authorization layers.

For every read-only workflow, treat configured regexes and proposed cases as inert strings.

## Plan a permission change

1. Identify the smallest permission or network-host object that owns the change.
2. For a fetch pattern or fetch default change, enumerate the required URL cases and expected configured decisions, including one deciding-source witness for the default and every nonempty bucket in both states. If a source is fully shadowed and no witness can be identified, report that the ordinary change workflow does not support the configuration and stop before creating a candidate. Do not change unrelated patterns merely to make a source reachable.

## Apply a fetch pattern or default change

1. Run the pattern matcher’s focused [contract test](permission-evaluator.md#run-focused-contract-tests). Stop before candidate creation when it fails.
2. In the task-specific directory selected through the global “Temporary files” policy, copy the complete current `.config/zed/settings.json` into separate baseline and candidate files. Do not intentionally modify the baseline.
3. Build a baseline layer manifest and run the [configured-layer route](permission-evaluator.md#validate-a-configured-fetch-layer) against the baseline. Require status `0` before candidate mutation. Correct an authored manifest disagreement, but treat an empty, overlong, or invalid baseline pattern as a settings repair that requires separate authorization.
4. Apply only the authorized change to the candidate. Build the candidate layer and comparison manifests, then require status `0` from the [configured-layer](permission-evaluator.md#validate-a-configured-fetch-layer) and [comparison](permission-evaluator.md#compare-fetch-permission-states) routes.
5. Review the complete baseline-to-candidate diff and both layer manifests plus the comparison manifest. Confirm that every candidate difference belongs to the authorized fetch fields.
6. Immediately before editing canonical settings, require their complete bytes to remain identical to the baseline. On drift, preserve the current file, rebuild the baseline and candidate from it, and repeat the applicable validation rather than overwriting the concurrent change.
7. Apply only the reviewed field-level delta to canonical settings with a native file-editing tool. Do not replace the complete canonical file with the candidate. Inspect the complete baseline-to-canonical and scoped repository diffs immediately afterward, and stop for reconciliation if either contains an unexpected change.

This workflow intentionally uses no promotion helper, review identity, lock, or transactional replacement. Candidate validation, the fresh live-file recheck, the field-level edit, and post-edit inspection reduce accidental overwrite without claiming a race-free update. Another writer can still act between those steps, which remains an ordinary concurrent-work risk rather than a separate security boundary.

## Validate a permission change

At the branch-specific step of the parent change-validation workflow, follow [Validate a change](fetch-permissions.md#validate-a-change). After a fetch pattern or default edit, rerun the candidate layer manifest against canonical settings and rerun the comparison manifest with the preserved baseline and canonical settings. Require status `0` from both evaluator routes before completing the parent validation.

## Validate a permission audit, review, or diagnosis

Apply the relevant fetch and network policy to the in-scope patterns or network hosts. Resolve effective permission behavior only when the audit, review, or diagnosis includes effective authorization, an observed permission outcome, or settings behavior. Use configured pattern matching as one input to that analysis, not as evidence of network access or runtime behavior.

## Extend the report

For a fetch permission change, state the direct initial URLs that become prompt-free, the URLs that remain confirmable at the fetch layer, and the independently approved host grants. Do not describe either layer as unconditional network access.

For a permission diagnosis, state the observed result, participating settings layer, configured decision, applicable operating-system sandbox or host-grant decision, root cause, and corrective action without applying it.
