# Refine Agent Workflow

## Bound the Examination

Invocation alone authorizes read-only examination and recommendations, not implementation. Separately established authority to make changes remains effective within its scope and lifetime. Run this examination only when invoked, not automatically at the end of another task.

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

Prefer applying adequate existing guidance over adding another rule. For a demonstrated gap, consider clarifying, consolidating, relocating, or removing guidance before adding instructions. Recommend a check or workflow change when it addresses the cause more directly than prose. Conclude that no change is needed when the evidence supports that result, including when the current detection point is appropriate.

Recommend the smallest useful intervention at its canonical owner. Explain how it would change the relevant decision in the next comparable task, and check it against the observed case and a nearby legitimate case to avoid unnecessary restriction. Weigh the expected benefit against recurring token cost and procedural overhead. State material limitations rather than promising that the issue can never recur.

Use available user-global and project-local knowledge of skill ownership and maintenance boundaries to identify likely owners. Don’t assume the current project owns the skills used during its tasks. Global or project instructions may be the appropriate owner instead. Consult only guidance relevant to the evidence. Treat ownership information as a routing aid, not an audit checklist or authorization to invoke workflows or edit skills. When that knowledge is unavailable, continue from the task evidence and state any material ownership uncertainty.

Lead with an explicit decision on whether you recommend an instruction, check, or workflow change, then answer the examined question with the demonstrated cause or remaining uncertainty. Distinguish changes you endorse from merely possible improvements. If the evidence is insufficient to decide, identify what would resolve that uncertainty. Support the decision with decisive evidence and material limitations, without a mandatory report template or an exhaustive list of possible improvements.

## Apply or Hand Off Improvements

Apply improvements only when separately authorized, using the affected domain workflow. For authorized agent documentation changes, identify their canonical owner, preserve applicable maintenance requirements, and validate the changed guidance and affected references. Ask only when there’s a material decision, an effect outside current authorization, or a separately required approval, not merely because examination is complete.

When recommending concrete changes, offer an implementation handoff unless the user has already requested implementation of those changes in the current conversation. Resolve the destination and scope before making the offer, asking the user when ownership or the destination is unclear or the changes span repositories.

Check whether the host can create a new top-level thread in the owning repository before choosing the handoff to offer. When supported, offer to create that thread to implement the identified scope, and ask for acceptance of both thread creation and implementation before proceeding. Otherwise, including when there is no suitable repository, offer a ready-to-use handoff prompt instead.

For both a new thread and a handoff prompt, include only the recommendations to implement, their target locations, necessary constraints and acceptance criteria, and the authorization actually granted by the user with its source and limits. Preserve separate approval requirements. Omit conversation history and the analysis behind the recommendations.

If accepted thread creation becomes unavailable or fails, or delivery cannot be verified, report the resulting state, including any thread already created, and provide a ready-to-use handoff prompt. Don’t create another thread merely because delivery is uncertain.

Once creation and delivery are verified, share the thread’s link. Don’t monitor progress, collect results, send follow-ups, integrate changes, or manage the new thread’s lifecycle unless the user explicitly asks. The new thread owns implementation and its handoff to the user.
