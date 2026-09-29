---
name: dom-sensible-commit-flow
description: |-
    Use whenever `sensible-commit-flow` applies.

metadata:
    internal: true
---

# Commit Flow

Load `sensible-commit-flow` and apply these personal conventions as defaults for newly authored messages under the global **Explicit user direction** policy. The public skill owns the workflow, supplied-message safeguards, confirmation, execution, and verification. These defaults take precedence over optional conventions inferred from repository history. Preserve repository requirements and exact supplied wording unless an explicit, task-scoped instruction overrides the applicable convention or authorizes a wording change.

## Contribution Preparation Authorization

Apply the global **Contribution Preparation Authorization** policy only when its managed-installation and task conditions hold. That policy alone delegates this mode. Supply its recorded setup authorization or checkpoint grant to `sensible-commit-flow` through its **Alternative Approval Modes** reference, then apply the public workflow’s inspection, execution, and verification requirements with any explicit user waiver covered by the global **Explicit user direction** policy. The grant itself supplies no waiver.

For each call, supply the verified target, actual change scope, and requested operation. Carry the grant’s express request for provisional message revisions without treating supplied exact wording or all inherited messages as adjustable. Return each commit result to `sensible-contribution-flow` while the recorded lifetime continues. Other calls follow the public skill’s applicable authorization path, including already authorized history updates.

## Message Form

- **Subject:** Write one compact, sentence case imperative clause, normally `<verb> <object>`, except for the recurring subjects below. Omit articles that add no meaning, colons, Conventional Commit prefixes, scope labels, and terminal punctuation. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Body:** Use the subject as the complete newly authored message by default. Add an explanatory body, issue reference paragraphs, testing checklists, or trailers only when explicitly requested or required by the applicable message constraints. Route a remaining conflict with supplied attribution or another required message element through the public skill’s message safeguards rather than discarding it.

## Recurring Subjects

- **Dependencies:** Use `Update dependencies` for a dependency maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Repository roots:** Use `Initial commit` for a repository root.
- **Releases:** Use a bare semantic version only for an actual release commit when that form is established in the repository.
