---
name: cold-review
description: |-
    Review committed changes with a fresh reviewer and resolve validated findings.

disable-model-invocation: true
metadata:
    internal: true
---

# Cold Review

When I invoke `cold-review` in T3 Code, I explicitly request a separate top-level thread by default for the cold reviewer in each review group. This includes new threads for required reviewer replacements and repeated cold reviews. The agent in my invoking thread coordinates the review.

When another command calls `cold-review`, carry forward its thread request. When a model-invocable workflow already has my request for this review through T3 Code, I explicitly request the same top-level threads within its approved scope. From any client, ask before opening a top-level thread in T3 Code that no existing request covers.

Follow [Cold Review](references/cold-review.md). Before selecting a reviewer or creating a session, read the installed `super-review` skill’s `references/reviewer-defaults.md` and `references/review-execution.md`. Use those shared references for the reviewer profile, execution interface, and reviewer session lifecycle.
