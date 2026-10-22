# Prepare Security Reports

Use this drafting fallback only when the entrypoint’s [peer choice](../SKILL.md#compose-with-peers) leaves `human-facing-writing` unavailable. Don’t rediscover the peer or bypass its required stop. Follow the shared [security assessment boundaries](../SKILL.md#assess-contribution-fit), [execution boundaries](execution-boundaries.md), and [post content workflow](prepare-post-content.md) for evidence, templates, checklists, and delivery.

## Establish Evidence

Identify the affected revision or version, relevant configuration, attacker-controlled input, crossed security boundary, and demonstrated consequence. Distinguish observations from inference and untested possibilities. Replace a disproved theory with the verified mechanism, and don’t infer a broader affected version range from one tested configuration. State material evidence gaps rather than claiming readiness.

Describe a complete, repeatable, minimally hazardous reproduction. Include the setup, configuration, attacker prerequisites, controls, and expected and actual behavior. Distinguish exercised steps from proposed verification. Use named placeholders for private values, preserving only necessary format constraints.

## Draft Security Content

Follow the receiving template. Where it separates these roles, give each fact one primary home:

- **Title:** Name the mechanism and concrete security consequence.
- **Summary:** State the verified mechanism, affected boundary, and representative impact without overstating exploitability.
- **Details:** Explain the causal sequence and evidence locating its important transitions.
- **Proof of concept:** Supply the safe reproduction, its controls, and observed result.
- **Impact:** Identify affected users, attacker prerequisites, supported consequences, and independent limits on harm. Those limits don’t restore the bypassed security decision.

Include mitigation only when supported, explaining any remaining limitation without promising an untested fix. Return the draft through [content finalization](prepare-post-content.md#finalize-content) and the [contribution handoff](../SKILL.md#hand-off-contributions), retaining the designated private destination and any unmet requirements.
