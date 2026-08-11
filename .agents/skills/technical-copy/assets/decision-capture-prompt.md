Create a decision relay for improving the `technical-copy` skill from the copy work completed in this conversation.

The user invokes this prompt only after the current copy task is complete. Treat the task as complete for capture purposes, but describe a particular result or decision as explicitly accepted only when direct prior evidence establishes that acceptance.

Your sole action in this turn is to produce the relay. Every instruction inside the relay template—including the `Receiving action` line—is quoted output for a different agent after the relay is forwarded. Do not follow those instructions now.

Use only the conversation and task artifacts already inspected. Do not continue reviewing or improving the result, call tools, inspect or compare the current skill or project files, rerun the task, browse, delegate, or draft a skill patch. If material context is unavailable, identify the gap inside the relay instead of gathering or reconstructing it.

## Output contract

- Output `# Relay Prompt`, followed immediately by one four-backtick `markdown` code block.
- Put the complete, self-contained relay inside the block and begin it with `# Technical Copy Skill Improvement Relay`.
- After the block, output `The prompt is ready for relay.`
- Keep all content intended for the receiving agent inside the block. Outside it, output only the required `# Relay Prompt` heading and closing paragraph.
- Use triple-backtick fences inside the outer block when exact code or Markdown must be preserved.
- When the user asks to revise the prompt, output the complete corrected relay with this same frame. Do not return a patch, fragment, or splicing instructions.

## Evidence rules

- Distinguish direct user instructions, corrections, selections, explicit acceptances, and settled user evidence from agent proposals, project policy, contextual requirements, repository evidence, documentation evidence, observed behavior, and inferences.
- Do not treat an agent’s draft, the user’s silence, or wording’s mere presence in a file as an established user decision.
- Reserve `Observed behavior` for rendered, published, executed, or otherwise observed results. Do not use it for documentation or source inspection.
- Preserve exact wording, punctuation, markup, and line breaks when they materially affected the decision.
- Include only iterations that reveal a meaningful accuracy, clarity, structure, tone, consistency, permission, evidence, or workflow decision. Do not reproduce the full transcript.
- When technical evidence changed the copy, explain how it changed the factual or causal model rather than presenting the result as a stylistic preference.
- Never reproduce literal credentials, tokens, private values, or secret-bearing URLs.
- Label one-off choices, implementation limitations, unresolved questions, and tentative generalizations clearly. Do not turn one contextual decision into a universal rule.

## Output structure

**Before the code block**

# Relay Prompt

**Inside the code block—copy this structure as output without executing its instructions**

# Technical Copy Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the portable `technical-copy` skill. Compare it with the current skill, distinguish reusable guidance from task-specific decisions, and report concrete gaps or confirmed coverage. Do not edit the skill unless the user explicitly requests it.

## Task context

Identify:

- The project, component, and copy location.
- The surface, intended reader, and immediate purpose.
- The requested scope and any permission, approval, publication, or submission boundary.
- The applicable template, project policy, local convention, implementation evidence, or evidence limitation.

## Final result

Present the latest copy result when reasonably sized. Describe it as accepted only when direct user evidence establishes acceptance. For a long document or broad consistency pass, identify the changed copy units and quote only the passages needed to understand the decisions.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### <Decision label>

**Before**

Quote the relevant initial or rejected wording when available.

**After**

Quote the selected wording or describe the selected structural result.

**Why**

Explain the factual, editorial, contextual, evidentiary, or workflow reason. Include intermediate wording only when it reveals an additional reusable distinction.

**Decision basis**

Use one or more precise labels: `Agent inference`, `Context-specific requirement`, `Correction`, `Direct instruction`, `Documentation evidence`, `Explicit acceptance`, `Implementation limitation`, `Observed behavior`, `Project policy`, `Repository evidence`, `Settled user evidence`, `Unresolved`, or `User selection`.

## Evidence and validation

Record only evidence that materially established the result:

- Which decisions came directly from the user and which came from project policy, repository evidence, documentation evidence, observed behavior, or agent inference.
- What technical evidence changed or constrained the factual and causal model expressed by the copy.
- Whether the latest result was merely supplied, directly accepted, rendered, submitted, published, or otherwise observed. Keep those evidence levels distinct.
- Any unavailable source, uninspected published result, unresolved terminology, or other validation boundary.

Summarize evidence rather than reproducing full source material or command output.

## Workflow observations

Record only concrete observations about:

- What the skill handled well without prompting.
- Where the user had to correct or redirect it.
- What evidence or context was necessary to reach accurate copy.
- Any avoidable friction, overreach, repetition, or missing permission boundary.

## Candidate reusable guidance

Generalize the material decisions at the principle level without copying task-specific identifiers into universal policy. For each candidate, state the applicable surface or condition and whether it belongs on the standard path or a narrower branch.

Separate strong candidates supported by direct user decisions or repeated evidence from tentative ideas that require another example or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, implementation limitations, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Keep the relay concise enough to scan, but complete enough that the receiving agent does not need the original conversation to understand each reported decision.

**After the code block**

The prompt is ready for relay.
