---
name: cold-review
description: |-
    Review committed changes with a fresh reviewer and resolve validated findings.

disable-model-invocation: true
metadata:
    internal: true
---

# Cold Review

When I invoke `cold-review` in T3 Code, directly or through another workflow, I explicitly request a separate top-level reviewer thread for each review group by default, including new top-level threads for required replacements and repeated cold reviews. The agent in my invoking thread coordinates the review.

Follow [Cold Review](references/cold-review.md). Before selecting a reviewer or creating a session, read the installed `super-review` skill’s `references/reviewer-defaults.md` and `references/review-execution.md`. Use those shared references for the reviewer profile, execution interface, and reviewer session lifecycle.
