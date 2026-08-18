# Technical documents

Apply the entrypoint’s [standard path](../SKILL.md#use-the-standard-path) before these surface-specific rules.

## Resolve the requested surface

- Interpret scope literally. A content-only request does not authorize restructuring, and a structure-only request does not authorize rewriting otherwise sound prose.
- Treat heading wording as copy. A structure-only request may move existing headings or change their level, but may not rename them or invent new heading text.
- When reproducing an existing heading, preserve its exact identifier spelling, significant characters, entities, and intentional markup. Omit an unchanged heading from a structural outline rather than normalizing its markup. Do not introduce nonbreaking characters, entities, or presentational markup where the source does not already use them unless project policy or the user requires it.
- Apply the entrypoint’s [decision-preservation rule](../SKILL.md#preserve-decisions-without-adding-friction) to selected or restored headings, section order, taglines, links, examples, wording, and structure.
- Treat supplied template sections as information-allocation boundaries. When a template separates the proposal, rationale, evidence, examples, and implementation ideas, keep the opening proposal to the smallest complete statement of the requested outcome and its essential boundary, then place each supporting detail in its designated section.
- Follow the README gate in the global “Documentation” policy. When it leaves mutation unauthorized, identify the target location and provide the proposed structure or ready-to-use copy without modifying the file.
- Follow the [`github-cli` workflow](../../github-cli/SKILL.md) when GitHub CLI use is needed for a GitHub-hosted surface. Drafting a GitHub issue, pull request, Discussion, security report, comment, or review does not authorize submitting it or modifying linked code or documentation unless the current request explicitly does so.
- For any security report—whether standalone or submitted through an issue or pull request form—also follow the [security-report workflow](security-reports.md).

## Design the reading path

1. After the surface-appropriate opening, arrange the remaining content in progressive depth. Move from the first useful action or conclusion through evidence, setup, core behavior, optional detail, caveats, alternatives, reference material, troubleshooting, or questions as the task requires.
2. Keep optional variants subordinate to the primary path. Use direct question headings when a question-and-answer format helps readers scan non-obvious concerns.
3. Treat an established `FAQ` as one question collection. In a structure-only pass, keep every existing question-form heading under that `FAQ`, and do not move individual questions into other sections. Do not propose a standalone `FAQ` containing only one question. When another grouping would violate either condition, preserve the current `FAQ` and identify the unresolved copy decision instead of rewriting or relocating questions.

Treat that sequence as a decision framework rather than a fixed template. A visual before-and-after example may precede setup when it communicates value faster. A short issue comment may need only the decision and its evidence. Omit any section that does not help the intended reader.

## Apply surface conditions

| Surface | Apply |
| --- | --- |
| Comment, reply, or code review | Lead with the answer, finding, or required action. Include only the thread context needed to support it, and distinguish required changes from optional suggestions. |
| Issue, pull request, or Discussion title and body | Follow the repository template. Make the problem, outcome, or scope recognizable immediately, then provide relevant evidence, impact, proposed or implemented changes, validation, and next action. |
| Other technical document | Follow the established document hierarchy. Lead with the document’s purpose, decision, or required action and include only the context needed to understand it. |
| README | Open with a compact project or package identity and practical value. Move through the first useful demonstration, installation, setup, and core usage before optional configuration or questions as applicable. Keep related projects, provenance, and license information near the end when established. Preserve intentional hero artwork, badges, and layout. |

## Compose the content

- Use short, title-cased headings unless another pattern is established. Keep FAQ questions and other quote-like headings in natural casing.
- Keep prerequisites, caveats, alternatives, and evidence close to the claim or action they constrain.
- Use a callout for an operational constraint or risk that readers could otherwise miss. Place it beside the affected action, use the least severe established callout type that fits, and keep it concise. Do not use callouts for ordinary notes or decorative emphasis.
- Present document fragments and heading outlines directly rather than wrapping the entire fragment in an outer code block. Preserve code blocks that are part of the document content.
- Keep equivalent package-manager or environment paths parallel when the project presents them as equal options.
- On GitHub surfaces that autolink same-repository references, prefer bare `#…` references for issues and pull requests when user or repository convention permits them. Reference Discussions by URL, using descriptive Markdown link text when the destination would otherwise be unclear. Use an explicit link for cross-repository, ambiguous, or off-platform references.

## Validate the result

As part of the entrypoint’s [final validation](../SKILL.md#use-the-standard-path), read the complete rendered path from the opening statement through the first useful result or decision, then through optional detail. Confirm that headings reveal the hierarchy, links still reach the intended targets, and durable details follow canonical ownership in the global “Documentation” policy.
