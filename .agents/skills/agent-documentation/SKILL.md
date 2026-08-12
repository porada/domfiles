---
name: agent-documentation
description: Edit, review, audit, and maintain agent documentation and project-authored skill scripts in software projects, including `AGENTS.md`, `.agents/PROJECT.md`, project-authored `.agents/skills/*` documentation, prompt-relay assets, and reusable scripts with adjacent tests owned by those skills. Use for agent-documentation authority, ownership, composition, routing, redundancy, token efficiency, and skill-owned script architecture. Defer to a more specific project agent-documentation workflow when one exists. Do not use for consumer documentation, release notes, public API documentation, ordinary project source unrelated to skill infrastructure, or source comments alone.
---

# Project agent documentation and skill scripts

Apply every applicable global and project instruction. Treat project instructions and a more specific project agent-documentation workflow as authoritative over this fallback.

## Apply the documentation principles

- Preserve requested scope and every applicable read-only, approval, mutation, and submission boundary. Findings alone never authorize edits.
- Write instructions that require no conversational context. Define non-obvious terms, and keep consuming-project documentation independent of this skill, its canonical repository, and its installation path.
- Keep guidance needed by most invocations on the direct path. Defer a coherent conditional rule set only when the saved routine context outweighs the added navigation.
- Remove duplicated normative guidance while retaining safety-critical summaries, routing context, surface-specific applications, examples, and declarative rationale.

## Resolve the local documentation model

1. Read every applicable `AGENTS.md` file before evaluating other project documentation.
2. Identify the project’s agent-documentation surfaces, authority model, project-specific documentation skills, and skill-management metadata.
3. Use a locally defined authority or ownership model when one exists. When an ownership decision is required and the project has not defined a model, read the [default authority model](references/default-authority-model.md).
4. Treat managed, vendored, generated, or third-party skills as outside project-authored documentation unless the user explicitly includes them and applicable project instructions permit the work.

## Choose the workflow

- When a task creates, revises, reviews, audits, or maintains a prompt relay or capture prompt, read the [prompt relay policy](references/prompt-relays.md) before resolving its canonical owner or composing it.
- When a task adds, changes, reviews, or audits a reusable script or adjacent test owned by a project-authored skill, read the [skill-owned script policy](references/skill-owned-scripts.md) before planning the work. For a change, follow applicable project and language implementation and validation workflows. Update agent documentation only when the script contract, routing, or documented invocation changes.
- An explicit request to change agent documentation uses the change workflow even when it also asks for a review or audit. Treat inspection as the evidence-gathering phase, then resolve the canonical owner, compose the change, and validate the final contents.
- For a standalone ordinary review, keep the task read-only. Resolve the canonical owner and validate the existing contents, but skip composition, formatting, and every mutation.
- For a standalone audit, keep the task read-only. Follow an applicable project audit workflow when one exists. Otherwise start from Git-tracked paths, add only explicitly named untracked documentation when local policy permits it, inspect the resolved documentation scope, report findings, and stop without formatting or mutation.
- Load applicable project and domain skills according to local routing immediately before the pass that requires them.

## Resolve the canonical owner

1. State the durable detail being changed or evaluated in one sentence.
2. Use the selected local or fallback authority model to identify its expected owner.
3. Search applicable project-authored agent documentation for existing definitions, rationale, inventories, and links concerning that detail.
4. Name one canonical owner and identify every other document that should link to it or remove a stale paraphrase.
5. Do not edit until one owner can be named. Do not choose an owner merely because the detail already appears there.

## Compose the change

- Update the selected canonical owner before adjusting secondary documents.
- Preserve exact user terminology only when the terminology itself is required or established.
- Write commit references as abbreviated hashes, following the repository’s established length. Use a full object ID only when disambiguation or an external format requires it.
- Let rationale explain why a policy exists and link to its owner without repeating maintenance steps or exact inventory.
- Keep `PROJECT.md` declarative and organized under broad second-level sections. Move agent actions, reporting exclusions, and workflows to the applicable `AGENTS.md` or domain skill, leaving facts, constraints, maintenance decisions, and rationale in `PROJECT.md`.
- Keep each skill description at most 1,024 UTF-8 bytes and limited to capability, triggering, exclusions, and essential routing language. Measure the decoded description value rather than the complete frontmatter or source line. Shorten redundant phrasing before dropping trigger distinctions, and validate every project-authored skill description after changing this policy or any description. Remove body text that only repeats why a skill loaded. When a description advertises review or audit, define an explicit read-only branch in the body that follows the workflow precedence above.
- Keep each project-authored skill’s frontmatter `name` identical to its directory name.
- Keep each `SKILL.md` as an entrypoint. Keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves.

## Validate the documentation

1. Reread every applicable `AGENTS.md` file and each in-scope documentation file.
2. Search applicable project-authored agent documentation for the affected identifiers and concepts. Confirm that one normative definition remains and every secondary mention links to it.
3. Verify every relevant project-relative link, heading anchor, and skill frontmatter name. Before renaming or deleting a heading, search project-authored agent documentation for links to its current anchor and update those links in the same change or preserve the heading.
4. For a change, run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
5. For a review, use only read-only diagnostics and identify anything that could not be verified.

For a follow-up `Verify` request, execute the applicable global `Verify` procedure.

## Report the result

- For a change, identify the canonical owner and any redundant definitions removed or replaced with links. Follow the applicable communication policy for validation reporting.
- For a review, lead with concrete findings, their evidence, the canonical owner, and the suggested fix.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
