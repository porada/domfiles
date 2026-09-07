# Compose Messages

Apply the entrypoint’s [message safeguards](../SKILL.md#preserve-message-constraints), then use this decision order for each group:

1. Identify the dominant intended change, using verified task context rather than patch mechanics alone. Distinguish the actual change from capabilities that can merely be inferred from it.
2. Choose the narrowest durable repository concept that captures that intent. Name a concrete artifact or surface when sufficient. Move up to a capability, maintenance class, or subsystem only when the narrower objects are supporting details. Reuse established compact vocabulary such as `config` and `README` when it fits.
3. Choose a semantic verb. Added lines do not necessarily mean `Add`, and deleted lines do not necessarily mean `Remove`. Use the role guide below as a vocabulary aid, not an exhaustive list or a rigid taxonomy.
4. Add a qualifier only when it distinguishes a material condition, mechanism, purpose, or scope. Describe the delta rather than inventorying the resulting state. A conjunction may join objects under one action, but should not combine unrelated changes.
5. Apply the [message form](#message-form), then use the entrypoint’s [writing composition](../SKILL.md#writing-composition) with the established intent and constraints. Check that the subject covers its assigned hunks and the complete message claims no unverified motivation or outcome.

The role guide is alphabetized by editorial role:

| Role | Verb Choices |
| --- | --- |
| Adoption | Use `Install` for managed provisioning, `Set up` for integrated first-time configuration, and `Use` for adopting a selected mechanism. |
| Creation | Use `Add` for a concrete artifact or supported case, `Establish` for durable architecture, and `Introduce` for a named public option or substantial capability. |
| Maintenance | Use `Adjust` or `Tweak` for a bounded refinement and `Update` for an existing surface or recurring maintenance class. Do not force a distinction between near-synonyms that context cannot establish. |
| Organization | Use `Clean up` for heterogeneous pruning within one area and `Refactor` when structure or ownership is the organizing decision. `Refactor` does not guarantee behavior preservation. Prefer a direct operation such as `Extract`, `Move`, `Remove`, or `Rename` when that operation defines the change. |
| Outcomes | Use `Disable` or `Enable` for the resulting inactive or active state and `Warn on` for warning severity. Use `Fix` for an established defect. Choose a precise outcome verb such as `Ensure`, `Preserve`, `Prevent`, or `Reject` when it states the effect more clearly. |

## Message Form

- **Conventions:** Apply mandatory repository requirements and supplied wording as constraints. Within those constraints, use explicit user or calling workflow preferences before conventions found in relevant history. Conventional Commit prefixes and scopes are permitted when the selected convention calls for them, not required by this skill.
- **Grammar:** Without a narrower convention, write one compact, sentence case imperative clause, normally `<verb> <object>`, without terminal punctuation. Omit articles that add no meaning. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Literal names:** Put exact searchable tokens in backticks, including commands, configuration keys, domains, file labels, package selectors, paths, and rule IDs. Leave conceptual categories and canonically styled product names in prose. For example, `typescript@7` is a literal selector, while TypeScript is a product name.
- **Bodies:** Use the subject alone unless a short body adds verified motivation, constraints, or consequences that the subject cannot adequately convey. Do not restate the subject, narrate the patch, or add a routine testing checklist. Preserve bodies and trailers required by the selected message constraints, but do not invent issue references, attribution, or boilerplate.

Return the messages to the calling route. Prospective messages remain part of its read-only plan. Concrete commit proposals proceed through the shared [confirmation](../SKILL.md#confirm-commits).
