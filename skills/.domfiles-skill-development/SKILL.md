---
name: skill-development
description: |-
    Use for creating, editing, reviewing, auditing, or maintaining project-authored skills, including their assets, descriptions, invocation modes, references, scripts, and adjacent contract tests. Also use for skill categories, discovery, installation reach, promotion, public skill READMEs, optional peers, standalone behavior, and script contract architecture.

    Use when a global instruction changes or is evaluated and supplies a public skill mirror, or when a project-authored asset canonically supplies a public skill surface, regardless of that asset’s source category.

    Do not use for ordinary project source or agent documentation that does not affect a skill contract. Defer to a more specific project skill development workflow when one exists.

metadata:
    internal: true
---

# Skill Development

## Resolve Skill Scope

Resolve each in-scope skill through [Skill Installation](references/skill-installation.md) before planning, including during reviews and audits. Apply the consuming project’s instructions rather than assuming that source location determines installation reach.

For documentation work, including metadata, references, assets, and public READMEs, compose with `agent-documentation`. It owns the shared authority, ownership, writing composition, change, review, validation, and reporting lifecycle. Add the contracts selected below without restarting that lifecycle. For script-only work without documentation changes, follow the script route directly.

An explicit change takes precedence when the request also uses review or audit language. Standalone reviews and audits remain read-only. Select every applicable contract before the work it governs, and load each once.

## Select the Contracts

- **Documentation:** When authoring, reviewing, auditing, or maintaining skill documentation, follow [Skill Authoring](references/skill-authoring.md) for entrypoints, references, overlays, examples, and inherited policy.
- **Descriptions:** When authoring or assessing a skill’s description or invocation mode, follow [Skill Descriptions](references/skill-descriptions.md).
- **Public behavior:** For every public skill or canonical public-surface asset in scope, follow [Public Skill Portability](references/public-skill-portability.md). Also follow it for every public mirror affected by a global instruction change or evaluation, even when the request names only the global source. Select its conditional creation, promotion, naming, and optional-peer contracts before the corresponding work. Its README contract applies when creating or updating a public skill README.
- **Script contracts:** Before planning a new or changed skill-owned script, adjacent contract test, or script-owned artifact, or reviewing or auditing one, follow [Skill-Owned Scripts](references/skill-owned-scripts.md). This skill owns necessity, canonical ownership and location, observable interfaces, operation modes, effects, artifacts, schemas, invocation, and contract coverage. The owning domain skill supplies domain semantics, and applicable implementation workflows govern internal concerns.

## Preserve Protected Mutation

Before changing any project-authored path under `.agents/skills` or `skills/human-facing-writing`, resolve its canonical owner. Follow the [protected skill mutation policy](references/protected-skill-mutation.md) before every mutation under `skills/human-facing-writing` and, when your agent identity is Zed Agent, under `.agents/skills`. Keep standalone reviews and audits read-only.

## Keep Skill Contents Operational

Never document auxiliary information anywhere in a project-authored skill directory. Auxiliary information records how the skill itself was authored or maintained without helping a reader understand or execute its advertised workflow, such as change histories, research logs, review records, and verification baselines. Keep durable context in the source project’s reference documentation and task-local evidence in the conversation or task artifacts outside the skill. Preserve operational requirements, safety boundaries, and supporting explanations needed to apply the skill.

## Validate the Selected Contracts

For every change, review, or audit, run the installation contract’s identity and link checks and every applicable check from the selected references. Validate complete declared scopes, including affected mirror families and the full description set when required. Conditional loading does not reduce an invariant to changed lines or omit validation of an unchanged declaration that the active contract covers.

For documentation work, return the skill-specific results to `agent-documentation`’s shared validation and reporting lifecycle. For script-only work, complete the script contract’s validation and report concrete contract findings or the resulting change and any limitations under the applicable communication policy. Do not turn a contract review into an implementation audit.
