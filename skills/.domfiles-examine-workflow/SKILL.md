---
name: examine-workflow
description: |-
    Examine agent task outcomes, findings, and recurring patterns to recommend targeted policy or workflow improvements.

disable-model-invocation: true
metadata:
    internal: true
---

# Agent Workflow Examination

Invocation alone authorizes read-only examination and recommendations, not implementation. Separately established change authority remains effective under the global **Task authorization** policy. Do not turn this command into an automatic task completion step.

## Resolve Focus

Use the task outcomes, findings, or recurring patterns the user identifies. Without an explicit focus, use the current task when the subject is clear. Ask one focused question only when materially different interpretations remain.

Identify the expected outcome and the question the user wants answered about the result or effort involved. Treat successful tasks and findings as valid subjects even when the agent followed all applicable guidance. Don’t presume agent error, a policy gap, or recurrence. Keep the investigation focused on that question rather than auditing the whole conversation, repository, or instruction set.

## Inspect Evidence

Start with the available conversation, user reports, and task results. Inspect narrowly relevant instructions, source, or existing artifacts only when needed to distinguish plausible explanations. Reconstruct only the decision sequence needed to answer that question, not a full timeline.

Distinguish observed actions, user-reported events, demonstrated causes, and hypotheses. Current instructions don’t prove what an earlier agent loaded or followed. Don’t invent unavailable history or reconstruct exact approvals from outcomes. If a material gap prevents a conclusion, name the missing evidence and request only the smallest detail needed, not the entire conversation. Otherwise, report the limitation and continue with what the evidence supports.

Stop gathering evidence once it supports a useful recommendation or establishes that the cause can’t be determined within scope. Keep the examination in the conversation unless the user requests a durable artifact. Don’t create diagnostic logs or a new retention system.

## Examine Causes

Distinguish useful practices and necessary work from inadequate, conflicting, or hard-to-find guidance, failure to follow adequate guidance, missing decision evidence, and tool or client constraints. Identify the decision, information, or workflow condition that explains the result or effort involved and the kinds of tasks to which that explanation could apply. Connect proposed causes to evidence, and preserve uncertainty when multiple explanations remain plausible.

For findings, distinguish between preventing the underlying issue and detecting it earlier. A review finding can show that detection worked as intended. Examine whether an earlier intervention would have been reasonable with the information available then, rather than treating fewer findings as the goal.

A required approval or sandbox prompt isn’t itself a policy defect. Distinguish avoidable interruption from a legitimate boundary, and don’t recommend weakening that boundary merely to reduce prompts.

## Recommend Improvements

Prefer applying adequate existing guidance over adding another rule. For a demonstrated gap, consider clarifying, consolidating, relocating, or removing guidance before adding instructions. Recommend a check or workflow change when it addresses the cause more directly than prose. Conclude that no policy change is needed when the evidence supports that result, including when the current detection point is appropriate.

Recommend the smallest useful intervention at its canonical owner. Explain how it would change the relevant decision in the next comparable task, and check it against the observed case and a nearby legitimate case to avoid unnecessary restriction. Weigh the expected benefit against recurring token cost and procedural overhead. State material limitations rather than promising that the issue can never recur.

Report the conclusion, decisive evidence, material uncertainty, and recommendation concisely. Include only the parts needed to support the decision, not a mandatory report template or an exhaustive list of possible improvements.

Apply improvements only when separately authorized, using the affected domain workflow. For authorized agent documentation changes, follow `agent-documentation` to resolve ownership and validation. Use **Execution checkpoints** only for a material decision, uncovered effect, or separately required gate, not merely because examination is complete.
