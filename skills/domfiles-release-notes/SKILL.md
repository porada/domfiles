---
name: release-notes
description: Draft, edit, review, and refine accurate package release notes and changelog entries from existing notes or verified changes. Use this skill immediately for the bare `Changelog` shorthand or whenever the user asks for release notes, changelog notes, changelog entries, or release-ready prose for any change scope, including a single commit hash, commit range, pull request, branch, tag, diff, or changes since the latest release. Also use it for hosted release descriptions such as GitHub Release bodies and package-release wording or consistency checks, regardless of package manager or registry. Do not use it for ordinary commit summaries that are not intended as release or changelog prose.
metadata:
    internal: true
---

# Package release notes

Follow the `github-cli` workflow when a release task uses GitHub CLI. Do not publish a release, create a tag, or bump a version unless the user explicitly requests that mutation.

## Apply the `Changelog` shorthand

Treat the exact prompt `Changelog` as this complete procedure:

1. Use the current repository and local `HEAD` as the target, including commits that have not been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. Resolve the affected publishable package scope and its release boundary or boundaries through [Resolve the release scope](#resolve-the-release-scope), using those defaults. Stop and ask whenever that workflow requires user direction.
3. For each resolved release unit, use the [initial-release marker](references/release-structures.md) when directed, or apply the [core principle](#apply-the-core-principle) to a complete [evidence inventory](#build-an-evidence-backed-change-inventory). If a complete required range or artifact cannot be inspected, stop before drafting and state the evidence boundary instead of continuing to the output-only step.
4. Output only the ready-to-paste changelog Markdown using the [draft delivery form](#deliver-the-result), with nothing before or after it and without mutating or submitting anything.

## Apply the core principle

For each non-initial release unit, keep two distinct layers throughout the task:

- **Evidence inventory:** inspect the complete resolved change scope and record every verified release-relevant change, including details that may not appear in the final prose.
- **Release note:** draft from independently material consumer outcomes rather than converting inventory clusters into prose. Select the smallest accurate set. Omit subordinate behavior when it only substantiates a broader outcome and does not change a consumer decision. Include internal maintenance only when it affects package consumers or the user requests another emphasis.

Treat an outcome as material when it changes what consumers can do, how they configure or migrate, what output they should expect, or what compatibility they can rely on. Keep implementation mechanisms, verification cases, cosmetic diagnostics, and internal refinements in the inventory unless they create a distinct consumer action or boundary. Completeness of investigation does not require completeness of narration.

## Choose the workflow

- To edit existing notes, treat the supplied file or text as the working draft. Apply the [approval gates](references/approval-gates.md) to a user-supplied or previously approved draft, and use the [evidence inventory](#build-an-evidence-backed-change-inventory) to verify it when repository evidence is available.
- To infer notes, honor the user’s explicit change scope. Otherwise [resolve the release scope](#resolve-the-release-scope), build the change inventory for the resolved range or ranges, and edit the supplied note target or return a new draft.
- For a review-only request, keep the task read-only and report evidence-backed omissions or concrete consistency outliers without rewriting the notes. Distinguish defects from intentional or harmless variations.
- Apply this skill’s structure, ordering, and prose rules without inspecting prior release notes or changelog entries for local conventions. Inspect them only when they are the requested draft or the user explicitly asks for comparison or consistency.

## Resolve the release scope

1. Identify the target repository, release version or unreleased state, and target ref.
    - If the target ref cannot be resolved from the request or repository, stop and ask for direction.
    - Do not invent a version. Use an explicit `Unreleased` label when the next version has not been chosen.
2. Resolve the target publishable packages and release grouping before selecting any default release boundary.
    - Use workspace manifests, release configuration, tag conventions, and package paths to determine whether packages release independently or as one synchronized set. Treat each independent package and each synchronized package set as a separate release unit.
    - If the request does not identify a package, treat each publishable release unit as a candidate. Resolve its boundary before using its range in step 5 to determine whether it has unreleased changes.
    - If package ownership or release grouping remains unclear, stop and ask for direction.
3. Resolve the release boundary for each release unit.
    - Treat an explicitly requested lone commit as that commit’s diff against its first parent. For a root commit, use its complete contents.
    - Honor an explicit user-provided change range.
    - Otherwise derive one shared preceding tag for a synchronized package set and a separate package-relevant preceding tag for each independent package. Each tag must be an ancestor of and strictly older than the target. Use each tag through the target ref as that release unit’s default range.
    - Never select the target’s own tag, a newer tag, or a tag outside the target’s history. If no preceding relevant tag can be located for a release unit, stop and ask for a release boundary or confirmation that it is an initial release. Do not infer an initial release from a missing tag.
    - When the user identifies a release unit as an initial release, intentionally stop inspecting its change history and source. Resolve only the package and release structure needed to use the exact [initial-release marker](references/release-structures.md).
4. Detect the package manager and registry from repository configuration only when they are relevant to the evidence. Keep the note format independent of either.
5. Map the changes from each resolved range to its publishable packages. If the affected package scope remains unclear, stop and ask for direction.

## Build an evidence-backed change inventory

Use repository evidence to infer a note and to verify the completeness of an existing draft. Review every complete non-initial change range rather than relying on commit subjects alone:

- Inspect the diff from each resolved release boundary to the target ref. When editing existing notes, use those same scopes to confirm their claims and identify material omissions.
- Inspect package manifests, workspace metadata, changesets, migration notes, public types, exports, tests, and relevant documentation.
- Treat added or expanded tests as verification, not proof that behavior changed. Promote a tested behavior only when source, artifacts, or before-and-after behavior establishes the consumer change.
- When a claim concerns published contents, exports, source maps, provenance, or other package artifacts that source and manifests cannot establish, inspect the repository’s packed or built output through its applicable workflow.
- Record consumer-facing capabilities, supported inputs, formatting behavior, APIs, configuration, interoperability, compatibility, fixes, breaking behavior, migrations, removed capabilities, changed defaults, and changes likely to reformat existing files.
- Include meaningful packaging changes such as removed source maps, corrected exports, build provenance, or changed engine and peer baselines when they affect consumption.
- Apply the [dependency update policy](#handle-dependency-updates).
- Use the [thematic-consolidation criteria](#propose-thematic-consolidation) when mapping implementation changes to consumer-facing outcomes.
- Verify exact identifiers, package names, version ranges, rule names, option names, and links against source before using them.
- State uncertainty instead of turning an inference into a release-note claim.

If any complete release range or relevant artifact cannot be inspected, state the evidence boundary—for example, that the draft is based only on supplied commits—instead of implying exhaustive verification.

## Handle dependency updates

- Never fetch or inspect a dependency’s changelog, release notes, repository history, or announcements to justify an item merely because its version changed.
- Always omit development-only dependency updates and all transitive dependency updates, including those represented only in a lockfile.
- Include one routine `Updated` bullet for each direct runtime dependency update not already subsumed by a material consumer-facing outcome. Omit versions and upstream-change summaries.
- Alphabetize routine dependency bullets by package name and place them at the end of the applicable package section.
- Apply the [peer-dependency wording reference](references/peer-dependency-wording.md) instead when a release changes peer dependency ranges or classifications.

## Preserve epistemic precision

- Reserve `vulnerability`, `exploit`, and equivalent confirmed-security wording for validated findings. Describe unconfirmed defensive work as a guard or hardening against a potential issue.
- Use chronology and causality terms such as `after`, `following`, `in response to`, and `for compatibility with` only when evidence establishes that relationship.
- Preserve qualifiers such as `may`, `potential`, `optional`, and `some`. Avoid `all`, `every`, and other universal claims unless the resolved evidence supports them.
- Add a general reformatting warning only when evidence establishes a material release-level risk beyond the output change already described. Do not infer one merely because a formatter fix changes output. Place a warranted warning such as “Some existing files may be reformatted as a result” near the relevant top-level summary rather than burying it in details.
- Inventory deprecation notices explicitly. Include one when it communicates material package status or a consumer action, and keep it concise.

## Propose thematic consolidation

Look for bullets that verified source evidence or user-provided context shows are parts of one consumer-facing outcome. Similar vocabulary, adjacent placement, or a broad relationship is not enough. Do not invent an umbrella concept such as interoperability, compatibility, integration, or workflow to justify merging items.

Treat exact rule identifiers as material outcomes. Keep distinct rules in separate, sortable bullets rather than replacing them with a category summary. Multiple changes to the same rule may share one bullet when they form one coherent change and splitting them would obscure the relationship.

When approval is required, use the [gated-consolidation proposal](references/approval-gates.md#propose-a-gated-consolidation).

Keep separate bullets when consolidation would hide a distinct consumer decision, downgrade a breaking change, combine unrelated package scopes, or make the resulting sentence harder to scan. When context is insufficient, improve scanability by reordering the separate bullets rather than merging them.

## Choose the smallest useful structure

Choose the smallest structure that communicates each material consumer decision. Use a flat bullet list when every change belongs to one package and no thematic section improves comprehension.

Read the [release-structure reference](references/release-structures.md) for an initial release or initial package section, aggregate release-note file, clear theme, migration context, substantial group of related changes, nested hierarchy, hosted release body, repeated migration guidance, or synchronized multi-package release.

## Order items for quick scanning

- Lead the overall note with its defining consumer-facing feature, breaking effect, or other material outcome.
- In an ordinary mixed release, lead with new capabilities, then concrete output or behavior improvements, then broad compatibility outcomes, and put packaging last. Override this order only for a breaking change, an explicit release theme, or clearly greater consumer impact. Implementation complexity never determines prominence.
- In a rule-heavy package section, put material peer or runtime requirements before routine rule maintenance when those requirements govern consumption.
- Group rule changes by action in this order when applicable: `Enabled`, `Re-enabled`, `Updated`, `Lowered`, then `Disabled`. Alphabetize rule identifiers within each action group.
- Order other detail lists by their function and consumer impact rather than imposing one universal alphabetic scheme.

## Write concise consumer-facing prose

- Use `*` for every unordered release-note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release-note failure.
- Write complete past-tense sentences with final punctuation.
- Start bullets with precise action verbs such as `Added`, `Bumped`, `Disabled`, `Dropped`, `Enabled`, `Expanded`, `Fixed`, `Improved`, `Lowered`, `Marked`, `Moved`, `Preserved`, `Re-enabled`, `Removed`, or `Updated`.
- Prefer the most informative verb. For example, write “Lowered the `engines` baseline” when a runtime minimum decreases rather than the vaguer “Updated the `engines` baseline.”
- Use `Fixed` only when a report, failing case, or before-and-after reproduction establishes a specific defect. Use `Improved` for broader stability or newly handled cases that were not established as a defect.
- Use `compatibility` for cross-plugin or general host-tool behavior. Use `support` for a named control or workflow. For a broad compatibility outcome, use the shortest familiar integration category and omit hook types, implementation variants, and verification cases unless a remaining boundary changes consumer use or configuration. Do not expand a familiar category into a host-tool name plus a descriptive clause.
- When an exact identifier is material, lead with it and its necessary qualifiers before broader package or integration context. Remove a generic phrase such as “handling of” when the sentence remains accurate without it.
- Include the changed concept when it improves parallel wording. For example, write “Lowered `rule-name` severity to `warn`.”
- State any affected scope that is narrower than the package default, such as “in test files.” Repeat the package, rule, or configuration identifier when a pronoun would make the scope ambiguous.
- Omit package, publication, or other scope already established by the release surface or heading unless the bullet narrows or contrasts that scope.
- Keep a concise, evidence-backed rationale when it identifies a replacement, temporary upstream limitation, or responsibility transfer that helps consumers interpret a disablement or removal. For a replacement, use the parenthetical form `Disabled X (in favor of Y).`
- Verification does not make every identifier release-worthy. Name the capability rather than its module or API entry point when consumers do not need that identifier to act. Preserve the domain syntax of identifiers that remain, such as `<element>`, `--flag`, or `@scope/package`.
- Use parallel wording for parallel changes without erasing intentional exceptions.
- In release-note prose outside headings, apply the [package-link map](references/package-links.md) to package names and wrap versions, options, rules, file patterns, errors, and other machine-readable tokens in backticks. Follow the canonical heading forms in the [release-structure reference](references/release-structures.md) without adding code formatting.
- Default to neutral language. Preserve intentional humor, repetition, or tone when the user identifies it as deliberate. Avoid commit-by-commit summaries, unsupported marketing claims, and vague statements that hide consumer impact.

## Keep platform metadata out of the note body

Unless the user explicitly requests it, omit:

- The release version heading or tag link already rendered by the hosting platform.
- Package and repository URLs presented only as metadata.
- Publication timestamps, tag names, and registry timestamps.
- Generated `What’s Changed`, contributor, or full-changelog boilerplate.
- Empty headings and redundant summaries of the same change.
- Stale text copied from another version.

## Review before delivery

Before delivery, reapply every rule above, including the [approval gates](references/approval-gates.md) for user-supplied or previously approved drafts and any applicable [conditional release-structure rules](references/release-structures.md).

## Deliver the result

For an editing request with a file target, edit the existing note file or release section in place and validate the final heading hierarchy, release count, links, and absence of unwanted metadata.

For an editing or drafting request without a file target, put the ready-to-paste Markdown first in a fenced `markdown` block. Do not create a file unless the user requests one. Follow the Markdown only with concise uncertainties or decisions that require review.
