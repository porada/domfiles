---
name: harmonize
description: |-
    Harmonize documentation, policies, and workflows across Dom’s projects.

disable-model-invocation: true
metadata:
    internal: true
---

# Repository Harmonization

Run a change-oriented consistency pass by default. An explicit review or comparison request remains read-only. When the resolved scope includes agent documentation, follow `agent-documentation` and its [repository independence contract](../../../skills/.domfiles-agent-documentation/references/repository-independence.md) for requirement selection, local ownership, shared wording, and new repository setup. Apply the global **Explicit user direction** policy to task-specific changes to this workflow.

## Resolve Repositories

- **Scope:** Use Git repositories within the explicitly supplied project scope or location, defaulting to `~/Projects`. Use the named concept, policy, or documentation family. Without a narrower content scope, compare project-authored agent documentation expressing shared policies, workflows, or terminology. Start from Git-tracked files and exclude agent task artifacts, generated files, managed files, third-party files, untracked files, and vendored files unless explicitly included. An authorized setup pass may create the necessary instruction surfaces.
- **Eligibility:** By default, discover repositories with tracked project-authored agent documentation and at least one configured Git remote pointing to `porada/*` or `standard-config/*`. A tracked `AGENTS.md` is not required. An explicitly selected repository may participate before it has agent documentation or a remote. Present the complete candidate list with stable repository numbers and distinguish source-only baselines from destinations. An explicit user selection confirms its stated role and covered effects. Ask before further inventory, comparison, or mutation only when the repository set or its role remains unconfirmed. Do not create files merely to satisfy discovery criteria.
- **Checkout boundary:** Preserve explicitly supplied checkout paths, including linked worktrees, rather than substituting a primary worktree. For directory-based discovery, do not expand through linked worktree registrations or inventory duplicate checkouts of one repository. When discovery finds multiple checkouts of one repository without an explicit selection, ask which to use during repository confirmation.
- **Inventory gate:** Before editing, complete the read-only inventory of the selected documentation family, working state check, and mutation feasibility check for every confirmed repository. Read every applicable `AGENTS.md` and repository-specific authority model. Identify existing owners, supported client entrypoints, and any necessary new surfaces without expanding into a general repository audit.
- **Baseline:** Use the explicitly selected baseline. Otherwise propose domfiles as the source-only baseline and obtain confirmation before inspecting it unless that role is already confirmed. A baseline need not be a destination. Source-only confirmation permits bounded read-only comparison, not mutation. Apply the same checkout and access boundaries to it. Its global policies and authoring tools are source material, not prerequisites for destination collaborators.

## Compare Requirements

- **Candidates:** Classify corresponding items using the shared wording rules below. An item a confirmed baseline expresses that a destination lacks is a candidate only when it may supply a required output property, not merely because the maintainer uses it. Treat an item the baseline does not express as a consistency candidate only when at least two in-scope surfaces express the same observable meaning and role. Preserve intentionally different requirements. Ambiguous policy and general quality defects remain outside scope unless the named family explicitly includes them. Inspect implementation and tests only as bounded evidence needed to establish observable meaning.
- **Canonical form and placement:** Use the [shared wording rules](../../../skills/.domfiles-agent-documentation/references/repository-independence.md#align-shared-wording) to select and reproduce equivalent formulations. Outside agent documentation, retain the applicable domain’s semantic owner and distribution model. Do not resolve an unsettled acceptance requirement by silently copying the baseline or hoisting it into the maintainer’s global instructions. Keep source-only repositories unchanged.

## Establish Repository Documentation

When the user requests setup or connection of a new repository, use the same confirmed repository set and baseline rather than requiring a preliminary documentation migration. Follow the repository independence contract to select the minimum local instruction surfaces and supported client bridges. Include proposed creations and their separately required permissions in the edit matrix before mutation. A setup request does not authorize every baseline rule, a new skill, or client configuration changes merely for uniformity.

Connection means selecting the repository and its authoring baseline for this pass. Future passes rediscover its project-authored surfaces through the same eligibility rules. Do not add a repository registry, synchronization manifest, generator, or runtime dependency on domfiles merely to record that relationship. An explicit future selection can include a repository outside default discovery.

## Apply and Validate

- **Edit matrix:** Before mutation, record one complete repository-and-file matrix identifying each requirement’s canonical source, local destination, proposed change or creation, and any permission or access requirement. Keep unresolved choices and intentional variants distinct from approved edits. A review-only pass records findings rather than applying the matrix.
- **Atomicity:** By default, apply each family of semantically equivalent items across every required safely writable repository as one coordinated unit. If a required destination is unavailable or has overlapping work, leave that family unchanged everywhere and report the limitation. When the user expressly authorizes a partial pass, apply it only to the named safely writable targets and report the remaining alignment gap. Do not overwrite overlapping work.
- **Coordination and boundaries:** When a repository is unavailable to its required tools or protected-path workflow, relay its edit pass to an agent running there or stop before mutation. When the global evidence isolation threshold is met, delegate inventory in small nonoverlapping groups, defaulting to one documentation-heavy repository per agent, and retain the authoritative edit matrix in the coordinating conversation or one coordinator-owned task artifact. Follow every repository’s instructions, disclosure boundary, concurrent work policy, protected-path workflow, and validation requirements. Do not transfer private facts or secret-bearing values between repositories.
- **Validation and report:** Use `agent-documentation`’s validation workflow, including repository independence checks, for affected agent documentation. For other selected families, use the applicable domain checks and `git diff --check`. Reread the complete compared family and confirm that every semantically equivalent item uses the canonical formulation. Report the source and destination roles, changed repositories, canonical ownership, intentional variants, unresolved decisions, and material verification limits. Distinguish source-based checks from live client behavior. Do not report discrepancies already resolved by the pass.
