---
name: technical-copy
description: Draft, edit, review, and refine human-facing technical copy. Use automatically for package or repository descriptions, README and technical-document content or information architecture, CLI help and output, JSDoc, docstrings, explanatory comments, runtime messages, test titles, labels, and other developer- or user-facing strings—including copy changed during feature implementation or consistency checks. Use it automatically for every GitHub issue, pull request, and Discussion title, body, comment, or review. Apply `prose` alongside it for every GitHub surface and for explanatory technical content that develops context, reasoning, causality, tradeoffs, or consequences. Defer release, changelog, and package-release prose to `release-notes`. Do not use for agent documentation, LICENSE text, formatting-only work, or tasks whose human-facing wording and information architecture are explicitly unchanged.
metadata:
    internal: true
---

# Technical copy

Apply this skill without announcing it or narrating its editorial heuristics.

Treat narrower project or domain skills as authoritative over this skill’s defaults.

## Choose the workflow

- For drafting or editing, compose the complete evidence-backed copy within the [authorized copy unit](#preserve-decisions-without-adding-friction).
- During feature implementation, apply this workflow to task-owned human-facing copy as part of the feature.
- For a review-only request, remain read-only and report only evidence-backed factual, clarity, consistency, or established-voice problems. Do not report optional rewrites merely because another phrasing is possible. This read-only rule takes precedence over the consistency and delivery defaults below.
- For an explicit consistency request, apply the standard path across the named family.

## Compose explanatory copy with prose

- Apply `prose` to every GitHub issue, pull request, and Discussion title, body, comment, or review, even when the copy is brief.
- On other technical surfaces, apply `prose` according to function rather than length. A single sentence or atomic string qualifies when it develops context, reasoning, causality, tradeoffs, consequences, or another explanation the reader must follow. Do not apply `prose` to strings that only name, instruct, or report one fact.
- This skill owns factual accuracy, exact terminology, required actions, template constraints, and observable behavior. `prose` owns the reading path, paragraph movement, cohesion, cadence, and voice within those boundaries.
- If their defaults differ, preserve the technical surface’s facts and constraints, then make the strongest prose possible inside them.

## Use the standard path

1. Resolve the exact copy unit, surface, technically capable reader, and immediate purpose. Inspect nearby copy for established terminology and formatting, treating it as context rather than automatic authority.
2. For new technical copy or a change to technical meaning, inspect the relevant implementation, tests, and other authoritative evidence. Treat verified facts as boundaries on what the copy may claim, not as a checklist of details to include. A meaning-neutral wording edit may rely on the supplied text. Ask for context when a material fact cannot be established rather than producing factual-sounding copy.
3. Lead with the answer, action, identity, failure, or observable behavior. Write for a short attention span without simplifying canonical technical terminology or burying the useful point in introductory context.
4. Use precise verbs, restrained natural language, and the most concise wording that preserves meaning, purpose, causal framing, technical relationships, and established tone. Contractions and direct second-person wording are welcome when the reader’s action matters. Ground quality, maintenance, compatibility, performance, and testing claims in concrete evidence or careful qualifiers. Never overpromise.
5. When a value’s representation is used only for comparison or lookup, describe those semantics without implying that the value itself is coerced, stringified, or mutated.
6. When propagating or unifying copy, align verified shared facts and canonical terminology, but reuse wording only across semantically equivalent roles, constraints, and observable behavior. Review destination-specific and adjacent copy independently after propagation.
7. Include implementation mechanics only when they explain a required action, limitation, non-obvious decision, or technical consequence. Validate the final copy against the evidence, requested scope, prior user decisions, nearby family, complete rendered output, and project formatting constraints.
8. Before delivery, search the complete copy unit for stale wording variants and review every project-authored human-facing string, including failure-only test diagnostics. Perform a final typography and token-formatting sweep under applicable project writing rules, distinguishing prose from machine syntax, fixture payloads, and preserved upstream text so exact data remains unchanged.

## Apply surface defaults

| Surface | Default |
| --- | --- |
| CLI documentation and command output | Treat `--help`, usage, option, operand, mode, default, side-effect, output-behavior, and exit-status text as human-facing documentation. Keep it accurate to the implementation. Use one canonical term and exact token spelling across help, project-authored stdout or stderr messages, and adjacent exact-string tests. Preserve machine-readable output contracts unless the task explicitly changes them. |
| Explanatory source comments | Explain non-obvious intent, constraints, invariants, or consequences rather than narrating nearby code. Proactively document a surprising tradeoff that could look accidental. Omit terminal periods from ordinary `//` comments. |
| JSDoc | Follow the [JSDoc workflow](references/jsdoc-comments.md), including for language-native documentation comments and docstrings. |
| Package and repository descriptions | Use one compact phrase without terminal punctuation. When both requested surfaces represent the same artifact, scope, and proposition, matching wording is the default. |
| README, technical-document, issue, pull request, and Discussion copy, including titles, bodies, and comments | Follow the [technical-document workflow](references/technical-documents.md). |
| Runtime errors and warnings | Follow the [runtime-message workflow](references/runtime-messages.md). |
| Test titles | Use a lowercase present-tense predicate that reads naturally after an implicit subject. Omit modal `should` and trailing punctuation. Include only the condition needed to distinguish the case. |

## Preserve decisions without adding friction

- An explicit request to compose or revise copy authorizes adding, removing, reordering, and rewriting wording within the task-owned copy unit and making the minimal adjacent changes needed to integrate it. Apply those changes directly without requesting approval for each one. Do not expand into unrelated copy, document structure, cross-surface synchronization, or behaviorally distinct items unless the request includes them.
- Carry wording, punctuation, formatting, and structure decisions established by direct user instruction, correction, selection, or explicit acceptance through later iterations. An agent draft or endorsement, user silence, or copy’s mere presence in a file does not establish a decision. Do not reopen copy the user marked intentional, final, or implemented unless the current request clearly does so.
- Do not remove an explanatory source comment whose purpose is to establish that behavior, a tradeoff, or an omission is intentional unless the current request explicitly authorizes its removal.
- If a requested shorter version would lose a material condition, exception, rationale, or qualifier, preserve the meaning and explain the limit rather than silently weakening the copy.

## Use examples and links sparingly

- Include an example only when it clarifies non-obvious behavior, a required action, or a meaningful contrast. Use the smallest example that remains complete and place it next to what it demonstrates.
- Add a canonical link only when it helps the reader act, verify a claim, or access necessary technical detail. Do not link every package, identifier, or named concept merely because a URL exists.

## Deliver the copy

- Edit the contextual file target directly. Without a file target, put the ready-to-use copy first and do not create a file.
- Provide one best evidence-backed version by default. Offer alternatives only when intent remains unresolved or the user requests options.
- Include supporting explanation only when a factual uncertainty or material decision needs the user’s attention.
