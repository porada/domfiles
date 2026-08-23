# Public skill portability

Apply the applicable skill classification’s independent-behavior requirement from the installed skill directory:

- Keep local guidance sufficient when repository-managed policy, sibling skills, the source repository, and network access are unavailable.
- **Standalone mirrors:** A public skill may mirror an applicable global instruction when the copy materially improves its independently installed behavior. Rephrase and arrange the standalone copy as needed for consistency with the skill, but preserve the source rule’s complete meaning, normative force, scope, conditions, exceptions, and standalone behavior. Treat it as required standalone context rather than a second definition, and realign it whenever either occurrence changes.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

## Compose public skill writing

Treat every project-authored writing surface in a public skill as both agent documentation and human-facing content for installation, evaluation, or maintenance.

- Resolve the agent-documentation contract before applying `human-facing-writing` to any public-skill writing. Apply the **Agent documentation and human-facing writing** check in the [documentation boundary checks](documentation-boundaries.md) when revising wording, structure, or presentation.
- Apply `human-facing-writing` to every in-scope frontmatter description, `SKILL.md` body, reference, and human-facing asset. Let the `human-facing-writing` entrypoint select the applicable routes for each surface. Preserve exact YAML structure, identifiers, code tokens, schemas, fixture payloads, quoted source text, and machine-readable contracts.
- When maintaining `human-facing-writing` itself, apply this composition once to each in-scope writing surface without routing recursively.
- Treat this as source-authoring composition rather than an installed dependency. A public skill does not name or require `human-facing-writing` at runtime unless its advertised behavior separately calls for that composition.

### Compose the entrypoint

- When the entrypoint includes introductory prose, use it to orient the reader to the skill’s purpose or governing principle without paraphrasing the description. Place precedence, routing, safety, and delivery rules in the sections that own them.
- Use noun phrases for second-level headings that name durable sections. Use action headings at lower levels only for bounded procedures or decisions, and keep peer headings grammatically parallel.
- Keep external source links out of the entrypoint unless most invocations need the source directly. State the operative rule locally, and place provenance, further reading, and optional verification links in the conditional reference that owns the detail.

### Compose the description

- Treat every public skill description as a human-facing discovery and marketing surface as well as routing metadata. Its capability, triggers, exclusions, essential routing, client limits, and factual accuracy remain part of the agent-documentation contract.
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

1. Keep only that decision and one explicit route in `SKILL.md`. Keep remote URLs, network handling, and recovery details out of the description and entrypoint. Preserve the originating skill’s complete local workflow so its advertised behavior does not depend on the peer.
2. Give each peer its own conditional reference, even when multiple peers share a trigger or retrieval lifecycle. Start from the [optional-peer reference template](../assets/optional-peer-reference.md). Replace `<skill-title>` with the human-readable title, `<skill-name>` with the stable frontmatter `name`, and `<owner>`, `<repository>`, and `<contribution>` with the declared peer’s details. Preserve `<full-object-id>` and `<ref>` as runtime placeholders, and adapt only the originating skill’s local fallback and task-specific contribution.
3. Keep behavior routing at the peer’s stable frontmatter `name`. The originating skill and bundled peer reference may state the source and contribution, but must not name or link the peer’s internal references, headings, route labels, or files beyond `SKILL.md`. After resolving the peer, provide the task context and let its entrypoint select the applicable internal routes and references.
4. Keep source-repository classification mechanics out of the distributed reference. Do not reproduce category tables, metadata conventions, repository topology, or generic installer selection. The source repository owns authoring-time validation that every declared target remains public.

## Validate public portability

1. Validate every in-scope public-skill writing surface against the [public skill writing composition contract](#compose-public-skill-writing) and the complete decoded description against the [description composition and format](#compose-the-description). A creation, promotion, or full-skill composition pass includes every project-authored writing surface. Evaluate the skill with repository-managed policy, optional peers, source-repository files, and network access removed. Its advertised behavior must remain complete.
2. Confirm that each remote-peer branch has one entrypoint route and one reference declaring exactly one peer and following the [optional-peer reference template](../assets/optional-peer-reference.md). The description must not name the optional peer or describe its fallback, and neither the description nor entrypoint may contain a remote protocol or URL.
3. Confirm that behavior routing stops at the declared skill name and `SKILL.md`. Reject peer-internal references, headings, route labels, or files selected outside the resolved peer.
4. When a global instruction or this contract changes, search public skills for affected standalone mirrors and close semantic variants, then align each mirror’s meaning and boundaries in the same change. When the optional-peer reference template changes, align every reference derived from it while preserving authorized local adaptations. When a public skill changes, reevaluate each standalone mirror against the complete canonical policy. Add newly required propositions, remove propositions that no longer add standalone value, and align retained propositions semantically.
5. In the source repository, validate each declared peer’s latest-source URL, public classification, identity, and complete routed document set from one snapshot against the canonical local policy. Do not treat a currently reachable mutable file as immutable evidence.
6. Resolve every local relative link from the independently installed skill root and reject any link that relies on an unavailable repository or sibling.
