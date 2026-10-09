# Skill Authoring

## Entrypoints and References

Keep each `SKILL.md` as an entrypoint. Keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves.

During `agent-documentation`’s **Check Workflow Compatibility** pass, trace each affected skill route from its actual invocation and supported installation state. Use only instructions and prerequisites guaranteed on that route, not context acquired while maintaining other routes. Include relevant optional peer availability cases.

When maintaining `posix-shell-scripting`, keep code block examples out of its `SKILL.md` because the skill’s two-space Prettier indentation override applies only to its references and would reindent the YAML frontmatter if extended to the entrypoint. Structure the surrounding guidance as a coherent routed reference instead of moving examples alone, and retain the governing rule and the reference’s activation condition in the entrypoint.

## Base Skills and Overlays

Both an overlay and its base must be [model-invocable](skill-descriptions.md#invocation-mode). Determine whether every supported installation guarantees the base.

Match the base’s complete trigger family and exclusions without adding or narrowing applicability. When the base is guaranteed, state in the description that the overlay applies whenever the base applies, naming the base by its stable frontmatter `name` without repeating the base description’s capability, trigger family, or exclusions. Applicability follows the resolved task whether the base is selected directly or through another workflow. Do not infer automatic client loading from that declaration.

Define the overlay’s customizations in its body and route from its entrypoint to the base. Keep the base unaware of the overlay and routing one-way from overlay to base. Do not make a base discover or activate overlays. Reuse already loaded guidance without skipping required workflow checkpoints for later operations.

When a supported installation does not guarantee the base, state the overlay’s complete applicability locally and keep it independently complete. For a public overlay, follow the [public skill portability contract](public-skill-portability.md), preserve standalone behavior, and make any base composition optional.

Keep each customization within the defaults the base permits or choices governing instructions expressly delegate, including an explicit user override under the global **Explicit user direction** policy. Overlay applicability supplies neither precedence nor execution authority. Preserve the base’s workflow unless such an override covers the departure, and retain every separate approval and security boundary. Pass separately established authorization through the base’s applicable authorization route rather than deriving it from customization.

Combine compatible differences after applying the governing instruction hierarchy and any expressly established customization precedence. Do not infer precedence from load order or narrower applicability. Do not use a blanket “strictest wins” rule. If a material conflict remains, pause the affected decision and ask one focused question. Do not treat composition itself as a reason to reopen valid approvals.

## Domain-Bounded Examples

In project-authored skill documentation, include a code example only when it clarifies one contract the skill owns. State the owned invariant in prose, and keep the behavior that demonstrates it within that domain. A realistic example may consume a verified external interface and state only the behavior needed to establish that premise. Use a domain-neutral operation for unrelated incidental behavior, and route any external behavior the example must teach, validate, or implement to its owner. Prefer removing the example over expanding it into a complete cross-domain workflow.

## Global Policy and Standalone Context

A skill may name an always-loaded global policy but must not restate it unless a distinct surface-specific application is required. A public skill that may be installed without that policy may carry a standalone mirror under the [public skill portability contract](public-skill-portability.md).
