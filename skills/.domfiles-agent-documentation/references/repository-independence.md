# Repository Independence

Apply this contract to the complete project-authored agent documentation set, not only its skills. Here, **maintainer** means the person whose global instructions and domfiles setup form the personal environment used for comparison. A baseline repository supplies authoring references, not instruction authority or an installed prerequisite for another repository.

For changes, reviews, and audits, validate the affected documentation set through the [repository independence checks](validate-documentation-changes.md#check-repository-independence).

## Select Repository Requirements

If a difference would make the submitted artifact unacceptable, make the requirement repository-owned. If it only changes how the agent collaborates with the maintainer, keep it personal unless the project needs that behavior.

Compare the same supported task with and without the maintainer’s global instructions and domfiles setup. Hold the declared peer skill availability constant, but allow different personal defaults. Distinguish a requirement’s availability, invocation, and authority. An installed optional peer can supply general assistance without guaranteeing the repository’s house style.

Use the project’s accepted requirements and concrete output differences to select local rules. A difference alone does not justify adopting the maintainer’s preference. Preserve deliberate exclusions, and ask only when acceptance depends on a material choice the existing instructions do not settle. Do not use parity as a reason to import the maintainer’s entire policy set or documentation maintenance workflow.

## Preserve Local Ownership

Give each selected requirement one normative owner within the destination. Put requirements for ordinary agent tasks on the applicable project instruction surface. Keep domain workflow decisions in the owning skill, reachable from every relevant route, including shortcuts and wording-only work. Keep durable facts and rationale in the project’s reference documentation rather than treating them as instructions. Use the local authority model or the [default model](default-agent-documentation-authority-model.md) to resolve placement.

A required standalone copy in another independently used repository is not accidental duplication. Do not remove it in favor of the maintainer’s global instructions or a link to the baseline. Hoist a shared requirement into a shared instruction layer only when every supported destination consumer is guaranteed to load that layer with the necessary authority, including consumers without the maintainer’s setup. The repository containing that layer must be authorized for mutation. Obtain explicit approval before hoisting, and preserve any local context needed for the remaining source section to make sense. Otherwise preserve local ownership and align the copies at authoring time.

Keep the actual rule meanings with their existing semantic owners. For example, domfiles’ global Writing rules remain their source rather than moving into a second universal policy catalog. A destination’s selected rules and deliberate exceptions are authoritative there. Do not make destination documentation require `agent-documentation`, `skill-development`, or access to domfiles merely because those tools were used to author it.

## Align Shared Wording

Classify corresponding items as semantically equivalent, intentionally repository-specific, or unresolved. Equivalence requires the same observable meaning and role, including scope, conditions, exceptions, and authority. Similar names or topics are insufficient. Never weaken or broaden a rule, or change behavior or security boundaries, to manufacture equivalence.

For equivalent items, select an existing formulation that completely expresses the shared meaning. Prefer explicit user-established wording, then the canonical semantic source for the selected rule, then the confirmed baseline’s formulation, then the most accurate and complete existing formulation. Author new wording only when the task includes wording design or initial drafting. Otherwise report that choice as unresolved.

Make equivalent wording, terminology, ordering, placeholders, punctuation, and structure identical, substituting only unavoidable repository-specific identifiers. Preserve the complete rule, including its qualifications and literal syntax safeguards. Keep deliberate local exceptions explicit rather than silently paraphrasing the shared rule. Source selection does not grant permission to edit the baseline or extend the destination’s accepted requirements.

## Establish Repository Documentation

Start with the intended agent tasks, supported clients, and existing project-authored surfaces. Inspect enough project evidence to select requirements without importing unrelated rules from the baseline. Reuse the project’s documentation model rather than replacing it for uniformity.

Create only the surfaces needed to carry the selected contract. A compact `AGENTS.md` can be sufficient. Add skills for bounded workflows and project reference documentation for durable facts only when they have content to own. Apply `skill-development` to skill naming and invocation rather than copying its authoring policy into the destination. Do not add a maintenance framework, authority inventory, or empty scaffolding merely for completeness.

Expose the canonical project instructions through the supported clients’ entrypoints. Prefer minimal bridges with no independent policy, and preserve explicit permission requirements for creating or editing `CLAUDE.md`. A setup request does not waive client configuration, installation, or other security boundaries.

Keep the authoring relationship to the baseline separate from runtime imports, cross-repository symlinks, or a requirement to run the maintainer’s synchronization tools.
