# Default agent-documentation authority model

Use this fallback only when a project has not defined its own agent-documentation authority or ownership model. Assign each durable detail to an existing relevant surface. Create a missing surface only when the requested task authorizes it and no existing surface can safely own the detail.

| Surface | Default authority and ownership |
| --- | --- |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| Project-authored `.agents/skills/*/` | Define delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Follow the [skill composition rules](../SKILL.md#compose-the-change) for entrypoints and conditional references. |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |
