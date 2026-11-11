# Outgoing Prompts

## Compose the Relay

A relay carries an assignment or established evidence into another conversation. Make its purpose, authority, and expected result clear. Let the receiving action determine which requirements apply.

- **Action:** Begin with a descriptive `# …` heading and the exact receiving action, distinguishing future work from evidence-only transfer. Apply the [session behavior test framing exception](#session-behavior-tests) when preparing a test prompt.
- **Context:** Include each material fact once when needed to evaluate the target, perform the assigned work, or respect a boundary. Reference accessible instructions and artifacts rather than copying them. Don’t investigate merely to shorten a prompt. Omit chronology, incidental identifiers, repeated rationale, and context needed only by the sender.
- **Authority:** Preserve user instructions, corrections, selections, explicit acceptances, and permission boundaries. Carry approvals with their actual source, scope, and lifetime, including the recipients, processing services, and content scope covered by [disclosure authorization](../SKILL.md#disclosure) when needed for continuation. Neither an agent proposal nor user silence supplies approval. A recipient can’t expand scope, provide user-only approval, transfer access, or bypass a boundary. Require the recipient to return uncovered effects or required decisions to the coordinator or user.
- **State:** State the resulting state, unavailable evidence, known limitations, and unsettled decisions directly.

Carry established dependency and tool choices without selecting new ones as part of relay composition. Before dependent work, including delegated work, a new dependency or tool choice or a change to prescribed features, source, or version needs explicit user approval or an applicable governing grant covering selection and its effects. Carry the grant’s source, conditions, effect boundaries, and lifetime, without treating temporary use or relay composition as authority. Return uncovered choices to the owning workflow or user. Carry commit authority only when it identifies the user’s explicit command or applicable governing grant for the covered commit-writing effects, including its scope, lifetime, and recipient’s permitted role. Neither prompt preparation nor permission to edit supplies that authority.

## Assign Work

For a live external assignment, establish its target, scope, source and access constraints, covered effects, and completion owner before drafting. Use the user’s direction and valid existing authorization. Ask one focused question before composing when a material choice or uncovered effect remains, without reconfirming covered effects. A tentative suggestion to delegate doesn’t authorize a handoff. Preparing or reviewing a reusable prompt asset, or preparing an explicitly requested subagent prompt, doesn’t require live-handoff confirmation.

Carry the selected checkout or isolation requirement when material. Leave checkout creation and lifecycle decisions to the owning workflow rather than choosing an isolated worktree merely to prepare a prompt. Don’t use a subagent to cross an access, authentication, or permission boundary. Rely only on access already available to the recipient, and never relay credentials.

When the recipient can access the same instruction sources, identify instruction files by path and skills by their stable frontmatter `name`. Identify a command-only project workflow by its project-relative instruction path so the recipient can load it without invoking the command.

Define the owned scope, exclusions, source constraints, stopping conditions, and expected result. Carry applicable reporting requirements, including the [opportunity reporting contract](../SKILL.md#findings-and-opportunities), into every assignment and require recipients to preserve them through further delegation. Apply the [session behavior test framing exception](#session-behavior-tests) to test prompts. A full external handoff transfers completion ownership to the receiving conversation by default. When the sender remains responsible for synthesis, integration, or follow-up, require a reply that carries its supporting evidence instead. Distinguish reusable validation from claims needing independent verification, and require reassessment of changed or uncertain premises.

When an assignment depends on host-rendered events that the recipient can’t observe through its tools, such as approval prompts, dialogs, or notifications, assign each required observation to someone who can observe it through the selected interface and remains responsible after dispatch or handoff. Count the sender only when it retains that responsibility. Otherwise assign the observation to a capable user or host. Give the user any observation duties or human-only response requests outside the receiving agent’s prompt. If no responsible observer can supply required evidence, return the limitation to the owning workflow. Ask each agent only for evidence it can observe, such as tool schemas, tool results, command output, and file state.

End every initial or follow-up assignment with the exact standalone line `**Do not drift.**`. Put every required result, process constraint, and handoff instruction before it. Omit that guard from evidence-only transfers, and apply the [session behavior test framing exception](#session-behavior-tests) to test prompts.

### Session Behavior Tests

When an assignment tests agent behavior in a particular environment, establish the required host, provider harness, model and settings, project checkout, and session context from the task. Verify that the receiving interface can supply them rather than inferring equivalence from a model name. For ordinary top-level behavior, use a fresh conversation with normal startup instructions and independent context, not a delegate inheriting the coordinator’s discussion. If the interface can’t establish the required context, report that limitation rather than treating a different session type as equivalent.

Give the receiving agent the neutral test task and necessary safety boundaries, without the maintenance discussion, expected tool choice, or expected answer. Request observable actions and results, and distinguish tool evidence from the agent’s own report. Don’t let a prompt override supply the behavior the test is meant to evaluate.

For these tests, omit the relay heading, closing guard, or extra reporting instructions when they would supply the behavior being measured or alter a required test input. Retain necessary safety boundaries, and report any limitation they impose on what the test establishes.

## Relay Established Evidence

Carry established results, decisions, evidence, and limitations without assigning further work. Use only already available task context unless the user explicitly requests additional evidence gathering. Don’t continue the underlying task, invoke tools, reopen files, rerun checks, or draft patches merely to produce the handoff. Report gaps instead of reconstructing unavailable evidence.

Separate new assignments from evidence-only handoffs by default. When the user requests a combined handoff, distinguish the assignment and its authorization from the evidence, and apply [Assign Work](#assign-work) to that portion. Comparing findings against current source, investigating, and reviewing are assignments, not evidence-only receiving actions.

## Revise Prompts

When delivering revised prompt text, return each affected prompt in full, or the complete affected set when the change spans coordinated prompts, unless the user requests another form. Preserve settled decisions and untouched boundaries.

For a follow-up in an established conversation, send the new request, changed constraints, and only the evidence or dispositions needed for it. Don’t replay unchanged roles, boundaries, validation, or history. When retained context is uncertain, supply a self-contained handoff and use it as the reference point for later turns. A session identifier alone doesn’t establish retained context. A material change to an external assignment’s target, scope, or effects returns to [Assign Work](#assign-work) to establish authorization.

## Deliver the Prompt

Execution belongs to the owning workflow. Before returning a live assignment for dispatch, establish that the selected interface can supply the required session context. A thread creation capability alone doesn’t establish that the interface supports the intended handoff.

When the sender retains responsibility for synthesis, integration, or follow-up, establish that it can retrieve results and manage the delegated session’s lifecycle. For a full handoff, require confirmation of delivery to the intended receiving conversation and identify the user or host responsible for the session’s lifecycle afterward. Continued sender monitoring isn’t required after that transfer. If the interface can’t support the selected mode’s requirements, return the limitation and a manual relay to the owning workflow rather than starting work without the required observation or ownership.

When the owning workflow specifies delivery or dispatch, return the composed prompt there without also emitting a copy-and-paste relay. Keep prompt-only and explicitly user-run requests as prompt delivery. Preparing or revising the prompt doesn’t authorize sending, publishing, or running it.

Otherwise, give each relay a descriptive heading and put its complete text in a three-backtick `markdown` block, increasing the outer fence length when the prompt contains a code block. For an entire response intended as a verbatim evidence handoff, return only the handoff, without an outer code block, extra relay heading, or readiness message.
