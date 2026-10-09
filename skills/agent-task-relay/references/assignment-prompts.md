# Assignment Prompts

## Assignment Contract

By default, end every initial or follow-up prompt that assigns future work with the exact standalone line `**Do not drift.**`. This applies to evidence gathering, mutation, and review assignments.

Define the bounded assignment, owned scope, exclusions, source and access constraints, stop conditions, and output contract. When the guard is included, put every required result, process, validation step, and handoff instruction before it. For follow-ups, use [Revisions](../SKILL.md#revisions) to select context rather than replaying unchanged terms.

For review assignments, distinguish supplied evidence eligible for reuse under [Evidence](../SKILL.md#evidence) from claims requiring independent verification. Ask the reviewer to reassess changed or uncertain premises with the smallest decisive check rather than rediscover established results, while preserving the required review scope.

When a reply is required, request conclusions or findings with supporting evidence, consequences, material uncertainty, and relevant validation under [Evidence](../SKILL.md#evidence). For reviews, require stable finding identifiers and precise source locations, with enough explanation to assess each claim. Do not request a replay of the assignment or investigation narrative, or use fixed size limits that truncate necessary evidence. Preserve required progress reports, error reporting, and stopping behavior.

Every assignment inherits the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent cannot expand scope, provide user-only approval, transfer access, or circumvent a boundary. Require it to return any boundary request to its coordinator or the user rather than crossing it.

An assignment may authorize an operation that writes a commit in any repository, directly or indirectly, only when it identifies the user’s explicit command for that operation. Completed work, staged changes, passing validation, a confirmed flow, an approved plan, and permission to edit authorize working tree changes only.

Before selecting a dependency or tool, requesting or carrying dependency approval, or interpreting authorization to acquire or integrate dependencies, follow [Dependency Approval](dependency-approval.md).

Use the guard only when the prompt assigns future work. Omit it from evidence-only decision relays and other transfers of established data. A receiving action alone does not turn an evidence handoff into an assignment.

## Delivery

When another applicable workflow invokes this skill for confirmation and assignment composition and explicitly defines the required final output and stopping behavior, return the composed assignment to that workflow instead of delivering it as a relay. Do not perform both.

Otherwise, put each complete assignment in its own three-backtick `markdown` block. Raise the fence to four backticks only when the prompt itself contains a three-backtick code block. Apply a route-specific heading or response ending only when that route specifies one. Keep the prompt guard governed by the [Assignment Contract](#assignment-contract).

## Direct Exchange Presentation

When a workflow delivers messages directly between agents, permit plain typography, sentence case headings, lists longer than five items, concise fragments, and omission of routine narration. Pass these exceptions into the assignment unless the receiving context already supplies them.

These exceptions do not apply to user-facing prose or authored repository documents. Preserve required syntax, scope, authority, security, evidence, validation, and delivery boundaries.

## User-Requested Subagent Prompts

Apply the [Assignment Contract](#assignment-contract) and the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) to explicitly requested subagent drafts and reviews. Preserve the inherited scope, source, access, approval, mutation, stopping, and output boundaries.

Do not frame the assignment as a relay, present a relay flow, or ask for confirmation merely because in-client delegation will occur. This workflow does not mediate autonomous in-client delegation. A boundary in the underlying task may still require a user decision through the policy that owns it.

For a draft or revision, apply [Delivery](#delivery), preceding each prompt with `# Subagent Prompt` or a descriptive numbered `# Subagent Prompt …` heading.
