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

Keep the source policy as the owner of broader behavior and identify the local default’s narrower responsibility during maintenance. Retain a complete standalone mirror only when an explicit surface contract requires semantic or verbatim equivalence, such as the editorial conventions contract below. Neither shortening nor classification authorizes weakening a security boundary.

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

Use the [general policies template](../assets/general-policies.txt) as the shared minimal standalone default. Keep essential boundaries for authority, scope, explicit user direction, and security inline, not the host’s complete policy. Keep domain-specific safeguards in their owning workflows and detailed recovery guidance behind a conditional route. Add shared wording only when the common standalone behavior needs it, not merely because a global policy contains it.

Replace an existing general policies section rather than supplementing it. Copy the template verbatim except for the heading choice, eligibility omissions, and editorial inlining defined below. Use `## General Policies` when the skill’s peer H2 headings name topics and `## Apply General Policies` when they name actions. Keep this section last and preserve the template’s subsection order and hierarchy.

Determine eligibility from the complete reachable workflow. Global policies retain ownership of the broader rules, and the templates own the shared standalone wording. Bundled copies must remain sufficient without another skill or the network. When a template conflicts with the required composition, update its canonical contract and every governed copy together rather than adding a skill-specific exception.

### Editorial Conventions

Include editorial conventions only when creating, editing, or reviewing prose for human readers is a core advertised capability or the skill defines standards for human-facing text in authored code. Producing or transporting instructions addressed to another agent, or prose as a supporting step in another workflow, does not, by itself, establish eligibility. Otherwise, inherit applicable user and project writing instructions. Standalone completeness alone does not justify editorial defaults.

For maintenance in domfiles, derive the [editorial conventions template](../assets/editorial-conventions.txt) from `.agents/GLOBAL.md`’s final `## Editorial Conventions` section through the end of the file. Reduce that heading and every descendant heading level by one, preserving all remaining content verbatim. Make shared rule changes in the global source before updating the template and its consumers.

For eligible skills, inline the conventions when every advertised workflow requires them. Assess the complete reachable workflow, not the skill’s name or category. Don’t count supporting steps or exceptional early exits as separate advertised workflows.

To inline, replace the general policies template’s Editorial Conventions subsection in place. Retain its heading and introductory paragraph, changing only the linked `editorial conventions` text to plain text. Follow that paragraph with the editorial template’s body, omitting its H1 and increasing descendant heading levels by two. This preserves the applicability condition and routine-communication exclusion. Don’t retain a separate editorial reference.

For other eligible skills, retain the template’s conditional route and bundle a verbatim copy of the editorial template at `references/editorial-conventions.md`. For ineligible skills, omit both the subsection and the reference. Each eligible skill must contain exactly one copy of the conventions. Apply narrower rules when the conventions permit them, without changing the shared rule text.

### Agent Conduct

Include the template’s core Agent Conduct paragraphs and Secrets and Authentication subsection verbatim in every public skill’s fully loaded `SKILL.md`. Domain guidance may add stricter constraints or surface-specific applications, but it must not replace, paraphrase, or weaken the shared rules.

### Guidance Recovery

Include guidance recovery when a reachable workflow depends on operational assets or peer guidance, follows bundled or remote instructional references, or prescribes external interface behavior, including file formats, that its directions depend on. Evaluate inline instructions as well as links. Task evidence isn’t itself a dependency on skill guidance, and the absence of references doesn’t establish ineligibility.

For eligible skills, retain the template’s Guidance Recovery subsection and bundle a verbatim copy of the [guidance recovery template](../assets/guidance-recovery.txt) at `references/guidance-recovery.md`. Load the complete entrypoint before following any reference, but load the recovery reference only when its failure condition occurs. Preserve any workflow-specific failure procedure, including optional peer recovery, rather than replacing it with the generic route. The inline Guidance Recovery subsection supplies the safe fallback when the recovery reference itself is unavailable.

For ineligible skills, omit both the subsection and the reference. Keep evidence limitations and applicable authorization, retrieval, and security boundaries complete in the workflow rather than adding unused recovery behavior.

## Validate Public Portability

Select the checks below by the affected public contract. A whole-skill audit, creation, or promotion requires the complete applicable set. For propagation-only changes, use [Mirror Alignment](#mirror-alignment).

### Whole-Skill Checks

- **Description and independent behavior:** Validate the complete decoded description against the [shared description contract](skill-descriptions.md) and the [public description portability contract](#keep-public-descriptions-portable). Evaluate the skill with network access, optional peers, repository-managed policy, and source repository files removed. Its advertised behavior must remain complete. For changed inherited guidance or standalone defaults, also verify the maintained combined flow under [Resolve Standalone Policy](#resolve-standalone-policy).
- **Shared public policies:** Confirm that one final general policies section matches the template after only the permitted heading choice, eligibility omissions, and editorial inlining. Check that the H2 matches the peer heading style and that mandatory conduct and security rules appear exactly once in the required hierarchy and order. Confirm that domain-specific additions have a distinct local responsibility rather than reproducing broader host policy.
- **Editorial conventions:** Check [eligibility and placement](#editorial-conventions) against the advertised capabilities and complete reachable workflow. For inline consumers, confirm that the conventions replace the routing subsection in place, preserve its applicability and routine-communication exclusion, match the editorial template after the specified heading transformation, and have no separate reference copy. For reference consumers, confirm that the bundled file matches the template and the route loads it before applicable wording work. For ineligible skills, confirm that the subsection and reference are absent. Check affected links and local workflow instructions so they neither point to retired locations nor broaden the shared applicability.
- **Authority paths:** Identify each ingestion source’s roles and authority status from current evidence, reusing applicable standalone dependency analysis when available. Trace every ingestion point through the selected workflow using those classifications. Confirm that every source whose authority status is untrusted reaches the instruction authority boundary before it can influence execution, mutation, relay behavior, or remote effects.
- **Sensitive and mutating paths:** Trace every mutating branch, opt-in, and sensitive operation to a terminal action or required stop. Do the same for an exception only when it bypasses an authorization or safety boundary or can reach a sensitive or mutating operation. Confirm who acts, what authorization is required, whether execution is agent-run or user-run, and whether standalone behavior remains complete without optional policies or peers.
- **Guidance recovery:** Check [recovery eligibility](#guidance-recovery) against every reachable workflow, including inline interface instructions. For eligible skills, confirm that the template’s recovery subsection appears verbatim exactly once as the final subsection under Agent Conduct, that `references/guidance-recovery.md` matches its template, and that recovery loads only on the applicable failure path. Trace the ordinary successful path without loading that reference. Exercise every reachable defect and recovery branch, including applicable access failures, guidance-specific outcomes, optional or supporting guidance, required guidance with and without a complete fallback, and an unavailable recovery reference. Preserve the precedence of workflow-specific failure procedures. For ineligible skills, confirm that the subsection and reference are absent and that evidence limitations still receive complete handling. In either case, confirm that recovery remains scoped, cannot infer missing content or substitute an unverified location, and cannot weaken a boundary.

### Conditional and Installation Checks

- **Creation and promotion:** Apply the [public promotion validation](public-skill-promotion.md#validate-public-promotion) for creation or promotion, including reviews and audits of those operations. Don’t require the promotion contract’s standalone dependency analysis for other public maintenance.
- **Optional peers:** Run the [optional public peer validation](optional-public-peers.md#validate-optional-public-peers) when the scope includes creation, promotion, an audit of peer behavior, or a change affecting peer declarations, routing, retrieval, or fallback behavior. Cover every relationship within that affected contract, including unchanged declarations. When the optional peer template changes, this includes every derived reference.

### Mirror Alignment

When a global instruction, this public contract, or its conditional references change, search public skills for affected standalone defaults, required mirrors, and close semantic variants. Recheck each default’s local responsibility and combined behavior without expanding it into a complete global mirror. Preserve semantic or verbatim equivalence where its surface contract requires it. When a shared public policy template changes, align every verbatim template-derived copy under that template’s copy contract in the same change, including conditional references.

Propagation-only changes carry an established source update into its consumers without unrelated skill changes. Review the canonical wording once, then verify the complete affected consumer set for exact template correspondence or preserved local meaning, link resolution, and local integration. Check both standalone sufficiency and the maintained combined flow for changed defaults. Select behavioral checks for the contract changed by the source, not merely the mechanics of copying it. Apply whole-skill and optional peer checks only where that contract affects them. Reopen settled wording only when a consumer requires adaptation or exposes a concrete conflict, and review that delta and its integration boundaries rather than restarting unrelated whole-skill reviews.

For changes beyond propagation, re-evaluate ownership, standalone defaults, and required mirrors under [Resolve Standalone Policy](#resolve-standalone-policy). Preserve the conditions and boundaries needed by the selected local responsibility, add newly required standalone behavior on its narrowest applicable surface, and remove wording that no longer adds standalone value. Don’t expand the shared contract to mirror unrelated global instructions.
