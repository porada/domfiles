# Assignment Prompts

## Assignment Contract

By default, end every initial or follow-up prompt that assigns future work with the exact standalone line `**Do not drift.**`. This applies to evidence gathering, mutation, and review assignments.

Define the bounded assignment, owned scope, exclusions, source and access constraints, stop conditions, and output contract. When the guard is included, put every required result, process, validation step, and handoff instruction before it.

Every assignment inherits the source task’s scope, mutation authority, approval requirements, and security boundaries. State that the receiving agent cannot expand scope, provide user-only approval, transfer access, or circumvent a boundary. Require it to return any boundary request to its coordinator or the user rather than crossing it.

An assignment may authorize an operation that writes a commit in any repository, directly or indirectly, only when it identifies the user’s explicit command for that operation. Completed work, staged changes, passing validation, a confirmed flow, an approved plan, and permission to edit authorize working tree changes only.

Require explicit user approval before introducing an agent-selected dependency or tool, or changing its prescribed features, source, or version. This applies even when acquisition is temporary, uses a package runner, or leaves repository files unchanged. Without approval, stop before dependency-premised implementation, mutation, installation, or mutating delegation. Before selecting a dependency or tool, requesting or carrying dependency approval, or interpreting authorization to acquire or integrate dependencies, follow [Dependency Approval](dependency-approval.md).

Use the guard only when the prompt assigns future work. Omit it from evidence-only decision relays and other transfers of established data. A receiving action alone does not turn an evidence handoff into an assignment.

## Delivery

When another applicable workflow invokes this skill for confirmation and assignment composition and explicitly defines the required final output and stopping behavior, return the composed assignment to that workflow instead of delivering it as a relay. Do not perform both.

Otherwise, put each complete assignment in its own three-backtick `markdown` block. Raise the fence to four backticks only when the prompt itself contains a three-backtick code block. Use the heading and ending specified by the selected route.

## User-Requested Subagent Prompts

Apply the [Assignment Contract](#assignment-contract) and the entrypoint’s [Relay Contract](../SKILL.md#relay-contract) to explicitly requested subagent drafts and reviews. Preserve the inherited scope, source, access, approval, mutation, stopping, and output boundaries.

Do not frame the assignment as a relay, present a relay flow, or ask for confirmation merely because in-client delegation will occur. This workflow does not mediate autonomous in-client delegation. A boundary in the underlying task may still require a user decision through the policy that owns it.

For a draft or revision, apply [Delivery](#delivery), preceding each prompt with `# Subagent Prompt` or a descriptive numbered `# Subagent Prompt …` heading.
