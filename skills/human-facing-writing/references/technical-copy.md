# Technical copy

Build on the entrypoint’s writing workflow to establish technical facts, reader-facing terminology, constraints, and observable behavior. In review-only tasks, report only evidence-backed factual, clarity, consistency, or established-voice problems.

## Use the standard path

1. Determine which technical concepts the intended reader can be expected to know. Inspect nearby copy for established terminology and formatting, treating it as context rather than automatic authority.
2. For new technical copy or a change to technical meaning, inspect the relevant implementation, tests, and other authoritative evidence. Treat verified facts as boundaries on what the copy may claim, not as a checklist of details to include. A meaning-neutral wording edit may rely on the supplied text.
3. Lead with the answer, action, identity, failure, or observable behavior. Write for a short attention span without hiding distinctions that affect behavior or burying the useful point in introductory context. Preserve exact literal tokens, but do not retain category terms merely because a parser, grammar, protocol, implementation, standard, or upstream tool treats them as canonical. Name what the reader supplies, sees, or does whenever direct wording preserves the full meaning and scope.
4. Preserve causal framing, technical relationships, and established tone while using the most concise wording that retains meaning and purpose. Replace claims of importance, quality, or magnitude with the mechanism, failure mode, boundary, or observable consequence that earns them. Technical precision does not require cold or impersonal wording. Use contractions, direct second-person wording, and an honest stance when they fit the surface and intended reader. Ground quality, maintenance, compatibility, performance, and testing claims in concrete evidence or careful qualifiers. Never overpromise.
5. When a value’s representation is used only for comparison or lookup, describe those semantics without implying that the value itself is coerced, stringified, or mutated.
6. When propagating or unifying copy, align verified shared facts and reader-facing terminology, but reuse wording only across semantically equivalent roles, constraints, and observable behavior. Review destination-specific and adjacent copy independently after propagation.
7. Include implementation mechanics only when they explain a required action, limitation, non-obvious decision, or technical consequence. Validate the final copy against the evidence, nearby family, complete rendered output, and project formatting constraints.
8. Before delivery, search the authorized writing unit for stale wording variants and review every project-authored human-facing string, including failure-only test diagnostics. Perform a final sweep under the [typography and technical-token conventions](typography.md), distinguishing prose from machine syntax, fixture payloads, and preserved upstream text so exact data remains unchanged.

## Apply surface defaults

| Surface | Default |
| --- | --- |
| CLI documentation and command output | Treat `--help` text, usage lines, option and input descriptions, modes, defaults, side effects, output behavior, and exit statuses as human-facing documentation. Keep it accurate to the implementation. Name accepted inputs directly when clear wording preserves their complete contract and lets readers locate any external documentation they need. Use a formal term only when no clearer description can retain the supported behavior or necessary discoverability. Use one reader-facing term consistently and preserve exact literal token spelling across help, project-authored stdout or stderr messages, and adjacent exact-string tests. Preserve machine-readable output contracts unless the task explicitly changes them. |
| Explanatory source comments | Explain non-obvious intent, constraints, invariants, or consequences rather than narrating nearby code. Proactively document a surprising tradeoff that could look accidental. Do not remove a comment whose purpose is to establish that behavior, a tradeoff, or an omission is intentional unless the current request explicitly authorizes its removal. Omit terminal periods from ordinary `//` comments. |
| JSDoc | Follow the [JSDoc workflow](documentation-comments.md), including for language-native documentation comments and docstrings. |
| Package and repository descriptions | Use one compact phrase without terminal punctuation. When both requested surfaces represent the same artifact, scope, and proposition, matching wording is the default. |
| README, technical-document, issue, pull request, and Discussion copy, including titles, bodies, and comments | Follow the [technical-document workflow](technical-documents.md). |
| Runtime errors and warnings | Follow the [runtime-message workflow](runtime-messages.md). |
| Test titles | Use a lowercase present-tense predicate that reads naturally after an implicit subject. Omit modal `should` and trailing punctuation. Include only the condition needed to distinguish the case. |

## Use examples and links sparingly

- Make load-bearing claims specific enough to check. Use exact names, values, conditions, or observable results as evidence only when they are appropriate to expose under the entrypoint’s [intent and boundary rules](../SKILL.md#preserve-intent-and-boundaries). Do not treat evidence and illustration as substitutes, and do not present an unnamed category or generic scenario as a concrete case.
- Include an example only when it clarifies non-obvious behavior, a required action, or a meaningful contrast. Use the smallest example that remains complete and place it next to what it demonstrates.
- Add a canonical link only when it helps the reader act, verify a claim, or access necessary technical detail. Do not link every package, identifier, or named concept merely because a URL exists.
