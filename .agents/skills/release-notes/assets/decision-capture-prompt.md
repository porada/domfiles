Create a decision-making relay for improving the `release-notes` skill from the release-note work completed in this conversation.

Do not edit the skill or any project files, rerun the task, or turn the relay into a skill patch. Analyze the completed workflow and return only a self-contained Markdown relay for another agent.

Use the conversation and task artifacts already inspected. Do not start a broad audit or gather unrelated examples. If material context is unavailable, identify the gap instead of reconstructing a plausible explanation.

## Output contract

- Output `# Relay Prompt`, followed immediately by one four-backtick `markdown` code block.
- Put the complete, self-contained relay inside the block and begin it with `# Release Notes Skill Improvement Relay`.
- Add no other content. Use triple-backtick fences inside the outer block when exact code or Markdown must be preserved.

## Evidence rules

- Distinguish direct user instructions, corrections, selections, and explicit acceptances from agent proposals, project policy, contextual requirements, and inferences.
- Do not treat an agent’s draft, the user’s silence, or wording’s mere presence in a file as an established user decision.
- Preserve exact wording, bullet markers, punctuation, links, heading hierarchy, and line breaks when they materially affected the decision.
- Separate facts retained in the evidence inventory from consumer outcomes selected for publication. Include an omitted or reclassified fact only when that decision materially shaped the result.
- Include only iterations that reveal a meaningful scope, materiality, evidence, structure, ordering, wording, approval, or workflow decision. When evidence corrected a claim or causal framing, explain the correction rather than presenting it as a stylistic preference.
- Label one-off choices, unresolved questions, and tentative generalizations clearly. Do not turn one contextual decision into a universal rule.

## Output structure

**Before the code block**

# Relay Prompt

**Inside the code block**

# Release Notes Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the portable `release-notes` skill. Compare it with the current skill, distinguish reusable guidance from context-specific decisions, and report concrete gaps or confirmed coverage. Do not edit the skill unless the user explicitly requests it.

## Task context

Identify:

- The repository, package or synchronized package set, and note location.
- The release target, version or unreleased state, and resolved change scope.
- Whether the task drafted, inferred, edited, or reviewed notes and any approval boundary.
- The note surface, intended consumer, applicable structure, and available evidence or evidence limitation.

## Final result

Present the latest release-note result when reasonably sized. Describe it as accepted only when direct user evidence establishes acceptance. For a long aggregate file or broad consistency pass, identify the changed release or package sections and quote only the passages needed to understand the decisions.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### <Decision label>

**Before**

Quote the relevant initial or rejected wording when available.

**After**

Quote the selected wording or describe the selected structural result.

**Why**

Explain the evidentiary, materiality, structural, editorial, contextual, or workflow reason. Include intermediate wording only when it reveals an additional reusable distinction.

**Decision basis**

Use one or more precise labels: `Direct instruction`, `Correction`, `User selection`, `Explicit acceptance`, `Verified evidence`, `Project policy`, `Context-specific requirement`, `Agent inference`, or `Unresolved`.

## Workflow observations

Record only concrete observations about:

- What the skill handled well without prompting.
- Where the user had to correct its scope, materiality, structure, ordering, or wording.
- What evidence or release context was necessary to reach accurate notes.
- Any avoidable friction, over-selection, over-compression, repetition, or missed approval boundary.

## Candidate reusable guidance

Generalize the material decisions at the principle level without copying package-specific identifiers into universal policy. For each candidate, state the applicable release shape or change class and whether it belongs in the core workflow, prose rules, or a narrower reference.

Separate strong candidates supported by direct user decisions from tentative ideas that require another example or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Keep the relay concise enough to scan, but complete enough that the receiving agent does not need the original conversation to understand each reported decision.
