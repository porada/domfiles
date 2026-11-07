---
name: full-review
description: |-
    Run two independent reviews and resolve validated findings.

disable-model-invocation: true
metadata:
    internal: true
---

# Full Review

For `full-review` in T3 Code, I explicitly request a separate top-level thread for each of the two reviewers. This standing request covers direct invocations and reviews routed by another workflow. Continue follow-ups and later reviews of the same work in those threads. When replacement is necessary, create a new top-level thread and transfer the review context. Delegate tasks, native subagents, and standalone CLI sessions are not substitutes for these threads. If the host cannot support this lifecycle, follow the limited-review path under [Maintain Reviewer Continuity](references/full-review.md#maintain-reviewer-continuity).

Follow [Full Review](references/full-review.md). For initial reviewer selection or a required replacement, also follow [Select Reviewers](references/select-reviewers.md) before choosing an execution interface.
