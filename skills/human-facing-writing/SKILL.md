---
name: human-facing-writing
description: |-
    Draft, edit, review, and refine all human-facing writing, from short-form copy to connected prose. Use it whenever readers must follow ideas, context, reasoning, narrative, or tone.

    Use it across technical surfaces such as package and repository descriptions, README and technical-document content or information architecture, CLI wording, JSDoc and docstrings, explanatory source comments, runtime messages, test titles, labels, and repository collaboration through issues, pull requests, and discussions.

    Do not use for LICENSE text, formatting-only work, or tasks where both wording and information architecture must remain unchanged. Agent responses are not a trigger on their own.
---

# Human-facing writing

Apply this skill without announcing it or narrating its editorial heuristics. Treat narrower project or domain workflows as authoritative over these defaults.

## Choose the workflow

- For drafting, produce the complete authorized writing unit from the user’s intent and available material.
- For editing, preserve the supplied writer’s meaning, material decisions, and recognizable voice while improving the requested dimensions.
- During implementation, apply the relevant writing route to task-owned human-facing wording as part of the feature.
- For a review-only request, remain read-only and report only concrete problems under the applicable route. Do not rewrite the work or present optional preferences as defects.
- For an explicit consistency request, apply the relevant route across the named family without expanding into behaviorally distinct surfaces.

## Apply the prose baseline

- Resolve the authorized writing unit, surface, intended reader, immediate purpose, and desired change in the reader’s understanding, feeling, judgment, or action.
- Make the useful point, action, or experience recognizable soon enough to sustain attention without forcing urgency or simplifying necessary complexity.
- Scale movement to the unit. Let an atomic string complete one clear job, while connected writing develops purposeful progression between sentences, paragraphs, or sections.
- Prefer precise, familiar, concrete wording and purposeful rhythm.
- Where immediate clarity is intended, state material agency, causality, conditions, contrasts, and dependencies directly instead of relying on adjacency, syntax, or cadence to imply them. Keep confidence proportional to the available material, and verify factual claims when the work makes them.
- Preserve the writer’s intended voice and the reader’s agency. Do not impose a house voice or technical conventions unless the context requires them.
- Apply the [writing conventions](references/typography.md) to every writing task.

## Add specialized routes

The prose baseline applies to every human-facing writing task.

1. Add the [connected-prose workflow](references/prose.md) whenever a reader must follow context, explanation, reasoning, causality, tradeoffs, consequences, narrative, tone, or another idea across more than one atomic unit. Use it independently for nontechnical, creative, narrative, reflective, persuasive, personal, and other general writing.
2. Add the [technical-copy overlay](references/technical-copy.md) when correctness depends on implementation or contract evidence. This includes package and repository descriptions, README and technical-document content or information architecture, CLI wording, JSDoc and docstrings, explanatory source comments, runtime messages, test titles, labels, and other technical or developer-facing strings.
3. Apply the prose baseline and technical-copy overlay to every GitHub issue, pull request, and Discussion title, body, comment, or review. Add the connected-prose workflow whenever the reader must follow more than an atomic fact or action, including most bodies, comments, and reviews.
4. When both specialized routes apply, technical copy owns factual accuracy, exact literal tokens, consistent reader-facing terminology, necessary technical distinctions, required actions, observable behavior, and document-level information architecture, including templates, headings, and section order. Connected prose owns the reading path within that architecture, including reading-unit progression, paragraph movement, cohesion, cadence, and voice. Preserve the technical facts and structural constraints, then make the strongest prose possible inside them.
5. For a substantive prose review, difficult revision, or multi-unit work, apply the relevant parts of the [prose revision rubric](references/prose-revision-rubric.md).

Load only the additional references selected by this routing decision.

## Preserve intent and boundaries

- **Secrets:** Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, or relays. Never request, inspect, echo, or invent a real secret value unless the user explicitly directs it.
- Carry wording, punctuation, formatting, structure, voice, point of view, terminology, formality, and intended ambiguity established by direct user instruction, correction, selection, or explicit acceptance through later iterations. An agent draft or endorsement, user silence, or existing text alone does not establish a decision. Do not reopen writing the user marked intentional, final, or implemented unless the current request clearly does so.
- Treat supplied templates and their fields, headings, section and field order, syntax, and formatting conventions as constraints unless the task authorizes changing them.
- If a requested shorter version would lose a material condition, exception, rationale, or qualifier, preserve the meaning and explain the limit rather than silently weakening the writing.
- Ask for context only when a material fact or intent cannot be established. Do not manufacture certainty, rationale, or evidence.
- A request to draft or revise authorizes adding, removing, reordering, and rewriting within the authorized writing unit and making the minimum adjacent changes needed for integration. Apply those changes directly without requesting approval for each one. Do not expand into unrelated writing, document structure, cross-surface synchronization, or behaviorally distinct items unless the request includes them.
- That editorial authorization does not authorize publishing, submitting, sending, disclosing, or making any remote change.

## Deliver the result

- Edit the contextual target directly when authorized. Without a file target, put the ready-to-use writing first and do not create a file.
- Provide one best version by default. Offer alternatives only when the user requests them or material intent remains unresolved.
- Preserve the requested format and explain only material uncertainties, constraints, or intentional departures that the user needs to assess.
