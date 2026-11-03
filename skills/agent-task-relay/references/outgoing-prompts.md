# Outgoing Prompts

Use the shared [Relay Contract](../SKILL.md#relay-contract) for assignments and evidence-only handoffs. Let the receiving action determine which requirements apply.

## Assign Work

For a live external assignment, establish its target, scope, source and access constraints, covered effects, and completion owner before drafting. Use the user’s direction and valid existing authorization. Ask one focused question before composing when a material choice or uncovered effect remains, without reconfirming covered effects. A tentative suggestion to delegate doesn’t authorize a handoff. Preparing or reviewing a reusable prompt asset, or preparing an explicitly requested subagent prompt, doesn’t require live-handoff confirmation.

Carry the selected checkout or isolation requirement when material. Leave checkout creation and lifecycle decisions to the execution workflow rather than choosing an isolated worktree merely to prepare a prompt. Don’t use a subagent to cross an access, authentication, or permission boundary. Rely only on access already available to the recipient, and never relay credentials.

When the recipient can access the same instruction sources, identify instruction files by path and skills by their stable frontmatter `name`. Identify a command-only project workflow by its project-relative instruction path so the recipient can load it without invoking the command.

Define the owned scope, exclusions, source constraints, stopping conditions, and expected result. Carry applicable reporting requirements, including the [opportunity reporting contract](../SKILL.md#findings-and-opportunities), into every assignment and require recipients to preserve them through further delegation. Apply the [session behavior test framing exception](#session-behavior-tests) to test prompts. A full external handoff transfers completion ownership to the receiving conversation by default. When the sender remains responsible for synthesis, integration, or follow-up, require a reply that carries its supporting evidence instead. For a review, request stable finding identifiers, precise source locations, supporting evidence, consequences, material uncertainty, relevant validation, and an explicit statement of whether any in-scope findings remain standing. Distinguish reusable validation from claims needing independent verification, and require reassessment of changed or uncertain premises.

When an assignment depends on host-rendered events that the recipient can’t observe through its tools, such as approval prompts, dialogs, or notifications, assign each required observation to someone who can observe it through the selected interface and remains responsible after dispatch or handoff. Count the sender only when it retains that responsibility. Otherwise assign the observation to a capable user or host. Give the user any observation duties or human-only response requests outside the receiving agent’s prompt. If no responsible observer can supply required evidence, return the limitation to the calling workflow. Ask each agent only for evidence it can observe, such as tool schemas, tool results, command output, and file state.

End every initial or follow-up assignment with the exact standalone line `**Do not drift.**`. Put every required result, process constraint, and handoff instruction before it. Omit that guard from evidence-only transfers, and apply the [session behavior test framing exception](#session-behavior-tests) to test prompts.

### Independent Review Context

For a first-pass independent review, give only the neutral context needed to evaluate the target: baseline, scope and exclusions, intended behavior, applicable instructions, execution boundaries, and material evidence limitations. Don’t front-load coordinator conclusions, self-assessment, anticipated findings, implementation defenses, or decision history unless needed to establish a governing requirement or an expressly requested review subject. Don’t withhold safety or authority requirements, the actual requested subject, or evidence needed to evaluate it.

Keep the full decision record with the coordinator rather than automatically forwarding it in the initial prompt. For re-review, provide relevant prior findings and changed context while preserving the calling workflow’s review scope. When a finding conflicts with an established decision, apply [decision conflict handling](inbound-findings.md#resolve-decision-conflicts) before preparing a focused follow-up.

### Session Behavior Tests

When an assignment tests agent behavior in a particular environment, establish the required host, provider harness, model and settings, project checkout, and session context from the task. Verify that the receiving interface can supply them rather than inferring equivalence from a model name or the ability to create a thread. For ordinary top-level behavior, use a fresh conversation with normal startup instructions and independent context, not a delegate inheriting the coordinator’s discussion. If the interface can’t establish the required context, report that limitation rather than treating a different session type as equivalent.

Give the receiving agent the neutral test task and necessary safety boundaries, without the maintenance discussion, expected tool choice, or expected answer. Request observable actions and results, and distinguish tool evidence from the agent’s own report. Don’t let a prompt override supply the behavior the test is meant to evaluate.

For these tests, omit the relay heading, closing guard, or extra reporting instructions when they would supply the behavior being measured or alter a required test input. Retain necessary safety boundaries, and report any limitation they impose on what the test establishes.

## Relay Established Evidence

Carry established results, decisions, evidence, and limitations without assigning further work. Use only already available task context unless the user explicitly requests additional evidence gathering. Don’t continue the underlying task, invoke tools, reopen files, rerun checks, or draft patches merely to produce the handoff. Report gaps instead of reconstructing unavailable evidence.

Separate new assignments from evidence-only handoffs by default. When the user requests a combined handoff, distinguish the assignment and its authorization from the evidence, and apply [Assign Work](#assign-work) to that portion. Comparing findings against current source, investigating, and reviewing are assignments, not evidence-only receiving actions.

## Deliver the Prompt

Execution belongs to the calling workflow. Before returning a live assignment for dispatch, establish that the selected interface can supply the required session context. A thread creation capability alone doesn’t establish that the interface supports the intended handoff.

When the sender retains responsibility for synthesis, integration, or follow-up, establish that it can retrieve results and manage the delegated session’s lifecycle. For a full handoff, require confirmation of delivery to the intended receiving conversation and identify the user or host responsible for the session’s lifecycle afterward. Continued sender monitoring isn’t required after that transfer. If the interface can’t support the selected mode’s requirements, return the limitation and a manual relay to the calling workflow rather than starting work without the required observation or ownership.

When the calling workflow specifies delivery or dispatch, return the composed prompt there without also emitting a copy-and-paste relay. Keep prompt-only and explicitly user-run requests as prompt delivery. Preparing the prompt doesn’t authorize sending, publishing, or running it.

Otherwise, give each assignment a descriptive heading and put its complete text in a three-backtick `markdown` block, increasing the outer fence length when the prompt contains a code block. For an entire response intended as a verbatim evidence handoff, return only the handoff, without an outer code block, extra relay heading, or readiness message.
