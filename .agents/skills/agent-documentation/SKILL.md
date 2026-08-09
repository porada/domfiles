---
name: agent-documentation
description: Edit, review, audit, and maintain agent documentation in software projects, including `AGENTS.md`, `.agents/PROJECT.md`, and project-authored `.agents/skills/*` content. Use for agent-documentation authority, ownership, composition, routing, redundancy, and token-efficiency work when the project uses this model or has no established model. Defer to a more specific project agent-documentation workflow when one exists. Do not use for consumer documentation, release notes, public API documentation, or source comments alone.
---

# Project agent documentation

Apply every applicable global and project instruction. Treat project instructions and a more specific project agent-documentation workflow as authoritative over this fallback.

## Apply the documentation principles

- Assign each in-scope policy, fact, rationale, or workflow one canonical owner before editing related agent documentation. Use links rather than maintaining synchronized paraphrases.
- Preserve requested scope and every applicable read-only, approval, mutation, and submission boundary. Findings alone never authorize edits.
- Write instructions that require no conversational context. Define non-obvious terms, and keep consuming-project documentation independent of this skill, its canonical repository, and its installation path.
- Keep guidance needed by most invocations on the direct path. Defer a coherent conditional rule set only when the saved routine context outweighs the added navigation.
- Remove duplicated normative guidance while retaining safety-critical summaries, routing context, surface-specific applications, examples, and declarative rationale.

## Resolve the local documentation model

1. Read every applicable `AGENTS.md` file before evaluating other project documentation.
2. Identify the project’s agent-documentation surfaces, authority model, project-specific documentation skills, and skill-management metadata.
3. When a more specific project agent-documentation workflow exists, follow it instead of this skill’s fallback composition rules.
4. Use a locally defined authority or ownership model when one exists. When an ownership decision is required and the project has not defined a model, read the [default authority model](references/default-authority-model.md).
5. Do not create missing documentation layers merely to reproduce the default model. Add a surface only when the requested change requires and authorizes it.
6. Treat managed, vendored, generated, or third-party skills as outside project-authored documentation unless the user explicitly includes them and applicable project instructions permit the work.

## Choose the workflow

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
- Let rationale explain why a policy exists and link to its owner without repeating maintenance steps or exact inventory.
- Keep `PROJECT.md` declarative and organized under broad second-level sections. Move agent actions, reporting exclusions, and workflows to the applicable `AGENTS.md` or domain skill, leaving facts, constraints, maintenance decisions, and rationale in `PROJECT.md`.
- Keep skill descriptions limited to capability, triggering, and invocation language. Remove body text that only repeats why a skill loaded. When a description advertises review or audit, define an explicit read-only branch in the body that follows the workflow precedence above.
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

- For a change, state the canonical owner selected, the redundant definitions removed or replaced with links, and the validation performed.
- For a review, lead with concrete findings, their evidence, the canonical owner, and the suggested fix. Do not edit files or return a rewritten replacement unless the user requests a change.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
