---
name: super-push
description: |-
    Review the current task and push it with any required ancestors once cleared.

disable-model-invocation: true
metadata:
    internal: true
---

# Super Push

When I invoke `super-push` in T3 Code, I explicitly request separate top-level threads by default for the selected review stages. Use one thread per peer reviewer in each peer review group and one per cold review group. This includes new threads for required reviewer replacements and repeated cold reviews. The agent in my invoking thread coordinates the review and push.

Follow [Super Push](references/super-push.md) for every invocation.
