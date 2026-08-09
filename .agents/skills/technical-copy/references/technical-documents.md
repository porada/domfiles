# Technical documents

Use this workflow for README files, other technical documents, and GitHub issue or pull request titles, descriptions, comments, and reviews. Apply the standard path from the skill entrypoint. Treat applicable project policy, repository templates, and explicit user decisions about wording, structure, links, examples, and scope as authoritative over these defaults.

## Resolve the requested surface

- Interpret scope literally. A content-only request does not authorize restructuring, and a structure-only request does not authorize rewriting otherwise sound prose.
- Treat heading wording as copy. A structure-only request may move existing headings or change their level, but may not rename them or invent new heading text.
- When reproducing an existing heading, preserve its exact identifier spelling, significant characters, entities, and intentional markup. Omit an unchanged heading from a structural outline rather than normalizing its markup. Do not introduce nonbreaking characters, entities, or presentational markup where the source does not already use them unless project policy or the user requires it.
- During feature work, update only the task-owned passage and the minimal navigation or adjacent copy needed to integrate it. Do not turn one documented addition into a broad document rewrite.
- Preserve headings, section order, taglines, links, examples, and wording that the user has selected or restored unless the current request targets them.
- Apply the global README permission rule to README edits. When permission is absent, identify the target location and provide the proposed structure or ready-to-use copy without modifying the file.
- Drafting a GitHub issue, pull request, security report, comment, or review does not authorize submitting it or modifying linked code or documentation unless the current request explicitly does so.
- For any security report—whether standalone or submitted through an issue or pull request form—also follow the [security-report workflow](security-reports.md).

## Design the reading path

1. Identify the intended reader, the immediate purpose, and the shortest path to a useful result, decision, or action.
2. Open with the project identity and practical value, the problem or outcome, or the current decision according to the surface. Expand only with concrete context that helps the reader continue.
3. Arrange the remaining content in progressive depth. Move from the first useful action or conclusion through evidence, setup, core behavior, optional detail, caveats, alternatives, reference material, troubleshooting, or questions as the task requires.
4. Keep optional variants subordinate to the primary path. Use direct question headings when a question-and-answer format helps readers scan non-obvious concerns.
5. Treat an established `FAQ` as one question collection. In a structure-only pass, keep every existing question-form heading under that `FAQ`, and do not move individual questions into other sections. Do not propose a standalone `FAQ` containing only one question. When another grouping would violate either condition, preserve the current `FAQ` and identify the unresolved copy decision instead of rewriting or relocating questions.

Treat that sequence as a decision framework rather than a fixed template. A visual before-and-after example may precede setup when it communicates value faster. A short issue comment may need only the decision and its evidence. Omit any section that does not help the intended reader.

## Apply surface conditions

| Surface | Apply |
| --- | --- |
| README | Open with a compact project or package identity and practical value. Move through the first useful demonstration, installation, setup, and core usage before optional configuration or questions as applicable. Keep related projects, provenance, and license information near the end when established. Preserve intentional hero artwork, badges, and layout. |
| Other technical document | Follow the established document hierarchy. Lead with the document’s purpose, decision, or required action and include only the context needed to understand it. |
| Issue or pull request title and description | Follow the repository template. Make the problem, outcome, or scope recognizable immediately, then provide relevant evidence, impact, proposed or implemented changes, validation, and next action. |
| Follow-up comment or code review | Lead with the answer, finding, or required action. Include only the thread context needed to support it, and distinguish required changes from optional suggestions. |

## Compose the content

- Use short, title-cased headings unless another pattern is established. Keep FAQ questions and other quote-like headings in natural casing. Preserve an established heading rather than replacing it with a merely synonymous label.
- Keep prerequisites, caveats, alternatives, and evidence close to the claim or action they constrain.
- Use a callout for an operational constraint or risk that readers could otherwise miss. Place it beside the affected action, use the least severe established callout type that fits, and keep it concise. Do not use callouts for ordinary notes or decorative emphasis.
- Present document fragments and heading outlines directly rather than wrapping the entire fragment in an outer code block. Preserve code blocks that are part of the document content.
- Keep equivalent package-manager or environment paths parallel when the project presents them as equal options.
- Across related documents or discussions, keep shared facts and terminology consistent while preserving each surface’s purpose and abstraction level.
- Preserve established layout and intentional link wording. Do not introduce or normalize presentation merely because another document uses it.

## Validate the result

Read the complete rendered path from the opening statement through the first useful result or decision, then through optional detail. Confirm that headings reveal the hierarchy, template requirements remain satisfied, examples and evidence stay adjacent to their claims, links still reach the intended targets, and no section repeats facts owned elsewhere. Remove only material that is redundant, unsupported, or outside the requested scope.
