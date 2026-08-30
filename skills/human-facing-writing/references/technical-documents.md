# Technical Documents

## Scope

Interpret the request literally. A content-only request does not authorize structural changes, and a structure-only request does not authorize rewriting otherwise sound copy. Heading wording counts as copy. During structure-only work, headings may move or change level, but they may not be renamed or newly written.

Preserve every existing heading’s exact identifier spelling, significant characters, entities, and intentional markup. Leave an unchanged heading out of a structural outline rather than normalizing it. Do not introduce nonbreaking characters, entities, or presentational markup unless project policy or the user requires them.

Apply the entrypoint’s [editorial boundaries](../SKILL.md#editorial-boundaries) to selected or restored headings, section order, taglines, links, examples, wording, and structure.

## Document Structure

Start with any supplied template. Its sections determine where each kind of information belongs. When a template separates the proposal, rationale, evidence, examples, and implementation ideas, keep the proposal’s opening to the smallest complete statement of the requested outcome and its essential boundary. Put each supporting detail in its designated section.

After the opening appropriate to the document type, arrange the remaining content in progressive depth. Move from the first useful action or conclusion through evidence, setup, core behavior, optional detail, caveats, alternatives, reference material, troubleshooting, or questions as the document requires. Keep optional variants subordinate to the primary path. Use direct question headings when a question-and-answer format helps readers scan non-obvious concerns.

Treat this sequence as a decision framework, not a fixed template. A visual before-and-after example may precede setup when it communicates value faster. A short issue comment may need only the decision and its evidence. Omit any section that does not help the intended reader.

### FAQ Collections

Treat an established FAQ as one question collection, and keep its question-form headings in natural casing. During structure-only work, keep every existing question-form heading under that FAQ rather than moving individual questions into other sections.

Do not propose a standalone FAQ with only one question. If another grouping would split an established FAQ or leave it with one question, preserve the current collection and identify the unresolved copy decision instead of rewriting or relocating questions.

## Document Types

Before drafting or revising an issue or pull request body for a specific repository, offer to look up the user’s previous submissions there. If the user accepts, identify the recurring structure, terminology, tone, and level of detail. Apply that pattern without copying incidental wording or overriding the repository’s current template. Continue without the lookup when the user declines, the lookup is unavailable, or no useful previous submissions exist.

For any security report, whether standalone or submitted through an issue or pull request form, also follow the [security-report workflow](security-reports.md).

| Document Type | Default |
| --- | --- |
| Comment, reply, or code review | Lead with the answer, finding, or required action. Include only the thread context needed to support it, and distinguish required changes from optional suggestions. |
| Issue, pull request, or Discussion title and body | Follow the repository template. Make the problem, outcome, or scope recognizable immediately, then provide relevant evidence, impact, proposed or implemented changes, validation, and the next action. |
| Other technical document | Follow the established document hierarchy. Lead with the document’s purpose, decision, or required action, and include only the context needed to understand it. |
| README | Open with a compact project or package identity and practical value. Move through the first useful demonstration, installation, setup, and core usage before optional configuration or questions, as applicable. Keep related projects, provenance, and license information near the end when established. Preserve intentional hero artwork, badges, and layout. |

## Content

- Place prerequisites, caveats, alternatives, and evidence close to the claim or action they constrain.
- Use a callout for an operational constraint or risk that readers could otherwise miss. Place it beside the affected action, use the least severe established callout type that fits, and keep it concise. Do not use callouts for ordinary notes or decorative emphasis.
- Present document fragments and heading outlines directly rather than wrapping the entire fragment in an outer code block. Preserve code blocks that belong to the document content.
- Keep equivalent package-manager or environment paths parallel when the project presents them as equal options.
- On GitHub surfaces that automatically link same-repository references, prefer bare `#…` references for issues and pull requests when user or repository convention permits them. Reference Discussions by URL, using descriptive Markdown link text when the destination would otherwise be unclear. Use an explicit link for cross-repository, ambiguous, or off-platform references.

## Validation

As part of the technical-copy workflow’s [final validation](technical-copy.md#validation), read the complete rendered document from its opening through the first useful result or decision, then continue through any optional detail. Confirm that the headings reveal the hierarchy and that every link reaches its intended destination.
