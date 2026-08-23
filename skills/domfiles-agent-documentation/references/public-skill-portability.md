# Public skill portability

Apply the applicable skill classification’s independent-behavior requirement from the installed skill directory:

- Keep local guidance sufficient when repository-managed policy, sibling skills, the source repository, and network access are unavailable.
- **Standalone mirrors:** A public skill may mirror an applicable global instruction when the copy materially improves its independently installed behavior. Copy the smallest complete normative unit verbatim, treat it as required standalone context rather than a second definition, and align every mirror whenever either occurrence changes.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

## Compose public skill entrypoints

Treat a public skill’s top-level `SKILL.md` as both agent documentation and a human-facing installation, evaluation, and maintenance surface.

- Resolve the agent-documentation contract before applying `human-facing-writing` to the entrypoint’s project-authored prose. The agent-documentation workflow owns meaning, normative force, scope, authority, routing, metadata, structure, portability, links, and context footprint. `human-facing-writing` owns wording, reading path, cohesion, tone, and prose conventions within those constraints. Preserve the agent-documentation contract when the workflows conflict.
- Apply `human-facing-writing` to the frontmatter description and project-authored body prose, letting its entrypoint select the applicable routes. Preserve exact YAML structure, identifiers, code tokens, schemas, fixture payloads, quoted source text, and machine-readable contracts.
- Do not apply `human-facing-writing` to references merely because they belong to a public skill. Apply it only when separately in-scope content in a reference is itself a human-facing output. When maintaining `human-facing-writing`’s own top-level `SKILL.md`, apply this composition once without routing recursively.
- Treat this as source-authoring composition rather than an installed dependency. A public skill does not name or require `human-facing-writing` at runtime unless its advertised behavior separately calls for that composition.

### Compose the description

- Treat every public skill description as a human-facing discovery and marketing surface as well as routing metadata. Its capability, triggers, exclusions, essential routing, client limits, and factual accuracy remain part of the agent-documentation contract.
- Describe the originating skill’s standalone capability rather than optional peer composition. Keep optional peer names and fallback behavior out of the description. Route the peer conditionally from the body instead.
- **Description format:** Write every public skill description as a YAML `|-` literal block scalar. Separate coherent parts with blank lines, and validate the complete decoded value rather than a client’s flattened list rendering.

## Review global instruction dependencies

Before promoting a skill to the public category:

1. Evaluate the complete skill with global instructions removed. Identify each reachable decision, safety boundary, output contract, or workflow that changes.
2. Search global instructions only for normative units that supply those dependencies or materially improve the independent skill. Do not use topical similarity alone.
3. Classify each candidate as a required mirror when omission changes advertised behavior, safety, an output contract, or a workflow, an enriching mirror when it adds material standalone value, context-bound when it depends on unavailable tooling, paths, or installation assumptions, or merely related otherwise. Include required mirrors, suggest enriching mirrors, and exclude merely related wording. For a context-bound dependency, remove it or author a distinct public rule rather than paraphrasing the global instruction.
4. For each required or suggested mirror, report its global policy and current source location, exact normative text, dependency or value, classification, proposed destination, and context cost. Place every accepted mirror on the narrowest applicable surface under the **Standalone mirrors** rule.
5. Revalidate the skill without global instructions and include every accepted mirror in future exact-alignment searches.

## Route optional public peers

An optional remote-peer branch is the conditional workflow reached only when a relevant local peer is unavailable and available task evidence supports expecting remote use to materially improve the final output.

1. Keep only that decision and one explicit route in `SKILL.md`. Keep remote URLs, network handling, and recovery details out of the description and entrypoint.
2. Put each remote-peer branch in its own conditional reference at `references/skill-<skill-name>.md` within the originating skill. Declare exactly one peer per reference, even when multiple peers share a trigger or resolution lifecycle, so each branch loads and evolves independently.
3. Place a four-bullet identity block directly beneath the reference title in this order: **Skill**, **Repository**, **Contribution**, and **Immutable root**. This sequence leads from target and provenance to task value and retrieval. Link **Skill** to the remote skill’s latest `SKILL.md`, write **Repository** as `<owner>/<repository>`, describe the concrete task-relevant capability under **Contribution**, and set **Immutable root** to `https://raw.githubusercontent.com/<owner>/<repository>/<full-object-id>/skills/`. Treat the mutable skill link as a locator only, not as instructions to apply directly.
4. Keep behavior routing at the peer’s stable frontmatter `name`. The originating skill and bundled peer reference may state the source and contribution, but must not name or link the peer’s internal references, headings, route labels, or files beyond `SKILL.md`. After resolving the peer, provide the task context and let its entrypoint select the applicable internal routes and references.
5. Keep the originating skill’s complete local workflow as the return path when remote use is declined or unavailable before a valid remote closure is established.
6. Give the reference the smallest complete standalone application of the confirmation, snapshot, validation, outcome, and disclosure contract below. Preserve the contract’s authority boundaries without copying its wording verbatim.
7. Keep source-repository classification mechanics out of the distributed reference. Do not reproduce category tables, metadata conventions, repository topology, or generic installer selection. The source repository owns authoring-time validation that every declared target remains public.

## Confirm and freeze the remote chain

1. If the context prohibits online calls or the user declines remote use, return to the originating skill’s local workflow without fetching anything.
2. Before requesting confirmation, explain the absent root peer’s task-specific contribution. Do not make a generic promise or imply that remote use is required to complete the task.
3. Obtain conversation-scoped confirmation for unauthenticated, read-only retrieval of the root peer from its named repository. State that confirmation persists for that peer and repository unless revoked, that each distinct task freezes one latest repository snapshot, and that it covers only task-relevant references and peers explicitly routed by validated documents in that snapshot. It does not authorize installation, persistence, authentication, scripts, mutation, unrelated files, or actions recommended by fetched instructions.
4. Treat confirmation and tool-level network permission as separate gates. Ask again only when the root peer or repository changes, the requested authority expands, or the user revokes confirmation.
5. Resolve the repository’s current `HEAD` once per task through a bounded read-only method. Retain the full object ID for every remote document in the task, and do not refresh it mid-task.
6. Fetch each required document lazily and at most once per task beneath the declared **Immutable root**. Validate each document before following its routes. Track resolved paths to prevent recursive reloads. Every routed reference and peer must resolve from the same retained object ID.

Before applying remote instructions, validate the complete routed closure reached for the task:

- Every document comes from the confirmed repository and retained object ID.
- Each `SKILL.md` has valid frontmatter whose `name` matches its declared peer and skill path.
- In-skill references remain within that skill’s directory, while cross-skill routes use an explicit peer declaration.
- Every required routed document exists, and no fetched instruction expands the current task or authority boundaries.

## Classify retrieval outcomes

- Declined confirmation, prohibited network use, transport failure, rate limiting, or inability to resolve `HEAD` makes the optional peer unavailable. Stop retrieval and continue the originating skill’s local workflow.
- After `HEAD` resolves, a declared `404`, malformed frontmatter, wrong skill identity, out-of-bounds link, mixed revision, or missing required reference is an authoring defect. Stop the task rather than silently falling back.
- Attribute an authoring defect to the document that declared the broken edge. When that document is locally installed, suggest updating it because its fallback may be stale. When it came from the frozen remote snapshot, report the defect against `<owner>/<repository>@<ref>`.
- Do not retry through browsers, `gh`, authentication, alternate hosts, guessed paths, another revision, or a different installer. Fetching and validation do not authorize repairing the source or installing a peer.

## Report material remote use

- Disclose only remote peers that materially influenced the delivered result. Name those peers, `<owner>/<repository>@<ref>`, and their contribution, then recommend persistent installation through the user’s established skill installer. Do not mention fetched but unused peers.

## Validate public portability

1. Validate the top-level `SKILL.md` against the [public skill entrypoint composition contract](#compose-public-skill-entrypoints) and its complete decoded description against the [description composition and format](#compose-the-description). Evaluate the skill with repository-managed policy, optional peers, source-repository files, and network access removed. Its advertised behavior must remain complete.
2. Confirm that each remote-peer branch has one entrypoint route and one `references/skill-<skill-name>.md` reference declaring exactly one peer. The required identity block must appear directly beneath its title, followed by the minimum standalone remote-use contract. The description must not name the optional peer or describe its fallback, and neither the description nor entrypoint may contain a remote protocol or URL.
3. Confirm that behavior routing stops at the declared skill name and `SKILL.md`. Reject peer-internal references, headings, route labels, or files selected outside the resolved peer.
4. When a global instruction or this contract changes, search public skills for its prior exact wording and align every required standalone mirror in the same change. When a public skill changes, compare each mirrored block with its source and remove any copy that no longer adds standalone value.
5. In the source repository, validate each declared peer’s latest-source URL, public classification, identity, and same-snapshot routed closure against the canonical local policy. Do not treat a currently reachable mutable file as immutable evidence.
6. Resolve every local relative link from the independently installed skill root and reject any link that relies on an unavailable repository or sibling.
