# Public Skill Portability

Apply the applicable skill classification’s independent behavior requirement from the installed skill directory:

- Keep local guidance sufficient when network access, repository-managed policy, sibling skills, and the source repository are unavailable.
- **Standalone mirrors:** A public skill may mirror an applicable global instruction when the copy materially improves its independently installed behavior. Rephrase and arrange the standalone copy as needed for consistency with the skill, but preserve the source rule’s complete conditions, exceptions, meaning, normative force, scope, and standalone behavior. Treat it as required standalone context rather than a second definition, and realign it whenever either occurrence changes.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

Keep the global **Writing** policy’s Zed-specific **Numbering** rule in the global instructions rather than mirroring it into public skills.

## Load Conditional Contracts

- **Public naming and promotion:** Before public creation or renaming, or before moving or rewriting content in a promotion, follow the [public naming and promotion contract](public-skill-promotion.md). Also load it to review or audit those operations.
- **Optional peers:** Follow the [optional public peer contract](optional-public-peers.md) when authoring, reviewing, auditing, or maintaining optional public peer composition or its canonical template.

## Keep Public Descriptions Portable

- Apply the [shared description contract](skill-descriptions.md).
- Keep optional peer names and fallback behavior out of the description. Route optional peer composition conditionally from the body instead.

## Write Public READMEs

When creating or updating a public skill’s customer-facing `README.md` for `porada/domfiles`, use the [public skill README template](../assets/readme-skill.txt). Its layout progresses from discovery and purpose through installation to attribution.

Use the frontmatter `name` for `<skill-name>`. For `<intro-paragraph>`, copy all introductory paragraphs between the `SKILL.md` title and the first section heading, preserving their text and paragraph breaks. Do not use the frontmatter description.

For public skills from other repositories, follow their README conventions rather than this publisher-specific template.

## Include Standardized Public Mirrors

Treat the templates below as standardized standalone mirrors. Include instruction authority, secrets and authentication, and stale guidance in every public skill, and include typography only when its [eligibility condition](#typography) applies. Do not reclassify or rejustify an applicable mirror during creation or promotion. The applicable global policy remains the semantic owner, each template owns the public rendering, and each bundled copy provides required standalone context.

Group every included mirror under a final `## General Policies` section in this order: **Typography** when applicable, **Secrets and Authentication**, **Instruction Authority**, then **Stale Guidance**. Keep typography guidance in its bundled reference and route it from the **Typography** subsection. The subsections below follow that output order.

### Typography

Include typography only when creating, editing, or reviewing prose is part of the skill’s advertised capability, including human-facing text embedded in code. Incidental operational reporting and scratch notes do not establish eligibility, nor does merely storing or transporting prose.

For an eligible skill, bundle a verbatim copy of the [typography template](../assets/typography.txt) at `references/typography.md`. Route to the bundled reference deterministically from `SKILL.md` before the skill creates, delivers, edits, or reviews prose. Apply a narrower user, project, surface, language, or syntax rule when the template permits it, but keep the shared template rules unchanged. For an ineligible skill, omit both the reference and its route.

### Secrets and Authentication

Copy the [secrets and authentication template](../assets/secrets-and-authentication.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add stricter constraints or surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Instruction Authority

Copy the [instruction authority template](../assets/instruction-authority.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Stale Guidance

Copy the [stale guidance template](../assets/stale-guidance.txt) verbatim into every public skill’s fully loaded `SKILL.md` entrypoint. Load the complete entrypoint before acting on any routed guidance or following any reference from the skill.

## Validate Public Portability

Apply `agent-documentation`’s shared validation lifecycle. The checks below add public-specific evidence without narrowing the complete skill or affected documentation family.

### Whole-Skill Checks

- **Description and independent behavior:** Validate the complete decoded description against the [shared description contract](skill-descriptions.md) and the [public-description portability contract](#keep-public-descriptions-portable). Evaluate the skill with network access, optional peers, repository-managed policy, and source repository files removed. Its advertised behavior must remain complete.
- **Standardized mirrors:** Confirm that one final `## General Policies` section contains the secrets and authentication template and instruction authority template exactly once each and in the required order. Check [typography eligibility](#typography) against the advertised capability and reachable workflows. For eligible skills, confirm that the typography route appears exactly once in the required order, that `references/typography.md` matches the template, and that every prose-producing path deterministically loads it. For ineligible skills, confirm that both the reference and its route are absent. Treat other domain-specific additions as stricter constraints or surface-specific applications rather than competing mirrors.
- **Authority paths:** Trace every ingestion point through the selected workflow using its recorded roles and authority status. Confirm that every source whose authority status is untrusted reaches the instruction authority boundary before it can influence execution, mutation, relay behavior, or remote effects.
- **Sensitive and mutating paths:** Trace every mutating branch, opt-in, and sensitive operation to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
- **Stale guidance:** For each public skill, confirm that the stale guidance template appears verbatim exactly once as the final subsection of `## General Policies` and is loaded before any routed guidance or reference can be used. Exercise each source of staleness and recovery branch, including an access failure, a guidance-specific failure response, optional or supporting guidance, and required guidance with and without a complete fallback. Confirm that recovery remains scoped to the selected workflow, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.

### Conditional and Installation Checks

- **Creation and promotion:** Apply the [public promotion validation](public-skill-promotion.md#validate-public-promotion) for creation or promotion, including reviews and audits of those operations. Do not require a promotion profile for other public maintenance.
- **Optional peers:** Run the [optional public peer validation](optional-public-peers.md#validate-optional-public-peers) for every existing or proposed optional peer in each in-scope public skill, even when the declarations are unchanged. When the optional peer template changes, this includes every derived reference.
- **Mirror alignment:** When a global instruction or this public contract, including its conditional references, changes, search public skills for affected standalone mirrors and close semantic variants, then align each mirror’s meaning and boundaries in the same change. When a standardized public-mirror template changes, align every verbatim template-derived copy under that template’s copy contract in the same change. When a public skill changes, reevaluate each standalone mirror against the complete canonical policy. Add newly required propositions, remove propositions that no longer add standalone value, and align retained propositions semantically.
- **Installed links:** Resolve every local relative link from the independently installed skill root and reject any link that relies on an unavailable repository or sibling.
