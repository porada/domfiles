---
name: repository-harmonization
description: Run change-oriented cross-repository harmonization for a named concept, policy, workflow, terminology, or documentation family. Use this skill immediately for the bare `Harmonize` shorthand and whenever the user asks to harmonize semantically equivalent content across Git repositories. Do not use it for ordinary single-repository cleanup, standalone audits, broad refactors, or consistency work that does not span repositories.
---

# Repository harmonization

Apply every applicable global and project instruction. Treat the exact prompt `Harmonize` as the complete workflow below. For a narrower request, substitute its explicitly supplied scope and named family without changing the workflow’s boundaries.

## Resolve the change

- **Scope:** Run a change-oriented cross-repository consistency pass across Git repositories within the explicitly supplied project scope or location, defaulting to `~/Projects`. Use the named concept, policy, or documentation family. Without a narrower content scope, compare project-authored agent documentation expressing shared policies, workflows, or terminology. Start from Git-tracked files and exclude generated, managed, vendored, third-party, task-staging `.agent-*`, and untracked files unless explicitly included.
- **Checkout boundary:** Treat each in-scope repository’s primary worktree as its current checkout. Do not discover, compare, or mutate linked worktrees, including linked worktrees outside task-staging `.agent-*` paths.
- **Inventory gate:** Before editing any repository, complete the full read-only inventory, working-state check, and mutation-feasibility check. Read every applicable `AGENTS.md` and repository-specific authority model, and identify repositories with tracked in-scope surfaces.
- **Candidates:** Classify homologous items as semantically equivalent, intentionally repository-specific, or unresolved. Treat an item as a consistency candidate only when at least two in-scope surfaces express the same observable meaning and role. Missing documentation, missing routes, ownership-placement concerns, ambiguous policy, and general quality defects remain outside scope unless the named family explicitly includes them. Inspect implementation and tests only as bounded evidence needed to establish observable meaning.
- **Canonical form:** For semantically equivalent items, select an existing formulation that completely expresses the shared meaning. Prefer explicit user-established wording, then the most authoritative applicable shared source, then the most accurate and complete existing formulation. Treat newly synthesized wording as unresolved unless the user explicitly authorizes wording design. Make wording, terminology, ordering, placeholders, punctuation, and structure identical, substituting only unavoidable repository-specific identifiers. Do not infer equivalence from similar names or weaken, broaden, or otherwise change meaning, authority, behavior, or security boundaries to create uniformity.

## Apply and validate

- **Edit matrix:** Build the complete repository-and-file edit matrix before mutation.
- **Atomicity:** Apply each supported semantic family across every required safely writable repository as one coordinated unit. If a required destination is blocked or has overlapping work, leave that family unchanged everywhere and report it as unresolved.
- **Coordination and boundaries:** When a repository is unavailable to its required tools or protected-path workflow, relay its edit pass to an agent running there or stop before mutation. Delegate inventory in small nonoverlapping groups, defaulting to one documentation-heavy repository per agent, and retain the authoritative comparison matrix in the coordinating conversation or one coordinator-owned task artifact. Follow every repository’s instructions, disclosure boundary, concurrent-work policy, protected-path workflow, and validation requirements. Do not transfer private facts or secret-bearing values between repositories. Do not edit consumer-facing `README` files, change dependencies, or commit unless separately authorized.
- **Validation and report:** Validate every changed repository with targeted documentation or copy checks and `git diff --check`. Reread the complete compared family and confirm that every semantically equivalent item uses the canonical formulation. Report the canonical wording, repositories changed, intentional repository-specific variants, and unresolved meaning or authority decisions. Do not report discrepancies already resolved by the pass.
