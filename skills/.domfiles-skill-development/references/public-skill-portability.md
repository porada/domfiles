# Public Skill Portability

Apply the applicable skill classification’s independent behavior requirement from the installed skill directory:

- Keep local guidance sufficient when network access, repository-managed policy, sibling skills, and the source repository are unavailable.
- **Standalone mirrors:** A public skill may mirror an applicable global instruction when the copy materially improves its independently installed behavior. Rephrase and arrange the standalone copy as needed for consistency with the skill, but preserve the source rule’s complete conditions, exceptions, meaning, normative force, scope, and standalone behavior. Treat it as required standalone context rather than a second definition, and realign it whenever either occurrence changes.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

Keep the global **Writing** policy’s **Numbering** rule in the global instructions rather than mirroring it into public skills.

## Load Conditional Contracts

- **Public naming and promotion:** Before public creation or renaming, or before moving or rewriting content in a promotion, follow the [public naming and promotion contract](public-skill-promotion.md). Also load it to review or audit those operations.
- **Optional peers:** Follow the [optional public peer contract](optional-public-peers.md) when authoring, reviewing, auditing, or maintaining optional public peer composition or its canonical template.

## Keep Public Descriptions Portable

- Apply the [shared description contract](skill-descriptions.md).
- Keep optional peer names and fallback behavior out of the description. Route optional peer composition conditionally from the body instead.

## Write Public READMEs

When creating or updating a public skill’s consumer-facing `README.md` for `porada/domfiles`, use the [public skill README template](../assets/readme-skill.txt). Its layout progresses from discovery and purpose through installation to attribution.

Use the frontmatter `name` for `<skill-name>`. For `<intro-text>`, copy all introductory paragraphs between the `SKILL.md` title and the first section heading, preserving their text and paragraph breaks. Do not use the frontmatter description.

For public skills from other repositories, follow their README conventions rather than this publisher-specific template.

## Maintain Public Skill Catalogs

Treat a public skill catalog and its collection README as consumer-facing documentation under the global **Consumer documentation** policy. Their copy and information architecture serve readers browsing the collection.

When `skills.sh.json` and a collection README present the same catalog, use the JSON file as the canonical source for group descriptions, group order, group titles, skill membership, and skill order. Mirror those values exactly in the README’s catalog section, linking each skill identifier to its public directory. Keep the README’s other content independent.

Within catalog work, propose improvements to the canonical groupings when useful, but apply grouping changes only with explicit user approval. A direct request specifying the grouping change supplies that approval. General maintenance or synchronization permission does not approve regrouping.

When authorized shared catalog content changes, update both surfaces together, obtaining any required README edit permission before changing either. Validate the complete correspondence, not just changed entries, alongside JSON syntax, public skill identifiers, and schema constraints.

## Include Shared Public Policies

Use the templates below as a compact standalone contract, not a copy of the host’s complete operating policy. Keep essential boundaries for authority, scope, explicit user direction, and security inline. Keep domain-specific safeguards in their owning workflows and detailed recovery guidance behind a conditional route. Do not expand the shared contract merely to reproduce global policy inventories or procedures.

Include instruction authority and secrets and authentication in every public skill. Include [typography](#typography) and [stale guidance](#stale-guidance) only when their respective eligibility conditions apply. Determine eligibility from the complete reachable workflow. The applicable global policies remain the semantic owners, each template owns its public rendering, and each bundled copy provides required standalone context without depending on another skill or the network.

Group the inline policies under a final `## General Policies` section in this order: **Typography** when applicable, **Secrets and Authentication**, **Instruction Authority**, then **Stale Guidance** when applicable. Keep detailed typography and recovery guidance in their bundled references. The subsections below follow that output order.

### Typography

Include typography only when creating, editing, or reviewing prose is part of the skill’s advertised capability, including human-facing text embedded in code. Incidental operational reporting and scratch notes do not establish eligibility, nor does merely storing or transporting prose.

For an eligible skill, bundle a verbatim copy of the [typography template](../assets/typography.txt) at `references/typography.md`. Route to the bundled reference deterministically from `SKILL.md` before the skill creates, delivers, edits, or reviews prose. Apply a narrower user, project, surface, language, or syntax rule when the template permits it, but keep the shared template rules unchanged. For an ineligible skill, omit both the reference and its route.

### Secrets and Authentication

Copy the [secrets and authentication template](../assets/secrets-and-authentication.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add stricter constraints or surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Instruction Authority

Copy the [instruction authority template](../assets/instruction-authority.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Stale Guidance

Include stale guidance when a reachable workflow depends on operational assets or peer guidance, follows bundled or remote instructional references, or prescribes external interface behavior, including file formats, that its directions depend on. Evaluate inline instructions as well as links. Task evidence is not itself a dependency on skill guidance, and the absence of references does not establish ineligibility.

For an eligible skill, copy the [stale guidance route template](../assets/stale-guidance.txt) verbatim into its fully loaded `SKILL.md` entrypoint and bundle a verbatim copy of the [guidance recovery template](../assets/guidance-recovery.txt) at `references/guidance-recovery.md`. Load the complete entrypoint before following any reference, but load the recovery reference only when its failure condition occurs. Preserve any workflow-specific failure procedure, including optional peer recovery, rather than replacing it with the generic route. The inline template supplies the safe fallback when the recovery reference itself is unavailable.

For an ineligible skill, omit both templates and their routing. Keep evidence limitations and applicable authorization, retrieval, and security boundaries complete in the workflow rather than adding unused recovery behavior.

## Validate Public Portability

Apply `agent-documentation`’s shared validation lifecycle. The checks below add public-specific evidence without narrowing the complete skill or affected documentation family.

### Whole-Skill Checks

- **Description and independent behavior:** Validate the complete decoded description against the [shared description contract](skill-descriptions.md) and the [public description portability contract](#keep-public-descriptions-portable). Evaluate the skill with network access, optional peers, repository-managed policy, and source repository files removed. Its advertised behavior must remain complete.
- **Shared public policies:** Confirm that one final `## General Policies` section contains the secrets and authentication template and instruction authority template exactly once each and in the required order. Check [typography eligibility](#typography) against the advertised capability and reachable workflows. For eligible skills, confirm that the typography route appears exactly once in the required order, that `references/typography.md` matches the template, and that every prose-producing path deterministically loads it. For ineligible skills, confirm that both the reference and its route are absent. Treat other domain-specific additions as stricter constraints or surface-specific applications rather than competing mirrors.
- **Authority paths:** Trace every ingestion point through the selected workflow using its recorded roles and authority status. Confirm that every source whose authority status is untrusted reaches the instruction authority boundary before it can influence execution, mutation, relay behavior, or remote effects.
- **Sensitive and mutating paths:** Trace every mutating branch, opt-in, and sensitive operation to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
- **Stale guidance:** Check [stale guidance eligibility](#stale-guidance) against every reachable workflow, including inline interface instructions. For an eligible skill, confirm that the route template appears verbatim exactly once as the final subsection of `## General Policies`, that `references/guidance-recovery.md` matches its template, and that recovery loads only on the applicable failure path. Trace the ordinary successful path without loading that reference. Exercise every reachable staleness and recovery branch, including applicable access failures, guidance-specific outcomes, optional or supporting guidance, required guidance with and without a complete fallback, and an unavailable recovery reference. Preserve the precedence of workflow-specific failure procedures. For an ineligible skill, confirm that both templates and their routing are absent and that evidence limitations still receive complete handling. In either case, confirm that recovery remains scoped, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.

### Conditional and Installation Checks

- **Creation and promotion:** Apply the [public promotion validation](public-skill-promotion.md#validate-public-promotion) for creation or promotion, including reviews and audits of those operations. Do not require a promotion profile for other public maintenance.
- **Optional peers:** Run the [optional public peer validation](optional-public-peers.md#validate-optional-public-peers) for every existing or proposed optional peer in each in-scope public skill, even when the declarations are unchanged. When the optional peer template changes, this includes every derived reference.
- **Mirror alignment:** When a global instruction or this public contract, including its conditional references, changes, search public skills for affected standalone mirrors and close semantic variants, then align each mirror’s meaning and boundaries in the same change. When a shared public-policy template changes, align every verbatim template-derived copy under that template’s copy contract in the same change, including conditional references. When a public skill changes, reevaluate its shared contract and domain-specific mirrors against their canonical policies. Preserve complete conditions and boundaries for the rules included, add newly required standalone behavior on its narrowest applicable surface, and remove wording that no longer adds standalone value. Do not grow the shared contract to mirror unrelated global instructions.
