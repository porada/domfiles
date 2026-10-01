# Default Agent Documentation Authority Model

Assign each durable detail to an existing relevant surface. Create a missing surface only when the requested task authorizes it and no existing surface can safely own the detail.

| Surface | Default Authority and Ownership |
| --- | --- |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| Project-authored skill directories under `.agents/skills/` and `skills/` | Define delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Follow `skill-development` for skill entrypoints and conditional references. |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |
