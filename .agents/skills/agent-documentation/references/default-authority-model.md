# Default agent-documentation authority model

Use this fallback only when a project has not defined its own agent-documentation authority or ownership model. The model assigns ownership when a relevant surface exists. It does not require creating every surface.

| Surface | Default authority and ownership |
| --- | --- |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| Project-authored `.agents/skills/*/` | Define delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Each `SKILL.md` is an entrypoint and may route to canonical references in its directory. |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

## Apply the fallback

- Apply a locally defined model instead whenever one becomes available.
- Use existing documentation layers before proposing a new one.
- Create a missing layer only when the requested task authorizes it and the durable detail has no safe existing owner.
- Keep each normative detail in one canonical owner and replace secondary definitions with links.
- Report an unresolved ownership decision instead of distributing the detail across several documents.
