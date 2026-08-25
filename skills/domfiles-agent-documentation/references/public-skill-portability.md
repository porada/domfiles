# Public skill portability

Apply the applicable skill classification’s independent-behavior requirement from the installed skill directory:

- Keep local guidance sufficient when repository-managed policy, sibling skills, the source repository, and network access are unavailable.
- **Standalone mirrors:** A public skill may mirror an applicable global instruction when the copy materially improves its independently installed behavior. Rephrase and arrange the standalone copy as needed for consistency with the skill, but preserve the source rule’s complete meaning, normative force, scope, conditions, exceptions, and standalone behavior. Treat it as required standalone context rather than a second definition, and realign it whenever either occurrence changes.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

## Public name

A public skill’s name is discovery metadata as well as a stable identifier. Before creating, renaming, or promoting one, identify the primary tool, domain, or task terms users are likely to search. When the target registry or index is available, inspect a bounded set of exact and close matches. Prefer a distinctive name that retains the primary tool or domain term, and avoid clever wording that obscures the trigger.

## Compose public skill writing

Treat every project-authored writing surface in a public skill as both agent documentation and human-facing content for installation, evaluation, or maintenance.

- Resolve the agent-documentation contract before applying `human-facing-writing` to any public-skill writing. Apply the **Agent documentation and human-facing writing** check in the [documentation boundary checks](documentation-boundaries.md) when revising wording, structure, or presentation.
- Apply `human-facing-writing` to every in-scope frontmatter description, `SKILL.md` body, reference, and human-facing asset. Let the `human-facing-writing` entrypoint select the applicable routes for each surface. Preserve exact YAML structure, identifiers, code tokens, schemas, fixture payloads, quoted source text, and machine-readable contracts.
- When maintaining `human-facing-writing` itself, apply this composition once to each in-scope writing surface without routing recursively.
- Treat this as source-authoring composition rather than an installed dependency. A public skill does not name or require `human-facing-writing` at runtime unless its advertised behavior separately calls for that composition.

### Compose the entrypoint

- Treat introductory prose in a public skill entrypoint as a human-facing marketing surface as well as direct-path agent documentation. Use it to orient readers to the skill’s purpose or governing principle and, when useful for evaluation, to restate a compact version of its capability, audience, or range from the description. Treat that overlap as a distinct surface-specific application rather than a second canonical definition. Keep triggers, exclusions, precedence, routing, safety, and delivery rules in the sections that own them instead of repeating them in the introduction.
- Use noun phrases for second-level headings that name durable sections. Use action headings at lower levels only for bounded procedures or decisions, and keep peer headings grammatically parallel.
- Keep external source links out of the entrypoint unless most invocations need the source directly. State the operative rule locally, and place provenance, further reading, and optional verification links in the conditional reference that owns the detail.

### Compose the description

- Treat every public skill description as a human-facing discovery and marketing surface as well as routing metadata. Its capability, triggers, exclusions, essential routing, client limits, and factual accuracy remain part of the agent-documentation contract.
- State trigger conditions as direct reasons to load the skill. Do not make loading depend on whether the task already exhibits a quality the skill is meant to enforce, such as being “focused,” “simple,” “safe,” or “bounded.”
- Describe the originating skill’s standalone capability rather than optional peer composition. Keep optional peer names and fallback behavior out of the description. Route the peer conditionally from the body instead.
- **Description format:** Write every public skill description as a YAML `|-` literal block scalar. Separate coherent parts with blank lines, and validate the complete decoded value rather than a client’s flattened list rendering.

## Review global instruction dependencies

Before promoting a skill to the public category:

1. Evaluate the complete skill with global instructions removed. Identify each reachable decision, safety boundary, output contract, or workflow that changes.
2. Search global instructions only for normative units that supply those dependencies or materially improve the independent skill. Do not use topical similarity alone.
3. Classify each candidate as a required mirror when omission changes advertised behavior, safety, an output contract, or a workflow, an enriching mirror when it adds material standalone value, context-bound when it depends on unavailable tooling, paths, or installation assumptions, or merely related otherwise. Include required mirrors, suggest enriching mirrors, and exclude merely related wording. For a context-bound dependency, remove it or author a distinct public rule rather than paraphrasing the global instruction.
4. For each required or suggested mirror, report its global policy and current source location, the meaning and boundaries the standalone version must preserve, its dependency or value, classification, proposed destination, and context cost. Place every accepted mirror on the narrowest applicable surface under the **Standalone mirrors** rule.
5. Revalidate the skill without global instructions and include every accepted mirror in future semantic-alignment searches.

## Route optional public peers

An optional remote-peer branch is the conditional workflow reached only when a relevant local peer is unavailable and available task evidence supports expecting remote use to materially improve the final output.

Local composition and remote fallback are separate decisions. A skill may route to a locally available public peer by its stable frontmatter `name` without declaring remote retrieval. The remote-peer workflow begins only when the local peer is unavailable and the originating skill offers GitHub-hosted retrieval under this section.

1. Keep only that decision and one explicit route in `SKILL.md`. Keep remote URLs, network handling, and recovery details out of the description and entrypoint. Preserve the originating skill’s complete local workflow so its advertised behavior does not depend on the peer.
2. Resolve each peer once. The originating `SKILL.md` owns local availability and the decision to invoke its optional-peer reference. That reference owns network confirmation, retrieval, validation, and the resolved-or-unavailable outcome. Every other routed reference consumes that result and must not repeat those decisions or operations.
3. Give each peer its own conditional reference, even when multiple peers share a trigger or retrieval lifecycle. Start from the [optional-peer reference template](../assets/optional-peer-reference.md). Replace `<skill-title>` with the human-readable title, `<skill-name>` with the stable frontmatter `name`, and `<owner>`, `<repository>`, and `<contribution>` with the declared peer’s details. Preserve `<full-object-id>` and `<ref>` as runtime placeholders, and adapt only the originating skill’s local fallback and task-specific contribution.
4. Keep behavior routing at the peer’s stable frontmatter `name`. The originating skill and bundled peer reference may state the source and contribution, but must not name or link the peer’s internal references, headings, route labels, or files beyond `SKILL.md`. After resolving the peer, provide the task context and let its entrypoint select the applicable internal routes and references. Keep the dependency one-way unless each skill independently needs the other. The originating skill may name the peer. The peer must expose a generic composition boundary, such as workflow-owned delivery, rather than naming the originating skill solely to support its integration.
5. Keep source-repository classification mechanics out of the distributed reference. Do not reproduce category tables, metadata conventions, repository topology, or generic installer selection. The source repository owns authoring-time validation that every declared target remains public.

## Include the stale-guidance contract

Copy the contents of the following blockquote verbatim into every public skill’s fully loaded `SKILL.md` entrypoint. Load the complete entrypoint before acting on any routed guidance or following any reference from the skill.

> ## Stale Guidance
>
> Classify each part of this skill’s guidance used by the selected workflow as required, optional, or supporting. Treat missing local targets, malformed destinations, and HTTP responses that report a resource as missing or permanently unavailable as broken references. Broken references and verified conflicts with the current interface or behavior mean the guidance is stale. Use any failure response the guidance defines. Otherwise, report the stale guidance and evidence, recommend updating this skill, and follow the appropriate recovery below.
>
> When required guidance is stale, stop only the affected branch and use any complete fallback provided by the available guidance. Without one, ask whether to continue. The choice applies only to this conversation and to work independent of the stale guidance. Stale optional or supporting guidance does not stop the workflow.
>
> Access restrictions, authentication problems, network failures, and HTTP server errors are not evidence of staleness. Use any relevant access or retrieval guidance. If none applies, stop retrieving the resource and report the resource, attempted method, exact error, and smallest corrective action.
>
> Never infer missing content. Never substitute an unverified location. Never weaken scope, approval, mutation, or security boundaries.

## Validate public portability

1. Validate every in-scope public-skill writing surface against the [public skill writing composition contract](#compose-public-skill-writing) and the complete decoded description against the [description composition and format](#compose-the-description). A creation, promotion, or full-skill composition pass includes every project-authored writing surface. Evaluate the skill with repository-managed policy, optional peers, source-repository files, and network access removed. Its advertised behavior must remain complete.
2. Trace every opt-in, sensitive operation, and mutating branch to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
3. Confirm that each remote-peer branch has one entrypoint route and one reference declaring exactly one peer and following the [optional-peer reference template](../assets/optional-peer-reference.md). The description must not name the optional peer or describe its fallback, and neither the description nor entrypoint may contain a remote protocol or URL.
4. Confirm that behavior routing stops at the declared skill name and `SKILL.md`. Reject peer-internal references, headings, route labels, or files selected outside the resolved peer.
5. When a global instruction or this contract changes, search public skills for affected standalone mirrors and close semantic variants, then align each mirror’s meaning and boundaries in the same change. When the optional-peer reference template changes, align every reference derived from it while preserving authorized local adaptations. When the stale-guidance template changes, copy its contents verbatim into every public skill’s `SKILL.md` entrypoint. When a public skill changes, reevaluate each standalone mirror against the complete canonical policy. Add newly required propositions, remove propositions that no longer add standalone value, and align retained propositions semantically.
6. In the source repository, validate each declared peer’s latest-source URL, public classification, identity, and complete routed document set from one snapshot against the canonical local policy. Do not treat a currently reachable mutable file as immutable evidence.
7. When coordinated unpublished changes affect both a peer declaration and the peer, validate the local sources as one candidate snapshot and the current remote snapshot separately. If the remote snapshot lacks the declared contribution or contradicts the candidate’s composition, terminal-delivery, or fallback contract, record a publication gate and confirm that the distributed peer reference classifies it as an authoring defect and continues through the local fallback.
8. Resolve every local relative link from the independently installed skill root and reject any link that relies on an unavailable repository or sibling.
9. For each public skill, confirm that the stale-guidance template appears verbatim in its complete entrypoint and is loaded before any routed guidance or reference can be used. Exercise each source of staleness and recovery branch, including a guidance-specific failure response, required guidance with and without a complete fallback, optional or supporting guidance, and an access failure. Confirm that recovery remains scoped to the selected workflow, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.
