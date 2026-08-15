Create one or more task relays from work assigned in this conversation. Do not perform or investigate the relayed work.

Use only already-inspected conversation context and artifacts. Do not call tools, browse, delegate, or invent missing facts. Ask one focused clarification instead when a material ambiguity prevents a safe assignment. When a mutating relay depends on adding or updating a dependency that the user has not explicitly approved, ask the user one focused approval question and output no relay. After approval, preserve the exact authorized dependency change and identify the user’s explicit response as its approval source in the relay.

## Output

- Put each relay in its own three-backtick `markdown` block under `# Relay Prompt` or a descriptive numbered relay heading. Raise the fence to four backticks only when the relay itself contains a three-backtick code block.
- Begin each block with `# <task-name> Task Relay` and `**Receiving action:**`.
- After one relay, write `The prompt is ready for relay.` After multiple relays, write `All prompts are ready for relay.`
- Keep all relayed text inside its block. When revising relays, return every affected relay in full, omit unrelated unchanged relays, and never return a patch, fragment, or splicing instructions.

## Requirements

- Keep each relay as short as the complete assignment permits. Use a succinct, scan-friendly format, include each material fact once, prefer compact bullets, and omit chronology, routine validation, repeated rationale, and incidental identifiers.
- Make the relay self-contained. Preserve every material safety, approval, mutation, submission, integration, evidence, and stopping boundary, along with exclusions, explicit user instructions, material limitations, required behavior, and settled evidence. Preserve the exact provenance of every material item of evidence, distinguishing agent inference, observed behavior, and source evidence from one another and from proposals.
- Omit the receiving location by default. Include a repository, checkout, worktree, directory, or host only when selecting it is necessary for execution, isolation, disambiguation, submission, or integration. Preserve material target paths.
- Preserve established commands, examples, normalized inputs, paths, punctuation, syntax, token ordering, URLs, versions, and wording exactly when they materially determine the assignment. Exclude secrets, unnecessary or unbounded inventories, long artifacts, and unrelated history. Preserve a bounded complete inventory when it materially defines the owned scope, preservation boundary, or required result.
- Add worktree, commit, branch, submission, or integration instructions only when the user or applicable policy requires them.

## Structure

Use only needed sections, but always include `Scope and boundaries` and end the complete relay with the exact standalone guard shown below:

# \<task-name\> Task Relay

**Receiving action:** _State the smallest complete action and whether mutation is authorized._

## Context and evidence

Include only the target state, authoritative instructions or settled evidence, current implementation facts, and material limitations needed to act. Do not reopen settled classifications.

## Scope and boundaries

Define the smallest complete owned scope and necessary supporting work. Name material exclusions, prohibited actions, and every inherited preservation, scope, mutation, approval, submission, integration, access, and security boundary. Require unrelated findings to remain untouched. Prohibit transferring access or circumventing a boundary. Tell the receiver to stop and ask the user directly before crossing any inherited boundary.

## Required result

State the complete observable result without prescribing optional implementation details.

## Process and validation

Include only mandatory workflow steps and the smallest checks that establish the result.

## Handoff

State the required report, preserved state, and stopping point.

**Do not drift.**
