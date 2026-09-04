# Task Relays

Task relays assign future work to an external agent. Apply the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) when composing them and its [Delivery](../SKILL.md#delivery) rules when returning them.

## Task Relay Confirmation

Confirm the proposed handoff before drafting a task relay.

### Resolve Repository Isolation

For Git repository work, determine whether the receiving task will use the current checkout or an isolated worktree before presenting the flow. Resolve the choice from direct user instructions, applicable project policy, and established task context. The task itself may settle the decision.

Use an isolated worktree only for an explicit user request, another active agent with overlapping write scope, required branch, dependency, build, or test isolation, or a broad or high-risk change that materially benefits from independent rollback and has a clear integration plan. Do not isolate merely because the repository is dirty, concurrent activity is possible, or the task changes files. Keep follow-up edits to the same uncommitted task in its existing checkout.

If a material choice remains unresolved, ask the user explicitly, and emit neither the flow nor the relay. Do not ask merely because no worktree was mentioned.

### Present the Flow

Present the final flow in its own response. Keep it succinct, but include the receiving action, material target environment, worktree decision when repository work is involved, scope and exclusions, mutation and approval boundaries, required execution steps, validation, and handoff mode.

For every required dependency addition or update, choose the smallest sufficient set. Prefer an existing dependency or standard-library capability when sufficient, and enable only required features. Before requesting approval, identify each proposed addition or update exactly and state its consumers, declaration location, installation location when relevant, and purpose. Explain why existing dependencies or standard-library capabilities are insufficient and why a custom implementation would be less correct, maintainable, proportionate, or secure. Disclose any material feature, licensing, runtime, supply-chain, or version implications.

Ask the user to confirm or correct the flow, and do not include the task relay in that response. After confirmation, emit the complete relay without recapping the flow.

### Keep Confirmation Narrow

Confirmation authorizes only what the flow states explicitly and what applicable approval gates permit. It never grants commit authorization, and it does not authorize unstated remote submissions, publication, secret access, dependencies, or scope expansion.

A confirmation grants dependency approval only when the flow names the exact addition or update. Only a direct user response can grant that approval. Do not infer it from intent, silence, an agent proposal, or permission for adjacent work. An agent or subagent cannot approve on the user’s behalf. A task relay may carry dependency approval only when it identifies the explicit user response that granted it. If the receiving agent later discovers an unapproved dependency addition or update, require it to stop and ask the user rather than treating the relay as authorization.

Present a revised flow whenever composition or a later revision materially changes the confirmed action, target, worktree decision, scope, approval, execution, validation, or handoff. Meaning-neutral compression and formatting do not require reconfirmation.

This confirmation gate applies only to task relays. It does not apply to autonomous in-client delegation or evidence-only decision relays.

## Assignment Contract

End every initial or follow-up prompt that assigns future work with the exact standalone line `**Do not drift.**`. This applies to evidence gathering, review, and mutation assignments.

Before the guard, define the bounded assignment, owned scope, exclusions, source and access constraints, stop conditions, and output contract. Put every required result, process, validation step, and handoff instruction before it.

Every assignment inherits the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent cannot expand scope, provide user-only approval, transfer access, or circumvent a boundary. Require it to return any boundary request to its coordinator or the user rather than crossing it.

For a repository the user works in, an assignment may authorize an operation that writes a commit only when it identifies the user’s explicit command for that operation. Completed work, staged changes, passing validation, a confirmed flow, an approved plan, and permission to edit authorize working-tree changes only. This restriction does not apply to fixture commits in disposable repositories when the source task’s policy already exempts them.

Use the guard only when the prompt assigns future work. Omit it from decision relays and other transfers of established data. A receiving action alone does not turn an evidence handoff into an assignment.

## Task Relay Composition

Compose the task relay from the confirmed flow. If drafting exposes a material ambiguity or requires a material change, return to confirmation instead of choosing silently.

Include only the applicable parts of this sequence:

1. Title and receiving action.
2. Task context, authoritative evidence, and material target information.
3. Scope, exclusions, mutation boundary, approval boundary, and behavior-preservation requirements.
4. Required result, mandatory process constraints, validation, and known limitations.
5. Handoff mode, stopping point, and exact final anti-drift guard.

Apply the [Assignment Contract](#assignment-contract), and carry only approvals whose applicable gate has been satisfied. Treat the confirmed flow as a record of those approvals rather than a substitute for their authorization source.

Omit the receiving location by default. Include a repository, checkout, worktree, directory, host, or other execution location only when it is needed to find the inputs, distinguish possible targets, preserve isolation, or satisfy a submission or integration boundary. Material target paths may still be required.

Default to a one-way handoff in which the receiving conversation owns completion, with no return relay required. Require an evidence-only [decision relay](decision-relays.md) in return only when the originating conversation remains responsible for synthesis, integration, validation, comparison, or follow-up.

Record only the confirmed isolation requirement. The receiving environment’s repository policy governs worktree creation, operation, and cleanup.

## User-Requested Subagent Prompts

When the user explicitly asks to draft or review a subagent assignment, apply the [Assignment Contract](#assignment-contract). Preserve the inherited scope, source, access, approval, mutation, stopping, and output boundaries.

Do not frame the assignment as a relay, present a relay flow, or ask for confirmation merely because in-client delegation will occur. This workflow does not mediate autonomous in-client delegation. A boundary in the underlying task may still require a user decision through the policy that owns it.
