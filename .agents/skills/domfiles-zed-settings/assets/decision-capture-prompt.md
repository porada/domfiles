Create a decision relay for improving the `domfiles-zed-settings` skill from the Zed settings work completed in this conversation.

The user invokes this prompt only after the current Zed settings task is complete. Treat the task as complete for capture purposes, but describe a particular result or decision as explicitly accepted only when direct prior evidence establishes that acceptance.

Your sole action in this turn is to produce the relay. Follow the composition directives in the template below to populate every applicable section from the established context, combining overlapping content and omitting empty optional sections. The generated `Receiving action` line is addressed to a different agent after the relay is forwarded. Do not follow that receiving action now.

A decision-capture prompt operates only on context and artifacts already available in the completed task. It does not continue the task, call tools, reopen files, rerun validation, browse, delegate, or draft a receiving-task patch. When material context is unavailable, record the gap instead of gathering or reconstructing it.

## Output contract

- Output the complete, self-contained relay as the entire response and begin it with `# Zed Settings Skill Improvement Relay`.
- Do not wrap the response in an outer code block or add a `# Relay Prompt` heading before it or a readiness message after it.
- Use ordinary fenced code blocks inside the relay when exact code, JSON, regexes, or command inputs must be preserved.
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

- Preserve direct user instructions, corrections, selections, explicit acceptances, settled evidence, and permission boundaries. Do not promote agent proposals, user silence, or a value’s mere presence in a file into accepted decisions.
- Treat approval as granted only by the user’s explicit response. An agent or subagent cannot approve on the user’s behalf.
- Preserve a user-supplied classification as settled evidence when the original task established that boundary. Do not retrospectively challenge or re-research it.
- Preserve exact normalized inputs, token ordering, flags, assignments, wrappers, case distinctions, URLs, paths, and precedence outcomes when they materially affected a permission decision.
- Never reproduce literal credentials, tokens, private values, or secret-bearing URLs. Describe the redacted security boundary and resulting classification instead.
- Do not dump complete settings objects, permission arrays, command inventories, test corpora, or long regex bodies. Quote only a small changed object or pattern when its exact form explains a decision. For a long pattern, identify its owner, permission bucket, case setting, approximate size, and material grammar or structural change.
- Include only iterations that reveal a meaningful scope, ownership, behavior, safety, generalization, regex, ordering, validation, or workflow decision. Group repetitive examples without losing the distinctions that determined their classifications.
- Label one-off choices, unresolved questions, stricter implementation limits, and tentative generalizations clearly. Do not turn one command family or contextual decision into universal policy.

## Output structure

**Complete response structure—populate every applicable section and output it from the heading onward**

# Zed Settings Skill Improvement Relay

**Receiving action:** Review this relay as evidence for improving the repository-scoped `domfiles-zed-settings` skill. Compare it with the current skill, distinguish reusable guidance from task-specific decisions, and report concrete gaps or confirmed coverage. Treat any identified user-approved classifications as evidence rather than reopening their behavioral research. Do not edit the skill, settings, scripts, or documentation unless the user explicitly requests it.

## Task context

Identify:

- The affected settings file or subtree only when needed to identify the owning surface.
- Whether the task was a change, audit, review, or diagnosis and which general or permission branches applied, such as terminal, Git, fetch and network, agent repository, or permission evaluation.
- The requested behavior and any user-supplied evidence declared settled.
- The smallest settings object, command-owner group, pattern family, domain or URL scope, or other unit that owned the result.
- Relevant Zed, command, toolchain, schema, or source versions when they materially constrained the work.
- The repository, checkout, worktree, or receiving location only when it materially disambiguates the work or affects isolation, submission, or integration.

Mention concurrent or uncommitted work only when it materially changed the implementation or validation boundary.

## Scope and boundaries

State the material safety, approval, mutation, submission, integration, evidence, and stopping boundaries. Require unrelated observations to remain outside the relay.

## Final result

State what the task ultimately changed or established and identify the affected files. Include a concise final excerpt only when it helps explain the decisions.

For permission work, summarize the final behavior boundary:

- Forms that became automatic.
- Forms that remained confirmable, including deliberately stricter cases.
- Forms that were denied.
- Relevant default or unmatched behavior and any precedence override.

For a non-permission settings task, replace those categories with the pertinent before-and-after values and runtime behavior.

## Representative inputs and ownership

For permission work, provide a compact set of exact normalized examples covering the material distinctions. Include applicable matching cases, hazardous forms, near misses, wrapper or assignment variants, case distinctions, and option or operand permutations. State the resulting decision for each example or grouped family.

Identify the owning pattern or object by command owner or scope, permission bucket, and case setting. Explain any consolidation, decomposition, finite-inventory, ordering, or URL-scope decision.

Omit this section when the task did not involve permission matching.

## Material decisions

Create one short subsection for each meaningful decision. Use this shape:

### \<decision-label\>

**Before**

Quote or describe the prior behavior, initial translation, rejected structure, or missing workflow step.

**After**

Quote or describe the selected behavior, structure, pattern boundary, or workflow result.

**Why**

Explain the behavioral, security, ownership, structural, efficiency, compatibility, or contextual reason. Include intermediate attempts only when they reveal an additional reusable distinction.

**Decision basis**

Use one or more labels defined under Evidence rules.

## Evidence and validation

Record only evidence that materially established the result:

- Which behavior classifications came from the user and which were derived from project policy, repository evidence, documentation evidence, observed behavior, or agent inference.
- Whether the inventory-first and candidate-first workflows were used and which repository-owned scripts or suite modes materially aided the work.
- Whether regex compilation, pattern expectations, configured-pattern precedence decisions, complete effective permission behavior, and any actual post-change Zed permission behavior were verified. Keep these evidence levels distinct.
- Any unavailable executable, untested runtime condition, regex-engine limitation, or other validation boundary.

Report counts or outcomes only when they materially establish coverage.

## Workflow observations

Record only concrete observations about:

- What the skill and its scripts handled efficiently without prompting.
- Where the user had to correct scope, ownership, classification, generalization, pattern structure, or validation.
- Any avoidable research, permission prompt, invalid live-regex attempt, repeated extraction, manual harness, excessive output, or context pressure.
- Whether an existing helper should gain a bounded capability, or whether a script change made the completed workflow materially more efficient.

## Candidate reusable guidance

Generalize the material decisions at the policy or workflow level. For each candidate, identify the applicable settings or permission branch and whether it belongs in the skill entrypoint, a conditional reference, or a skill-owned script contract.

Separate strong candidates supported by direct user decisions or repeated evidence from tentative ideas that require another task or evaluation.

## Context-specific and unresolved items

List decisions that should not be generalized, stricter boundaries caused by current matcher limits, open questions, contradictory signals, and areas where more examples are needed. Omit this section when none remain.

Use a succinct, scan-friendly format, but preserve all detail needed to understand each reported decision without the original conversation.
