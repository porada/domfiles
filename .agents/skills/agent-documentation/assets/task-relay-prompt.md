Create one or more self-contained task relays from the work assigned in this conversation. Your sole action in this turn is to produce the relay prompts. Do not perform, investigate, validate, or continue the relayed tasks.

Use only the conversation and task artifacts already inspected. Do not call tools, inspect files, browse, delegate, or invent missing paths, revisions, versions, commands, evidence, or project requirements. When a material ambiguity prevents a safe and complete task assignment, ask one focused clarification instead of emitting a relay.

## Output contract

- Put every complete relay in its own four-backtick `markdown` block.
- Precede each block with `# Relay Prompt` for one prompt or a descriptive numbered heading such as `# Relay Prompt 1—Git permissions` for multiple prompts.
- Begin the content inside each block with `# <Task name> Task Relay`.
- Follow a single block with `The prompt is ready for relay.`
- For multiple prompts, let the next relay heading immediately follow each intermediate block and finish with `All prompts are ready for relay.`
- Keep all text intended for the receiving agent inside the block. Keep framing and supporting explanation outside it.
- When the user asks to revise a prompt, output each affected prompt in full with the change applied. Do not return a patch, fragment, or splicing instructions.

## Relay requirements

- Begin with `**Receiving action:**` and state the exact action the next agent should take.
- Make each prompt understandable without access to this conversation.
- Preserve every established scope, exclusion, approval, mutation, submission, and integration boundary.
- Distinguish user-supplied settled evidence from proposals, project policy, repository evidence, documentation evidence, observed behavior, and agent inference.
- Include exact syntax, commands, paths, URLs, examples, or versions only when already established and material to the task.
- Exclude secrets, complete inventories, long generated artifacts, and unrelated conversation history.
- Do not add a worktree requirement unless the user explicitly requested one or an already applicable policy requires one. Do not infer one merely from parallel activity, a dirty checkout, or task size.
- Do not add commit, branch, submission, or integration instructions unless the task or applicable policy establishes them.

## Relay structure

Use only the sections the task needs, in this general order:

# <Task name> Task Relay

**Receiving action:** _Describe the smallest complete action._

## Task context

Identify the project, target state, affected surface, task type, and immediate purpose.

## Authority and evidence

Identify direct instructions, settled evidence, applicable policy, current implementation evidence, and material evidence limitations. Do not reopen settled classifications.

## Scope and boundaries

State the owned files, objects, behaviors, exclusions, preservation requirements, approval boundaries, and prohibited actions.

## Required result

Describe the complete observable or structural result. Separate required behavior from implementation suggestions.

## Process

Include only workflows, ordering constraints, or investigation limits already required by the task or applicable policy.

## Validation

State the smallest material checks that establish the result and any behavior that cannot be verified.

## Handoff

State what the receiving agent must report, what state it must preserve, and where it must stop.

Keep each relay concise enough to scan but complete enough that the receiving agent does not need the source conversation.
