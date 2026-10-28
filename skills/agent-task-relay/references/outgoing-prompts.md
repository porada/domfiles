# Outgoing Prompts

Use the shared [Relay Contract](../SKILL.md#relay-contract) for assignments and evidence-only handoffs. Let the receiving action determine which requirements apply.

## Assign Work

For a live external assignment, establish its target, scope, source and access constraints, covered effects, and completion owner before drafting. Use the user’s direction and valid existing authorization. Ask one focused question before composing when a material choice or uncovered effect remains, without reconfirming covered effects. A tentative suggestion to delegate doesn’t authorize a handoff. Preparing or reviewing a reusable prompt asset, or preparing an explicitly requested subagent prompt, doesn’t require live-handoff confirmation.

Carry the selected checkout or isolation requirement when material. Leave checkout creation and lifecycle decisions to the execution workflow rather than choosing an isolated worktree merely to prepare a prompt. Don’t use a subagent to cross an access, authentication, or permission boundary. Rely only on access already available to the recipient, and never relay credentials.

When the recipient can access the same instruction sources, identify instruction files by path and skills by their stable frontmatter `name`. Identify a command-only project workflow by its project-relative instruction path so the recipient can load it without invoking the command.

Define the owned scope, exclusions, source constraints, stopping conditions, and expected result. Carry applicable reporting requirements, including the [opportunity reporting contract](../SKILL.md#findings-and-opportunities), into every assignment and require recipients to preserve them through further delegation. A full external handoff transfers completion ownership to the receiving conversation by default. When the sender remains responsible for synthesis, integration, or follow-up, require a reply that carries its supporting evidence instead. For a review, request stable finding identifiers, precise source locations, supporting evidence, consequences, material uncertainty, relevant validation, and an explicit statement of whether any in-scope findings remain standing. Distinguish reusable validation from claims needing independent verification, and require reassessment of changed or uncertain premises.

When an assignment depends on host-rendered events that the recipient can’t observe through its tools, such as approval prompts, dialogs, or notifications, assign those observations and any required responses to the relaying user outside the prompt. Ask the recipient only for evidence it can observe, such as tool schemas, tool results, command output, and file state.

End every initial or follow-up assignment with the exact standalone line `**Do not drift.**`. Put every required result, process constraint, and handoff instruction before it. Omit that guard from evidence-only transfers.

### Independent Review Context

For a first-pass independent review, give only the neutral context needed to evaluate the target: baseline, scope and exclusions, intended behavior, applicable instructions, execution boundaries, and material evidence limitations. Don’t front-load coordinator conclusions, self-assessment, anticipated findings, implementation defenses, or decision history unless needed to establish a governing requirement or an expressly requested review subject. Don’t withhold safety or authority requirements, the actual requested subject, or evidence needed to evaluate it.

Keep the full decision record with the coordinator rather than automatically forwarding it in the initial prompt. For re-review, provide relevant prior findings and changed context while preserving the calling workflow’s review scope. When a finding conflicts with an established decision, apply [decision conflict handling](inbound-findings.md#resolve-decision-conflicts) before preparing a focused follow-up.

## Relay Established Evidence

Carry established results, decisions, evidence, and limitations without assigning further work. Use only already available task context unless the user explicitly requests additional evidence gathering. Don’t continue the underlying task, invoke tools, reopen files, rerun checks, or draft patches merely to produce the handoff. Report gaps instead of reconstructing unavailable evidence.

Separate new assignments from evidence-only handoffs by default. When the user requests a combined handoff, distinguish the assignment and its authorization from the evidence, and apply [Assign Work](#assign-work) to that portion. Comparing findings against current source, investigating, and reviewing are assignments, not evidence-only receiving actions.

## Deliver the Prompt

Execution belongs to the calling workflow. When it specifies delivery or dispatch, return the composed prompt there without also emitting a copy-and-paste relay. This doesn’t authorize sending, publishing, or running the prompt.

Otherwise, give each assignment a descriptive heading and put its complete text in a three-backtick `markdown` block, increasing the outer fence length when the prompt contains a code block. For an entire response intended as a verbatim evidence handoff, return only the handoff, without an outer code block, extra relay heading, or readiness message.
