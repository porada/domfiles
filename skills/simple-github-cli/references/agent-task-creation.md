# Agent Task Creation

`gh agent-task create` assigns future work to an external agent and creates remote GitHub state. When the entrypoint resolves `agent-task-relay`, use it for handoff confirmation and assignment composition only, then continue at [Command Execution](#command-execution). Command execution replaces its normal delivery, so do not also return the assignment as a relay. Otherwise, follow the standalone [Handoff Confirmation](#handoff-confirmation) and [Task Description](#task-description) workflow below.

## Handoff Confirmation

Before drafting the task description or composing the command, present the proposed handoff flow in its own response. Include the receiving action, target repository and base, scope and exclusions, source and access constraints, mutation and approval boundaries, required process and validation, and completion or return mode.

Resolve any material ambiguity before presenting the flow. Ask the user to confirm or correct it, and do not include the task description or command in that response.

Confirmation authorizes only the stated handoff. It satisfies the entrypoint’s [Remote Changes](../SKILL.md#remote-changes) gate only when the user explicitly authorizes creating the agent task against the named target.

Confirmation never grants commit authorization. It does not authorize unstated remote submissions, publication, secret access, dependency changes, or scope expansion. When the receiving task requires a commit, obtain the user’s explicit command authorizing it before command execution.

Before presenting a flow that includes a dependency addition or update, apply the entrypoint’s [Dependency Changes](../SKILL.md#dependency-changes) policy.

Confirmation grants dependency approval only when the flow names the exact addition or update and the user explicitly approves it. An agent cannot provide that approval on the user’s behalf. Carry approval into the task description only when it identifies the user’s direct response that granted it. If the receiving agent discovers an unapproved dependency addition or update, require it to stop and ask the user.

## Task Description

After confirmation, compose a task description with a descriptive heading and an explicit receiving action. Define the bounded assignment, owned scope, exclusions, source and access constraints, mutation and approval boundaries, required process and validation, stop conditions, and output or handoff contract.

Preserve the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent cannot expand scope, provide user-only approval, transfer access, or circumvent a boundary, and must return any boundary request to the user rather than crossing it.

For a repository the user works in, authorize an operation that writes a commit only when the task description identifies the user’s explicit command to commit. Completed work, staged changes, passing validation, a confirmed flow, an approved plan, and permission to edit authorize working-tree changes only.

Do not include credentials, tokens, private keys, secret values, secret-bearing URLs, or other private material. Rely only on access already available in the receiving environment.

End the task description with the exact standalone line:

`**Do not drift.**`

## Command Execution

Write the confirmed task description to a task-local temporary file. Apply the entrypoint’s [Remote Changes](../SKILL.md#remote-changes) gate, then use explicit repository and base targets:

```sh
gh agent-task create \
    --from-file=<task-description-file> \
    --repo=<host-owner-repository> \
    --base=<base-ref>
```

Do not add `--follow` unless the user explicitly requests log following.
