# Outgoing Prompts

Use the shared [Relay Contract](../SKILL.md#relay-contract) for assignments and evidence-only handoffs. Let the receiving action determine which requirements apply.

## Assign Work

For a live external assignment, establish its target, scope, source and access constraints, covered effects, and completion owner before drafting. Use the user’s direction and valid existing authorization. Ask one focused question before composing when a material choice or uncovered effect remains, without reconfirming covered effects. A tentative suggestion to delegate does not authorize a handoff. Preparing or reviewing a reusable prompt asset, or preparing an explicitly requested subagent prompt, does not require live-handoff confirmation.

Carry the selected checkout or isolation requirement when material. Leave checkout creation and lifecycle decisions to the execution workflow rather than choosing an isolated worktree merely to prepare a prompt. Do not use a subagent to cross an access, authentication, or permission boundary. Rely only on access already available to the recipient, and never relay credentials.

Define the owned scope, exclusions, source constraints, stopping conditions, and expected result. A full external handoff transfers completion ownership to the receiving conversation by default. When the sender remains responsible for synthesis, integration, or follow-up, require an evidence-bearing reply instead. For a review, request stable finding identifiers, precise source locations, supporting evidence, consequences, material uncertainty, relevant validation, and an explicit statement of whether any in-scope findings remain standing. Distinguish reusable validation from claims needing independent verification, and require reassessment of changed or uncertain premises.

End every initial or follow-up assignment with the exact standalone line `**Do not drift.**`. Put every required result, process constraint, and handoff instruction before it. Omit that guard from evidence-only transfers.

## Relay Established Evidence

Carry established results, decisions, evidence, and limitations without assigning further work. Use only already available task context unless the user explicitly requests additional evidence gathering. Do not continue the underlying task, invoke tools, reopen files, rerun checks, or draft patches merely to produce the handoff. Report gaps instead of reconstructing unavailable evidence.

Separate new assignments from evidence-only handoffs by default. When the user requests a combined handoff, distinguish the assignment and its authorization from the evidence, and apply [Assign Work](#assign-work) to that portion. Comparing findings against current source, investigating, and reviewing are assignments, not evidence-only receiving actions.

## Deliver the Prompt

Execution belongs to the calling workflow. When it specifies delivery or dispatch, return the composed prompt there without also emitting a copy-and-paste relay. This does not authorize sending, publishing, or running the prompt.

Otherwise, give each assignment a descriptive heading and put its complete text in a three-backtick `markdown` block, increasing the outer fence length when the prompt contains a code block. For an entire response intended as a verbatim evidence handoff, return only the handoff, without an outer code block, extra relay heading, or readiness message.
