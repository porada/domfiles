---
name: super-review
description: |-
    Review changes thoroughly and resolve validated findings.

disable-model-invocation: true
metadata:
    internal: true
---

# Super Review

For `super-review` in T3 Code, I explicitly request separate top-level reviewer threads by default for the selected stages, including required replacements and repeated cold reviews. This standing request covers direct invocations and reviews routed by another workflow.

Follow [Super Review](references/super-review.md) for every invocation. Before selecting reviewer settings or creating sessions, read [Reviewer Defaults](references/reviewer-defaults.md) and [Review Execution](references/review-execution.md).

When cold review is selected, apply the `cold-review` entrypoint before preparing its committed target or starting its reviewer.
