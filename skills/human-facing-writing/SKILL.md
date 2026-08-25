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

Start with the authorized writing unit, where it will appear, and the reader it must serve. Establish its immediate purpose and what should change in the reader’s understanding, feeling, judgment, or action. Bring the useful point, action, or experience into view soon enough to sustain attention. Do not force urgency or flatten necessary complexity.

Match the writing’s movement to the work it must do. Give an atomic string one clear job. In connected prose, let each sentence, paragraph, and section carry the thought forward.

Choose precise, familiar, concrete words. When clarity should be immediate, say who acts, what causes what, and which conditions, contrasts, or dependencies matter. Keep confidence proportional to the available material, and verify factual claims the writing makes.

Preserve the writer’s intended voice and the reader’s agency. Do not impose a house voice or technical conventions unless the context requires them. Apply the [typography conventions](references/typography.md) to every writing task.

## Specialized Routes

The prose baseline applies to every human-facing writing task. Load only the references the task requires.

- **Connected Prose:** Load the [connected-prose workflow](references/connected-prose.md) when a reader must follow an idea across more than one atomic unit. This includes context, explanation, reasoning, causality, tradeoffs, consequences, narrative, and tone. Use it independently for nontechnical, creative, narrative, reflective, persuasive, personal, and other general writing.
- **Technical Copy:** Load the [technical-copy overlay](references/technical-copy.md) when correctness depends on implementation or contract evidence. This includes package and repository descriptions, README and technical-document content or information architecture, CLI wording, JSDoc and docstrings, explanatory source comments, runtime messages, test titles, labels, and other technical or developer-facing strings.
- **Revision Rubric:** For a substantive review, difficult revision, or work involving multiple reading units, load the relevant parts of the [prose revision rubric](references/prose-revision-rubric.md).
- **GitHub Collaboration:** Apply the prose baseline and technical-copy overlay to every GitHub issue, pull request, and Discussion title, body, comment, or review. Add Connected Prose whenever the reader must follow more than an atomic fact or action, including most bodies, comments, and reviews.

When Connected Prose and Technical Copy both apply, Technical Copy owns factual accuracy, exact literal tokens, consistent reader-facing terminology, necessary technical distinctions, required actions, observable behavior, and document-level information architecture, including templates, headings, and section order. Connected Prose owns the reading path within that architecture, including reading-unit progression, paragraph movement, cohesion, cadence, and voice. Preserve the technical facts and structural constraints, then make the strongest prose possible within them.

## Editorial Boundaries

Follow any more specific instruction that governs the same choice, whether it comes from the user, project, or subject area.

Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, or relays. Never request, inspect, echo, or invent a real secret value unless the user explicitly directs it.

Treat a choice as settled when the user establishes it through a direct instruction, correction, selection, or explicit acceptance in a later iteration. This applies to wording, punctuation, formatting, structure, voice, point of view, terminology, formality, and intended ambiguity. Existing text alone, an agent draft or endorsement, and user silence do not establish a decision. Do not reopen writing the user has marked intentional, final, or implemented unless the current request clearly includes it.

Treat a supplied template’s fields, headings, section and field order, syntax, and formatting conventions as constraints unless the task authorizes changing them.

Do not trade a material condition, exception, rationale, or qualifier for brevity. If a shorter version would weaken the meaning, preserve the full meaning and explain the limit. Ask for context only when a material fact or intent cannot be established. Do not manufacture certainty, rationale, or evidence.

A request to draft or revise authorizes adding, removing, reordering, and rewriting within the authorized writing unit, plus the minimum adjacent changes needed for integration. Make those changes directly without requesting separate approval.

That authorization does not extend to unrelated writing, document structure, cross-surface synchronization, or behaviorally distinct items unless the request includes them. It also does not authorize publishing, submitting, sending, disclosing, or making any remote change.

## Delivery

When authorized, edit the contextual target directly. Without a file target, put the ready-to-use writing first and do not create a file. Provide one best version unless the user requests alternatives or material intent remains unresolved.

Do not announce the skill or narrate the editorial process while applying it. Preserve the requested format, and explain only material uncertainties, constraints, or intentional departures that the user needs to assess.
