# Skill Descriptions

## Invocation Mode

Treat a project-authored skill as **command-only** when it sets `disable-model-invocation: true`, and as **model-invocable** otherwise.

For every model-invocable project-authored skill, preserve complete positive and negative trigger boundaries and precedence conditions, and state them as direct reasons to load the skill. Include a peer name only when it defines one such condition. Don’t make loading depend on whether the task already exhibits a quality the skill enforces, such as being “focused,” “simple,” “safe,” or “bounded.”

For every command-only project-authored skill, write concise human-facing UI copy instead. Lead with the action and outcome, add only task scope or exclusions that distinguish the command, and omit the invocation condition and literal slash command.

When a description advertises review or audit, define an explicit read-only branch in the body or routed references that follows the shared workflow precedence in `agent-documentation`.

## Description and Body

For internal and global skills, keep necessary implementation details, operational guidance, rationale, and workflows in the [body or routed references](skill-authoring.md#entrypoints-and-references), including behavioral defaults, validation, optional composition, and internal workflow. Don’t let the body merely paraphrase the description’s trigger or exclusion. For model-invocable skills in these categories, also move capability explanations, behavior, and outputs from the description to the body, and remove body text that only repeats why the skill loaded.

For public skills in either invocation mode, also apply the [public description additions](public-skill-portability.md#keep-public-descriptions-portable).

## Composition and Exclusions

Treat two skills matching one task as ordinary composition. Narrow a description only when the skills state contradictory rules for the same decision or duplicate one normative rule. Prefer a deferral clause naming the sibling over an exclusion that removes the surface. An exclusion that ends a correct overlap fails silently because the skill simply stops loading.

Before adding or retaining a description exclusion, search project-authored skills for explicit routes that compose the excluded skill or surface. When the broad skill remains independently useful, place domain-specific non-composition policy in the narrower workflow rather than excluding the domain from the broad skill.

## Encoding and Size

Encode every project-authored skill description as a YAML `|-` literal block scalar. Treat each thematic unit as a paragraph, and separate paragraphs with a blank line. When another frontmatter field follows the description, place one blank line between the description value and that field.

For every project-authored skill description, keep the decoded value within a conservative authoring cap of 1,024 UTF-8 bytes and any stricter limit imposed by a supported client. Measure the decoded value rather than the complete frontmatter or source line. Shorten redundant phrasing before dropping trigger distinctions.

## Validation Scope

- **Discovery, invocation, or routing changes:** Validate affected descriptions and their relationships with competing or composed skills and entrypoints across the complete affected contract.
- **Isolated description changes:** Validate each complete changed description for encoding, decoded size, and applicable semantic requirements, including trigger distinctions. An isolated edit doesn’t require checking unrelated descriptions. If it changes discovery, invocation, or routing, also apply that broader coverage.
- **Shared policy changes:** Validate every description governed by the changed requirement against that requirement. Changes to the shared encoding or size policy require checking every project-authored description for encoding and decoded size, without an unrelated semantic re-review of the inventory.
- **Broader audits:** Apply `agent-documentation`’s **Select Validation Scope** to the requested description inventory and checks.
