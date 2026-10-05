---
name: diagnose
description: |-
    Diagnose friction in agent work and recommend the smallest useful process correction.

disable-model-invocation: true
metadata:
    internal: true
---

# Agent Workflow Diagnosis

Invocation alone authorizes read-only diagnosis and recommendations, not implementation. Separately established change authority remains effective under the global **Task authorization** policy. Do not turn this command into an automatic task completion step.

## Resolve Focus

Use the episode or recurring pattern the user identifies. Without an explicit focus, use the current task when the relevant friction is clear. Ask one focused question only when materially different interpretations remain.

Identify the expected outcome and the interruption, repeated work, or other cost being examined. Keep the investigation within that question rather than auditing the whole conversation, repository, or instruction set. Do not assume a reported episode establishes a recurring pattern.

## Inspect Evidence

Start with the available conversation, user reports, and task results. Inspect narrowly relevant instructions, source, or existing artifacts only when needed to distinguish plausible causes. Reconstruct only the decision sequence needed to explain the friction, not a full timeline.

Distinguish observed actions, user-reported events, demonstrated causes, and hypotheses. Current instructions do not prove what an earlier agent loaded or followed. Do not invent unavailable history or reconstruct exact approvals from outcomes. If a material gap prevents a conclusion, name the missing evidence and request only the smallest detail needed, not the entire conversation. Otherwise report the limitation and continue with what the evidence supports.

Stop gathering evidence once it supports a useful recommendation or establishes that the cause cannot be determined within scope. Keep the diagnosis in the conversation unless the user requests a durable artifact. Do not create diagnostic logs or a new retention system.

## Diagnose Causes

Separate inadequate or conflicting guidance from failure to follow adequate guidance, missing decision evidence, a tool or client constraint, and necessary work. Explain which evidence connects the observed cost to the proposed cause. When multiple causes remain plausible, say so rather than forcing a single root cause.

For example, repeated validation may reflect changed inputs, insufficient evidence to justify reuse, conflicting workflow requirements, or an agent overlooking **Validation evidence**. Check which explanation fits before recommending fewer checks.

A required approval or sandbox prompt is not itself a policy defect. Distinguish avoidable interruption from a legitimate boundary, and do not recommend weakening that boundary merely to reduce prompts.

## Recommend Corrections

Prefer applying adequate existing guidance over adding another rule. For a demonstrated gap, recommend the smallest correction at its canonical owner and explain how the next comparable task should differ. Consider the correction’s recurring token cost and procedural overhead, not just the friction it removes. Conclude that no policy change is needed when the evidence supports that result.

Report the diagnosis, decisive evidence, material uncertainty, and recommended correction concisely. Include only the parts needed to support the decision, not a mandatory report template or an exhaustive list of possible improvements.

Apply corrections only when separately authorized, using the affected domain workflow. For authorized agent documentation changes, follow `agent-documentation` to resolve ownership and validation. Use **Execution checkpoints** only for a material decision, uncovered effect, or separately required gate, not merely because diagnosis is complete.
