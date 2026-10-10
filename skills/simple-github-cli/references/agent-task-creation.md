# Agent Task Creation

`gh agent-task create` assigns future work to an external agent and creates remote GitHub state. When the entrypoint resolves `agent-task-relay`, use it for handoff confirmation and assignment composition only, then continue at [Command Execution](#command-execution). Command execution replaces its normal delivery, so do not also return the assignment as a relay. Otherwise, follow the standalone [Handoff Confirmation](#handoff-confirmation) and [Task Description](#task-description) workflow below.

## Handoff Confirmation

Establish the receiving action, target repository and base, scope and exclusions, source and access constraints, mutation and approval boundaries, required process and validation, and completion or return mode before execution.

When a direct user request already authorizes the clear, bounded handoff and task creation, present covered effects as a notice and continue without duplicate confirmation. Otherwise, present the proposed flow in its own response before drafting the task description or command, and ask only for the missing authorization or material decision.

Confirmation authorizes only the stated handoff. It satisfies the entrypoint’s [Remote Changes](../SKILL.md#remote-changes) gate only when the user explicitly authorizes creating the agent task against the named target.

Handoff confirmation alone does not grant commit authorization. Carry an explicit user command to commit when the receiving task needs it, whether supplied in the same response or separately. Do not infer permission for unstated remote submissions, publication, secret access, dependency changes, or scope expansion.

Carry existing approvals with their exact user source and scope. Require the receiving agent to follow applicable project policies and request any required approval not already covered by the assignment.

## Task Description

After establishing authorization, compose a task description with a descriptive heading and an explicit receiving action. Define the bounded assignment, owned scope, exclusions, source and access constraints, mutation and approval boundaries, required process and validation, stop conditions, and output or handoff contract. For a review assignment, require an explicit statement of whether any in-scope findings remain standing.

Preserve the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent cannot expand scope, provide user-only approval, transfer access, or circumvent a boundary, and must return any boundary request to the user rather than crossing it.

Authorize an operation that writes a commit in any repository, directly or indirectly, only when the task description identifies the user’s explicit command to commit. Completed work, staged changes, passing validation, a confirmed flow, an approved plan, and permission to edit authorize working tree changes only.

Do not include credentials, tokens, private keys, secret values, secret-bearing URLs, or other private material. Rely only on access already available in the receiving environment.

End the task description with the exact standalone line:

`**Do not drift.**`

## Command Execution

Write the authorized task description to a task-local temporary file. Apply the entrypoint’s [Remote Changes](../SKILL.md#remote-changes) gate, then use explicit repository and base targets:

```sh
gh agent-task create \
    --from-file=<task-description-file> \
    --repo=<host-owner-repository> \
    --base=<base-ref>
```

Do not add `--follow` unless the user explicitly requests log following.
