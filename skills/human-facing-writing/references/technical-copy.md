# Technical copy

Build on the entrypoint’s writing workflow to establish technical facts, terminology, constraints, and observable behavior. In review-only tasks, report only evidence-backed factual, clarity, consistency, or established-voice problems.

## Use the standard path

1. Determine which technical concepts the intended reader can be expected to know. Inspect nearby copy for established terminology and formatting, treating it as context rather than automatic authority.
2. For new technical copy or a change to technical meaning, inspect the relevant implementation, tests, and other authoritative evidence. Treat verified facts as boundaries on what the copy may claim, not as a checklist of details to include. A meaning-neutral wording edit may rely on the supplied text.
3. Lead with the answer, action, identity, failure, or observable behavior. Write for a short attention span without simplifying canonical technical terminology or burying the useful point in introductory context.
4. Preserve canonical terminology, causal framing, technical relationships, and established tone while using the most concise wording that retains meaning and purpose. Contractions and direct second-person wording are welcome when the reader’s action matters. Ground quality, maintenance, compatibility, performance, and testing claims in concrete evidence or careful qualifiers. Never overpromise.
5. When a value’s representation is used only for comparison or lookup, describe those semantics without implying that the value itself is coerced, stringified, or mutated.
6. When propagating or unifying copy, align verified shared facts and canonical terminology, but reuse wording only across semantically equivalent roles, constraints, and observable behavior. Review destination-specific and adjacent copy independently after propagation.
7. Include implementation mechanics only when they explain a required action, limitation, non-obvious decision, or technical consequence. Validate the final copy against the evidence, nearby family, complete rendered output, and project formatting constraints.
8. Before delivery, search the authorized writing unit for stale wording variants and review every project-authored human-facing string, including failure-only test diagnostics. Perform a final sweep under the [typography and technical-token conventions](typography.md), distinguishing prose from machine syntax, fixture payloads, and preserved upstream text so exact data remains unchanged.

## Apply surface defaults

| Surface | Default |
| --- | --- |
| CLI documentation and command output | Treat `--help`, usage, option, operand, mode, default, side-effect, output-behavior, and exit-status text as human-facing documentation. Keep it accurate to the implementation. Use one canonical term and exact token spelling across help, project-authored stdout or stderr messages, and adjacent exact-string tests. Preserve machine-readable output contracts unless the task explicitly changes them. |
| Explanatory source comments | Explain non-obvious intent, constraints, invariants, or consequences rather than narrating nearby code. Proactively document a surprising tradeoff that could look accidental. Do not remove a comment whose purpose is to establish that behavior, a tradeoff, or an omission is intentional unless the current request explicitly authorizes its removal. Omit terminal periods from ordinary `//` comments. |
| JSDoc | Follow the [JSDoc workflow](documentation-comments.md), including for language-native documentation comments and docstrings. |
| Package and repository descriptions | Use one compact phrase without terminal punctuation. When both requested surfaces represent the same artifact, scope, and proposition, matching wording is the default. |
| README, technical-document, issue, pull request, and Discussion copy, including titles, bodies, and comments | Follow the [technical-document workflow](technical-documents.md). |
| Runtime errors and warnings | Follow the [runtime-message workflow](runtime-messages.md). |
| Test titles | Use a lowercase present-tense predicate that reads naturally after an implicit subject. Omit modal `should` and trailing punctuation. Include only the condition needed to distinguish the case. |

## Use examples and links sparingly

- Include an example only when it clarifies non-obvious behavior, a required action, or a meaningful contrast. Use the smallest example that remains complete and place it next to what it demonstrates.
- Add a canonical link only when it helps the reader act, verify a claim, or access necessary technical detail. Do not link every package, identifier, or named concept merely because a URL exists.
