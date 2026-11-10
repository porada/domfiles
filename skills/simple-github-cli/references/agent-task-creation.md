# Agent Task Creation

`gh agent-task create` assigns future work to an external agent and creates remote GitHub state. Follow [Handoff Confirmation](#handoff-confirmation) before composing the assignment. Use the communication workflow resolved by the entrypoint, or [Task Description](#task-description) when none is available. Then continue at [Command Execution](#command-execution), which replaces the communication workflow’s normal delivery. Do not also return the assignment as a relay.

## Handoff Confirmation

Establish the receiving action, target repository and base, scope and exclusions, source and access constraints, mutation and approval boundaries, required process and validation, and completion or return mode before execution.

When a direct user request already authorizes the clear, bounded handoff and task creation, present covered effects as a notice and continue without duplicate confirmation. Otherwise, present the proposed flow in its own response before drafting the task description or command, and ask only for the missing authorization or material decision.

Confirmation authorizes only the stated handoff. It satisfies the entrypoint’s [Remote Changes](../SKILL.md#remote-changes) gate only when the user explicitly authorizes creating the agent task against the named target.

For any direct or indirect commit-writing operation, carry in the assignment the explicit user command or applicable governing grant covering the receiving agent, target, effects, and lifetime. Completed work, staged changes, passing validation, a confirmed handoff, an approved plan, and permission to edit don’t independently supply that authority. Don’t infer permission for unstated remote submissions, publication, secret access, dependency changes, or scope expansion.

Carry existing approvals with their exact user source and scope. Require the receiving agent to follow applicable project policies and request any required approval not already covered by the assignment.

## Task Description

After establishing authorization, compose a task description with a descriptive heading and an explicit receiving action. Define the bounded assignment, owned scope, exclusions, source and access constraints, mutation and approval boundaries, required process and validation, stop conditions, and output or handoff contract. For a review assignment, require an explicit statement of whether any in-scope findings remain standing.

Preserve the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent can’t expand scope, provide user-only approval, transfer access, or circumvent a boundary, and that it must return any boundary request to the user rather than crossing it.

Don’t include credentials, tokens, private keys, secret values, secret-bearing URLs, or private machine or account values. Establish the task description’s audience and the authorized agent processing services before including necessary non-secret context. Keep content destined for public repository state suitable for publication. An agent assignment isn’t inherently private, and authorization to send context to one recipient doesn’t authorize its publication or transfer to another provider. Reuse an existing direct user instruction or applicable governing grant only within its scope and lifetime. Resolve uncertain audience or uncovered disclosure effects before sending. Rely only on access already available in the receiving environment.

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

Don’t add `--follow` unless the user explicitly requests log following.
