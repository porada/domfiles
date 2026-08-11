# Agent permissions

Use this conditional branch whenever the resolved scope includes `agent.tool_permissions`, `agent.sandbox_permissions`, terminal rules, native path-tool permissions, fetch or network allowances, agent repository permissions, or a tool or command unexpectedly allowing, confirming, or denying. Agent permissions are configured through `.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for investigation, mutation boundaries, general validation, and reporting.

Do not read every permission reference by default. Select only the branches required by the resolved scope.

## Apply the shared permission policy

- Treat Zed agent permissions as layered security boundaries.
- Preserve `agent.tool_permissions.default` as `allow`.

## Select a permission branch

- For terminal commands and their permission patterns, read [Terminal permissions](terminal-permissions.md).
- For Git commands or permission patterns, read both [Terminal permissions](terminal-permissions.md) and [Git permissions](git-permissions.md). The Git branch partitions command ownership within the terminal policy’s `git` executable family.
- For fetch patterns, domains, URLs, and sandbox hosts, read [Fetch and network permissions](fetch-permissions.md).
- For agent worktree or disposable fixture repository permissions, read [Agent repository permissions](agent-repository-permissions.md).
- For pattern inventory, owner auditing, candidate promotion, pattern compilation or matching, Zed regex compatibility, permission-decision reconstruction, or pattern-family comparison, read the [Permission evaluator](permission-evaluator.md).

Read every applicable branch when one task crosses these boundaries. Do not load terminal policy for fetch-only work or fetch policy for terminal-only work.

## Extend the parent workflow

- For an explicit permission change, including a request that also uses review or audit language, follow the parent change workflow and apply the shared policy plus every selected permission branch.
    - For a permission-pattern change, follow [Build and promote a permission candidate](permission-evaluator.md#build-and-promote-a-permission-candidate) before modifying live settings.
    - For an explicitly requested domain or URL allowance, follow [Translate approved domains and URLs](fetch-permissions.md#translate-approved-domains-and-urls) before any network access to the requested destination.
    - For an explicitly authorized Zed regex compatibility repair, follow [Audit Zed regex compatibility](permission-evaluator.md#audit-zed-regex-compatibility).
- For a standalone documentation audit that includes the Zed permission regex compatibility rationale, follow [Audit Zed regex compatibility](permission-evaluator.md#audit-zed-regex-compatibility) and keep the comparison read-only.
- For a standalone permission audit, keep the task read-only, follow the [repository audit process](../../domfiles-repository-audit/SKILL.md), and use the permission read-only validation below.
- For a standalone permission review, keep the task read-only and use the permission read-only validation below without change planning, implementation, or formatting.
- For a standalone permission diagnosis, keep the task read-only, follow the parent diagnosis workflow, apply every selected branch, and use the permission read-only validation below.

For every read-only workflow:

- Treat terminal command candidates as inert strings and evaluate them only through permission-pattern matching.
- Limit shell execution to the bounded, non-mutating inspection and validation utilities required by the selected branches.

## Plan a permission change

1. Apply the shared policy and every selected branch to the observed behavior.
2. Identify the smallest existing permission object or command-owner group that owns the change.
3. Enumerate the required syntactic variants before writing a regex.

## Validate a permission change

At the branch-specific step of the parent change-validation workflow:

1. Audit every complete in-scope owner group through [Audit permission ownership](permission-evaluator.md#audit-permission-ownership).
2. Compile and validate every changed pattern, participating overlap, and configured-precedence case through [Compile and match permission patterns](permission-evaluator.md#compile-and-match-permission-patterns).
3. Compare baseline and candidate bucket unions plus configured decisions through [Compare baseline and candidate behavior](permission-evaluator.md#compare-baseline-and-candidate-behavior), except when object and match behavior are both unchanged.
4. Resolve the candidate’s complete effective behavior through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior), verify every selected-branch invariant, then run the candidate’s guarded verification and promotion.
5. Reparse and format the promoted settings, then confirm each live promoted scope matches the validated candidate.

## Validate a permission audit, review, or diagnosis

At the branch-specific step of the parent read-only validation workflow:

1. Inventory relevant patterns and audit complete in-scope owner groups through the [permission evaluator](permission-evaluator.md).
2. Compile and validate relevant patterns and configured precedence against representative intended inputs, hazardous forms, and near misses through [Compile and match permission patterns](permission-evaluator.md#compile-and-match-permission-patterns).
3. Use [Compare baseline and candidate behavior](permission-evaluator.md#compare-baseline-and-candidate-behavior) when the read-only scope includes two settings states or a proposed transformation.
4. Resolve complete effective behavior for each representative operation through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior), then verify every shared and selected-branch invariant.

## Extend the report

For a permission change, state which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable.

For a permission diagnosis, state the observed result, evaluated inputs, matching precedence, root cause, and corrective action without applying it.

When one operation crosses terminal, fetch, or sandbox network boundaries, evaluate and state each applicable layer’s result independently. Do not describe one layer’s allowance as unconditional execution or imply that a terminal allowance grants network access.
