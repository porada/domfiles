# Agent permissions

Agent permissions are configured through `.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for general validation, investigation, and mutation boundaries.

Do not read every permission reference by default. Select only the branches required by the resolved scope, and within each branch read the sections the task needs rather than the complete file.

## Apply the shared permission policy

- Preserve `agent.tool_permissions.default` as `allow`.
- Keep `delete_path`, `move_path`, and terminal free of repository-configured overrides unless the task explicitly changes the repository’s [permission model](../../../PROJECT.md#zed-agent-permission-model).
- Treat the always-loaded global agent policy as the canonical owner of authentication handling, command intent, security-boundary restrictions, and task authorization. Do not encode those policies as terminal command patterns.
- Treat configured tool permissions and Zed’s operating-system sandbox as independent layers. A tool-permission `allow` does not grant a filesystem, Git metadata, or network effect outside the sandbox.
- Preserve fetch as the only tool with a separate repository-configured prompt model.

## Select a permission branch

- For fetch patterns, domains, URLs, redirects, or sandbox hosts, read [Fetch and network permissions](fetch-permissions.md).
- For fetch pattern compilation, configured decisions, before-and-after comparison, or Zed regex compatibility, read the [Permission evaluator](permission-evaluator.md).

## Extend the parent workflow

Apply the shared policy and every selected branch throughout the workflow chosen in the parent skill, with these additions:

- For an explicitly requested domain or URL allowance, follow [Translate approved domains and URLs](fetch-permissions.md#translate-approved-domains-and-urls) before any network access to the requested destination.
- For a fetch pattern or fetch default change, [prepare a fetch settings candidate](#prepare-a-fetch-settings-candidate) instead of editing canonical settings directly.
- For a standalone documentation audit that includes the Zed permission regex compatibility rationale, follow [Audit Zed regex compatibility](permission-evaluator.md#audit-zed-regex-compatibility) read-only. Treat any requested dependency change as a separate task under the global “Dependencies” policy.
- For an unexpected native path or terminal permission outcome, first establish that no repository-configured override participates. Then distinguish Zed’s built-in tool behavior, the effective global default, task authorization, and the applicable sandbox decision.

For every read-only workflow, treat configured regexes and proposed cases as inert strings. Do not make a live request merely to validate a permission setting.

## Plan a permission change

1. Identify the smallest permission or sandbox-host object that owns the change.
2. For a fetch pattern or fetch default change, enumerate the required URL cases and expected configured decisions, including one deciding-source witness for the default and every nonempty bucket in both states. If a source is fully shadowed and no witness can be identified, report that the ordinary change workflow does not support the configuration and stop before creating a candidate. Do not change unrelated patterns merely to make a source reachable.

## Prepare a fetch settings candidate

For every fetch pattern or fetch default change:

1. Check the evaluator’s [target-contract gate](permission-evaluator.md#apply-the-matcher-contract). While the gate remains closed, stop before creating a candidate or mutating canonical settings.
2. In the task-specific directory selected through the global “Temporary files” policy, copy the exact current `.config/zed/settings.json` into separate immutable baseline and editable candidate files.
3. Construct the baseline configured-layer manifest from the planned corpus, then validate the baseline’s [configured fetch layer](permission-evaluator.md#validate-a-configured-fetch-layer). Any decision-source, pattern, or settings finding makes the ordinary candidate workflow unavailable. Stop before candidate mutation, report the invalid baseline, and require a separately authorized repair plan.
4. Apply the authorized change only to the candidate, then construct its configured-layer manifest and the comparison manifest from the planned cases.
5. Validate the candidate’s [configured fetch layer](permission-evaluator.md#validate-a-configured-fetch-layer) and [compare it with the baseline](permission-evaluator.md#compare-fetch-permission-states).
6. Immediately before promotion, verify that canonical settings remain byte-identical to the baseline. If they differ, stop and report the concurrent change rather than overwriting it.
7. Promote the candidate contents to the canonical path while preserving its file type and mode. Verify that canonical settings are byte-identical to the candidate, then resume the parent change-validation workflow.

## Validate a permission change

At the branch-specific step of the parent change-validation workflow:

- For a fetch pattern or fetch default change, require the completed candidate workflow’s baseline-to-candidate and configured-layer results, then verify that canonical settings remain byte-identical to the promoted candidate.
- For a `network_hosts`-only change, verify the approved exact and wildcard coverage plus complete-array ordering without invoking the pattern matcher.
- Resolve fetch-tool and sandbox-host behavior as independent layers, then resume the parent workflow against the edited settings file.

## Validate a permission audit, review, or diagnosis

Apply the relevant read-only branch to the in-scope fetch patterns or sandbox hosts. Use configured pattern matching as one input to the effective-permission analysis, not as evidence of network access or runtime behavior.

## Extend the report

For a fetch permission change, state the direct initial URLs that become prompt-free, the URLs that remain confirmable at the fetch layer, and the independently approved sandbox hosts. Do not describe either layer as unconditional network access.

For a permission diagnosis, state the observed result, participating settings layer, configured decision, sandbox decision, root cause, and corrective action without applying it.
