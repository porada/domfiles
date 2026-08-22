---
name: agent-documentation
description: Edit, review, audit, and maintain agent documentation and the observable contracts of skill scripts, including `AGENTS.md`, `.agents/PROJECT.md`, project-authored `.agents/skills/*` and `skills/*` documentation, prompt-relay assets, reusable script interfaces and artifacts, and adjacent contract tests. Use for agent-documentation authority, ownership, composition, routing, redundancy, token efficiency, and skill-script contract architecture. Load it for project-authored skill-script work, but defer internal implementation quality to the applicable language or domain workflow and relay composition to `prompt-relays`. Defer to a more specific project agent-documentation workflow when one exists. Do not use for consumer documentation, release notes, public API documentation, ordinary project source unrelated to skill infrastructure, or source comments alone.
metadata:
    internal: true
---

# Project agent documentation and skill scripts

## Apply the documentation principles

- Preserve requested scope and every applicable read-only, approval, mutation, and submission boundary. Findings alone never authorize edits.
- Write instructions that require no conversational context. Define non-obvious terms, and keep consuming-project documentation independent of this skill, its canonical repository, and its installation path.
- Optimize the complete context path loaded for a task rather than an individual file’s size. Treat applicable `AGENTS.md` files, skill descriptions, and `SKILL.md` entrypoints as direct-path context. Keep wording there only when most invocations need it, and move coherent conditional detail into a conditional reference in the existing skill when the saved direct-path context exceeds the navigation cost.
- Before deferring a section, separate and rehome the rules that only look domain-specific. Retain an explicit route on the surface that still loads, and keep any rule the route depends on conditional there, because a skill description is not a deterministic trigger for policy the agent initiates rather than the user.
- Weigh a deferral against its own overhead. A new skill’s description loads in every session, so deferring content that does not clearly exceed the description it requires costs more than it saves. Move smaller conditional detail into a reference of an existing skill instead.
- Give each proposition one canonical definition and classify every secondary occurrence as routing, surface-specific application, rationale, example, or required standalone context. Remove a secondary occurrence when it merely paraphrases the definition. Keep it only when its distinct role requires wording at that surface, using the smallest wording that preserves that role.
- Give any identifier scheme referenced from code or documentation, such as numbered checks or requirement labels, one canonical definition in the same repository. Drop the identifiers when no such definition exists, because a reader cannot resolve the reference or tell which members are missing.

## Keep distributed skill links installation-safe

For a skill with a supported installation outside its canonical repository:

- Keep relative links within the installed skill directory. Link to a sibling only when every supported installation guarantees that sibling and the same relative path resolves from the canonical source and every installation. Otherwise refer to the sibling by its frontmatter `name` without a Markdown link.
- Do not use relative links that leave the installed skills root or target a client-specific global-instruction path. Refer to an already-loaded global policy by its stable policy or section name instead.
- When authoring, reviewing, or maintaining a public skill, follow the [public skill portability contract](references/public-skill-portability.md).

## Resolve the local documentation model

1. Read every applicable `AGENTS.md` file before evaluating other project documentation.
2. Identify the project’s agent-documentation surfaces, authority model, project-specific documentation skills, and skill-management metadata.
3. Use a locally defined authority or ownership model when one exists. When an ownership decision is required and the project has not defined a model, read the [default authority model](references/default-authority-model.md).
4. Treat managed, vendored, generated, or third-party skills as outside project-authored documentation unless the user explicitly includes them and applicable project instructions permit the work.

## Choose the workflow

- Before changing any project-authored path under `.agents/skills`, resolve its canonical owner, then follow the [protected skill mutation policy](references/protected-skill-staging.md) before mutation. Keep standalone reviews and audits read-only.
- When a task creates, revises, reviews, audits, or maintains a prompt relay or capture prompt, load `prompt-relays` before resolving its canonical owner or composing it.
- When a task adds, changes, reviews, or audits a reusable script or adjacent contract test owned by a project-authored skill, read the [skill-owned script policy](references/skill-owned-scripts.md) before planning the work. Agent documentation owns whether the script belongs, its canonical owner and location, observable interface, operation modes, side effects, artifacts and schemas, documented invocation, and required contract coverage. The owning project skill supplies domain semantics within those boundaries.
- For a documentation-only or script-contract review or audit, inspect implementation and adjacent tests only as bounded evidence for a specific observable contract, then stop once the claim is established. Do not assess algorithms, internal structure, language idioms, performance, dead code, duplication, or general test quality unless the user explicitly includes implementation. Evaluation criteria such as security, maintainability, or project values apply within the resolved scope and do not expand it.
- When the resolved scope explicitly includes implementation, follow applicable project, domain, and language implementation and validation workflows for internal concerns. Keep the agent-documentation pass focused on contract consequences, and update agent documentation only when the contract, routing, or documented invocation changes.
- For an explicit change, including a request that also uses review or audit language, use the change workflow. Treat inspection as the evidence-gathering phase, then resolve the canonical owner, compose the change, and validate the final contents.
- For a standalone ordinary review, keep the task read-only. Resolve the canonical owner and validate the existing contents, but skip composition, formatting, and every mutation.
- For a standalone audit, keep the task read-only. Follow an applicable project audit workflow when one exists. Otherwise start from Git-tracked paths, add only explicitly named untracked documentation when local policy permits it, inspect the resolved documentation scope, report findings, and stop without formatting or mutation.
- When reviewing a change against a file-scoped policy such as ordering, naming, or prose punctuation, evaluate the complete file rather than the diff alone. A diff-only pass reports whichever violations the change happened to touch and silently accepts the rest.
- Load applicable project and domain skills according to local routing immediately before the pass that requires them.

## Resolve the canonical owner

1. State the durable detail being changed or evaluated in one sentence.
2. Use the selected local or fallback authority model to identify its expected owner.
3. Search applicable project-authored agent documentation for existing definitions, rationale, inventories, and links concerning that detail.
4. Name one canonical owner and identify every other document that should link to it or remove a stale paraphrase.
5. Do not edit until one owner can be named. Do not choose an owner merely because the detail already appears there.

## Compose the change

- Update the selected canonical owner before adjusting secondary documents.
- When a set’s order encodes information a reader must recover, such as precedence, priority, complexity, or containment, state its ordering principle beside the set or in the rule that governs it, so every new entry has a determinable position. This applies to table rows, category sequences, and section sequences alike. Agent-documentation authority tables use one canonical principle instead, listing instruction surfaces before reference surfaces, each from the most general to the most specific, with a client bridge following the surface it imports.
- Preserve exact user terminology only when the terminology itself is required or established.
- Before compressing a rule or introducing a case beside it, identify what carries its scope. Labels, unqualified quantifiers, and the range of cases that existed when it was written all bind meaning without reading as conditions. State the scope explicitly whenever the edit changes any of them.
- Let rationale explain why a policy exists and link to its owner without repeating maintenance steps or exact inventory.
- Keep `PROJECT.md` declarative and organized under broad second-level sections. Move agent actions, reporting exclusions, and workflows to the applicable `AGENTS.md` or domain skill, leaving facts, constraints, maintenance decisions, and rationale in `PROJECT.md`.
- Keep each skill description limited to capability, triggering, exclusions, and essential routing language, and within the strictest description limit any supported client imposes. Treat 1,024 UTF-8 bytes as that limit unless a supported client documents a stricter one. Measure the decoded description value rather than the complete frontmatter or source line. Shorten redundant phrasing before dropping trigger distinctions, and validate every project-authored skill description after changing this policy or any description. Remove body text that only repeats why a skill loaded. When a description advertises review or audit, define an explicit read-only branch in the body that follows the workflow precedence above.
- Treat two skills matching one task as ordinary composition. Narrow a description only when the skills state contradictory rules for the same decision or duplicate one normative rule, and prefer a deferral clause naming the sibling over an exclusion that removes the surface. An exclusion that ends a correct overlap fails silently, because the skill simply stops loading.
- Keep each project-authored skill’s frontmatter `name` identical to its discovery name. For internal and public skills, this is the canonical directory basename. For global skills, omit the canonical source directory’s `domfiles-` prefix so the name matches every final symlink basename.
- Keep each `SKILL.md` as an entrypoint. Keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves.
- Before composing a change across routed or layered surfaces, use the [documentation boundary checks](references/documentation-boundaries.md) to identify the canonical side of each boundary.

## Validate the documentation

After capturing all task-authorized documentation updates intended for the current change, perform one bounded final alignment pass over the changed documentation against the [documentation principles](#apply-the-documentation-principles), the resolved local authority model, applicable project values, and explicit user decisions. Correct concrete discrepancies within the authorized scope before delivery. Treat this as a completion check rather than a drafting gate: do not withhold useful documentation, reopen settled decisions, repeatedly rewrite compliant content, or expand scope for speculative improvements. If a correction requires new authorization, preserve the completed changes and report that boundary.

1. Reread every applicable `AGENTS.md` file and each in-scope documentation file that the current task has not already loaded unchanged. Use Git status and diff to identify what changed since it was loaded.
2. Search the complete applicable documentation family for each changed proposition, its distinctive wording, and close semantic variants. Apply the [documentation boundary checks](references/documentation-boundaries.md) to routed or layered surfaces. When a global instruction changes, search for verbatim public-skill mirrors and apply the [public skill portability contract](references/public-skill-portability.md) to each. Confirm that one normative definition remains and that every secondary occurrence has a distinct required role or links to the canonical owner.
3. For every changed direct-path surface, compare its before-and-after context footprint. Any increase must be required by most invocations or necessary to route conditional guidance. In a change workflow, move unjustified growth into a conditional reference in the existing skill and remove obsolete direct-path wording in the same change. In a review workflow, report unjustified growth without editing.
4. Verify every relevant project-relative link, heading anchor, and skill frontmatter name. For a skill with a supported installation outside its canonical repository, resolve every relative link from each supported installation root and reject links that escape the installed skills tree or target an unavailable peer. Before renaming or deleting a heading, search project-authored agent documentation for links to its current anchor and update those links in the same change or preserve the heading.
5. When the task moved content between files or surfaces, recheck every reference that resolved through its old location, including directional prose such as “above” and “below,” phrases naming the containing document, and terms defined only in the origin. Apply any policy the destination surface newly imposes.
6. For a change, run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
7. For a review, use only read-only diagnostics and identify anything that could not be verified.

For a follow-up `Verify` request, execute the applicable global `Verify` procedure.

## Report the result

- For a change, identify the canonical owner and any redundant definitions removed or replaced with links. Follow the applicable communication policy for validation reporting.
- For a review, lead with concrete findings, their evidence, the canonical owner, and the suggested fix.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
