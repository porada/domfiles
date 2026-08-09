# Security reports

Follow the receiving platform’s template. When it separates the summary, details, proof of concept, and impact, give each fact one primary role and remove cross-section repetition without removing unique evidence.

- **Title:** Prefer a familiar mechanism paired with a concrete security consequence. Keep lower-level terminology in the details unless it is needed for accuracy or distinguishability.
- **Summary:** State the verified mechanism, affected security boundary, and representative impact. Retain any prerequisite or limitation needed to avoid overstating exploitability, while leaving its complete explanation to the impact section.
- **Details:** Preserve the verified causal sequence and only the source identifiers needed to locate important transitions. If investigation disproves the original theory, rewrite the narrative around the verified mechanism rather than retaining the old framing through a caveat.
- **Proof of concept:** Provide a complete, repeatable, minimally hazardous reproduction. Preserve the exact setup, configuration, control cases, and expected and actual behavior.
- **Impact:** Identify affected users, attacker prerequisites, possible consequences, and limitations imposed by independent security boundaries. Distinguish those limitations from restoration of the bypassed security decision.
