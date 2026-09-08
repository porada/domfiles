# Skill Descriptions

Apply this contract when authoring or assessing a skill’s description or invocation mode. Resolve identity and installation through [Skill Installation](skill-installation.md).

## Invocation Mode

Treat a project-authored skill as **command-only** when it sets `disable-model-invocation: true`, and as **model-invocable** otherwise.

For every model-invocable project-authored skill, preserve complete positive and negative trigger boundaries and precedence conditions, and state them as direct reasons to load the skill. Include a peer name only when it defines one such condition. Do not make loading depend on whether the task already exhibits a quality the skill enforces, such as being “focused,” “simple,” “safe,” or “bounded.”

For every command-only project-authored skill, write concise human-facing UI copy instead. Lead with the action and outcome, add only task scope or exclusions that distinguish the command, and omit the invocation condition and literal slash command.

When a description advertises review or audit, define an explicit read-only branch in the body that follows the shared workflow precedence in `agent-documentation`.

## Description and Body

For internal and global skills, keep necessary implementation details, operational guidance, rationale, and workflows in the body. In model-invocable descriptions, also move capability exposition, behavior, and outputs to the body, and remove body text that only repeats why the skill loaded.

Apply that description content limit in both directions. Keep behavioral defaults, validation, optional composition, and internal workflow in the body, and do not let the body merely paraphrase the description’s trigger or exclusion.

For public skills in either invocation mode, also apply the [public description additions](public-skill-portability.md#keep-public-descriptions-portable).

## Composition and Exclusions

Treat two skills matching one task as ordinary composition. Narrow a description only when the skills state contradictory rules for the same decision or duplicate one normative rule, and prefer a deferral clause naming the sibling over an exclusion that removes the surface. An exclusion that ends a correct overlap fails silently, because the skill simply stops loading.

Before adding or retaining a description exclusion, search project-authored skills for explicit routes that compose the excluded skill or surface. When the broad skill remains independently useful, place domain-specific non-composition policy in the narrower workflow rather than excluding the domain from the broad skill.

## Encoding and Size

Encode every project-authored skill description as a YAML `|-` literal block scalar. Treat each thematic unit as a paragraph, and separate paragraphs with a blank line. When another frontmatter field follows the description, place one blank line between the description value and that field.

For every project-authored skill description, keep the decoded value within the strictest limit any supported client imposes, treating 1,024 UTF-8 bytes as that limit unless a supported client documents a stricter one. Measure the decoded value rather than the complete frontmatter or source line. Shorten redundant phrasing before dropping trigger distinctions, and validate every project-authored skill description after changing this policy or any description.
