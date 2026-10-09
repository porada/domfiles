# Technical Copy

Build on [Writing Principles](../SKILL.md#writing-principles) to keep technical facts, reader-facing terminology, constraints, and observable behavior exact. In review-only work, report only evidence-backed problems with factual accuracy, clarity, consistency, or the established voice.

## Workflow

Move from evidence to composition, propagation, and final validation.

### Evidence

Start by identifying the technical concepts the intended reader can be expected to know. Inspect nearby copy for established terminology and formatting, but do not treat it as automatic authority.

For new technical copy or any edit that changes technical meaning, inspect the relevant implementation, tests, and other authoritative evidence. Use verified facts to bound what the copy may claim rather than dictate every detail it must include. A meaning-neutral wording edit may rely on the supplied text.

Verify documented defaults against shipped values, not merely another documentation copy. Agreement between documents does not establish correctness. Correct stale copy rather than changing implementation to match it unless a behavior change is explicitly in scope.

### Composition

Lead with the answer, action, identity, failure, or observable behavior. Put the useful point before introductory context, but keep every distinction that affects behavior.

Preserve exact literal tokens. Do not retain a category term solely because a parser, grammar, protocol, implementation, standard, or upstream tool treats it as canonical. When direct wording preserves the complete meaning and scope, name what the reader supplies, sees, or does.

Prefer concise wording that readers can understand directly. When a compact label hides who acts, what happens, or how its parts relate, express that meaning with a verb or clause. Keep established technical terms when they aid understanding or preserve a necessary distinction.

Preserve causal framing, technical relationships, and the established tone. Instead of asserting importance, quality, or magnitude, state the mechanism, failure mode, boundary, or observable consequence that earns the claim. Ground claims about quality, maintenance, compatibility, performance, and testing in concrete evidence or careful qualifiers. Never overpromise.

Technical precision need not sound cold or impersonal. Use contractions, direct second-person wording, and an honest stance when they suit the surface and intended reader.

When a value’s representation is used only for comparison or lookup, describe those semantics without suggesting that the value itself is coerced, stringified, or mutated. Include implementation detail only when it explains a required action, limitation, non-obvious decision, or technical consequence.

### Propagation

When propagating or unifying copy, align verified shared facts and use consistent reader-facing terminology for the same concept in the same role. Reuse wording only when the roles, constraints, and observable behavior are semantically equivalent. After each replacement, move, or removal, review every affected passage and its surrounding copy independently in context. Confirm that surviving headings, lead sentences, and quoted fragments still match the remaining content, and that references such as “both,” “below,” and “that decision” still identify the intended subjects or locations. When behavior changes, revise contradictory existing wording rather than appending a correct explanation beneath it.

### Validation

Validate the final copy against the evidence, nearby family, complete rendered output, and project formatting constraints. Search the authorized writing unit for stale wording variants, and review every project-authored human-facing string, including failure-only test diagnostics.

Apply the [typography and technical token conventions](typography.md). Distinguish prose from machine syntax, fixture payloads, and preserved upstream text so exact data remains unchanged.

## Surface Conventions

- **CLI documentation and command output:** Treat `--help` text and usage lines as human-facing documentation, along with descriptions of options, inputs, modes, defaults, side effects, output behavior, and exit statuses. Keep every claim accurate to the implementation. Name accepted inputs directly when clear wording preserves the complete contract and lets readers locate any external documentation they need. Use a formal term only when no clearer description can retain the supported behavior or necessary discoverability. Use one reader-facing term consistently. Preserve exact literal token spelling across help, project-authored `stdout` or `stderr` messages, and adjacent exact-string tests. Preserve machine-readable output contracts unless the task explicitly changes them.
- **Explanatory source comments:** Explain non-obvious intent, constraints, invariants, or consequences rather than narrating nearby code. Proactively document a surprising tradeoff that could look accidental. Do not remove a comment whose purpose is to establish that behavior, a tradeoff, or an omission is intentional unless the current request explicitly authorizes its removal. Omit terminal periods from ordinary `//` comments.
- **Documentation comments and docstrings:** Follow the [documentation comment workflow](documentation-comments.md), including for JSDoc and language-native forms.
- **Package and repository descriptions:** Use one compact phrase without terminal punctuation. When both requested surfaces represent the same artifact, scope, and proposition, matching wording is the default.
- **README and collaboration copy:** For README, technical document, issue, pull request, and discussion copy, including titles, bodies, and comments, follow the [technical document workflow](technical-documents.md).
- **Runtime errors and warnings:** Follow the [runtime errors and warnings workflow](runtime-errors-and-warnings.md).
- **Test titles:** Use a lowercase, present tense predicate that reads naturally after an implicit subject. Omit modal `should` and terminal punctuation. Include only the condition needed to distinguish the case.

## Supporting Material

Make every material claim specific enough to check. Use exact names, values, conditions, or observable results as evidence only when they are appropriate to expose under the entrypoint’s [Secrets and Authentication](../SKILL.md#secrets-and-authentication) policy.

Evidence and illustration are not substitutes for each other. Neither an unnamed category nor a generic scenario is a concrete case. Include an example only when it clarifies non-obvious behavior, a required action, or a meaningful contrast. Use the smallest example that remains complete, and place it next to what it demonstrates.

Add a canonical link only when it helps the reader act, verify a claim, or reach necessary technical detail. Do not link every package, identifier, or named concept merely because a URL exists.
