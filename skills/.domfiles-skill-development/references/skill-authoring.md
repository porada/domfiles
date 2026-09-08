# Skill Authoring

Apply these skill-specific requirements within the shared documentation workflow owned by `agent-documentation`. Its composition with `human-facing-writing` also governs skill metadata, references, assets, and public READMEs.

## Entrypoints and References

Keep each `SKILL.md` as an entrypoint. Keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves. Apply [Skill Descriptions](skill-descriptions.md) when authoring or assessing the description or invocation mode.

When maintaining `posix-shell-scripting`, keep code block examples out of its `SKILL.md` because the skill’s two-space Prettier indentation override applies only to its references and would reindent the YAML frontmatter if extended to the entrypoint. Structure the surrounding guidance as a coherent routed reference instead of moving examples alone, and retain the governing rule and the reference’s activation condition in the entrypoint.

## Base Skills and Overlays

Classify whether the overlay co-applies across the base skill’s complete trigger family or narrows that family, and whether every supported installation guarantees the base. Follow [Skill Descriptions](skill-descriptions.md) when composing the description.

For a model-invocable co-applying overlay with a guaranteed base, state in the description that the overlay applies whenever the base applies, name the base by its stable frontmatter `name`, and add any overlay-only triggers or exclusions. Do not repeat the base description’s capability, trigger family, or exclusions.

For a model-invocable scope-narrowing overlay, preserve its narrower triggers and exclusions in the description, and do not claim broader co-application.

In every invocation mode, define the overlay’s added or narrower behavior in its body, and route from its entrypoint to the base wherever their scopes overlap. When a supported installation does not guarantee the base, keep the overlay independently complete. For a public overlay, follow the [public skill portability contract](public-skill-portability.md), preserve standalone behavior, and make any base composition optional. In every branch, keep the base unaware of the overlay and keep any dependency one-way.

## Domain-Bounded Examples

In project-authored skill documentation, include a code example only when it clarifies one contract the skill owns. State the owned invariant in prose, and keep the behavior that demonstrates it within that domain. A realistic example may consume a verified external interface and state only the behavior needed to establish that premise. Use a domain-neutral operation for unrelated incidental behavior, and route any external behavior the example must teach, validate, or implement to its owner. Prefer removing the example over expanding it into a complete cross-domain workflow.

## Global Policy and Standalone Context

A skill may name an always-loaded global policy but must not restate it unless a distinct surface-specific application is required. A public skill that may be installed without that policy may carry a standalone mirror under the [public skill portability contract](public-skill-portability.md).
