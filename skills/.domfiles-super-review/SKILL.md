---
name: super-review
description: |-
    Review changes thoroughly and resolve validated findings.

disable-model-invocation: true
metadata:
    internal: true
---

# Super Review

When I invoke `super-review` in T3 Code, I explicitly request separate top-level threads by default for the selected review stages. Use one thread per peer reviewer in each peer review group and one per cold review group. This includes new threads for required reviewer replacements and repeated cold reviews. The agent in my invoking thread coordinates the review.

When another command calls `super-review`, carry forward its thread request. When a model-invocable workflow already has my request for this review through T3 Code, I explicitly request the same top-level threads within its approved scope. From any client, ask before opening a top-level thread in T3 Code that no existing request covers.

Follow [Super Review](references/super-review.md) for every invocation. Before selecting reviewer settings or creating sessions, read [Reviewer Defaults](references/reviewer-defaults.md) and [Review Execution](references/review-execution.md).

When cold review is selected, apply the `cold-review` entrypoint before preparing its committed target or starting its reviewer.
