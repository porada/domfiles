---
name: agent-documentation
description: |-
    Use for editing, reviewing, auditing, or maintaining project-authored agent documentation, including `AGENTS.md`, `.agents/PROJECT.md`, skill documentation, reusable handoff prompts and templates, canonical public-surface assets, and public skill READMEs. Also use for documentation authority, ownership, composition, routing, redundancy, and token efficiency.

    Defer to a more specific project agent documentation workflow when one exists.

    Do not use for ordinary project source, script-only work without documentation changes, other consumer documentation, public API documentation, release notes, or source comments alone.

metadata:
    internal: true
---

# Project Agent Documentation

## Apply Documentation Principles

- Apply `human-facing-writing` whenever authoring, reviewing, auditing, or maintaining any project-authored agent documentation writing surface or any human-facing writing in an asset owned by that documentation. Preserve the agent documentation contract and exact machine-readable, externally owned, and quoted content. Treat this as source authoring composition rather than an installed runtime dependency. When maintaining `human-facing-writing` itself, apply the composition once without routing recursively.
- Apply the global **Explicit user direction** policy to this workflow’s procedural requirements and conventions, including its routed references. Preserve requested scope and every applicable approval, mutation, security, and submission boundary.
- Before deferring a section, separate and rehome the rules that only look domain-specific. Ensure the destination skill’s description states every applicability condition supported clients must recognize, including conditions encountered during a task, and keep any prerequisite needed to recognize those conditions on the surface that always loads. Retain an explicit route only for deliberate skill composition or when the description cannot carry the required condition.
- Give any identifier scheme referenced from code or documentation, such as numbered checks or requirement labels, one canonical definition in the same repository. Drop the identifiers when no such definition exists, because a reader cannot resolve the reference or tell which members are missing.
- Give each proposition one canonical definition and classify every secondary occurrence as routing, surface-specific application, rationale, example, or required standalone context. Remove a secondary occurrence when it merely paraphrases the definition. Keep it only when its distinct role requires wording at that surface, using the smallest wording that preserves that role. When a secondary occurrence contains a more complete rule than its expected owner, promote the complete rule to the canonical owner before removing or reducing the secondary copy.
- Optimize the complete context path loaded for a task rather than an individual file’s size. Treat applicable `AGENTS.md` files, skill descriptions, and `SKILL.md` entrypoints as direct-path context. Keep wording there only when most invocations need it, and move coherent conditional detail into a conditional reference in an existing skill that owns the relevant domain when the saved direct-path context exceeds the navigation cost.
- Weigh a deferral against its own overhead. A model-invocable skill’s description adds recurring context when the client exposes it, so deferring content that does not clearly exceed that overhead costs more than it saves. Move smaller conditional detail into a reference of an existing skill instead.
- Write instructions that require no conversational context. Define non-obvious terms, and keep consuming project documentation independent of this skill, its canonical repository, and its installation path.

## Resolve Local Documentation Model

1. Read every applicable `AGENTS.md` file before evaluating other project documentation.
2. Identify the project’s agent documentation surfaces, authority model, project-specific documentation skills, and skill management metadata.
3. Use a locally defined authority or ownership model when one exists. When an ownership decision is required and the project has not defined a model, read the [Default Agent Documentation Authority Model](references/default-agent-documentation-authority-model.md).
4. Treat generated, managed, third-party, or vendored skills as outside project-authored documentation unless the user explicitly includes them and applicable project instructions permit the work.

## Choose Workflow

- When the task affects a project-authored skill’s documentation, metadata, assets, category, or supported installation, apply `skill-development` for the skill-specific contracts before the affected work. Also apply it when a global instruction changes or is evaluated and supplies a public skill mirror, even when the request names only the global source. Compose once, then continue the shared workflow here rather than restarting either skill.
- When a task creates, revises, reviews, audits, or maintains a relay or decision capture prompt, load `agent-task-relay` before resolving its canonical owner or composing it.
- For a documentation-only review or audit, inspect implementation and adjacent tests only as bounded evidence for a specific observable contract, then stop once the claim is established. Do not assess algorithms, internal structure, language idioms, performance, dead code, duplication, or general test quality unless the user explicitly includes implementation. Evaluation criteria such as security, maintainability, or project values apply within the resolved scope and do not expand it.
- When the resolved scope explicitly includes implementation, follow applicable project, domain, and language implementation and validation workflows for internal concerns. Keep the agent documentation pass focused on contract consequences, and update agent documentation only when the contract, routing, or documented invocation changes.
- For an explicit change, including a request that also uses review or audit language, use the change workflow. Treat inspection as the evidence gathering phase, then resolve the canonical owner, compose the change, and validate the final contents.
- For a standalone ordinary review, keep the task read-only. Resolve the canonical owner and validate the existing contents, but skip composition, formatting, and every mutation.
- For a standalone audit, keep the task read-only. Follow an applicable model-invocable project audit workflow when one exists. Otherwise start from Git-tracked paths, add only explicitly named untracked documentation when local policy permits it, inspect the resolved documentation scope, report findings, and stop without formatting or mutation.
- Treat naming and prose punctuation policies as file-scoped unless a narrower surface contract defines another scope.
- Resolve each pass’s scope before applying project and domain skills. Let supported clients discover them from their descriptions, and do not preload skills for later passes.

## Resolve Canonical Owner

1. State the durable detail being changed or evaluated in one sentence.
2. Use the selected local or fallback authority model to identify its expected owner.
3. Search applicable project-authored agent documentation for existing definitions, rationale, inventories, and links concerning that detail.
4. Name one canonical owner and identify every other document that should link to it or remove a stale paraphrase.
5. Do not edit until one owner can be named. Do not choose an owner merely because the detail already appears there.

## Compose Changes

Before editing, follow [Compose Documentation](references/compose-documentation.md). Reviews and audits use its applicable authoring rules as criteria without composing or mutating content.

## Validate Documentation

Use [Select Validation Scope](references/validate-documentation-changes.md#select-validation-scope) before loading detailed checks. Keep the complete declared scope of each affected invariant.

### Run Complete-Scope Checks

For every change, review, or audit:

1. Reread every applicable `AGENTS.md` file and each in-scope documentation file that the current task has not already loaded unchanged. Use Git status and diff to identify what changed since it was loaded.
2. For each affected proposition or shared wording contract, search the complete applicable documentation family for its distinctive wording and close semantic variants. Apply the [Documentation Boundary Checks](references/documentation-boundary-checks.md) to routed or layered surfaces. When a global instruction changes or is evaluated, identify every affected public skill mirror, including rephrased variants, and apply `skill-development` to each for public mirror alignment. Confirm that one normative definition remains and that every secondary occurrence has a distinct required role or links to the canonical owner.
3. For every in-scope change to a direct-path surface, compare its before-and-after context footprint. In a review or audit, report unjustified growth without editing.
4. When an in-scope change moved guidance from a `SKILL.md` into references, map every removed proposition to its destination and confirm that every task that previously received it still deterministically loads that destination. Treat a missing behavioral distinction, condition, exception, or route as a contract regression.

For a review or audit, use only read-only diagnostics and identify anything that could not be verified.

### Complete Change Validation

For changes, complete [Validate Documentation Changes](references/validate-documentation-changes.md) after the shared checks above and before delivery. Reviews and audits remain on the read-only validation path.

## Report Results

- For a change, identify the canonical owner and any redundant definitions removed or replaced with links. Follow the applicable communication policy for validation reporting.
- For a review or audit, lead with concrete findings, their evidence, the canonical owner, and the suggested fix.
- Report any ownership decision that remains unresolved instead of distributing the detail across multiple documents.
