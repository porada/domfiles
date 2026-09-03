---
name: harmonize
description: |-
    Harmonize documentation, policies, and workflows across Dom’s projects.
disable-model-invocation: true
metadata:
    internal: true
---

# Repository harmonization

Changing the default scope does not change the workflow’s gates.

When the resolved scope includes agent documentation, follow `agent-documentation` for documentation authority, ownership, and composition within each repository.

## Resolve the change

- **Scope:** Run a change-oriented cross-repository consistency pass across Git repositories within the explicitly supplied project scope or location, defaulting to `~/Projects`. Use the named concept, policy, or documentation family. Without a narrower content scope, compare project-authored agent documentation expressing shared policies, workflows, or terminology. Start from Git-tracked files and exclude generated, managed, vendored, third-party, task-staging `.agent-*`, and untracked files unless explicitly included.
- **Eligibility:** Include a repository only when it has a tracked `AGENTS.md` and at least one configured Git remote points to `porada/*` or `standard-config/*`. Never establish `AGENTS.md` as part of a harmonization pass, because an ineligible repository is out of scope rather than one large gap to fill. After determining eligibility, present the complete eligible-repository list as an ordered list with a stable number for each repository, and stop for user confirmation before further inventory, comparison, or mutation. Treat only confirmed repositories as in scope, without waiving any other gate.
- **Checkout boundary:** Treat each in-scope repository’s primary worktree as its current checkout. Do not discover, compare, or mutate linked worktrees, including linked worktrees outside task-staging `.agent-*` paths.
- **Inventory gate:** Before editing any repository, complete the full read-only inventory, working-state check, and mutation-feasibility check. Read every applicable `AGENTS.md` and repository-specific authority model, and identify repositories with tracked in-scope surfaces.
- **Baseline:** Every baseline repository must satisfy eligibility and be user-confirmed. The shared instruction layer is the agent instructions every confirmed destination loads in common. Without an explicitly named baseline, use the repository that owns that layer only when it is already confirmed. Present any implicit or explicitly named baseline outside the confirmed set as a source-only candidate and stop for confirmation before inspecting it. Source-only confirmation permits only bounded read-only baseline comparison, not mutation. An item a confirmed baseline expresses that a confirmed destination lacks is a candidate gap rather than an out-of-scope omission.
- **Candidates:** Classify homologous items as semantically equivalent, intentionally repository-specific, or unresolved. Treat an item the baseline does not express as a consistency candidate only when at least two in-scope surfaces express the same observable meaning and role. Ambiguous policy and general quality defects remain outside scope unless the named family explicitly includes them. Inspect implementation and tests only as bounded evidence needed to establish observable meaning.
- **Canonical form:** For semantically equivalent items, select an existing formulation that completely expresses the shared meaning. Prefer explicit user-established wording, then the most authoritative applicable shared source, then the baseline repository’s formulation, then the most accurate and complete existing formulation. Treat newly synthesized wording as unresolved unless the user explicitly authorizes wording design. Make wording, terminology, ordering, placeholders, punctuation, and structure identical, substituting only unavoidable repository-specific identifiers. Do not infer equivalence from similar names or weaken, broaden, or otherwise change meaning, authority, behavior, or security boundaries to create uniformity.
- **Placement:** Before replicating an item across repositories, propose hoisting it to the shared instruction layer when the repository owning that layer is confirmed for mutation and the item’s meaning does not depend on repository-specific scope, disclosure, or identifiers. Prefer one hoisted rule over identical per-repository copies, and remove the copies it replaces. Keep the item repository-local when the source section would lose coherence without it, and apply a hoist only after explicit user approval.

## Apply and validate

- **Edit matrix:** Build the complete repository-and-file edit matrix before mutation.
- **Atomicity:** Apply each supported semantic family across every required safely writable repository as one coordinated unit. If a required destination is blocked or has overlapping work, leave that family unchanged everywhere and report it as unresolved.
- **Coordination and boundaries:** When a repository is unavailable to its required tools or protected-path workflow, relay its edit pass to an agent running there or stop before mutation. When the global evidence-isolation threshold is met, delegate inventory in small nonoverlapping groups, defaulting to one documentation-heavy repository per agent, and retain the authoritative comparison matrix in the coordinating conversation or one coordinator-owned task artifact. Follow every repository’s instructions, disclosure boundary, concurrent-work policy, protected-path workflow, and validation requirements. Do not transfer private facts or secret-bearing values between repositories.
- **Validation and report:** Validate every changed repository with targeted documentation or copy checks and `git diff --check`. Reread the complete compared family and confirm that every semantically equivalent item uses the canonical formulation. Report the canonical wording, repositories changed, intentional repository-specific variants, and unresolved meaning or authority decisions. Do not report discrepancies already resolved by the pass.
