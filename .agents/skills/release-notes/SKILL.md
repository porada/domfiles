---
name: release-notes
description: Draft, edit, review, and refine accurate package release notes and changelog entries from existing notes or verified changes. Use this skill immediately whenever the user asks for release notes, changelog notes, changelog entries, or release-ready prose for any change scope, including a single commit hash, commit range, pull request, branch, tag, diff, or changes since the latest release. Also use it for hosted release descriptions such as GitHub Release bodies and package-release wording or consistency checks, regardless of package manager or registry. Do not use it for ordinary commit summaries that are not intended as release or changelog prose.
---

# Package release notes

Do not publish a release, create a tag, or bump a version unless the user explicitly requests that mutation.

## Apply the core principle

Keep two distinct layers throughout the task:

- **Evidence inventory:** inspect the complete resolved change scope and record every verified release-relevant change, including details that may not appear in the final prose.
- **Release note:** draft from independently material consumer outcomes rather than converting inventory clusters into prose. Select the smallest accurate set. Omit subordinate behavior when it only substantiates a broader outcome and does not change a consumer decision. Include internal maintenance only when it affects package consumers or the user requests another emphasis.

Treat an outcome as material when it changes what consumers can do, how they configure or migrate, what output they should expect, or what compatibility they can rely on. Keep implementation mechanisms, verification cases, cosmetic diagnostics, and internal refinements in the inventory unless they create a distinct consumer action or boundary. Completeness of investigation does not require completeness of narration.

### Compression example

When one change set contains a new capability, a concrete behavior improvement, broad compatibility work, and supporting diagnostic cleanup, compress an evidence-shaped draft:

```text
* Added support for standalone formatting by loading the host’s `standalone` entry point.
* Fixed the `wrap: "always"` option for the `alpha-box`, `beta-box`, and `gamma-box` elements.
* Improved compatibility with other host plugins that format documents by composing lifecycle hooks and rejecting incompatible internal formats.
* Improved error messages by quoting file paths and changing separators.
```

into independent consumer outcomes:

```text
* Added support for standalone formatting.
* Improved wrapping around `<alpha-box>`, `<beta-box>`, and `<gamma-box>` elements.
* Improved compatibility with other document-formatting plugins.
```

The outcome-shaped version applies the [prose rules](#write-concise-consumer-facing-prose), orders capability → concrete behavior → compatibility, and leaves implementation proof and cosmetic diagnostics in the inventory.

## Choose the workflow

- To edit existing notes, treat the supplied file or text as the working draft and verify it against the relevant changes when repository evidence is available.
- To infer notes, honor the user’s explicit change scope. Otherwise compare the nearest relevant published tag strictly preceding the requested target with that target, build the change inventory, and edit the supplied note target or return a new draft.
- For a review-only request, keep the task read-only and report evidence-backed omissions or concrete consistency outliers without rewriting the notes. Distinguish defects from intentional or harmless variations.
- Apply this skill’s structure, ordering, and prose rules without inspecting prior release notes or changelog entries for local conventions. Inspect them only when they are the requested draft or the user explicitly asks for comparison or consistency.

## Resolve the release scope

1. Identify the target repository, package or synchronized package set, release version or unreleased state, and target ref.
    - Treat an explicitly requested lone commit as that commit’s diff against its first parent. For a root commit, use its complete contents.
    - Honor an explicit user-provided change range.
    - Otherwise derive the nearest relevant published tag whose commit is an ancestor of and strictly older than the target. Use that tag through the target ref as the default range.
    - Never select the target’s own tag, a newer tag, or a tag outside the target’s history. If no preceding relevant tag exists, treat the scope as an initial release or ask when the history is ambiguous.
    - Do not invent a version. Use an explicit `Unreleased` label when the next version has not been chosen.
2. Detect the package manager and registry from repository configuration only when they are relevant to the evidence. Keep the note format independent of either.
3. In a monorepo, determine which packages actually publish together, which release tag applies to each package, and whether the repository uses one shared release note or package-specific notes.
4. Ask one focused question only when a missing release boundary or package scope cannot be resolved safely from the repository.

## Build an evidence-backed change inventory

Use repository evidence to infer a note and to verify the completeness of an existing draft. Review the complete change range rather than relying on commit subjects alone:

- Inspect the diff from the resolved release boundary to the target ref. When editing existing notes, use that same scope to confirm their claims and identify material omissions.
- Inspect package manifests, workspace metadata, changesets, migration notes, public types, exports, tests, and relevant documentation.
- Treat added or expanded tests as verification, not proof that behavior changed. Promote a tested behavior only when source, artifacts, or before-and-after behavior establishes the consumer change.
- When a claim concerns published contents, exports, source maps, provenance, or other package artifacts that source and manifests cannot establish, inspect the repository’s packed or built output through its applicable workflow.
- Record consumer-facing capabilities, supported inputs, formatting behavior, APIs, configuration, interoperability, compatibility, fixes, breaking behavior, migrations, removed capabilities, changed defaults, and changes likely to reformat existing files.
- Include meaningful packaging changes such as removed source maps, corrected exports, build provenance, or changed engine and peer baselines when they affect consumption.
- Record dependency updates only when they are release-relevant. Name the resulting compatibility, behavior, or security impact when known.
- Map implementation commits to a shared consumer-facing outcome only when source evidence supports that relationship. Otherwise, retain separate outcomes.
- Verify exact identifiers, package names, version ranges, rule names, option names, and links against source before using them.
- State uncertainty instead of turning an inference into a release-note claim.

If the complete release range or relevant artifact cannot be inspected, state the evidence boundary—for example, that the draft is based only on supplied commits—instead of implying exhaustive verification.

## Preserve epistemic precision

- Reserve `vulnerability`, `exploit`, and equivalent confirmed-security wording for validated findings. Describe unconfirmed defensive work as a guard or hardening against a potential issue.
- Use chronology and causality terms such as `after`, `following`, `in response to`, and `for compatibility with` only when evidence establishes that relationship.
- Preserve qualifiers such as `may`, `potential`, `optional`, and `some`. Avoid `all`, `every`, and other universal claims unless the resolved evidence supports them.
- Add a general reformatting warning only when evidence establishes a material release-level risk beyond the output change already described. Do not infer one merely because a formatter fix changes output. Place a warranted warning such as “Some existing files may be reformatted as a result” near the relevant top-level summary rather than burying it in details.
- Inventory deprecation notices explicitly. Include one when it communicates material package status or a consumer action, and keep it concise.

## Apply approval gates

The approval gates below apply only to user-supplied or previously approved drafts. An explicit request to consolidate or restructure grants approval for that operation within the requested scope. Organize and consolidate an initial draft directly when evidence supports the result.

For those drafts, propose and obtain approval before:

- Removing or reclassifying a supplied release-note item, including treating a supplied refactor as internal-only.
- Removing a supplied rationale or exact dependency version.
- Adding, removing, or materially changing a consumer warning or evidence link.
- Strengthening or weakening a supplied technical claim or qualifier.

## Propose thematic consolidation

Look for bullets that verified source evidence or user-provided context shows are parts of one consumer-facing outcome. Similar vocabulary, adjacent placement, or a broad relationship is not enough. Do not invent an umbrella concept such as interoperability, compatibility, integration, or workflow to justify merging items.

A shorter thematic bullet is easier to scan only when it preserves every material outcome and does not blur separate compatibility, migration, or breaking effects. It does not need to enumerate supporting behaviors merely to demonstrate completeness.

Treat exact rule identifiers as material outcomes. Keep distinct rules in separate, sortable bullets rather than replacing them with a category summary. Multiple changes to the same rule may share one bullet when they form one coherent change and splitting them would obscure the relationship.

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
- Creating, renaming, removing, or materially reorganizing a heading or package section.
- Moving a change into or out of `All Packages`.

## Choose the smallest useful structure

Read the [release-structure reference](references/release-structures.md) when selecting or validating the note’s hierarchy. It defines the default shapes and rules for concise, aggregate, themed, and synchronized multi-package releases.

## Order items for quick scanning

- Lead the overall note with its defining consumer-facing feature, breaking effect, or other material outcome. Put a warranted reformatting warning near the relevant summary.
- In an ordinary mixed release, lead with new capabilities, then concrete output or behavior improvements, then broad compatibility outcomes, and put packaging last. Override this order only for a breaking change, an explicit release theme, or clearly greater consumer impact. Implementation complexity never determines prominence.
- In a rule-heavy package section, put material peer or runtime requirements before routine rule maintenance when those requirements govern consumption.
- Group rule changes by action in this order when applicable: `Enabled`, `Re-enabled`, `Updated`, `Lowered`, then `Disabled`. Alphabetize rule identifiers within each action group.
- Put routine non-peer package or plugin updates at the end of the package section. Let material compatibility, migration, or security impact override that placement.
- Order other detail lists by their function and consumer impact rather than imposing one universal alphabetic scheme.

## Write concise consumer-facing prose

- Use `*` for every unordered release-note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release-note failure.
- Write complete past-tense sentences with final punctuation.
- Start bullets with precise action verbs such as `Added`, `Bumped`, `Disabled`, `Dropped`, `Enabled`, `Expanded`, `Fixed`, `Improved`, `Lowered`, `Marked`, `Moved`, `Preserved`, `Re-enabled`, `Removed`, or `Updated`.
- Prefer the most informative verb. For example, write “Lowered the `engines` baseline” when a runtime minimum decreases rather than the vaguer “Updated the `engines` baseline.”
- Use `Fixed` only when a report, failing case, or before-and-after reproduction establishes a specific defect. Use `Improved` for broader stability or newly handled cases that were not established as a defect.
- Use `compatibility` for cross-plugin or general host-tool behavior. Use `support` for a named control or workflow. For a broad compatibility outcome, use the shortest familiar integration category and omit hook types, implementation variants, and verification cases unless a remaining boundary changes consumer use or configuration. Do not expand a familiar category into a host-tool name plus a descriptive clause.
- When a release changes peer dependency ranges or classifications, apply the [peer-dependency wording reference](references/peer-dependency-wording.md).
- Include the changed concept when it improves parallel wording. For example, write “Lowered `rule-name` severity to `warn`.”
- State any affected scope that is narrower than the package default, such as “in test files.” Repeat the package, rule, or configuration identifier when a pronoun would make the scope ambiguous.
- Keep a concise, evidence-backed rationale when it identifies a replacement, temporary upstream limitation, or responsibility transfer that helps consumers interpret a disablement or removal. For a replacement, use the parenthetical form `Disabled X (in favor of Y).`
- Verification does not make every identifier release-worthy. Name the capability rather than its module or API entry point when consumers do not need that identifier to act. Preserve the domain syntax of identifiers that remain, such as `<element>`, `--flag`, or `@scope/package`.
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
- The [epistemic precision](#preserve-epistemic-precision) rules.
- The [approval gates](#apply-approval-gates), [thematic consolidation](#propose-thematic-consolidation), and [approved structure](#protect-approved-structure) rules.
- The [release structures](references/release-structures.md), [ordering](#order-items-for-quick-scanning), and [prose](#write-concise-consumer-facing-prose) rules.
- The [platform metadata](#keep-platform-metadata-out-of-the-note-body) exclusions.

Correct any discrepancy before delivering.

## Deliver the result

For an editing request with a file target, edit the existing note file or release section in place and validate the final heading hierarchy, release count, links, and absence of unwanted metadata.

For an editing or drafting request without a file target, put the ready-to-paste Markdown first in a fenced `markdown` block. Do not create a file unless the user requests one. Follow the Markdown only with concise uncertainties or decisions that require review.
