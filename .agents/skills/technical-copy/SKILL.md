---
name: technical-copy
description: Draft, edit, review, and refine human-facing copy in engineering projects. Use this skill automatically whenever a task creates, changes, or reviews package or repository descriptions, README or other technical-document content or information architecture, GitHub issue or pull request titles, descriptions, comments, or reviews, JSDoc, language-native documentation comments or docstrings, explanatory source comments, runtime errors or warnings, test titles, labels, or other developer- or user-facing strings, including copy written during feature implementation and explicit consistency checks. Defer release notes, changelog entries, release-ready prose, hosted release descriptions, and package-release wording or consistency checks to `release-notes`. Do not use it for agent documentation or LICENSE text. Do not use it when all human-facing wording and document information architecture are explicitly unchanged, including formatting-only tasks.
---

# Technical copy

Apply this skill without announcing it or narrating its editorial heuristics. Continue required progress, result, validation, uncertainty, and failure reporting.

Treat applicable project policy—including `AGENTS.md` instructions and narrower project or domain skills—as authoritative over this skill’s defaults.

## Choose the workflow

- For drafting or editing, compose the complete evidence-backed copy directly within the task-owned copy unit. Make only the minimal adjacent edits needed to integrate it naturally.
- During feature implementation, treat relevant JSDoc, language-native documentation comments or docstrings, explanatory comments, runtime messages, and test titles as part of the feature. Do not turn a local addition into a broader copy pass.
- For a review-only request, remain read-only and report only evidence-backed factual, clarity, consistency, or established-voice problems. Do not report optional rewrites merely because another phrasing is possible. This read-only rule takes precedence over the consistency and delivery defaults below.
- For an explicit consistency request, inspect the named family and align shared facts and canonical terminology. Unify wording only across equivalent surfaces and scopes, preserving behaviorally distinct items and each surface’s purpose and abstraction level.

## Use the standard path

1. Resolve the exact copy unit, surface, technically capable reader, and immediate purpose. Inspect nearby copy for established terminology and formatting, treating it as context rather than automatic authority.
2. For new technical copy or a change to technical meaning, inspect the relevant implementation, tests, and other authoritative evidence. Treat verified facts as boundaries on what the copy may claim, not as a checklist of details to include. A meaning-neutral wording edit may rely on the supplied text. Ask for context when a material fact cannot be established rather than producing factual-sounding copy.
3. Lead with the answer, action, identity, failure, or observable behavior. Write for a short attention span without simplifying canonical technical terminology or burying the useful point in introductory context.
4. Use precise verbs, restrained natural language, and the most concise wording that preserves meaning, purpose, causal framing, technical relationships, and established tone. Contractions and direct second-person wording are welcome when the reader’s action matters. Ground quality, maintenance, compatibility, performance, and testing claims in concrete evidence or careful qualifiers. Never overpromise.
5. When a value’s representation is used only for comparison or lookup, describe those semantics without implying that the value itself is coerced, stringified, or mutated.
6. When propagating or unifying copy, reuse wording only across semantically equivalent roles, constraints, and observable behavior. Review destination-specific and adjacent copy independently after propagation.
7. Include implementation mechanics only when they explain a required action, limitation, non-obvious decision, or technical consequence. Validate the final copy against the evidence, requested scope, prior user decisions, nearby family, complete rendered output, and project formatting constraints.

## Apply surface defaults

| Surface | Default |
| --- | --- |
| Package and repository descriptions | Use one compact phrase without terminal punctuation. When both surfaces represent the same artifact, scope, and proposition, matching wording is the default. Do not enforce synchronization or edit an unrequested surface. |
| README, technical-document, issue, and pull request copy | Follow the [technical-document workflow](references/technical-documents.md). |
| JSDoc | Follow the [JSDoc workflow](references/jsdoc-comments.md), including for language-native documentation comments and docstrings. |
| Explanatory source comments | Explain non-obvious intent, constraints, invariants, or consequences rather than narrating nearby code. Proactively document a surprising tradeoff that could look accidental. Omit terminal periods from ordinary `//` comments. |
| Runtime errors and warnings | Follow the [runtime-message workflow](references/runtime-messages.md). |
| Test titles | Use a lowercase present-tense predicate that reads naturally after an implicit subject. Omit modal `should` and trailing punctuation. Include only the condition needed to distinguish the case. |

## Preserve decisions without adding friction

- An explicit request to compose or revise copy authorizes adding, removing, reordering, and rewriting wording within the task-owned copy unit. Apply those changes directly without requesting approval for each one.
- Carry wording, punctuation, formatting, and structure decisions established by direct user instruction, correction, selection, or explicit acceptance through later iterations. An agent draft or endorsement, user silence, or copy’s mere presence in a file does not establish a decision. Do not reopen copy the user marked intentional, final, or implemented unless the current request clearly does so.
- Do not remove an explanatory source comment whose purpose is to establish that behavior, a tradeoff, or an omission is intentional unless the current request explicitly authorizes its removal.
- Do not expand into unrelated copy, change document structure, enforce cross-surface synchronization, merge behaviorally distinct items, or reverse an established decision unless the current request explicitly authorizes it.
- If a requested shorter version would lose a material condition, exception, rationale, or qualifier, preserve the meaning and explain the limit rather than silently weakening the copy.

## Use examples and links sparingly

- Include an example only when it clarifies non-obvious behavior, a required action, or a meaningful contrast. Use the smallest example that remains complete and place it next to what it demonstrates.
- Preserve intentionally chosen links. Add a canonical link only when it helps the reader act, verify a claim, or access necessary technical detail. Do not link every package, identifier, or named concept merely because a URL exists.

## Deliver the copy

- Edit the contextual file target directly. Without a file target, put the ready-to-use copy first and do not create a file.
- Provide one best evidence-backed version by default. Offer alternatives only when intent remains unresolved or the user requests options.
- Include supporting explanation only when a factual uncertainty or material decision needs the user’s attention.
