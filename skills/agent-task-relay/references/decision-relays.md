# Decision Relays

A decision relay carries established results, evidence, material decisions when any exist, and limitations into another conversation. Its default receiving action is evidence-only and non-mutating. The evidence does not authorize edits or other effects. Any continuation needs authority from the receiving task or explicit user direction under [Instruction Authority](../SKILL.md#instruction-authority).

By default, put fresh assignments, including comparison against new or current source material, investigation, and review, in a separate [task relay](task-relays.md). If the user expressly requests a combined handoff, distinguish the assignment and its authorization from the evidence. Apply [Task Relay Confirmation](task-relays.md#task-relay-confirmation), including its assignment contract, to every assignment, whether separate or combined.

Apply the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) when composing a decision relay and [Delivery](#delivery) when returning one.

## Available Evidence

Use only context and artifacts already available from the completed task unless the user expressly requests additional evidence gathering. Otherwise, do not browse, call tools, continue the task, delegate, draft a receiving task patch, reopen files, or rerun validation. Record any material gap instead of gathering or reconstructing what is unavailable. A request for more evidence does not authorize a patch or other mutation.

## Handoff Structure

Use only the material parts of this sequence. Combine overlapping items, and omit empty sections.

1. Title, explicit receiving action, and stopping point.
2. Task context, final result, and acceptance status.
3. Representative evidence and validation.
4. Material decisions using `Before`, `After`, `Why`, and `Decision basis` when those fields clarify the result.
5. Known limitations, unavailable evidence, and context-specific or unresolved items.

Use the labels in [Decision Basis](#decision-basis) when a material decision or fact needs its basis made explicit.

## Decision Basis

Use one or more of these labels to state the basis of a material decision or fact. Choose the most specific applicable label for each basis.

| Label | Meaning |
| --- | --- |
| **Agent inference** | A conclusion the agent drew from available evidence rather than a choice the user made directly. |
| **Context-specific requirement** | A constraint established by the task’s surface, environment, template, or local situation. |
| **Correction** | A direct user correction to an earlier claim, structure, classification, or wording. |
| **Direct instruction** | An explicit user command governing scope, behavior, process, or wording. |
| **Documentation evidence** | Local help, manuals, official documentation, or another authoritative source consulted as documentation. |
| **Explicit acceptance** | Direct evidence that the user accepted the identified result or decision. |
| **Implementation limitation** | A boundary imposed by the implementation, matcher, format, tool, or environment. |
| **Observed behavior** | A command result, runtime outcome, rendered result, or other behavior that was actually observed. |
| **Project policy** | An applicable repository or project instruction, rationale, or established workflow. |
| **Repository evidence** | Current source, configuration, tests, history, or other inspected repository state. |
| **Settled user evidence** | A user-supplied fact or classification declared authoritative for the task. |
| **Unresolved** | A material decision or fact the available evidence did not resolve. |
| **User selection** | The user chose one proposed alternative without necessarily accepting adjacent details. |

Reserve **Observed behavior** for results that were actually observed. Do not collapse a known basis into a less specific label.

## Skill Improvement

When a decision relay supports improvement of an existing skill, add only material workflow observations and candidate reusable guidance to the [Handoff Structure](#handoff-structure). Include concrete gaps, confirmed coverage, context-specific decisions, and reusable guidance separately only when established by the task’s permitted evidence gathering.

## Delivery

When an entire response is a decision relay, evidence handoff, status return, completed work report, or other response intended for verbatim relay, make the relay the whole response. Do not wrap it in an outer code block, add a relay heading, or append a readiness message.

## Domain Profiles

A domain profile is a standalone maintainer asset measured against this skill rather than a runtime extension of it. It must restate every rule it needs because an ordinary invocation of the profile may not load this skill. A decision capture prompt asks the current agent to turn available conversation context into a decision relay without continuing the underlying task.

A profile may specialize context fields, representative evidence, validation levels, workflow observations, and candidate-guidance destinations. Carry the entrypoint’s [Instruction Authority](../SKILL.md#instruction-authority), [Relay Contract](../SKILL.md#relay-contract), and [Revisions](../SKILL.md#revisions), together with the [Available Evidence](#available-evidence) defaults, [Delivery](#delivery), and the distinction between evidence and user-authorized assignments.

A standalone decision capture prompt must implement the applicable delivery and full-revision defaults in its own output contract. Keep its output source-closed, evidence-only, and non-mutating by default, with explicit user changes governed by **Instruction Authority**. It cannot depend on the receiving agent loading this skill.
