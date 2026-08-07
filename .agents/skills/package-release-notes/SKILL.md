---
name: package-release-notes
description: Draft, edit, review, and refine accurate release-note Markdown for package repositories from existing notes or verified changes in a requested scope, usually since the latest tag. Use this skill whenever the user asks to write or revise a hosted release description such as a GitHub Release body, infer release notes from code changes, prepare a package changelog entry, or check package-release wording and consistency, regardless of package manager or registry.
---

# Package release notes

Do not publish a release, create a tag, or bump a version unless the user explicitly requests that mutation.

## Apply the core principle

Keep two distinct layers throughout the task:

- **Evidence inventory:** inspect the complete resolved change scope and record every verified release-relevant change, including details that may not appear in the final prose.
- **Release note:** select the smallest set of accurate, material consumer-facing outcomes. Omit subordinate behavior when it only substantiates a broader outcome and does not change a consumer decision. Include internal maintenance only when it affects package consumers or the user requests another emphasis.

## Choose the workflow

- To edit existing notes, treat the supplied file or text as the working draft and verify it against the relevant changes when repository evidence is available.
- To infer notes, honor the user’s explicit change range. Otherwise compare the nearest relevant published tag strictly preceding the requested target with that target, build the change inventory, and edit the supplied note target or return a new draft.
- For a review-only request, keep the task read-only and report evidence-backed omissions or concrete consistency outliers without rewriting the notes. Distinguish defects from intentional or harmless variations.

## Resolve the release scope

1. Identify the target repository, package or synchronized package set, release version or unreleased state, and target ref.
    - Honor an explicit user-provided change range.
    - Otherwise derive the nearest relevant published tag whose commit is an ancestor of and strictly older than the target. Use that tag through the target ref as the default range.
    - Never select the target’s own tag, a newer tag, or a tag outside the target’s history. If no preceding relevant tag exists, treat the scope as an initial release or ask when the history is ambiguous.
    - Do not invent a version. Use an explicit `Unreleased` label when the next version has not been chosen.
2. Detect the package manager and registry from repository configuration only when they are relevant to the evidence. Keep the note format independent of either.
3. In a monorepo, determine which packages actually publish together, which release tag applies to each package, and whether the repository uses one shared release note or package-specific notes.
4. Ask one focused question only when a missing release boundary or package scope cannot be resolved safely from the repository.

## Establish the repository’s voice

Inspect enough existing releases to identify stable conventions before editing or drafting:

- Read the most recent releases and a comparable release of the same type, such as the previous major or compatibility release.
- Record the usual sentence punctuation, heading hierarchy, terminology, level of detail, dependency-link style, and treatment of packages with no functional changes.
- Treat repeated conventions as the baseline.
- Do not convert generated platform boilerplate into a local convention unless the repository consistently retains it.

When no useful history exists, use the defaults in this skill.

## Build an evidence-backed change inventory

Use repository evidence to infer a note and to verify the completeness of an existing draft. Review the complete change range rather than relying on commit subjects alone:

- Inspect the diff from the resolved release boundary to the target ref. When editing existing notes, use that same scope to confirm their claims and identify material omissions.
- Inspect package manifests, workspace metadata, changesets, migration notes, public types, exports, tests, and relevant documentation.
- When a claim concerns published contents, exports, source maps, provenance, or other package artifacts that source and manifests cannot establish, inspect the repository’s packed or built output through its applicable workflow.
- Record consumer-facing capabilities, supported inputs, formatting behavior, APIs, configuration, interoperability, compatibility, fixes, breaking behavior, migrations, removed capabilities, changed defaults, and changes likely to reformat existing files.
- Include meaningful packaging changes such as removed source maps, corrected exports, build provenance, or changed engine and peer baselines when they affect consumption or match the repository’s established detail level.
- Record dependency updates only when they are release-relevant. Name the resulting compatibility, behavior, or security impact when known.
- Map implementation commits to a shared consumer-facing outcome only when source evidence supports that relationship. Otherwise, retain separate outcomes.
- Verify exact identifiers, package names, version ranges, rule names, option names, and links against source before using them.
- State uncertainty instead of turning an inference into a release-note claim.

If the complete release range or relevant artifact cannot be inspected, state the evidence boundary—for example, that the draft is based only on supplied commits—instead of implying exhaustive verification.

## Preserve epistemic precision

- Reserve `vulnerability`, `exploit`, and equivalent confirmed-security wording for validated findings. Describe unconfirmed defensive work as a guard or hardening against a potential issue.
- Use chronology and causality terms such as `after`, `following`, `in response to`, and `for compatibility with` only when evidence establishes that relationship.
- Preserve qualifiers such as `may`, `potential`, `optional`, and `some`. Avoid `all`, `every`, and other universal claims unless the resolved evidence supports them.
- When behavior may reformat existing consumer files, place a neutral warning such as “Some existing files may be reformatted as a result” near the relevant top-level summary rather than burying it in details.
- Inventory deprecation notices explicitly. Include one when it communicates material package status or a consumer action, and keep it concise.

## Apply approval gates

The approval gates below apply only to user-supplied or previously approved drafts. An explicit request to consolidate or restructure grants approval for that operation within the requested scope. Organize and consolidate an initial draft directly when evidence supports the result.

## Propose thematic consolidation

Look for bullets that verified source evidence or user-provided context shows are parts of one consumer-facing outcome. Similar vocabulary, adjacent placement, or a broad relationship is not enough. Do not invent an umbrella concept such as interoperability, compatibility, integration, or workflow to justify merging items.

A shorter thematic bullet is easier to scan only when it preserves every material outcome and does not blur separate compatibility, migration, or breaking effects. It does not need to enumerate supporting behaviors merely to demonstrate completeness.

When approval is required, use this proposal format:

**Before**

```text
* Added support for Prettier’s `checkIgnorePragma`, `insertPragma`, and `requirePragma` options.
* Fixed cursor positioning and partial-range formatting.
```

**After**

```text
* Improved support for Prettier’s native formatting controls, including pragma options, cursor positioning, and partial-range formatting.
```

**Evidence**

Explain the verified source or user context that connects the items.

**Approval**

Ask whether to apply the proposed wording.

Keep separate bullets when consolidation would hide a distinct consumer decision, downgrade a breaking change, combine unrelated package scopes, rely on an unsupported theme, or make the resulting sentence harder to scan. When context is insufficient, improve scanability by reordering the separate bullets rather than merging them.

## Protect approved structure

When approval is required for a major structural change, show the relevant before and after, explain the benefit, and ask whether to apply it before:

- Converting bullets to prose or prose to bullets.
- Introducing a named release theme.
- Creating, renaming, removing, or materially reorganizing a subsection or package section.
- Moving a change into or out of `All Packages`.

## Choose the smallest useful structure

Read the [release-structure reference](references/release-structures.md) when selecting or validating the note’s hierarchy. It defines the default shapes and rules for concise, themed, and synchronized multi-package releases.

## Order items for quick scanning

- Lead the overall note with its defining consumer-facing feature, breaking effect, or other material outcome. Put a likely-reformatting warning near the relevant summary.
- In a rule-heavy package section, put material peer or runtime requirements before routine rule maintenance when those requirements govern consumption.
- Group rule changes by action in this order when applicable: `Enabled`, `Re-enabled`, `Updated`, `Lowered`, then `Disabled`. Alphabetize rule identifiers within each action group.
- Put routine non-peer package or plugin updates at the end of the package section. Let material compatibility, migration, or security impact override that placement.
- Order other detail lists by their function and consumer impact rather than imposing one universal alphabetic scheme.

## Write concise consumer-facing prose

- Use `*` for every unordered release-note bullet.
- Write complete past-tense sentences with final punctuation.
- Start bullets with precise action verbs such as `Added`, `Bumped`, `Disabled`, `Dropped`, `Enabled`, `Fixed`, `Improved`, `Lowered`, `Preserved`, `Removed`, or `Updated`.
- Prefer the most informative verb. For example, write “Lowered the `engines` baseline” when a runtime minimum decreases rather than the vaguer “Updated the `engines` baseline.”
- Include the changed concept when it improves parallel wording. For example, write “Lowered `rule-name` severity to `warn`.”
- Use parallel wording for parallel changes without erasing intentional exceptions.
- Wrap package names, versions, options, rules, file patterns, errors, and other machine-readable tokens in backticks.
- Hyperlink a package name only when its source repository is under `github.com/standard-config/*` or `github.com/porada/*`. Keep every other external package name backticked and unlinked, especially in `Updated ...` bullets.
- Apply that allowlist only to package links. Continue to link advisories, migrations, specifications, pull requests, and other non-package evidence when the link helps consumers understand the change.
- Default to neutral language. Preserve intentional humor, repetition, or tone when the user identifies it as deliberate.
- Treat user-approved wording as authoritative. Do not silently rewrite portions outside the requested revision.

Avoid implementation narration, commit-by-commit summaries, unsupported marketing claims, and vague statements that hide consumer impact.

## Keep platform metadata out of the note body

Unless the user explicitly requests it, omit:

- The release version heading or tag link already rendered by the hosting platform.
- Package and repository URLs presented only as metadata.
- Publication timestamps, tag names, and registry timestamps.
- Generated `What’s Changed`, contributor, or full-changelog boilerplate.
- Empty headings and redundant summaries of the same change.
- Stale text copied from another version.

## Review before delivery

Before delivery, reapply:

- The [core principle](#apply-the-core-principle) and [evidence inventory](#build-an-evidence-backed-change-inventory).
- The repository’s [voice](#establish-the-repositorys-voice) and [epistemic precision](#preserve-epistemic-precision).
- The [approval gates](#apply-approval-gates), [thematic consolidation](#propose-thematic-consolidation), and [approved structure](#protect-approved-structure) rules.
- The [release structures](references/release-structures.md), [ordering](#order-items-for-quick-scanning), and [prose](#write-concise-consumer-facing-prose) rules.
- The [platform metadata](#keep-platform-metadata-out-of-the-note-body) exclusions.

Correct any discrepancy before delivering.

## Deliver the result

For an editing request with a file target, edit the existing note file or release section in place and validate the final heading hierarchy, release count, links, and absence of unwanted metadata.

For an editing or drafting request without a file target, put the ready-to-paste Markdown first in a fenced `markdown` block. Do not create a file unless the user requests one. Follow the Markdown only with concise uncertainties or decisions that require review.
