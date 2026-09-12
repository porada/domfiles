---
name: commit-flow
description: |-
    Use whenever `sensible-commit-flow` applies.

metadata:
    internal: true
---

# Commit Flow

Load `sensible-commit-flow` and apply these personal conventions to newly authored messages. The public skill owns the workflow, supplied-message safeguards, confirmation, execution, and verification. These conventions take precedence over optional conventions inferred from repository history, not mandatory repository requirements or exact supplied wording.

## Contribution Preparation Authorization

Apply the global **Contribution Preparation Authorization** policy only when its managed-installation and task conditions hold. That policy alone delegates this mode. Supply its recorded setup authorization or checkpoint grant to `sensible-commit-flow` at **Confirm Commits → Alternative Approval Modes**, then retain the public workflow’s inspection, execution, and verification requirements.

For each call, supply the verified target, actual change scope, and requested operation. Carry the grant’s express request for provisional message revisions without treating supplied exact wording or all inherited messages as adjustable. Return each commit result to `contribution-flow` while the recorded lifetime continues. Other calls use the public skill’s exact-batch default.

## Message Form

- **Subject:** Write one compact, sentence case imperative clause, normally `<verb> <object>`, except for the recurring subjects below. Omit articles that add no meaning, colons, Conventional Commit prefixes, scope labels, and terminal punctuation. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Body:** The subject is the complete newly authored message. Do not add an explanatory body, issue reference paragraphs, testing checklists, or trailers. Route a conflict with supplied attribution or another required message element through the public skill’s message safeguards rather than discarding it.

## Recurring Subjects

- **Dependencies:** Use `Update dependencies` for a dependency maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Repository roots:** Use `Initial commit` for a repository root.
- **Releases:** Use a bare semantic version only for an actual release commit when that form is established in the repository.
