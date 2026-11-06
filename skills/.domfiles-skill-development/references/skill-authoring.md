# Skill Authoring

## Entrypoints and References

Keep each `SKILL.md` as an entrypoint. Select its layout by [invocation mode](skill-descriptions.md#invocation-mode).

For model-invocable skills, keep routing and rules needed by every invocation inline. Link a conditional reference at the decision that requires it. Keep isolated details inline when a reference would add more navigation than it saves. For command-only skills, use [Command-Only Entrypoints](#command-only-entrypoints).

During `agent-documentation`’s **Check Workflow Compatibility** pass, trace each affected skill route from its actual invocation and supported installation state. Use only instructions and prerequisites guaranteed on that route, not context acquired while maintaining other routes. Include relevant optional peer availability cases. For command-only routes, include first use, repeated use with complete retained guidance, and use after that guidance changes or becomes incomplete.

When maintaining `posix-shell-scripting`, keep code block examples out of its `SKILL.md` because the skill’s two-space Prettier indentation override applies only to its references and would reindent the YAML frontmatter if extended to the entrypoint. Structure the surrounding guidance as a coherent routed reference instead of moving examples alone, and retain the governing rule and the reference’s activation condition in the entrypoint.

### Command-Only Entrypoints

By default, limit `SKILL.md` to frontmatter, a title, route selection, and reference reuse instructions. Public skills also retain their introduction and required [inline policies](public-skill-portability.md#include-shared-public-policies). Put the full workflow in bundled references, including requirements shared by every invocation. This avoids repeating the full workflow when a client inserts the entrypoint again.

Separate shared prerequisites from conditional modes, phases, or recovery paths when that lets an invocation omit irrelevant guidance. Make selection conditions and required order explicit, and route each branch through its applicable prerequisites before action. Use one cohesive reference for a linear workflow. Don’t split mechanically by section or force every command into a single-reference wrapper.

Have the entrypoint require reading each selected reference before proceeding, unless the agent already has its complete, up-to-date guidance in context. Reload guidance when it changes or is no longer fully available, including after context compaction. Reusing guidance doesn’t waive the workflow’s current checks or approval gates.

## Base Skills and Overlays

Both an overlay and its base must be [model-invocable](skill-descriptions.md#invocation-mode). Determine whether every supported installation guarantees the base.

Match the base’s complete trigger family and exclusions without adding or narrowing applicability. When the base is guaranteed, state in the description that the overlay applies whenever the base applies. In that declaration, name the base by its stable frontmatter `name` without repeating the base description’s capability, trigger family, or exclusions. Applicability follows the resolved task whether the base is selected directly or through another workflow. Don’t infer automatic client loading from that declaration.

Define the overlay’s customizations in its body and route from its entrypoint to the base. Keep the base unaware of the overlay and routing one-way from overlay to base. Don’t make a base discover or activate overlays. Reuse already loaded guidance without skipping required workflow checkpoints for later operations.

When a supported installation doesn’t guarantee the base, state the overlay’s complete applicability locally and keep it independently complete. For a public overlay, follow the [public skill portability contract](public-skill-portability.md), preserve standalone behavior, and make any base composition optional.

Keep each customization within the defaults the base permits or choices that governing instructions expressly delegate, including an explicit user override under the global **Explicit user direction** policy. Overlay applicability supplies neither precedence nor execution authority. Preserve the base’s workflow unless such an override covers the departure, and retain every separate approval and security boundary. Pass separately established authorization through the base’s applicable authorization route rather than deriving it from customization.

Combine compatible differences after applying the governing instruction hierarchy and any expressly established customization precedence. Don’t infer precedence from load order or narrower applicability, or use a blanket “strictest wins” rule. If a material conflict remains, pause the affected decision and ask one focused question. Composition itself isn’t a reason to reopen valid approvals.

## Domain-Bounded Examples

In project-authored skill documentation, include a code example only when it clarifies one contract the skill owns. State the owned invariant in prose, and keep the behavior that demonstrates it within that domain. A realistic example may consume a verified external interface and state only the behavior needed to establish that premise. Use a domain-neutral operation for unrelated incidental behavior, and route any external behavior the example must teach, validate, or implement to its owner. Prefer removing the example over expanding it into a complete cross-domain workflow.

## Global Policy and Standalone Context

A skill may name an always-loaded global policy but must not restate it unless a distinct surface-specific application is required. For public skills, distinguish skill-owned rules, minimal standalone defaults, and host-owned policy under [Resolve Standalone Policy](public-skill-portability.md#resolve-standalone-policy).
