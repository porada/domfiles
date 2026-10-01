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

Apply these command-specific choices and constraints within the base workflow:

1. Use the current repository and local `HEAD` as the target, including commits that have not been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. If required evidence cannot be inspected or a required check is omitted, state the limitation and stop before drafting unless the user explicitly authorizes a draft from the available evidence. Do not imply exhaustive verification.
3. Apply the [presentation conventions](#presentation-conventions).
4. By default, output only the ready-to-paste changelog Markdown, with any required disclosures or material evidence limitations reported separately. Drafting alone does not authorize file changes or submission.

## Presentation Conventions

- In release note prose outside headings, apply the [package link map](references/package-links.md) to package names.
- Use `*` for every unordered release note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release note failure.
