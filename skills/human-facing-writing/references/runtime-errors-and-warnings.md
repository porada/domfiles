# Runtime Errors and Warnings

Use this workflow for project-authored runtime failure messages and warnings, including messages that preserve an upstream error.

## Ownership and Audience

Separate project-authored context, destination-supplied values, and original error text owned by an upstream dependency. Edit only the layer owned by the task. Determine what surrounding context identifies the message’s source and which dynamic values would help the intended reader act. Before exposing any value, apply the entrypoint’s [editorial boundaries](../SKILL.md#editorial-boundaries).

Add a module-name prefix only when a message may appear in a shared or ambient context where its source would otherwise be difficult to identify, such as CLI output. Use the established module name and prefix syntax. Omit attribution when the reporting context already makes ownership clear, following any applicable nearby prefix or project policy instead of imposing a universal bracketed form.

## Project-Authored Context

Use `Failed to …` for an operation failure. Avoid `cannot` and `can’t`. State a concrete incompatibility directly rather than turning it into a generic failure headline. Keep an accurate project-authored explanation on the same line, separated from the headline by a period, and include it only when it is succinct, useful, and established by the project’s own evidence.

Do not hedge a known condition, list speculative causes, repeat the headline, or invent an explanation for an upstream failure. Keep independently surfaced failures separate and self-contained. Omit trailing punctuation from the final sentence unless project policy requires it.

## Actionable Detail

Include the consumer-facing source file path when it identifies the failing input, especially in multi-file operations. Keep it in the headline, preserve it exactly, and do not substitute a synthetic internal path. Apply the [code-token convention](typography.md) only when the prose refers to a value as a path, identifier, option, format, or another code token. A word that matches a symbol name does not require identifier formatting for that reason alone.

Add the exact case, expected or received values, or a reason only when it helps the intended reader act and is appropriate to expose on that surface. When the audience or exposure boundary is unclear, omit exact values rather than assuming they are safe to reveal. For a known contract incompatibility, name the unsupported property and present expected values before the received value, adapting the wording when several values are accepted.

Replace internal architecture language with the concrete consumer-facing incompatibility when the evidence establishes one. Do not replace it with another broad claim, such as a universal inability to use a component.

## Upstream Errors

Evaluate exposure before including an upstream error. When appropriate, preserve it completely and unchanged. Otherwise, omit it or apply only the redactions defined by an explicit project policy, and do not present a redacted error as complete or unchanged.

When an original error follows, end the project-authored context with a colon and place one blank line before the unchanged upstream text. Keep a useful file path or other project-authored context in the headline before the colon, without adding a period before it. Do not merge, normalize, improve, summarize, duplicate, or speculatively explain wording owned by the upstream dependency.

## Related Messages

Adapt only destination-owned dynamic values and module attribution, confirming that any prefix names the destination module rather than the source whose wording was consulted. Keep related failures that share an operation structurally parallel. When one uses a `Failed to …` headline, preserve that headline across the family unless a materially different failure requires another framing.

Apply the entrypoint’s [editorial boundaries](../SKILL.md#editorial-boundaries) to the selected prefix, punctuation, line breaks, code formatting, and lack of terminal punctuation across equivalent paths. Align related test titles to the same verified behavior and terminology without copying runtime-message syntax into a different surface.
