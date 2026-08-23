---
name: human-facing-writing
description: |-
    Draft, edit, review, and refine all human-facing writing, from short-form copy to connected prose. Use it whenever readers must follow ideas, context, reasoning, narrative, or tone.

    Use it across technical surfaces such as package and repository descriptions, README and technical-document content or information architecture, CLI wording, JSDoc and docstrings, explanatory source comments, runtime messages, test titles, labels, and repository collaboration through issues, pull requests, and discussions.

    Do not use for LICENSE text, formatting-only work, or tasks that neither evaluate nor change wording or information architecture. Agent responses are not a trigger on their own.
---

# Human-Facing Writing

Writing works when meaning, voice, and necessary complexity take a form that the reader can follow. This skill applies that principle to everything from a single label to connected prose.

## Workflow

- **Drafting:** Produce the complete authorized writing unit from the user’s intent and available material.
- **Editing:** Preserve the supplied writer’s meaning, material decisions, and recognizable voice while improving the requested dimensions.
- **Implementation:** Apply the relevant writing route to task-owned human-facing wording as part of the feature.
- **Review:** Keep review-only work read-only. Report only concrete problems under the applicable route, and do not rewrite the work or present optional preferences as defects.
- **Consistency:** For an explicit consistency request, apply the relevant route across the named family without expanding into behaviorally distinct surfaces.

## Prose Baseline

Establish the authorized writing unit, its surface, intended reader, immediate purpose, and desired change in the reader’s understanding, feeling, judgment, or action. Make the useful point, action, or experience recognizable soon enough to sustain attention without forcing urgency or simplifying necessary complexity.

Match the writing’s movement to the unit. Give an atomic string one clear job, and let connected prose develop purposeful progression between sentences, paragraphs, or sections.

Prefer precise, familiar, concrete wording and purposeful rhythm. Where immediate clarity is intended, make material agency, causality, conditions, contrasts, and dependencies explicit rather than leaving adjacency, syntax, or cadence to imply them. Keep confidence proportional to the available material, and verify factual claims when the writing makes them.

Preserve the writer’s intended voice and the reader’s agency. Do not impose a house voice or technical conventions unless the context requires them. Apply the [typography conventions](references/typography.md) to every writing task.

## Specialized Routes

The prose baseline applies to every human-facing writing task. Load only the additional references selected below.

- **Connected Prose:** Load the [connected-prose workflow](references/connected-prose.md) whenever a reader must follow context, explanation, reasoning, causality, tradeoffs, consequences, narrative, tone, or another idea across more than one atomic unit. Use it independently for nontechnical, creative, narrative, reflective, persuasive, personal, and other general writing.
- **Technical Copy:** Load the [technical-copy overlay](references/technical-copy.md) when correctness depends on implementation or contract evidence. This includes package and repository descriptions, README and technical-document content or information architecture, CLI wording, JSDoc and docstrings, explanatory source comments, runtime messages, test titles, labels, and other technical or developer-facing strings.
- **Revision Rubric:** For a substantive review, difficult revision, or work involving multiple reading units, load the relevant parts of the [prose revision rubric](references/prose-revision-rubric.md).
- **GitHub Collaboration:** Apply the prose baseline and technical-copy overlay to every GitHub issue, pull request, and Discussion title, body, comment, or review. Add the connected-prose workflow whenever the reader must follow more than an atomic fact or action, including most bodies, comments, and reviews.
- **Combined Routes:** When both specialized routes apply, technical copy owns factual accuracy, exact literal tokens, consistent reader-facing terminology, necessary technical distinctions, required actions, observable behavior, and document-level information architecture, including templates, headings, and section order. Connected prose owns the reading path within that architecture, including reading-unit progression, paragraph movement, cohesion, cadence, and voice. Preserve the technical facts and structural constraints, then make the strongest prose possible within them.

## Editorial Boundaries

Follow any more specific instructions that apply to the same choice, whether they come from the user, project, or subject area.

Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, or relays. Never request, inspect, echo, or invent a real secret value unless the user explicitly directs it.

Preserve wording, punctuation, formatting, structure, voice, point of view, terminology, formality, and intended ambiguity that the user establishes through a direct instruction, correction, selection, or explicit acceptance in a later iteration. An agent draft or endorsement, user silence, or existing text alone does not establish a decision. Do not reopen writing the user has marked intentional, final, or implemented unless the current request clearly includes it. Treat a supplied template’s fields, headings, section and field order, syntax, and formatting conventions as constraints unless the task authorizes changing them.

If a shorter version would lose a material condition, exception, rationale, or qualifier, preserve the meaning and explain the limit rather than silently weakening the writing. Ask for context only when a material fact or intent cannot be established, and do not manufacture certainty, rationale, or evidence.

A request to draft or revise authorizes adding, removing, reordering, and rewriting within the authorized writing unit, plus the minimum adjacent changes needed for integration. Make those changes directly without requesting separate approval for each one. Do not expand into unrelated writing, document structure, cross-surface synchronization, or behaviorally distinct items unless the request includes them. Editorial authorization does not authorize publishing, submitting, sending, disclosing, or making any remote change.

## Delivery

When authorized, edit the contextual target directly. Without a file target, put the ready-to-use writing first and do not create a file. Provide one best version unless the user requests alternatives or material intent remains unresolved.

Do not announce the skill or narrate the editorial process while applying it. Preserve the requested format, and explain only material uncertainties, constraints, or intentional departures that the user needs to assess.
