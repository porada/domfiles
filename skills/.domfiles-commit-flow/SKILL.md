---
name: commit-flow
description: |-
    Use whenever `sensible-commit-flow` applies.

metadata:
    internal: true
---

# Commit Flow

Load `sensible-commit-flow` and apply these personal conventions to newly authored messages. The public skill owns the workflow, supplied-message safeguards, confirmation, execution, and verification. These conventions take precedence over optional conventions inferred from repository history, not mandatory repository requirements or exact supplied wording.

## Message Form

- **Subject:** Write one compact, sentence case imperative clause, normally `<verb> <object>`, except for the recurring subjects below. Omit articles that add no meaning, colons, Conventional Commit prefixes, scope labels, and terminal punctuation. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Body:** The subject is the complete newly authored message. Do not add an explanatory body, issue reference paragraphs, testing checklists, or trailers. Route a conflict with supplied attribution or another required message element through the public skill’s message safeguards rather than discarding it.

## Recurring Subjects

- **Dependencies:** Use `Update dependencies` for a dependency maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Repository roots:** Use `Initial commit` for a repository root.
- **Releases:** Use a bare semantic version only for an actual release commit when that form is established in the repository.
