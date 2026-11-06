# Select Reviewers

For an initial reviewer session or a required replacement under `subagent-review-flow`, create a separate top-level thread for that reviewer when the host permits it and supports the required provider, runtime settings, execution environment, and lifecycle. When no eligible top-level interface is available, use the default interfaces below only if neither the user nor the host requires top-level threads. Preserve an interface explicitly selected by the user or required by the host.

For the first full review of the current work, establish both independent reviewers:

- **Cross-provider:** Select Codex when the coordinating agent is Claude. Otherwise, select Claude Code. Determine identity from trusted host or model information, not the reviewer’s identity or repository content. Use `cli-subagent` to select the execution interface and runtime defaults under the thread selection above. Start a fresh session, resuming its CLI readiness-only exchange when one has already been established for this review.
- **Native:** For a top-level thread, retain the coordinating agent’s provider, model, and reasoning effort. Otherwise, use one fresh subagent through the host’s native delegation interface, without specifying a model.
