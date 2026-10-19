# Public Skill Portability

Apply the applicable skill classification’s independent behavior requirement from the installed skill directory:

- Keep local guidance sufficient when network access, repository-managed policy, sibling skills, and the source repository are unavailable.
- Resolve inherited guidance through [Resolve Standalone Policy](#resolve-standalone-policy) rather than copying complete global policies.
- Treat every remote peer as an optional enhancement rather than a substitute for behavior the skill advertises.

## Resolve Standalone Policy

Classify each inherited requirement by its responsibility:

- **Host-owned policy:** Keep general collaboration preferences and operating rules with their existing owner. Don’t copy them merely because they also apply to the skill. Keep the skill compatible with applicable host requirements without depending on their presence.
- **Skill-owned rules:** Put the domain protocol in the skill as its canonical owner. Absorb inherited guidance only when that domain genuinely owns it. Remove or replace an old definition only within authorized scope and when every affected task still receives the rule. Don’t relocate a broader global policy into a narrower skill.
- **Standalone defaults:** When the advertised workflow needs a local rule without the host’s policy, write the smallest sufficient rule on the narrowest applicable surface. This is a minimal fork for that workflow, not a reproduction of the global policy. Preserve the conditions, exceptions, and safeguards needed within its stated scope without importing unrelated procedures or inventories.

Keep the source policy as the owner of broader behavior and identify the local default’s narrower responsibility during maintenance. Retain a complete standalone mirror only when an explicit surface contract requires semantic or verbatim equivalence, such as the typography contract below. Neither shortening nor classification authorizes weakening a security boundary.

Validate the skill alone for its advertised behavior and with the maintainer’s global policies for compatible composition. For a new skill, derive expected behavior from the authorized task rather than inventing a prior skill flow. For promotions and changes to existing skills, preserve the prior combined flow unless a behavior change is expressly authorized. Compare scope, permissions, reporting, checkpoints, continuation, and completion. Don’t add runtime detection of the maintainer’s setup, duplicate checks, or another approval merely to compose the rules.

## Load Conditional Contracts

- **Public naming and promotion:** Before public creation or renaming, or before moving or rewriting content in a promotion, follow the [public naming and promotion contract](public-skill-promotion.md). Also load it to review or audit those operations.
- **Optional peers:** Follow the [optional public peer contract](optional-public-peers.md) when authoring, reviewing, auditing, or maintaining optional public peer composition or its canonical template.

## Keep Public Descriptions Portable

- Apply the [shared description contract](skill-descriptions.md).
- Keep optional peer names and fallback behavior out of the description. Route optional peer composition conditionally from the body instead.

## Write Public READMEs

When creating or updating a public skill’s consumer-facing `README.md` for `porada/domfiles`, use the [public skill README template](../assets/readme-skill.txt). Its layout progresses from discovery and purpose through installation to attribution.

Use the frontmatter `name` for `<skill-name>`. For `<intro-text>`, copy all introductory paragraphs between the `SKILL.md` title and the first section heading, preserving their text and paragraph breaks. Don’t use the frontmatter description.

For public skills from other repositories, follow their README conventions rather than this publisher-specific template.

## Maintain Public Skill Catalogs

Treat a public skill catalog and its collection README as consumer-facing documentation under the global **Consumer documentation** policy. Their copy and information architecture serve readers browsing the collection.

When `skills.sh.json` and a collection README present the same catalog, use the JSON file as the canonical source for group descriptions, group order, group titles, skill membership, and skill order. Mirror those values exactly in the README’s catalog section, linking each skill identifier to its public directory. Keep the README’s other content independent.

Within catalog work, propose improvements to the canonical groupings when useful, but apply grouping changes only with explicit user approval. A direct request specifying the grouping change supplies that approval. General maintenance or synchronization permission doesn’t approve regrouping.

When authorized shared catalog content changes, update both surfaces together. Obtain any required README edit permission before changing either. Validate the complete correspondence, not just changed entries, alongside JSON syntax, public skill identifiers, and schema constraints.

## Include Shared Public Policies

Use the templates below as shared minimal standalone defaults. Keep essential boundaries for authority, scope, explicit user direction, and security inline, not the host’s complete policy. Keep domain-specific safeguards in their owning workflows and detailed recovery guidance behind a conditional route. Add shared wording only when the common standalone behavior needs it, not merely because a global policy contains it.

Include instruction authority and secrets and authentication in every public skill. Include [typography](#typography) and [stale guidance](#stale-guidance) only when their respective eligibility conditions apply. Determine eligibility from the complete reachable workflow. Global policies retain ownership of the broader rules. Each template owns the shared standalone wording, and bundled copies remain sufficient without another skill or the network. When a template conflicts with the required composition, update its canonical contract and every governed copy together rather than adding a skill-specific exception.

Group the inline policies under a final `## General Policies` section in this order: **Typography** when applicable, **Secrets and Authentication**, **Instruction Authority**, then **Stale Guidance** when applicable. Keep detailed typography and recovery guidance in their bundled references. The subsections below follow that output order.

### Typography

Include typography only when creating, editing, or reviewing prose for human readers is a core advertised capability or the skill defines standards for human-facing text in authored code. Producing or transporting instructions addressed to another agent, or prose as a supporting step in another workflow, does not, by itself, establish eligibility. Otherwise, inherit applicable user and project writing instructions. Standalone completeness alone does not justify typography defaults.

For maintenance in domfiles, derive the [typography template](../assets/typography.txt) from `.agents/GLOBAL.md`’s final `## Writing` section through the end of the file. Replace `## Writing` with `# Typography`, reduce every descendant heading level by one, and preserve all remaining content verbatim. Make shared rule changes in the global source before updating the template and bundled copies.

For an eligible skill, bundle a verbatim copy of the typography template at `references/typography.md`. Route to the bundled reference deterministically from `SKILL.md` before the skill creates, delivers, edits, or reviews prose. Apply a narrower user, project, surface, language, or syntax rule when the template permits it, but keep the shared template rules unchanged. For an ineligible skill, omit both the reference and its route.

### Secrets and Authentication

Copy the [secrets and authentication template](../assets/secrets-and-authentication.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add stricter constraints or surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Instruction Authority

Copy the [instruction authority template](../assets/instruction-authority.txt) verbatim into every public skill’s fully loaded `SKILL.md`. Domain guidance may add surface-specific applications, but it must not replace, paraphrase, or weaken the template.

### Stale Guidance

Include stale guidance when a reachable workflow depends on operational assets or peer guidance, follows bundled or remote instructional references, or prescribes external interface behavior, including file formats, that its directions depend on. Evaluate inline instructions as well as links. Task evidence isn’t itself a dependency on skill guidance, and the absence of references doesn’t establish ineligibility.

For an eligible skill, copy the [stale guidance route template](../assets/stale-guidance.txt) verbatim into its fully loaded `SKILL.md` entrypoint and bundle a verbatim copy of the [guidance recovery template](../assets/guidance-recovery.txt) at `references/guidance-recovery.md`. Load the complete entrypoint before following any reference, but load the recovery reference only when its failure condition occurs. Preserve any workflow-specific failure procedure, including optional peer recovery, rather than replacing it with the generic route. The inline template supplies the safe fallback when the recovery reference itself is unavailable.

For an ineligible skill, omit both templates and their routing. Keep evidence limitations and applicable authorization, retrieval, and security boundaries complete in the workflow rather than adding unused recovery behavior.

## Validate Public Portability

Select the checks below by the affected public contract. A whole-skill audit, creation, or promotion requires the complete applicable set. For propagation-only changes, use [Mirror Alignment](#mirror-alignment).

### Whole-Skill Checks

- **Description and independent behavior:** Validate the complete decoded description against the [shared description contract](skill-descriptions.md) and the [public description portability contract](#keep-public-descriptions-portable). Evaluate the skill with network access, optional peers, repository-managed policy, and source repository files removed. Its advertised behavior must remain complete. For changed inherited guidance or standalone defaults, also verify the maintained combined flow under [Resolve Standalone Policy](#resolve-standalone-policy).
- **Shared public policies:** Confirm that one final `## General Policies` section contains the secrets and authentication template and instruction authority template exactly once each and in the required order. Check [typography eligibility](#typography) against the advertised capability and reachable workflows. For eligible skills, confirm that the typography route appears exactly once in the required order, that `references/typography.md` matches the template, and that every prose-producing path deterministically loads it. For ineligible skills, confirm that both the reference and its route are absent. Confirm that domain-specific additions have a distinct local responsibility rather than reproducing broader host policy.
- **Authority paths:** Identify each ingestion source’s roles and authority status from current evidence, reusing applicable standalone dependency analysis when available. Trace every ingestion point through the selected workflow using those classifications. Confirm that every source whose authority status is untrusted reaches the instruction authority boundary before it can influence execution, mutation, relay behavior, or remote effects.
- **Sensitive and mutating paths:** Trace every mutating branch, opt-in, and sensitive operation to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
- **Stale guidance:** Check [stale guidance eligibility](#stale-guidance) against every reachable workflow, including inline interface instructions. For an eligible skill, confirm that the route template appears verbatim exactly once as the final subsection of `## General Policies`, that `references/guidance-recovery.md` matches its template, and that recovery loads only on the applicable failure path. Trace the ordinary successful path without loading that reference. Exercise every reachable staleness and recovery branch, including applicable access failures, guidance-specific outcomes, optional or supporting guidance, required guidance with and without a complete fallback, and an unavailable recovery reference. Preserve the precedence of workflow-specific failure procedures. For an ineligible skill, confirm that both templates and their routing are absent and that evidence limitations still receive complete handling. In either case, confirm that recovery remains scoped, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.

### Conditional and Installation Checks

- **Creation and promotion:** Apply the [public promotion validation](public-skill-promotion.md#validate-public-promotion) for creation or promotion, including reviews and audits of those operations. Don’t require the promotion contract’s standalone dependency analysis for other public maintenance.
- **Optional peers:** Run the [optional public peer validation](optional-public-peers.md#validate-optional-public-peers) when the scope includes creation, promotion, an audit of peer behavior, or a change affecting peer declarations, routing, retrieval, or fallback behavior. Cover every relationship within that affected contract, including unchanged declarations. When the optional peer template changes, this includes every derived reference.

### Mirror Alignment

When a global instruction, this public contract, or its conditional references change, search public skills for affected standalone defaults, required mirrors, and close semantic variants. Recheck each default’s local responsibility and combined behavior without expanding it into a complete global mirror. Preserve semantic or verbatim equivalence where its surface contract requires it. When a shared public policy template changes, align every verbatim template-derived copy under that template’s copy contract in the same change, including conditional references.

Propagation-only changes carry an established source update into its consumers without unrelated skill changes. Review the canonical wording once, then verify the complete affected consumer set for exact template correspondence or preserved local meaning, link resolution, and local integration. Check both standalone sufficiency and the maintained combined flow for changed defaults. Select behavioral checks for the contract changed by the source, not merely the mechanics of copying it. Apply whole-skill and optional peer checks only where that contract affects them. Reopen settled wording only when a consumer requires adaptation or exposes a concrete conflict, and review that delta and its integration boundaries rather than restarting unrelated whole-skill reviews.

For changes beyond propagation, re-evaluate ownership, standalone defaults, and required mirrors under [Resolve Standalone Policy](#resolve-standalone-policy). Preserve the conditions and boundaries needed by the selected local responsibility, add newly required standalone behavior on its narrowest applicable surface, and remove wording that no longer adds standalone value. Don’t expand the shared contract to mirror unrelated global instructions.
