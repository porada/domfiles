Create a decision relay for improving the `technical-copy` skill from the copy work completed in this conversation.

The user invokes this prompt only after the current copy task is complete. Treat the task as complete for capture purposes.

Your sole action in this turn is to produce the relay. Follow the composition directives in the template below to populate every applicable section from the established context, combining overlapping content and omitting empty optional sections. The generated `Receiving action` line is addressed to a different agent after the relay is forwarded. Do not follow that receiving action now.

Use only the conversation and task artifacts already inspected. Do not continue reviewing or improving the result, call tools, inspect or compare the current skill or project files, rerun the task, browse, delegate, or draft a skill patch. If material context is unavailable, identify the gap inside the relay instead of gathering or reconstructing it.

## Output contract

- Output the complete, self-contained relay as the entire response and begin it with `# Technical Copy Skill Improvement Relay`.
- Do not wrap the response in an outer code block or add a `# Relay Prompt` heading before it or a readiness message after it.
- Use ordinary fenced code blocks inside the relay when exact code or Markdown must be preserved.
- When the user asks to revise the prompt, output the complete corrected relay in this same whole-response form. Do not return a patch, fragment, or splicing instructions.
- This is an evidence relay, not a task assignment. Omit assignment-only focus guards from the generated relay.

## Evidence rules

Use one or more of these `Decision basis` labels when a material decision needs provenance:

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

- Treat approval, acceptance, and other user decisions as established only by direct user evidence. An agent or subagent’s proposal, draft, or endorsement, user silence, and wording’s mere presence in a file establish none of them.
- Preserve exact wording, punctuation, markup, and line breaks when they materially affected the decision.
- Include only iterations that reveal a meaningful accuracy, clarity, structure, tone, consistency, permission, evidence, or workflow decision. Do not reproduce the full transcript.
- Never reproduce literal credentials, tokens, private values, or secret-bearing URLs.

## Output structure

**Complete response structure—populate every applicable section and output it from the heading onward**

# Technical Copy Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the portable `technical-copy` skill. Compare it with the current skill, distinguish reusable guidance from task-specific decisions, and report concrete gaps or confirmed coverage. Do not edit the skill unless the user explicitly requests it.

## Task context

Identify:

- The component, surface, intended reader, immediate purpose, and requested result.
- The applicable template, project policy, local convention, or implementation evidence.
- The project, checkout, receiving location, or copy path only when it materially disambiguates the work.

## Scope and boundaries

State the material permission, approval, mutation, publication, submission, and stopping boundaries. Include unavailable evidence, uninspected results, and other limits on what the relay can establish. Require unrelated observations to remain outside the relay.

## Final result

Present the latest copy result when reasonably sized. For a long document or broad consistency pass, identify the changed copy units and quote only the passages needed to understand the decisions.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### \<decision-label\>

**Before**

Quote the relevant initial or rejected wording when available.

**After**

Quote the selected wording or describe the selected structural result.

**Why**

Explain the factual, editorial, contextual, evidentiary, or workflow reason. Include intermediate wording only when it reveals an additional reusable distinction.

**Decision basis**

Use one or more precise labels: `Agent inference`, `Context-specific requirement`, `Correction`, `Direct instruction`, `Documentation evidence`, `Explicit acceptance`, `Implementation limitation`, `Observed behavior`, `Project policy`, `Repository evidence`, `Settled user evidence`, `Unresolved`, or `User selection`.

## Evidence and validation

Record only cross-cutting evidence and validation not already captured by the per-decision `Decision basis` labels:

- What technical evidence changed or constrained the factual and causal model expressed by the copy.
- Whether the latest result was merely supplied, directly accepted, written to or edited in a file, rendered, submitted, published, or otherwise observed. Keep those evidence levels distinct.

Summarize evidence rather than reproducing full source material or command output.

## Workflow observations

Record only concrete observations about:

- What the skill handled well without prompting.
- Where the user had to correct or redirect it.
- What evidence or context was necessary to reach accurate copy.
- Any avoidable friction, overreach, repetition, or failure to preserve an established constraint.

## Candidate reusable guidance

Generalize the material decisions at the principle level without copying task-specific identifiers into universal policy. For each candidate, state the applicable surface or condition and whether it belongs on the standard path or a narrower branch.

Separate strong candidates supported by direct user decisions or repeated evidence from tentative ideas that require another example or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, implementation limitations, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Use a succinct, scan-friendly format, but preserve all detail needed to understand each reported decision without the original conversation.
