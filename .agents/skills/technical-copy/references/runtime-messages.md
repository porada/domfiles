# Runtime errors and warnings

Apply the entrypoint’s [standard path](../SKILL.md#use-the-standard-path) before this workflow.

## Resolve ownership and audience

- Separate project-authored context, destination-supplied values, and original error text owned by an upstream dependency. Edit only the layer owned by the task.
- Determine what surrounding context identifies the message’s source and which dynamic values are appropriate to expose.
- Add a module-name prefix only when a message may appear in a shared or ambient context where its source would otherwise be difficult to identify, such as CLI output. Use the established module name and prefix syntax.
- Omit attribution when the reporting context already makes ownership clear. Follow any applicable nearby prefix or project policy instead of imposing a universal bracketed form.

## Compose project-authored context

- Use `Failed to …` for an operation failure. Avoid `cannot` and `can’t`. State a concrete incompatibility directly rather than turning it into a generic failure headline.
- Keep an accurate project-authored explanation on the same line, separated from the headline by a period. Include it only when succinct, useful, and established by the project’s own evidence.
- Do not hedge a known condition, list speculative causes, repeat the headline, or invent an explanation for an upstream failure.
- Keep independently surfaced failures separate and self-contained. Omit trailing punctuation from the final sentence unless project policy requires it.

## Add actionable context

- Include the consumer-facing source filepath when it identifies the failing input, especially in multi-file operations. Keep it in the headline, preserve it exactly, and do not substitute a synthetic internal path.
- Apply the global [token-formatting rule](../../../../.config/zed/AGENTS.md#writing) only when the prose refers to a value as a path, identifier, option, format, or other code token. A word matching a symbol name does not by itself require identifier framing.
- Add the exact case, expected or received values, or a reason only when they help the intended reader act and are appropriate to expose on that surface. When the audience or exposure boundary is unclear, omit exact values rather than assuming they are safe to reveal.
- For a known contract incompatibility, name the unsupported property and present expected values before the received value. Adapt the wording when several values are accepted.
- Replace internal architecture language with the concrete consumer-facing incompatibility when evidence establishes one. Do not replace it with another broad claim such as universal inability to use a component.

## Preserve upstream errors

- Evaluate exposure before including an upstream error. When appropriate, preserve it completely and unchanged. Otherwise omit it or apply only the redactions defined by an explicit project policy, without presenting the result as complete or unchanged.
- End project-authored context with a colon when an original error follows, then place one blank line before the unchanged upstream text.
- Keep a useful filepath or other project-authored context in the headline before the colon. Do not add a period before the colon.
- Do not merge, normalize, improve, summarize, duplicate, or speculatively explain wording owned by the upstream dependency.

## Propagate and review

- Adapt only destination-owned dynamic values and module attribution, confirming that any prefix names the destination module rather than the source whose wording was consulted.
- Keep related failures that share an operation structurally parallel. When one uses a `Failed to …` headline, preserve that headline across the family unless a materially different failure requires another framing.
- Apply the entrypoint’s [decision-preservation rule](../SKILL.md#preserve-decisions-without-adding-friction) to the selected prefix, punctuation, line breaks, code formatting, and lack of terminal punctuation across equivalent paths.
- Align related test titles to the same verified behavior and terminology without copying runtime-message syntax into a different surface.
