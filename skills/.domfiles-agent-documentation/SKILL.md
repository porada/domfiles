---
name: agent-documentation
description: |-
    Use for editing, reviewing, auditing, or maintaining agent documentation and the observable contracts of project-authored skill scripts, including `AGENTS.md`, `.agents/PROJECT.md`, project-authored skill documentation under `.agents/skills/` and `skills/`, relay assets, reusable script interfaces and artifacts, and adjacent contract tests. Also use for agent-documentation authority, ownership, composition, routing, redundancy, token efficiency, skill categories, and skill-script contract architecture.

    Defer to a more specific project agent-documentation workflow when one exists.

    Do not use for consumer documentation, release notes, public API documentation, ordinary project source unrelated to skill infrastructure, or source comments alone.

metadata:
    internal: true
---

# Project Agent Documentation and Skill Scripts

## Apply the Documentation Principles

- Preserve requested scope and every applicable read-only, approval, mutation, and submission boundary. Findings alone never authorize edits.
- Write instructions that require no conversational context. Define non-obvious terms, and keep consuming-project documentation independent of this skill, its canonical repository, and its installation path.
- Apply `human-facing-writing` whenever authoring, reviewing, auditing, or maintaining any project-authored agent-documentation writing surface or any human-facing writing in an asset owned by that documentation. Preserve the agent-documentation contract and exact machine-readable, externally owned, and quoted content. Treat this as source-authoring composition rather than an installed runtime dependency. When maintaining `human-facing-writing` itself, apply the composition once without routing recursively.
- Optimize the complete context path loaded for a task rather than an individual file’s size. Treat applicable `AGENTS.md` files, skill descriptions, and `SKILL.md` entrypoints as direct-path context. Keep wording there only when most invocations need it, and move coherent conditional detail into a conditional reference in the existing skill when the saved direct-path context exceeds the navigation cost.
- Before deferring a section, separate and rehome the rules that only look domain-specific. Ensure the destination skill’s description states every applicability condition supported clients must recognize, including conditions encountered during a task, and keep any prerequisite needed to recognize those conditions on the surface that always loads. Retain an explicit route only for deliberate skill composition or when the description cannot carry the required condition.
- Weigh a deferral against its own overhead. A new skill’s description loads in every session, so deferring content that does not clearly exceed the description it requires costs more than it saves. Move smaller conditional detail into a reference of an existing skill instead.
- Give each proposition one canonical definition and classify every secondary occurrence as routing, surface-specific application, rationale, example, or required standalone context. Remove a secondary occurrence when it merely paraphrases the definition. Keep it only when its distinct role requires wording at that surface, using the smallest wording that preserves that role. When a secondary occurrence contains a more complete rule than its expected owner, promote the complete rule to the canonical owner before removing or reducing the secondary copy.
- Keep the global **Writing** policy’s Zed-specific **Numbering** rule in the global instructions rather than mirroring it into public skills.
- Give any identifier scheme referenced from code or documentation, such as numbered checks or requirement labels, one canonical definition in the same repository. Drop the identifiers when no such definition exists, because a reader cannot resolve the reference or tell which members are missing.

## Keep Distributed Skill Links Installation-Safe

For a skill with a supported installation outside its canonical repository:

- Keep relative links within the installed skill directory. Link to a sibling only when every supported installation guarantees that sibling and the same relative path resolves from the canonical source and every installation. Otherwise refer to the sibling by its frontmatter `name` without a Markdown link.
- Do not use relative links that leave the installed skills root or target a client-specific global-instruction path. Refer to an already-loaded global policy by its stable policy or section name instead.
- When authoring, reviewing, auditing, or maintaining a public skill or a project-authored asset that canonically supplies a public-skill surface, follow the [public skill portability contract](references/public-skill-portability.md).

## Resolve the Local Documentation Model

1. Read every applicable `AGENTS.md` file before evaluating other project documentation.
2. Identify the project’s agent-documentation surfaces, authority model, project-specific documentation skills, and skill-management metadata.
3. Use a locally defined authority or ownership model when one exists. When an ownership decision is required and the project has not defined a model, read the [default authority model](references/default-agent-documentation-authority-model.md).
4. Treat managed, vendored, generated, or third-party skills as outside project-authored documentation unless the user explicitly includes them and applicable project instructions permit the work.

## Choose the Workflow

- Before changing any project-authored path under `.agents/skills` or `skills/human-facing-writing`, resolve its canonical owner. Follow the [protected skill mutation policy](references/protected-skill-mutation.md) before every mutation under `skills/human-facing-writing` and, when your agent identity is Zed Agent, under `.agents/skills`. Keep standalone reviews and audits read-only.
- When a task creates, revises, reviews, audits, or maintains a relay or decision-capture prompt, load `agent-task-relay` before resolving its canonical owner or composing it.
- When a task adds, changes, reviews, or audits a reusable script or adjacent contract test owned by a project-authored skill, read the [skill-owned script policy](references/skill-owned-scripts.md) before planning the work. Agent documentation owns whether the script belongs, its canonical owner and location, observable interface, operation modes, side effects, artifacts and schemas, documented invocation, and required contract coverage. The owning project skill supplies domain semantics within those boundaries.
- When authoring, reviewing, auditing, or maintaining project-authored skill documentation, changing a skill’s category, or maintaining its supported installation reach, follow the [skill category maintenance rules](references/skill-category-maintenance.md). Keep each project-authored skill’s frontmatter `name` identical to its discovery name. For internal and public skills, this is the canonical directory basename. For global skills, omit the canonical source directory’s `.domfiles-` prefix so the name matches every final symlink basename.
- For a documentation-only or script-contract review or audit, inspect implementation and adjacent tests only as bounded evidence for a specific observable contract, then stop once the claim is established. Do not assess algorithms, internal structure, language idioms, performance, dead code, duplication, or general test quality unless the user explicitly includes implementation. Evaluation criteria such as security, maintainability, or project values apply within the resolved scope and do not expand it.
- When the resolved scope explicitly includes implementation, follow applicable project, domain, and language implementation and validation workflows for internal concerns. Keep the agent-documentation pass focused on contract consequences, and update agent documentation only when the contract, routing, or documented invocation changes.
- For an explicit change, including a request that also uses review or audit language, use the change workflow. Treat inspection as the evidence-gathering phase, then resolve the canonical owner, compose the change, and validate the final contents.
- For a standalone ordinary review, keep the task read-only. Resolve the canonical owner and validate the existing contents, but skip composition, formatting, and every mutation.
- For a standalone audit, keep the task read-only. Follow an applicable model-invocable project audit workflow when one exists. Otherwise start from Git-tracked paths, add only explicitly named untracked documentation when local policy permits it, inspect the resolved documentation scope, report findings, and stop without formatting or mutation.
- When reviewing a change against a file-scoped policy such as naming or prose punctuation, evaluate the complete file rather than the diff alone. A diff-only pass reports whichever violations the change happened to touch and silently accepts the rest.
- Resolve each pass’s scope before applying project and domain skills. Let supported clients discover them from their descriptions, and do not preload skills for later passes.

## Resolve the Canonical Owner

1. State the durable detail being changed or evaluated in one sentence.
2. Use the selected local or fallback authority model to identify its expected owner.
3. Search applicable project-authored agent documentation for existing definitions, rationale, inventories, and links concerning that detail.
4. Name one canonical owner and identify every other document that should link to it or remove a stale paraphrase.
5. Do not edit until one owner can be named. Do not choose an owner merely because the detail already appears there.

## Compose the Change

- Update the selected canonical owner before adjusting secondary documents.
- When a set’s order encodes information a reader must recover, such as precedence, priority, complexity, or containment, state its ordering principle beside the set or in the rule that governs it, so every new entry has a determinable position. This applies to table rows, category sequences, and section sequences alike. Agent-documentation authority tables use one canonical principle instead, listing instruction surfaces before reference surfaces, each from the most general to the most specific, with a client bridge following the surface it imports.
- Preserve exact user terminology only when the terminology itself is required or established.
- Before compressing a rule or introducing a case beside it, identify what carries its scope. Labels, unqualified quantifiers, and the range of cases that existed when it was written all bind meaning without reading as conditions. State the scope explicitly whenever the edit changes any of them.
- Phrase reusable guidance against the subject’s declared contract rather than a presumed success model. Do not assume that “completion” means failure-free execution, that a stable identifier proves unchanged state, or that a wrapper must reproduce an underlying tool’s automatic behavior.
- In project-authored skill documentation, include a code example only when it clarifies one contract the skill owns. State the owned invariant in prose, and keep the behavior that demonstrates it within that domain. A realistic example may consume a verified external interface and state only the behavior needed to establish that premise. Use a domain-neutral operation for unrelated incidental behavior, and route any external behavior the example must teach, validate, or implement to its owner. Prefer removing the example over expanding it into a complete cross-domain workflow.
- Let rationale explain why a policy exists and link to its owner without repeating maintenance steps or exact inventory.
- Keep `PROJECT.md` declarative and organized under broad second-level sections. Move agent actions, reporting exclusions, and workflows to the applicable `AGENTS.md` or domain skill, leaving facts, constraints, maintenance decisions, and rationale in `PROJECT.md`.
- Treat a project-authored skill as **command-only** when it sets `disable-model-invocation: true`, and as **model-invocable** otherwise.
- For every model-invocable project-authored skill, preserve complete positive and negative trigger boundaries and precedence conditions, and state them as direct reasons to load the skill. Include a peer name only when it defines one such condition. Do not make loading depend on whether the task already exhibits a quality the skill enforces, such as being “focused,” “simple,” “safe,” or “bounded.”
- For every command-only project-authored skill, write concise human-facing UI copy instead. Lead with the action and outcome, add only task scope or exclusions that distinguish the command, and omit the invocation condition and literal slash command.
- Encode every project-authored skill description as a YAML `|-` literal block scalar. Treat each thematic unit as a paragraph, and separate paragraphs with a blank line. When another frontmatter field follows the description, place one blank line between the description value and that field.
- For internal and global skills, keep workflow, operational details, rationale, and implementation details in the body. In model-invocable descriptions, also move capability exposition, behavior, and outputs to the body, and remove body text that only repeats why the skill loaded.
- For public skills in either invocation mode, also apply the [public-description portability contract](references/public-skill-portability.md#keep-public-descriptions-portable).
- For every project-authored skill description, keep the decoded value within the strictest limit any supported client imposes, treating 1,024 UTF-8 bytes as that limit unless a supported client documents a stricter one. Measure the decoded value rather than the complete frontmatter or source line. Shorten redundant phrasing before dropping trigger distinctions, and validate every project-authored skill description after changing this policy or any description. When a description advertises review or audit, define an explicit read-only branch in the body that follows the workflow precedence above.
- For agent documentation about a versioned tool, runtime, or language, establish one authoritative behavioral baseline before editing. Use its most recent stable release unless direct user instruction or authoritative target-environment evidence establishes another version. For any newly authored or materially revised security-boundary claim, record the exact upstream revision together with one or more upstream source-file paths that establish it. Evaluate conflicting evidence against the selected baseline before changing documentation. Do not combine current documentation, pinned source, and upstream `main` as if they describe one implementation. Write only the interfaces, semantics, and syntax of the selected baseline. Do not add compatibility branches, historical caveats, legacy forms, version detection, or version-migration guidance. Report when the baseline cannot be verified rather than guessing.
- Treat two skills matching one task as ordinary composition. Narrow a description only when the skills state contradictory rules for the same decision or duplicate one normative rule, and prefer a deferral clause naming the sibling over an exclusion that removes the surface. An exclusion that ends a correct overlap fails silently, because the skill simply stops loading.
- For a new, renamed, or rewritten project-authored Markdown document without a required filename, let its content and scope determine the top-level title, then derive the filename as `<lower-kebab-case>.md`. Never choose or rewrite a title to preserve an existing filename. If the current filename does not match the resulting title, rename the file and update every inbound link in the same change. A filename required by a client, tool, ecosystem, document format, or more specific contract takes precedence.
- Keep each `SKILL.md` as an entrypoint. Keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves.
- Before composing a change across routed or layered surfaces, use the [documentation boundary checks](references/documentation-boundary-checks.md) to identify the canonical side of each boundary.

## Validate the Documentation

### Run the Complete-Scope Checks

For every change, review, or audit:

1. Reread every applicable `AGENTS.md` file and each in-scope documentation file that the current task has not already loaded unchanged. Use Git status and diff to identify what changed since it was loaded.
2. Search the complete applicable documentation family for each proposition being changed or evaluated, its distinctive wording, and close semantic variants. Apply the [documentation boundary checks](references/documentation-boundary-checks.md) to routed or layered surfaces. When a global instruction changes or is evaluated, identify every affected public-skill mirror, including rephrased variants, and apply the [public skill portability contract](references/public-skill-portability.md) to each. Confirm that one normative definition remains and that every secondary occurrence has a distinct required role or links to the canonical owner.
3. Verify every relevant project-relative link, heading anchor, and skill frontmatter name. For a skill with a supported installation outside its canonical repository, resolve every relative link from each supported installation root and reject links that escape the installed skills tree or target an unavailable peer. Before renaming or deleting a heading, search project-authored agent documentation for links to its current anchor and update those links in the same change or preserve the heading.
4. For every in-scope change to a direct-path surface, compare its before-and-after context footprint. In a review or audit, report unjustified growth without editing.
5. When an in-scope change moved content between files or surfaces, recheck every reference that resolved through its old location, including directional prose such as “above” and “below,” phrases naming the containing document, and terms defined only in the origin. Apply any policy the destination surface newly imposes.
6. When an in-scope change moved guidance from a `SKILL.md` into references, map every removed proposition to its destination and confirm that every task that previously received it still deterministically loads that destination. Treat a missing route, condition, exception, or behavioral distinction as a contract regression.

For a review or audit, use only read-only diagnostics and identify anything that could not be verified.

### Complete Change Validation

After capturing all task-authorized documentation updates intended for the current change:

1. Resolve every unjustified direct-path increase found by the complete-scope footprint check. Move conditional guidance into a reference in the existing skill, and remove obsolete direct-path wording in the same change.
2. Resolve every stale reference found by the complete-scope moved-content check, including directional prose, phrases naming the containing document, and terms defined only in the origin.
3. Resolve every missing route, condition, exception, or behavioral distinction found by the complete-scope moved-guidance check.
4. Run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
5. Perform one bounded final alignment pass over the changed documentation against the [documentation principles](#apply-the-documentation-principles), the resolved local authority model, applicable project values, and explicit user decisions. Correct concrete discrepancies within the authorized scope before delivery. Treat this as a completion check rather than a drafting gate: do not withhold useful documentation, reopen settled decisions, repeatedly rewrite compliant content, or expand scope for speculative improvements. If a correction requires new authorization, preserve the completed changes and report that boundary.

## Report the Result

- For a change, identify the canonical owner and any redundant definitions removed or replaced with links. Follow the applicable communication policy for validation reporting.
- For a review or audit, lead with concrete findings, their evidence, the canonical owner, and the suggested fix.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
