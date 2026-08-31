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
- For a creation, promotion, or full-skill composition pass, evaluate each complete writing surface rather than only the changed lines. Include content moved without substantive wording changes. A surface may remain unchanged after complete evaluation through `human-facing-writing`, but portability, link, metadata, formatting, and heading edits alone do not establish that evaluation.
- For a template-derived surface, preserve the template’s contract and apply `human-facing-writing` only to project-authored substitutions and permitted adaptations.
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

## Include standardized public mirrors

Treat the secrets, typography, and stale-guidance templates required by this contract as standardized standalone mirrors for every public skill. Do not reclassify or rejustify them during creation or promotion. The applicable global policy remains the semantic owner, each template owns the public rendering, and each bundled copy provides required standalone context.

### Secrets and authentication

Copy the [secrets and authentication template](../assets/secrets-and-authentication.md) verbatim into every public skill’s fully loaded `SKILL.md`. Place it in an always-loaded location that governs every workflow. Domain guidance may add stricter constraints or surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Typography

Bundle `references/typography.md` in every public skill. For every skill except `human-facing-writing`, make it a verbatim copy of the [typography template](../assets/typography.md). `human-facing-writing` owns its additional heading convention and keeps that extension only in its own typography reference. Route to the bundled reference deterministically from `SKILL.md` before the skill creates, edits, reviews, or delivers prose. Apply a narrower user, project, surface, language, or syntax rule when the template permits it, but keep the shared template rules unchanged.

### Stale guidance

Copy the [stale-guidance template](../assets/stale-guidance.md) verbatim into every public skill’s fully loaded `SKILL.md` entrypoint. Load the complete entrypoint before acting on any routed guidance or following any reference from the skill.

## Build the public promotion profile

Complete the public promotion profile before creating a public skill, or before moving or rewriting content during a promotion. Evaluate the intended public capability and every reachable workflow with global instructions removed. For a promotion, include the complete source skill and proposed target behavior. Inventory each decision, safety boundary, output contract, or workflow that changes without the global instruction layer, then capture the result in one task-local profile. Use any promoted, retained, or removed classification required by category maintenance as input. Keep the profile out of the distributed skill. It is the authoring record for deciding which inherited policies need standalone behavior, not another public contract.

Record standardized mirrors as template-covered without reopening their inclusion, wording, or ownership. Apply the following five mandatory lenses to the complete behavior-delta inventory. The lenses are not exhaustive and do not limit which dependencies the profile must discover. Mark a lens not applicable only after checking it:

1. **Authority and provenance:** Identify the instruction surfaces, evidence, user-supplied artifacts, optional peers, and external guidance the workflow consumes. Determine which sources may govern behavior, which are evidence only, and which distinctions would disappear without the global instruction layer.
2. **Review behavior:** For every advertised review or audit route, resolve its read-only boundary, evidence standard, allowed output, and terminal delivery. Reconcile every required disclosure or evidence limitation with any output restriction.
3. **Tool execution:** Inventory every operation the workflow selects or recommends, classify it under the global **Terminal execution** policy, and identify each execution safeguard or deferral that independent behavior requires.
4. **External services:** Identify required network access, authenticated state, transmitted data, service-specific interfaces, and optional peers. Decide whether the public skill owns the service interaction, defers it to another workflow, or preserves a complete local fallback. Apply the [optional-peer contract](#route-optional-public-peers) only to optional public peers.
5. **Mutation and approval:** Trace every local mutation, dependency change, remote effect, publication step, and user-only operation to the actor, required authorization, terminal action, or required stop. Keep drafting, review, preparation, and tool availability from implying authority for a later effect.

Search global instructions only for normative units that supply a dependency exposed by the complete inventory or materially improve the independent skill. Do not use topical similarity alone. Classify each candidate as a required mirror when omission changes advertised behavior, safety, an output contract, or a workflow, an enriching mirror when it adds material standalone value, context-bound when it depends on unavailable tooling, paths, or installation assumptions, or merely related otherwise. Include required mirrors, suggest enriching mirrors, and exclude merely related wording. For a context-bound dependency, remove it or author a distinct public rule rather than paraphrasing the global instruction.

For each required or suggested mirror, report its global policy and current source location, the meaning and boundaries the standalone version must preserve, its dependency or value, classification, proposed destination, and context cost. Place every accepted mirror on the narrowest applicable surface under the **Standalone mirrors** rule. Revalidate the skill without global instructions, and include every accepted mirror in future semantic-alignment searches.

## Route optional public peers

An optional remote-peer branch is the conditional workflow reached only when a relevant local peer is unavailable and available task evidence supports expecting remote use to materially improve the final output.

Local composition and remote fallback are separate decisions. A skill may route to a locally available public peer by its stable frontmatter `name` without declaring remote retrieval. The remote-peer workflow begins only when the local peer is unavailable and the originating skill offers GitHub-hosted retrieval under this section.

1. Keep only that decision and one explicit route in `SKILL.md`. Keep remote URLs, network handling, and recovery details out of the description and entrypoint. Preserve the originating skill’s complete local workflow so its advertised behavior does not depend on the peer.
2. Resolve each peer once. The originating `SKILL.md` owns local availability and the decision to invoke its optional-peer reference. That reference owns network confirmation, retrieval, validation, and the resolved-or-unavailable outcome. Every other routed reference consumes that result and must not repeat those decisions or operations.
3. Give each peer its own conditional reference, even when multiple peers share a trigger or retrieval lifecycle. Start from the [optional-peer reference template](../assets/optional-peer-reference.md). Replace `<skill-title>` with the human-readable title, `<skill-name>` with the stable frontmatter `name`, and `<owner>`, `<repository>`, and `<contribution>` with the declared peer’s details. Preserve `<full-object-id>` and `<ref>` as runtime placeholders, and adapt only the originating skill’s local fallback and task-specific contribution.
4. Keep behavior routing at the peer’s stable frontmatter `name`. The originating skill and bundled peer reference may state the source and contribution, but must not name or link the peer’s internal references, headings, route labels, or files beyond `SKILL.md`. After resolving the peer, provide the task context and let its entrypoint select the applicable internal routes and references. Keep the dependency one-way unless each skill independently needs the other. The originating skill may name the peer. The peer must expose a generic composition boundary, such as workflow-owned delivery, rather than naming the originating skill solely to support its integration.
5. Keep source-repository classification mechanics out of the distributed reference. Do not reproduce category tables, metadata conventions, repository topology, or generic installer selection. The source repository owns authoring-time validation that every declared target remains public.

## Validate public portability

1. For a creation or promotion, confirm that the promotion profile contains the complete behavior-delta inventory, applies all five mandatory lenses, and accepts standardized mirrors as template-covered. Confirm that every other inherited policy dependency has a classification, named source owner, destination or retained owner, and independently valid result.
2. Validate every in-scope public-skill writing surface against the [public skill writing composition contract](#compose-public-skill-writing) and the complete decoded description against the [description composition and format](#compose-the-description). A creation, promotion, or full-skill composition pass includes every project-authored writing surface. Classify each surface as revised, retained after complete writing evaluation, or template-constrained, and confirm that every project-authored template adaptation received that evaluation. Evaluate the skill with repository-managed policy, optional peers, source-repository files, and network access removed. Its advertised behavior must remain complete.
3. Confirm that the secrets and authentication template appears verbatim exactly once in the complete `SKILL.md`, that every `references/typography.md` other than `human-facing-writing` matches its template, and that every prose-producing path deterministically loads the typography reference. Confirm that `human-facing-writing` retains the shared typography rules while keeping its skill-owned heading extension local. Treat other domain-specific additions as stricter constraints or surface-specific applications rather than competing mirrors.
4. Trace every opt-in, sensitive operation, and mutating branch to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
5. Confirm that each remote-peer branch has one entrypoint route and one reference declaring exactly one peer and following the [optional-peer reference template](../assets/optional-peer-reference.md). The description must not name the optional peer or describe its fallback, and neither the description nor entrypoint may contain a remote protocol or URL.
6. Confirm that behavior routing stops at the declared skill name and `SKILL.md`. Reject peer-internal references, headings, route labels, or files selected outside the resolved peer.
7. When a global instruction or this contract changes, search public skills for affected standalone mirrors and close semantic variants, then align each mirror’s meaning and boundaries in the same change. When a standardized public-mirror template changes, align every verbatim template-derived copy under that template’s copy contract in the same change. For typography, also align each owner-specific reference’s shared rules while preserving local extensions. When the optional-peer reference template changes, align every reference derived from it while preserving authorized local adaptations. When a public skill changes, reevaluate each standalone mirror against the complete canonical policy. Add newly required propositions, remove propositions that no longer add standalone value, and align retained propositions semantically.
8. In the source repository, validate each declared peer’s latest-source URL, public classification, identity, and complete routed document set from one snapshot against the canonical local policy. Do not treat a currently reachable mutable file as immutable evidence.
9. When coordinated unpublished changes affect both a peer declaration and the peer, validate the local sources as one candidate snapshot and the current remote snapshot separately. If the remote snapshot lacks the declared contribution or contradicts the candidate’s composition, terminal-delivery, or fallback contract, record a publication gate and confirm that the distributed peer reference classifies it as an authoring defect and continues through the local fallback.
10. Resolve every local relative link from the independently installed skill root and reject any link that relies on an unavailable repository or sibling.
11. For each public skill, confirm that the stale-guidance template appears verbatim in its complete entrypoint and is loaded before any routed guidance or reference can be used. Exercise each source of staleness and recovery branch, including a guidance-specific failure response, required guidance with and without a complete fallback, optional or supporting guidance, and an access failure. Confirm that recovery remains scoped to the selected workflow, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.
