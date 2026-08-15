# Prompt relays

Use this reference when creating, revising, reviewing, auditing, or maintaining a prompt intended to be copied from one conversation into another. It owns relay terminology, composition, delivery, evidence provenance, and domain-profile maintenance.

## Use canonical terms

- A **capture prompt** asks the current agent to turn established conversation context into a relay without continuing the underlying task.
- A **decision relay** carries the result, evidence, decisions, and workflow observations from completed work into a later review or improvement task.
- A **receiving action** tells the next agent exactly what to do with the relay and whether mutation is authorized.
- A **relay** is the complete prompt copied into another conversation.
- A **task relay** assigns future work to another agent.

## Apply the shared relay contract

- Lead the relayed content with a descriptive `# …` heading and an explicit receiving action.
- Write every relay in a succinct, scan-friendly format. Include each material fact once, prefer compact bullets over narrative, and omit chronology, routine validation, incidental identifiers, repeated rationale, and context the receiving action does not need. Preserve all detail required for accuracy, evidence provenance, safety, approval, mutation, submission, integration, and stopping boundaries.
- Preserve direct user instructions, corrections, selections, explicit acceptances, settled evidence, and permission boundaries. Do not promote agent proposals, user silence, or a value’s mere presence in a file into accepted decisions.
- Distinguish source evidence, observed behavior, and agent inference. State unavailable evidence and implementation limitations directly rather than reconstructing them.
- Preserve exact syntax, wording, token ordering, punctuation, paths, URLs, or normalized inputs only when those details materially determine the task or decision.
- Never reproduce literal credentials, tokens, private values, secret-bearing URLs, unnecessary or unbounded inventories, long generated artifacts, or transcript-like iteration history. Use bounded representative evidence. Preserve a bounded complete inventory when it materially defines the relay’s owned scope, preservation boundary, or required result.
- Apply the positive-state outcome rule in the global “Communication” policy to relay instructions and outcomes.
- Keep the receiving action non-mutating unless the user explicitly authorized changes in the receiving task.

## Apply relay delivery defaults

- **Outgoing prompts:** Put each complete prompt intended for another conversation in its own three-backtick `markdown` block, raising the fence to four backticks only when the prompt itself contains a three-backtick code block. Precede it with `# Relay Prompt` or a descriptive numbered `# Relay Prompt …` heading. Follow it with the next relay heading or a short statement that the prompt is ready for relay.
- **Verbatim returns:** When an entire response is a decision relay, evidence handoff, status return, completed-work report, or other response intended for verbatim relay, make the relay the whole response. Do not wrap it in an outer code block or add a relay heading or readiness message around it.
- **Prompt revisions:** When asked to change a prompt, return every affected prompt in full with the change applied. Do not provide a patch, fragment, or splice instructions. When one requested change affects a coordinated prompt set, apply this rule across that set, omit unrelated unchanged prompts, and preserve established decisions and untouched boundaries in each replacement.
- **Worktree instructions:** Do not add them to a relayed prompt unless the user explicitly requests them or an applicable policy already requires them.

Make every standalone relay asset implement the applicable delivery and complete-revision defaults above in its own output contract. A task-relay asset must frame each outgoing assignment prompt as a copyable code block. A decision-capture asset whose entire response is the returned evidence relay must emit that relay directly as the complete response without an outer relay frame. A capture prompt cannot depend on the receiving agent loading this reference.

## Compose a task relay

- Omit the receiving location by default. Include a repository path, checkout, worktree, directory, host, or other execution location only when selecting that location is necessary to find the task inputs, distinguish among possible targets, preserve isolation, or satisfy an established submission or integration boundary. Do not add an absolute repository path merely for orientation. Material target paths may still be required even when the receiving location is not.
- Before emitting a dependency-premised mutating relay, apply the approval gate in the global “Dependencies” policy.
- Because a task relay assigns work, end its complete prompt with the exact standalone line `**Do not drift.**`. Define the smallest complete owned scope and supporting work first, name material exclusions, and require unrelated findings to remain untouched. Preserve every inherited scope, mutation, approval, submission, integration, access, and security boundary. Explicitly prohibit transferring access or circumventing a boundary, and tell the receiving agent to stop and ask the user directly before crossing one. Place every required result, process, validation, and handoff instruction before the guard. Do not use the guard to exclude supporting edits or validation already required for the stated result.
- Use the guard only in a task relay whose primary purpose is to assign future work. Never include it in decision relays, evidence handoffs, status returns, completed-work reports, or other relays whose primary purpose is to transfer established data. Receiving-action guidance for later evidence consumption does not turn that transfer into a task relay.

Include the applicable parts of this sequence:

1. Title and receiving action.
2. Task context and authoritative evidence.
3. Scope, exclusions, mutation boundary, and approval boundary.
4. Required result and behavior-preservation requirements.
5. Process constraints and applicable project workflows.
6. Validation requirements and known limitations.
7. Handoff contents and the receiving agent’s stopping point.
8. Exact final anti-drift guard.

Execution remains governed by the global “Git worktrees” policy.

## Compose a decision relay

A decision-capture prompt operates only on context and artifacts already available in the completed task. It does not continue the task, call tools, reopen files, rerun validation, browse, delegate, or draft a receiving-task patch. When material context is unavailable, record the gap instead of gathering or reconstructing it.

Use only the material parts of this sequence, adding a bounded domain section only when it captures a material distinction. Combine overlapping items and omit empty sections:

1. Task context.
2. Final result and acceptance status.
3. Representative inputs, output, or evidence when the domain requires them.
4. Material decisions using `Before`, `After`, `Why`, and `Decision basis`.
5. Evidence and validation.
6. Workflow observations.
7. Candidate reusable guidance.
8. Context-specific and unresolved items when any remain.

The receiving action reviews the relay as evidence, compares it with the current canonical guidance, and reports confirmed coverage or concrete gaps. If the evidence later motivates a follow-up assignment, compose that assignment as a separate task relay.

## Use consistent decision evidence

Use one or more of these labels when a material decision needs provenance:

| Label | Meaning |
| --- | --- |
| `Agent inference` | A conclusion drawn by the agent from available evidence rather than selected directly by the user. |
| `Context-specific requirement` | A constraint established by the task’s surface, environment, template, or local situation. |
| `Correction` | A direct user correction to an earlier claim, structure, classification, or wording. |
| `Direct instruction` | An explicit user command that determines scope, behavior, process, or wording. |
| `Documentation evidence` | Local help, manuals, official documentation, or authoritative source consulted as documentation. |
| `Explicit acceptance` | Direct evidence that the user accepted the identified result or decision. |
| `Implementation limitation` | A boundary imposed by the available implementation, matcher, format, tool, or environment. |
| `Observed behavior` | A command result, runtime outcome, rendered result, or other behavior actually observed. |
| `Project policy` | An applicable repository or project instruction, rationale, or established workflow. |
| `Repository evidence` | Current source, configuration, tests, history, or other inspected repository state. |
| `Settled user evidence` | A user-supplied fact or classification explicitly declared authoritative for the task. |
| `Unresolved` | A material decision or fact that the available evidence did not resolve. |
| `User selection` | The user chose one proposed alternative without necessarily accepting every adjacent detail. |

Use the most specific applicable label. Reserve `Observed behavior` for results that were actually observed, and do not collapse a known evidence source into a less specific label.

## Maintain domain relay profiles

Treat this reference as the maintainer standard rather than a runtime include.

A domain profile may specialize context fields, representative evidence, validation levels, and candidate-guidance destinations. It must preserve the [shared relay contract](#apply-the-shared-relay-contract), [delivery defaults](#apply-relay-delivery-defaults), and [source-closed decision-capture workflow](#compose-a-decision-relay).
