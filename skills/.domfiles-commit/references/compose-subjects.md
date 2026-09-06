# Compose Subjects

Apply the entrypoint’s [message safeguards](../SKILL.md#preserve-message-constraints), then use this decision order for each group:

1. Identify the dominant intended change, using verified task context rather than patch mechanics alone. Distinguish the actual change from capabilities that can merely be inferred from it.
2. Choose the narrowest durable repository concept that captures that intent. Name a concrete artifact or surface when sufficient. Move up to a capability, maintenance class, or subsystem only when the narrower objects are supporting details. Reuse established compact vocabulary such as `config` and `README` when it fits.
3. Choose a semantic verb. Added lines do not necessarily mean `Add`, and deleted lines do not necessarily mean `Remove`. Use the role guide below as a vocabulary aid, not an exhaustive list or a rigid taxonomy.
4. Add a qualifier only when it distinguishes a material condition, mechanism, purpose, or scope. Describe the delta rather than inventorying the resulting state. A conjunction may join objects under one action, but should not combine unrelated changes.
5. Apply the [message form](#message-form). Check that the subject covers its assigned hunks without claiming an unverified motivation or outcome.

The role guide is alphabetized by editorial role:

| Role | Verb Choices |
| --- | --- |
| Adoption | Use `Install` for managed provisioning, `Set up` for integrated first-time configuration, and `Use` for adopting a selected mechanism. |
| Creation | Use `Add` for a concrete artifact or supported case, `Establish` for durable architecture, and `Introduce` for a named public option or substantial capability. |
| Maintenance | Use `Adjust` or `Tweak` for a bounded refinement and `Update` for an existing surface or recurring maintenance class. Do not force a distinction between near-synonyms that context cannot establish. |
| Organization | Use `Clean up` for heterogeneous pruning within one area and `Refactor` when structure or ownership is the organizing decision. `Refactor` does not guarantee behavior preservation. Prefer a direct operation such as `Extract`, `Move`, `Remove`, or `Rename` when that operation defines the change. |
| Outcomes | Use `Disable` or `Enable` for the resulting inactive or active state and `Warn on` for warning severity. Use `Fix` for an established defect. Choose a precise outcome verb such as `Ensure`, `Preserve`, `Prevent`, or `Reject` when it states the effect more clearly. |

## Recurring Forms

- **Dependencies:** Use `Update dependencies` for a dependency-maintenance batch, including directly caused compatibility, configuration, or generated changes. Name one package when its update is deliberately singled out. Use `Use` when adopting a selected tool or version is the dominant decision.
- **Documentation:** Use `Update documentation` for a documentation follow-up in a contribution to another repository. A specific maintained surface may instead justify a narrower subject, such as “Update `README`.”
- **Special commits:** Use `Initial commit` for a repository root. Use a bare semantic version only for an actual release commit when that form is established in the repository. Do not choose or change a release version as part of composing its subject.

## Message Form

- **Grammar:** Write one compact, sentence-case imperative clause, normally `<verb> <object>`, subject to the special forms above. Omit articles that add no meaning, colons, Conventional Commit prefixes, scope labels, and terminal punctuation. Preserve necessary precision rather than imposing a fixed word or character limit.
- **Literal names:** Put exact searchable tokens in backticks, including commands, configuration keys, domains, file labels, package selectors, paths, and rule IDs. Leave conceptual categories and canonically styled product names in prose. For example, `typescript@7` is a literal selector, while TypeScript is a product name.
- **Bodyless output:** The subject is the complete proposed message. Do not generate a body, explanatory prose, issue-reference paragraphs, testing checklists, or trailers. Preserve an existing hosted `(#<number>)` subject suffix when it belongs to the selected message, but never invent one.

Return the messages to the calling route. Prospective subjects remain part of its read-only plan. Concrete commit proposals proceed through the shared [confirmation](../SKILL.md#confirm-commits).
