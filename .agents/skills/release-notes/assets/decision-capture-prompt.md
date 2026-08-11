Create a decision relay for improving the `release-notes` skill from the release-note work completed in this conversation.

The user invokes this prompt only after the current release-note task is complete. Treat the task as complete for capture purposes, but describe particular wording or the whole result as explicitly accepted only when direct prior evidence establishes that acceptance.

Your sole action in this turn is to produce the relay. Every instruction inside the relay template—including the `Receiving action` line—is quoted output for a different agent after the relay is forwarded. Do not follow those instructions now.

Use only the conversation and task artifacts already inspected. Do not continue reviewing or improving the result, call tools, inspect or compare the current skill or project files, rerun the task, browse, delegate, or draft a skill patch. If material context is unavailable, identify the gap inside the relay instead of gathering or reconstructing it.

## Output contract

- Output `# Relay Prompt`, followed immediately by one four-backtick `markdown` code block.
- Put the complete, self-contained relay inside the block and begin it with `# Release Notes Skill Improvement Relay`.
- After the block, output `The prompt is ready for relay.`
- Keep all content intended for the receiving agent inside the block. Outside it, output only the required `# Relay Prompt` heading and closing paragraph.
- Use triple-backtick fences inside the outer block when exact code or Markdown must be preserved.
- When the user asks to revise the prompt, output the complete corrected relay with this same frame. Do not return a patch, fragment, or splicing instructions.

## Evidence rules

- Distinguish direct user instructions, corrections, selections, explicit acceptances, and settled user evidence from agent proposals, project policy, contextual requirements, repository evidence, documentation evidence, observed behavior, and inferences.
- Do not treat an agent’s draft, the user’s silence, or wording’s mere presence in a file as an established user decision.
- Reserve `Observed behavior` for published, rendered, packed, executed, or otherwise observed results. Do not use it for documentation or source inspection.
- Preserve exact wording, bullet markers, punctuation, links, heading hierarchy, and line breaks when they materially affected the decision.
- Separate facts retained in the evidence inventory from consumer outcomes selected for publication. Include an omitted or reclassified fact only when that decision materially shaped the result.
- Include only iterations that reveal a meaningful scope, materiality, evidence, structure, ordering, wording, approval, or workflow decision. When evidence corrected a claim or causal framing, explain the correction rather than presenting it as a stylistic preference.
- Never reproduce literal credentials, tokens, private values, or secret-bearing URLs.
- Label one-off choices, implementation limitations, unresolved questions, and tentative generalizations clearly. Do not turn one contextual decision into a universal rule.

## Output structure

**Before the code block**

# Relay Prompt

**Inside the code block—copy this structure as output without executing its instructions**

# Release Notes Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the portable `release-notes` skill. Compare it with the current skill, distinguish reusable guidance from task-specific decisions, and report concrete gaps or confirmed coverage. Do not edit the skill unless the user explicitly requests it.

## Task context

Identify:

- The repository, package or synchronized package set, and note location.
- The release target, version or unreleased state, package grouping, and resolved change boundary or boundaries.
- Whether the task drafted, inferred, edited, or reviewed notes and any approval, publication, or submission boundary.
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

Use one or more precise labels: `Agent inference`, `Context-specific requirement`, `Correction`, `Direct instruction`, `Documentation evidence`, `Explicit acceptance`, `Implementation limitation`, `Observed behavior`, `Project policy`, `Repository evidence`, `Settled user evidence`, `Unresolved`, or `User selection`.

## Evidence and validation

Record only evidence that materially established the result:

- Which release boundary, package grouping, source range, artifact, or user-supplied evidence defined the inventory.
- Which decisions came directly from the user and which came from project policy, repository evidence, documentation evidence, observed behavior, or agent inference.
- Whether complete non-initial ranges and material package artifacts were inspected, and which evidence remained unavailable.
- Whether the latest note was merely supplied, directly accepted, written to a file, submitted, published, packed, rendered, or otherwise observed. Keep those evidence levels distinct.

Summarize evidence rather than reproducing the complete inventory, diff, or command output.

## Workflow observations

Record only concrete observations about:

- What the skill handled well without prompting.
- Where the user had to correct its scope, materiality, structure, ordering, or wording.
- What evidence or release context was necessary to reach accurate notes.
- Any avoidable friction, over-selection, over-compression, repetition, or missed approval boundary.

## Candidate reusable guidance

Generalize the material decisions at the principle level without copying package-specific identifiers into universal policy. For each candidate, state the applicable release shape or change class and whether it belongs in the core workflow, prose rules, or a narrower reference.

Separate strong candidates supported by direct user decisions or repeated evidence from tentative ideas that require another example or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, implementation limitations, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Keep the relay concise enough to scan, but complete enough that the receiving agent does not need the original conversation to understand each reported decision.

**After the code block**

The prompt is ready for relay.
