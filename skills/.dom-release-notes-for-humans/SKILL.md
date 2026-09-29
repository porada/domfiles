---
name: dom-release-notes-for-humans
description: |-
    Compose release notes for Dom’s packages.

disable-model-invocation: true
metadata:
    internal: true
---

# Release Notes

## Public Workflow

For every invocation, load `release-notes-for-humans` and follow its complete workflow. This overlay adds the `/dom-release-notes-for-humans` procedure, bullet rendering, and package links below. Apply the global **Explicit user direction** policy to task-scoped overrides of this overlay’s procedure and presentation conventions.

## `/dom-release-notes-for-humans` Command

The `/dom-release-notes-for-humans` command runs this complete procedure:

1. Use the current repository and local `HEAD` as the target, including commits that have not been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. Use `release-notes-for-humans` to resolve the affected publishable release units and their release boundaries. Stop and ask whenever that workflow requires user direction.
3. For each resolved release unit, either use the user-confirmed initial release status item or draft the release notes from a complete evidence inventory and material consumer outcomes. If required evidence cannot be inspected or a required check is omitted, state the limitation and stop before drafting unless the user explicitly authorizes a draft from the available evidence. Do not imply exhaustive verification.
4. Apply the [presentation conventions](#presentation-conventions).
5. By default, output only the ready-to-paste changelog Markdown, with any required disclosures or material evidence limitations reported separately. Drafting alone does not authorize file changes or submission.

## Presentation Conventions

- Use `*` for every unordered release note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release note failure.
- Render every semantic status item selected by the public skill with the same `*` marker.
- In release note prose outside headings, apply the [package link map](references/package-links.md) to package names.
