# Prompt relays

Use this reference when creating, revising, reviewing, or maintaining a prompt intended to be copied from one conversation into another. Follow the global [prompt relay delivery defaults](../../../../.config/zed/AGENTS.md#prompt-relays). This reference owns relay terminology, composition, evidence provenance, and domain-profile maintenance.

## Use canonical terms

- A **capture prompt** asks the current agent to turn established conversation context into a relay without continuing the underlying task.
- A **decision relay** carries the result, evidence, decisions, and workflow observations from completed work into a later review or improvement task.
- A **receiving action** tells the next agent exactly what to do with the relay and whether mutation is authorized.
- A **relay** is the complete prompt copied into another conversation.
- A **task relay** assigns future work to another agent.

## Apply the shared relay contract

- Lead the relayed content with a descriptive `# …` heading and an explicit receiving action.
- Make the relay understandable without the source conversation. Include the smallest complete context, authority, evidence, scope, result or required outcome, validation boundary, and handoff contract.
- Preserve direct user instructions, corrections, selections, explicit acceptances, settled evidence, and permission boundaries. Do not promote agent proposals, user silence, or a value’s mere presence in a file into accepted decisions.
- Distinguish source evidence, observed behavior, and agent inference. State unavailable evidence and implementation limitations directly rather than reconstructing them.
- Preserve exact syntax, wording, token ordering, punctuation, paths, URLs, or normalized inputs only when those details materially determine the task or decision.
- Never reproduce literal credentials, tokens, private values, secret-bearing URLs, complete inventories, long generated artifacts, or transcript-like iteration history. Use bounded representative evidence.
- Keep instructions and reported outcomes in positive, direct language. Do not use negated negative-state terms to describe a successful result.
- Keep the receiving action non-mutating unless the user explicitly authorized changes in the receiving task.

## Apply relay delivery defaults

Make every standalone relay asset implement the global framing and complete-revision defaults in its own output contract. A capture prompt cannot depend on the receiving agent loading this reference, and its generated relay must remain visually bounded without additional user instruction.

When one requested change affects a coordinated prompt set, return every affected prompt in full and omit unrelated unchanged prompts. Preserve established decisions and untouched boundaries in each complete replacement.

## Compose a task relay

Include the applicable parts of this sequence:

1. Title and receiving action.
2. Task context and authoritative evidence.
3. Scope, exclusions, mutation boundary, and approval boundary.
4. Required result and behavior-preservation requirements.
5. Process constraints and applicable project workflows.
6. Validation requirements and known limitations.
7. Handoff contents and the receiving agent’s stopping point.

Omit irrelevant sections instead of filling them with generic boilerplate. Do not invent paths, revisions, branches, versions, commands, or validation requirements.

Apply the global prompt-relay default for worktree instructions. The receiving agent remains responsible for applying the current [worktree policy](../../../../.config/zed/AGENTS.md#git-worktrees) when execution begins.

## Compose a decision relay

A decision-capture prompt operates only on context and artifacts already available in the completed task. It does not continue the task, call tools, reopen files, rerun validation, browse, delegate, or draft a receiving-task patch. When material context is unavailable, record the gap instead of gathering or reconstructing it.

Use this common sequence, adding a bounded domain section only when it captures a material distinction:

1. Task context.
2. Final result and acceptance status.
3. Representative inputs, output, or evidence when the domain requires them.
4. Material decisions using `Before`, `After`, `Why`, and `Decision basis`.
5. Evidence and validation.
6. Workflow observations.
7. Candidate reusable guidance.
8. Context-specific and unresolved items when any remain.

The receiving action reviews the relay as evidence, compares it with the current canonical guidance, and reports confirmed coverage or concrete gaps. It does not authorize edits unless the user explicitly requests them in the receiving conversation.

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

Keep every domain asset standalone so it remains usable when its owning portable skill is installed independently. Treat this reference as the maintainer standard rather than a runtime include.

A domain profile may specialize context fields, representative evidence, validation levels, and candidate-guidance destinations. It must preserve the shared framing, complete-revision, provenance, acceptance, mutation-boundary, and source-closed capture requirements above.
