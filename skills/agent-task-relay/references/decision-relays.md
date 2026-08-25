# Decision Relays

A decision relay carries established results, evidence, material decisions when any exist, and limitations into another conversation. It is always evidence-only and non-mutating. Its receiving action may inspect, compare, synthesize, or report on the evidence, but it cannot authorize edits or other mutation. If the evidence motivates implementation, assign that work through a separate [task relay](task-relays.md) with its own confirmed flow.

Apply the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) when composing a decision relay and its [Delivery](../SKILL.md#delivery) rules when returning one.

## Available Evidence

Use only context and artifacts already available from the completed task. Do not continue the task, call tools, reopen files, rerun validation, browse, delegate, or draft a receiving-task patch. Record any material gap instead of gathering or reconstructing what is unavailable.

## Handoff Structure

Use only the material parts of this sequence. Combine overlapping items, and omit empty sections.

1. Title, explicit evidence-only receiving action, and stopping point.
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

When a decision relay supports improvement of an existing skill, add only material workflow observations and candidate reusable guidance to the [Handoff Structure](#handoff-structure). Ask the receiving agent to compare that evidence with the current skill and report confirmed coverage, concrete gaps, reusable guidance, and context-specific decisions separately.

## Domain Profiles

A domain profile is a standalone maintainer asset measured against this skill rather than a runtime extension of it. It must restate every rule it needs because an ordinary invocation of the profile may not load this skill.

A profile may specialize context fields, representative evidence, validation levels, workflow observations, and candidate-guidance destinations. It must preserve the entrypoint’s [Relay Contract](../SKILL.md#relay-contract), [Delivery](../SKILL.md#delivery), source-closed [Available Evidence](#available-evidence) workflow, and evidence-only non-mutation rule.

A standalone decision-capture asset must implement the applicable delivery and full-revision behavior in its own output contract. Its output is always source-closed, evidence-only, and non-mutating. It cannot depend on the receiving agent loading this skill.
