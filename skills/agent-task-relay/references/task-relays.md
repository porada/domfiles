# Task Relays

Task relays assign future work to an external agent operating in another conversation or execution environment rather than as an in-client subagent.

## External Handoffs

A direct request to assign work begins this workflow. A tentative question suggests a relay and waits for the user’s choice. An incidental or quoted mention of another agent does not route the task.

When the task requires access available only in another conversation, client, host, authenticated session, or project, suggest a relay as soon as that boundary is established. If an attempted access operation revealed the boundary, report the exact limitation before proposing the handoff.

Never use an in-client subagent to cross or circumvent an environment, access, authentication, repository, or permission boundary. Identify the receiving environment only as precisely as execution requires, never transfer credentials or other secret material, and rely only on access already available to the external agent.

## Task Relay Confirmation

Apply the [Assignment Contract](assignment-prompts.md#assignment-contract) before resolving or confirming the flow. Confirm the proposed handoff before drafting unless the user’s direct instruction already settles the flow’s targets and covered effects or expressly replaces this process under [Instruction Authority](../SKILL.md#instruction-authority).

### Resolve Repository Isolation

For Git repository work, determine whether the receiving task will use the current checkout or an isolated worktree before presenting the flow. Resolve the choice from direct user instructions, applicable project policy, and established task context. The task itself may settle the decision.

Use an isolated worktree only for an explicit user request, another active agent with overlapping write scope, required branch, dependency, build, or test isolation, or a broad or high-risk change that materially benefits from independent rollback and has a clear integration plan. Do not isolate merely because the repository is dirty, concurrent activity is possible, or the task changes files. Keep follow-up edits to the same uncommitted task in its existing checkout.

If a material choice remains unresolved, ask the user explicitly, and emit neither the flow nor the relay. Do not ask merely because no worktree was mentioned.

### Present Relay Flow

When confirmation is still needed, present the final flow in its own response. Keep it succinct, but include the receiving action, material target environment, worktree decision when repository work is involved, scope and exclusions, mutation and approval boundaries, required execution steps, validation, and handoff mode.

When flow confirmation is still needed, ask the user to confirm or correct it, and do not include the task relay in that response. Once the flow is authorized, emit the complete relay without recapping the flow.

### Keep Confirmation Narrow

Confirmation authorizes only what the flow states explicitly and what applicable approval gates permit. It does not substitute for the user command required for commit-writing effects under the **Assignment Contract** or authorize unstated dependency choices, publication, remote submissions, scope expansion, or secret access.

Reassess the flow whenever composition or a later revision materially changes the confirmed action, target, worktree decision, scope, approval, execution, validation, or handoff. Ask only about material ambiguity, uncovered effects, or a separate gate, not changes already expressly authorized by the user. Meaning-neutral compression and formatting do not require reconfirmation.

This confirmation gate applies only to live task handoffs. It does not apply to autonomous in-client delegation, evidence-only decision relays, or reusable artifact maintenance.

## Task Relay Composition

Compose the task relay under the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) from the flow established by direct user instruction or confirmation. If drafting exposes a material ambiguity or an effect outside that authorization, return to confirmation instead of choosing silently.

Include only the applicable parts of this sequence:

1. Title and receiving action.
2. Task context, authoritative evidence, and material target information.
3. Scope, exclusions, mutation boundary, approval boundary, and behavior preservation requirements.
4. Required result, mandatory process constraints, validation, and known limitations.
5. Handoff mode, stopping point, and exact final anti-drift guard when applicable.

Carry only approvals whose applicable gate has been satisfied. Treat the confirmed flow as a record of those approvals rather than a substitute for their authorization source.

Omit the receiving location by default. Include a repository, checkout, worktree, directory, host, or other execution location only when it is needed to find the inputs, distinguish possible targets, preserve isolation, or satisfy a submission or integration boundary. Material target paths may still be required.

Default to a one-way handoff in which the receiving conversation owns completion, with no return relay required. Require an evidence-only [decision relay](decision-relays.md) in return only when the originating conversation remains responsible for synthesis, integration, validation, comparison, or follow-up.

Record only the confirmed isolation requirement. The receiving environment’s repository policy governs worktree creation, operation, and cleanup.

## Delivery

Once the handoff is authorized, apply [Delivery](assignment-prompts.md#delivery). Unless delivery returns to a calling workflow, precede each fenced prompt with `# Relay Prompt` or a descriptive numbered `# Relay Prompt …` heading. Follow it with the next relay heading or a short statement that the prompt is ready to relay.
