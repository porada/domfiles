# Technical Documents

## Scope

Interpret the request literally. A content-only request doesn’t authorize structural changes, and a structure-only request doesn’t authorize rewriting otherwise sound copy. Heading wording counts as copy. During structure-only work, headings may move or change level, but they may not be renamed or newly written.

Preserve every existing heading’s exact identifier spelling, significant characters, entities, and intentional markup. Leave an unchanged heading out of a structural outline rather than normalizing it. Do not introduce non-breaking characters, entities, or presentational markup unless project policy or the user requires them.

Apply the entrypoint’s [editorial boundaries](../SKILL.md#editorial-boundaries) to selected or restored headings, section order, taglines, links, examples, wording, and structure.

## Document Structure

Start with any supplied template. Its sections determine where each kind of information belongs. When a template separates the proposal, rationale, evidence, examples, and implementation ideas, keep the proposal’s opening to the smallest complete statement of the requested outcome and its essential boundary. Put each supporting detail in its designated section.

After the opening appropriate to the document type, arrange the remaining content in increasing depth. Move from the first useful action or conclusion through evidence, setup, core behavior, optional detail, caveats, alternatives, reference material, troubleshooting, or questions as the document requires. Keep optional variants subordinate to the primary path. Use direct question headings when a question-and-answer format helps readers scan non-obvious concerns.

Treat this sequence as a decision framework, not a fixed template. A visual before-and-after example may precede setup when it communicates value faster. A short issue comment may need only the decision and its evidence. Omit any section that doesn’t help the intended reader.

### FAQ Collections

Treat an established FAQ as one question collection, and keep its question form headings in natural casing. During structure-only work, keep every existing question form heading under that FAQ rather than moving individual questions into other sections.

Don’t propose a standalone FAQ with only one question. If another grouping would split an established FAQ or leave it with one question, preserve the current collection and identify the unresolved copy decision instead of rewriting or relocating questions.

## Document Types

Before drafting or revising an issue or pull request body for a specific repository, check the supplied context or accessible repository for an applicable template. Use the template when available and the document type defaults below when none applies. If you don’t know whether a template applies, you can still draft from the available context, but don’t claim template compliance. Ask only when a known required template is unavailable or a material ambiguity prevents a useful draft.

For any security report, whether standalone or submitted through an issue or pull request form, also follow the [security report workflow](security-reports.md).

| Document Type | Default |
| --- | --- |
| Comment, reply, or code review | Lead with the answer, finding, or required action. Include only the thread context needed to support it, and distinguish required changes from optional suggestions. |
| Issue, pull request, or discussion title and body | Follow any applicable repository template. Make the problem, outcome, or scope recognizable immediately. Without a template, a small change, narrow report, or focused question may need only a sentence or two. Develop the body with evidence, impact, changes, validation, or next actions as needed for the reader to judge or act on it, not as a checklist of sections. |
| Other technical document | Follow the established document hierarchy. Lead with the document’s purpose, decision, or required action, and include only the context needed to understand it. |
| README | Open with a compact project or package identity and practical value. Move through the first useful demonstration, installation, setup, and core usage before optional configuration or questions, as applicable. Keep related projects, provenance, and license information near the end when established. Preserve intentional hero artwork, badges, and layout. |

Select collaboration process details for their value to the reader’s decision. Include validation evidence, limitations, and scope boundaries when they affect trust, review, or use. Don’t recount the workflow merely to demonstrate diligence. First-person wording can clarify who acted or holds a view, but isn’t required. Ask a question when a real decision is open, not to simulate conversation.

## Content

- In GitHub collaboration prose, prefer bare `#<number>` references for same-repository issues, pull requests, and discussions. State the relationship directly in the surrounding prose rather than wrapping descriptive text in a Markdown link. Use qualified `<owner>/<repository>#<number>` references across repositories where supported. Use explicit links for destinations or surfaces that don’t support reference shorthand.

    Use a comment or review permalink only when the reader needs that exact location, not merely because it supplied evidence during research. Keep precise research citations in the supporting evidence without automatically carrying them into the final copy.

- Keep equivalent package manager or environment paths parallel when the project presents them as equal options.
- Place prerequisites, caveats, alternatives, and evidence close to the claim or action they constrain.
- Present document fragments and heading outlines directly rather than wrapping the entire fragment in an outer code block. Preserve code blocks that belong to the document content.
- Use a callout for an operational constraint or risk that readers could otherwise miss. Place it beside the affected action, use the least severe established callout type that fits, and keep it concise. Don’t use callouts for ordinary notes or decorative emphasis.

## Validation

As part of the technical copy workflow’s [final validation](technical-copy.md#validation), read the complete rendered document from its opening through the first useful result or decision, then continue through any optional detail. Confirm that the headings reveal the hierarchy and that every link reaches its intended destination.
