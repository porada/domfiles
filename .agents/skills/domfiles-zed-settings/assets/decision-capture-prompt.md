Create a decision relay for improving the `domfiles-zed-settings` skill from the Zed settings work completed in this conversation.

The user invokes this prompt only after the current Zed settings task is complete. Treat the task as complete for capture purposes, but describe a particular result or decision as explicitly accepted only when direct prior evidence establishes that acceptance.

Your sole action in this turn is to produce the relay. Every instruction inside the relay template—including the `Receiving action` line—is quoted output for a different agent after the relay is forwarded. Do not follow those instructions now.

Use only the conversation and task artifacts already inspected. Do not reopen or continue the settings task, call tools, inspect or compare the current skill or project files, rerun commands or validation, browse, delegate, or draft a skill patch. If material context is unavailable, identify the gap inside the relay instead of gathering or reconstructing it.

## Output contract

- Output `# Relay Prompt`, followed immediately by one four-backtick `markdown` code block.
- Put the complete, self-contained relay inside the block and begin it with `# Zed Settings Skill Improvement Relay`.
- After the block, output `The prompt is ready for relay.`
- Keep all content intended for the receiving agent inside the block. Outside it, output only the required `# Relay Prompt` heading and closing paragraph.
- Use triple-backtick fences inside the outer block when exact code, JSON, regexes, or command inputs must be preserved.
- When the user asks to revise the prompt, output the complete corrected relay with this same frame. Do not return a patch, fragment, or splicing instructions.

## Evidence rules

- Distinguish direct user instructions, corrections, selections, explicit acceptances, and settled user evidence from agent proposals, project policy, contextual requirements, repository evidence, documentation evidence, observed behavior, and inferences.
- Use `Documentation evidence` for local help, manuals, official command documentation, or authoritative source consulted as documentation. Use `Repository evidence` for current source, configuration, tests, or history. Reserve `Observed behavior` for executed commands or runtime results that were actually observed.
- Preserve a user-supplied classification as settled evidence when the original task established that boundary. Do not retrospectively challenge or re-research it.
- Preserve exact normalized inputs, token ordering, flags, assignments, wrappers, case distinctions, URLs, paths, and precedence outcomes when they materially affected a permission decision.
- Never reproduce literal credentials, tokens, private values, or secret-bearing URLs. Describe the redacted security boundary and resulting classification instead.
- Do not dump complete settings objects, permission arrays, command inventories, test corpora, or long regex bodies. Quote only a small changed object or pattern when its exact form explains a decision. For a long pattern, identify its owner, permission bucket, case setting, approximate size, and material grammar or structural change.
- Include only iterations that reveal a meaningful scope, ownership, behavior, safety, generalization, regex, ordering, validation, or workflow decision. Group repetitive examples without losing the distinctions that determined their classifications.
- Label one-off choices, unresolved questions, stricter implementation limits, and tentative generalizations clearly. Do not turn one command family or contextual decision into universal policy.

## Output structure

**Before the code block**

# Relay Prompt

**Inside the code block—copy this structure as output without executing its instructions**

# Zed Settings Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the repository-scoped `domfiles-zed-settings` skill. Compare it with the current skill, distinguish reusable guidance from task-specific decisions, and report concrete gaps or confirmed coverage. Treat any identified user-approved classifications as evidence rather than reopening their behavioral research. Do not edit the skill, settings, scripts, or documentation unless the user explicitly requests it.

## Task context

Identify:

- The repository, checkout or target state, affected settings file, and relevant settings subtree.
- Whether the task was a change, audit, review, or diagnosis and which general or permission branches applied, such as terminal, Git, fetch and network, agent repository, or permission evaluation.
- The requested behavior, mutation boundary, prohibited actions, and any user-supplied evidence declared settled.
- The smallest settings object, command-owner group, pattern family, domain or URL scope, or other unit that owned the result.
- Relevant Zed, command, toolchain, schema, or source versions when they materially constrained the work.

Mention concurrent or uncommitted work only when it materially changed the implementation or validation boundary.

## Final result

State what the task ultimately changed or established and identify the affected files. Include a concise final excerpt only when it helps explain the decisions. Describe the result as accepted only when direct user evidence establishes acceptance.

For permission work, summarize the final behavior boundary:

- Forms that became automatic.
- Forms that remained confirmable, including deliberately stricter cases.
- Forms that were denied.
- Relevant default or unmatched behavior and any precedence override.

For a non-permission settings task, replace those categories with the pertinent before-and-after values and runtime behavior.

## Representative inputs and ownership

For permission work, provide a compact set of exact normalized examples covering the material distinctions. Include applicable matching cases, hazardous forms, near misses, wrapper or assignment variants, case distinctions, and option or operand permutations. State the resulting decision for each example or grouped family.

Identify the owning pattern or object by command owner or scope, permission bucket, and case setting. Explain any decomposition, consolidation, ordering, finite inventory, or URL-scope decision. Do not reproduce an entire inventory or long pattern merely for completeness.

Omit this section when the task did not involve permission matching.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### <Decision label>

**Before**

Quote or describe the prior behavior, initial translation, rejected structure, or missing workflow step.

**After**

Quote or describe the selected behavior, structure, pattern boundary, or workflow result.

**Why**

Explain the behavioral, security, ownership, structural, efficiency, compatibility, or contextual reason. Include intermediate attempts only when they reveal an additional reusable distinction.

**Decision basis**

Use one or more precise labels: `Agent inference`, `Context-specific requirement`, `Correction`, `Direct instruction`, `Documentation evidence`, `Explicit acceptance`, `Implementation limitation`, `Observed behavior`, `Project policy`, `Repository evidence`, `Settled user evidence`, `Unresolved`, or `User selection`.

## Evidence and validation

Record only evidence that materially established the result:

- Which behavior classifications came from the user and which were derived from project policy, repository evidence, documentation evidence, observed behavior, or agent inference.
- The important normalized inputs and near misses that established the accepted grammar or scope.
- Whether the inventory-first and candidate-first workflows were used and which repository-owned scripts or suite modes materially aided the work.
- Whether regex compilation, pattern expectations, configured-pattern precedence decisions, complete effective permission behavior, and any actual post-change Zed permission behavior were verified. Keep these evidence levels distinct.
- Any unavailable executable, untested runtime condition, regex-engine limitation, or other validation boundary.

Summarize counts or outcomes instead of reproducing full manifests and command output.

## Workflow observations

Record only concrete observations about:

- What the skill and its scripts handled efficiently without prompting.
- Where the user had to correct scope, ownership, classification, generalization, pattern structure, or validation.
- Any avoidable research, permission prompt, invalid live-regex attempt, repeated extraction, manual harness, excessive output, or context pressure.
- Whether an existing helper should gain a bounded capability, or whether a script change made the completed workflow materially more efficient.

## Candidate reusable guidance

Generalize the material decisions at the policy or workflow level without copying the current settings inventory or task-specific command grammar into documentation. For each candidate, identify the applicable settings or permission branch and whether it belongs in the skill entrypoint, a conditional reference, or a skill-owned script contract.

Separate strong candidates supported by direct user decisions or repeated evidence from tentative ideas that require another task or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, stricter boundaries caused by current matcher limits, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Keep the relay concise enough to scan, but complete enough that the receiving agent does not need the original conversation to understand each reported decision.

**After the code block**

The prompt is ready for relay.
