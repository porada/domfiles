---
name: dom-release-notes-for-humans
description: |-
    Use whenever `release-notes-for-humans` applies.

metadata:
    internal: true
---

# Release Notes

## Public Workflow

For every invocation, load `release-notes-for-humans` and follow its complete workflow. This overlay adds scope and delivery defaults, release version recommendations, bullet rendering, and package links below. Apply the global **Explicit user direction** policy to task-scoped overrides of this overlay’s procedure and presentation conventions.

## Workflow Defaults

Apply these defaults and constraints within the base workflow:

1. When the base workflow needs a default evidence scope, use the current repository and local `HEAD` as the target, including commits that haven’t been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. Apply the [presentation conventions](#presentation-conventions).
3. When drafting or editing without a file target, default to ready-to-paste changelog Markdown. Present approval proposals and questions, decisions requiring review, required disclosures, material evidence limitations, and applicable [release version recommendations](#release-version-recommendations) separately from the changelog Markdown. Omit other commentary. Use the base workflow’s delivery for file edits and read-only reviews. Drafting alone doesn’t authorize file changes or submission.

## Release Version Recommendations

When drafting or editing notes for a non-initial release that hasn’t been published, recommend a version for each release unit without an assigned version when the evidence permits. If a version is already assigned, report any material mismatch with the evidence or versioning policy instead of proposing a replacement. Follow the project’s versioning policy, including its rules for versions below `1.0.0`, prereleases, and synchronized packages. For stable releases using Semantic Versioning, recommend the smallest version increment that covers the most significant verified change: major for incompatible changes, minor for compatible additions or public API deprecations, or patch for compatible fixes.

Give the release type and a brief rationale. Include an exact next version only when the previous release version and applicable policy are established. When a limited evidence scope or unclear policy prevents a firm recommendation, state that limitation or qualify the recommendation instead of implying a complete release assessment.

Deliver the recommendation after the changelog Markdown or in the response accompanying a file edit, outside the release note itself. A recommendation doesn’t select or change the release version. In read-only reviews, discuss versioning only when it supports a concrete finding under the base workflow’s review contract.

## Presentation Conventions

- In release note prose outside headings, apply the [package link map](references/package-links.md) to package names.
- Use `*` for every unordered release note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and don’t treat the marker-only result as a release note failure.
