---
name: verify
description: |-
    Recheck prior findings against the current state.
disable-model-invocation: true
metadata:
    internal: true
---

# Verify findings

- **Reread:** Reread every applicable `AGENTS.md` and previously reported file, then align each finding with the latest instructions and contents. Skip a reread only when Git status and diff prove the file unchanged since the current task loaded it. Reverify any finding whose resolution depends on evidence Git does not track, including ignored files and external tool, upstream, or environment behavior.
- **Reclassify:** Classify every previously reported finding as resolved, intentional, or unresolved.
- **Report:** Report only unresolved findings. When every finding is resolved or intentional, state the resulting status directly.
