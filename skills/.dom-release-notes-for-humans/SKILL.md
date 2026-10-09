---
name: dom-release-notes-for-humans
description: |-
    Use whenever `release-notes-for-humans` applies.

metadata:
    internal: true
---

# Release Notes

## Public Workflow

For every invocation, load `release-notes-for-humans` and follow its complete workflow. This overlay adds scope and delivery defaults, bullet rendering, and package links below. Apply the global **Explicit user direction** policy to task-scoped overrides of this overlay’s procedure and presentation conventions.

## Workflow Defaults

Apply these defaults and constraints within the base workflow:

1. When the base workflow needs a default evidence scope, use the current repository and local `HEAD` as the target, including commits that have not been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. If required evidence cannot be inspected or a required check is omitted, state the limitation and stop before drafting unless the user explicitly authorizes a draft from the available evidence.
3. Apply the [presentation conventions](#presentation-conventions).
4. For drafting or editing without a file target, default to ready-to-paste changelog Markdown. Present approval proposals and questions, decisions requiring review, required disclosures, and material evidence limitations separately from the changelog Markdown. Omit other commentary. Use the base workflow’s delivery for file edits and read-only reviews. Drafting alone does not authorize file changes or submission.

## Presentation Conventions

- In release note prose outside headings, apply the [package link map](references/package-links.md) to package names.
- Use `*` for every unordered release note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release note failure.
