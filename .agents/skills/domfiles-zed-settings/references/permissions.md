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
- For pattern compilation or matching, Zed regex compatibility, permission-decision reconstruction, or pattern-family comparison, read the [Permission evaluator](permission-evaluator.md).

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

1. Compile every pattern declared by the complete candidate suite through the [pattern-compilation workflow](permission-evaluator.md#compile-permission-patterns).
2. Validate every declared pattern and configured-pattern precedence through the [pattern-matching workflow](permission-evaluator.md#match-permission-patterns).
3. Resolve the candidate’s complete effective permission behavior through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior).
4. Verify the shared and selected-branch permission invariants against the candidate, then promote only the validated settings subtree.
5. Reparse and format the promoted settings, then confirm the live subtree matches the candidate.

## Validate a permission audit, review, or diagnosis

At the branch-specific step of the parent read-only validation workflow:

1. Compile relevant existing regexes through the [pattern-compilation workflow](permission-evaluator.md#compile-permission-patterns).
2. Validate the patterns and configured-pattern precedence against representative intended inputs, hazardous forms, and near misses required by the selected branch through the [pattern-matching workflow](permission-evaluator.md#match-permission-patterns).
3. Resolve the complete effective permission behavior for each representative operation through [Evaluate permission behavior](permission-evaluator.md#evaluate-permission-behavior).
4. Verify the shared and selected-branch permission invariants against the audited contents.

## Extend the report

For a permission change, state which forms are now allowed, which hazardous forms still require confirmation, and which requested forms were intentionally left confirmable.

For a permission diagnosis, state the observed result, evaluated inputs, matching precedence, root cause, and corrective action without applying it.
