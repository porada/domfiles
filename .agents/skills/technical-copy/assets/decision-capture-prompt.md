Create a decision-making relay for improving the `technical-copy` skill from the copy work completed in this conversation.

Do not edit the skill or any project files, rerun the task, or turn the relay into a skill patch. Analyze the completed workflow and return only a self-contained Markdown relay for another agent.

Use the conversation and task artifacts already inspected. Do not start a broad audit or gather unrelated examples. If material context is unavailable, identify the gap instead of reconstructing a plausible explanation.

## Output contract

- Output `# Relay Prompt`, followed immediately by one four-backtick `markdown` code block.
- Put the complete, self-contained relay inside the block and begin it with `# Technical Copy Skill Improvement Relay`.
- Add no other content. Use triple-backtick fences inside the outer block when exact code or Markdown must be preserved.

## Evidence rules

- Distinguish direct user instructions, corrections, selections, and explicit acceptances from agent proposals, project policy, contextual requirements, and inferences.
- Do not treat an agent’s draft, the user’s silence, or wording’s mere presence in a file as an established user decision.
- Preserve exact wording, punctuation, markup, and line breaks when they materially affected the decision.
- Include only iterations that reveal a meaningful accuracy, clarity, structure, tone, consistency, permission, or workflow decision. Do not reproduce the full transcript.
- When technical evidence changed the copy, explain how it changed the factual or causal model rather than presenting the result as a stylistic preference.
- Label one-off choices, unresolved questions, and tentative generalizations clearly. Do not turn one contextual decision into a universal rule.

## Output structure

**Before the code block**

# Relay Prompt

**Inside the code block**

# Technical Copy Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the portable `technical-copy` skill. Compare it with the current skill, distinguish reusable guidance from context-specific decisions, and report concrete gaps or confirmed coverage. Do not edit the skill unless the user explicitly requests it.

## Task context

Identify:

- The project, component, and copy location.
- The surface, intended reader, and immediate purpose.
- The requested scope and any permission or submission boundary.
- The applicable template, project policy, local convention, or technical evidence.

## Final result

Present the final accepted copy when reasonably sized. For a long document or broad consistency pass, identify the changed copy units and quote only the passages needed to understand the decisions.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### <Decision label>

**Before**

Quote the relevant initial or rejected wording when available.

**After**

Quote the selected wording or describe the selected structural result.

**Why**

Explain the factual, editorial, contextual, or workflow reason. Include intermediate wording only when it reveals an additional reusable distinction.

**Decision basis**

Use one or more precise labels: `Direct instruction`, `Correction`, `User selection`, `Explicit acceptance`, `Project policy`, `Context-specific requirement`, `Agent inference`, or `Unresolved`.

## Workflow observations

Record only concrete observations about:

- What the skill handled well without prompting.
- Where the user had to correct or redirect it.
- What evidence or context was necessary to reach accurate copy.
- Any avoidable friction, overreach, repetition, or missing permission boundary.

## Candidate reusable guidance

Generalize the material decisions at the principle level without copying task-specific identifiers into universal policy. For each candidate, state the applicable surface or condition and whether it belongs on the standard path or a narrower branch.

Separate strong candidates supported by direct user decisions from tentative ideas that require another example or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Keep the relay concise enough to scan, but complete enough that the receiving agent does not need the original conversation to understand each reported decision.
