# Agent permissions

Agent permissions are configured through `.config/zed/settings.json`, not project `.zed/settings.json`. Follow the parent [Zed settings workflow](../SKILL.md) for general validation, investigation, and mutation boundaries.

Do not read every permission reference by default. Select only the branches required by the resolved scope, and within each branch read the sections the task needs rather than the complete file.

## Apply the shared permission policy

- Treat Zed agent permissions as layered security boundaries.
- Treat configured Zed terminal allowances as a prompt-friction inventory of verified command forms, not a mirror of agent policy or task authorization. Agent policy may intentionally be stricter and change independently.
- For audits, do not report policy-to-allowance differences or align settings solely because documentation prohibits an automatically allowed form. Report a permission issue only when a pattern misstates verified command behavior, violates an explicit permission-layer invariant, or an applicable policy expressly requires `allow`, `confirm`, or `deny` enforcement.
- Preserve `agent.tool_permissions.default` as `allow`.

## Select a permission branch

- For terminal commands and their permission patterns, read [Terminal permissions](terminal-permissions.md).
- For Git commands or permission patterns, read both [Terminal permissions](terminal-permissions.md) and [Git permissions](git-permissions.md). The Git branch partitions command ownership within the terminal policy’s `git` executable family.
- For fetch patterns, domains, URLs, and sandbox hosts, read [Fetch and network permissions](fetch-permissions.md), adding the [Fetch candidate](fetch-candidate.md) workflow only when the task authorizes mutation.
- For agent-directory-scoped terminal allowances, agent worktree permissions, or disposable fixture repository permissions, read [Agent repository permissions](agent-repository-permissions.md).
- For pattern inventory, owner auditing, pattern compilation or matching, Zed regex compatibility, permission-decision reconstruction, or pattern-family comparison, read the [Permission evaluator](permission-evaluator.md).
- For building, sealing, and promoting a permission change, read the [Permission candidate](permission-candidate.md) as well. Every read-only workflow can stop at the evaluator.

## Extend the parent workflow

Apply the shared policy and every selected branch throughout the workflow chosen in the parent skill, with these additions:

- For a permission-pattern change, follow [Build and promote a permission candidate](permission-candidate.md#build-and-promote-a-permission-candidate) instead of modifying live settings directly.
- For an explicitly requested domain or URL allowance, follow [Translate approved domains and URLs](fetch-permissions.md#translate-approved-domains-and-urls) before any network access to the requested destination.
- For a standalone documentation audit that includes the Zed permission regex compatibility rationale, follow [Audit Zed regex compatibility](permission-evaluator.md#audit-zed-regex-compatibility) read-only. Enter that reference’s repair steps only when the user explicitly authorizes a compatibility repair.

For every read-only workflow, treat terminal command candidates as inert strings and do not execute them. Use permission-pattern matching as one input to the complete effective-permission evaluation. Limit shell execution to the bounded, non-mutating inspection and validation utilities required by the selected branches.

## Plan a permission change

1. Within the settings object selected by the parent workflow, identify the smallest permission object or command-owner group that owns the change.
2. Enumerate the required syntactic variants before writing a regex.

## Validate a permission change

At the branch-specific step of the parent change-validation workflow, complete every applicable [permission evaluator](permission-evaluator.md) workflow and resolve [effective permission behavior](permission-evaluator.md#evaluate-permission-behavior) for each representative operation. Verify every shared and selected-branch invariant, then follow [Build and promote a permission candidate](permission-candidate.md#build-and-promote-a-permission-candidate) through its evidence, sealing, rehearsal, approval, promotion or refresh, and post-promotion checks. Resume the parent workflow against the promoted settings files.

## Validate a permission audit, review, or diagnosis

At the branch-specific step of the parent read-only validation workflow, apply the relevant [permission evaluator](permission-evaluator.md) workflows to the in-scope patterns and complete owner groups, then resolve [effective permission behavior](permission-evaluator.md#evaluate-permission-behavior) for each representative operation. Compare baseline and candidate behavior only when the scope includes two settings states or a proposed transformation.

## Extend the report

For a permission change, state which forms are now allowed, which forms require confirmation, which forms are denied, and the relevant unmatched or default behavior. Distinguish intentionally stricter requested forms and any material precedence override without repeating unchanged inventories.

For a permission diagnosis, state the observed result, evaluated inputs, matching precedence, root cause, and corrective action without applying it.

When one operation crosses terminal, fetch, or sandbox network boundaries, evaluate and state each applicable layer’s result independently. Do not describe one layer’s allowance as unconditional execution or imply that a terminal allowance grants network access.
