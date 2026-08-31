---
name: release-notes
description: Apply additional release note conventions whenever `release-notes-for-humans` applies, routing generic release evidence, structure, writing, and review through that skill. Also use it immediately for the exact bare `Changelog` shorthand.
metadata:
    internal: true
---

# Release notes

## Public workflow

For every invocation, load `release-notes-for-humans` and follow its complete workflow. This overlay changes only the shorthand, bullet rendering, and package links defined below.

## `Changelog` shorthand

Treat the exact prompt `Changelog` as this complete procedure:

1. Use the current repository and local `HEAD` as the target, including commits that have not been pushed to a remote. Exclude uncommitted changes unless explicitly requested.
2. Use `release-notes-for-humans` to resolve the affected publishable release units and their release boundaries. Stop and ask whenever that workflow requires user direction.
3. For each resolved release unit, use the user-confirmed initial-release status item or draft from a complete evidence inventory and material consumer outcomes. If a complete required range or artifact cannot be inspected, stop before drafting and state the evidence boundary instead of continuing to the output-only step.
4. Apply the [presentation conventions](#presentation-conventions).
5. Output only the ready-to-paste changelog Markdown, with nothing before or after it and without mutating or submitting anything.

## Presentation conventions

- Use `*` for every unordered release-note bullet. If a generic Markdown formatter’s only disagreement is normalizing this marker to `-`, preserve `*` and do not treat the marker-only result as a release-note failure.
- Render every semantic status item selected by the public skill with the same `*` marker.
- In release-note prose outside headings, apply the [package-link map](references/package-links.md) to package names. Keep versions, options, rules, file patterns, errors, and other machine-readable tokens backticked.
