---
name: domfiles-documentation
description: Edit, review, and maintain agent documentation in domfiles. Use this skill whenever the resolved task scope includes `.config/zed/AGENTS.md`, `AGENTS.md`, `.agents/PROJECT.md`, or content under `.agents/skills/*/`, including authority, ownership, composition, routing, redundancy, or token-efficiency changes. Do not use it for consumer documentation or source comments alone.
---

# Domfiles documentation

Select one canonical owner through the [agent-documentation authority table](../../../AGENTS.md#agent-documentation) before editing. Do not reproduce that table or paraphrase its ownership rules here.

Apply the global [documentation](../../../.config/zed/AGENTS.md#documentation) and [writing](../../../.config/zed/AGENTS.md#writing) rules by reference rather than copying them into this skill.

## Choose the workflow

- For a documentation change, resolve the canonical owner, compose the change, and validate the final contents.
- For an ordinary review, keep the task read-only. Resolve the canonical owner and validate the existing contents, but skip composition, formatting, and every mutation.
- For an audit, follow the [repository audit process](../domfiles-repository-audit/SKILL.md) and apply this skill only as domain-specific policy. Do not create a parallel audit workflow.
- Continue to load applicable domain skills through the repository’s [skill routing](../../../AGENTS.md#skills). Let those skills determine technical policy while this skill determines documentation placement and composition.

## Resolve the canonical owner

1. State the durable detail being changed or evaluated in one sentence.
2. Use the authority table to identify its expected owner.
3. Search tracked agent documentation for existing definitions, rationale, inventories, and links concerning that detail.
4. Name one canonical owner and identify every other document that should link to it or remove a stale paraphrase.
5. Do not edit until one owner can be named. Do not choose an owner merely because the detail already appears there.

## Compose the change

- Update the canonical owner first. In other documents, replace normative paraphrases with direct links and delete obsolete duplication instead of synchronizing multiple definitions.
- Keep each documentation layer within the owner selected by the authority table. Rationale may explain why a policy exists and link to it, but must not repeat its maintenance steps or exact inventory.
- Keep PROJECT declarative and organized under broad second-level sections. Move agent actions, reporting exclusions, and workflows to the applicable `AGENTS.md` or domain skill, leaving facts, constraints, maintenance decisions, and rationale in PROJECT.
- Keep skill descriptions limited to capability, triggering, and invocation language. Remove body text that only repeats why a skill loaded. When a description advertises review or audit, define an explicit read-only branch in the body.
- Keep always-loaded context lean by keeping project or domain policy out of global instructions. Treat each `SKILL.md` as an entrypoint, and move detailed material needed by only some workflows into canonical references linked at the decision that requires them.

## Validate the documentation

1. Reread every applicable `AGENTS.md` file and each in-scope documentation file.
2. Search tracked agent documentation for the affected identifiers and concepts. Confirm that one normative definition remains and every secondary mention links to it.
3. Verify every relevant relative link, heading anchor, and skill frontmatter name. Before renaming or deleting a heading, search tracked agent documentation for links to its current anchor and update those links in the same change or preserve the heading.
4. For a change, run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
5. For a review, use only read-only diagnostics and identify anything that could not be verified.

For a follow-up verification request, execute the global [`Verify`](../../../.config/zed/AGENTS.md#verify) macro.

## Report the result

- For a change, state the canonical owner selected, the redundant definitions removed or replaced with links, and the validation performed.
- For a review, lead with concrete findings, their evidence, the canonical owner, and the suggested fix. Do not edit files or return a rewritten replacement unless the user requests a change.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
