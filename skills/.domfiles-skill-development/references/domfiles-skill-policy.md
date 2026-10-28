# Domfiles Skill Policy

Apply these requirements only to skills maintained in domfiles. They supplement `skill-development` without imposing domfiles conventions on other repositories.

## Classify Skills

Classify every project-authored skill by canonical source and supported installation. `metadata.internal: true` means public installation is unsupported, not that tracked source is private. Categories below widen installation reach, with the global overlay variant beside its category.

| Category | Canonical Source | `name` | `metadata.internal` | Supported Installation |
| --- | --- | --- | --- | --- |
| Internal | `.agents/skills/<skill-name>` | `<skill-name>` | `true` | Project-local to domfiles. |
| Global | `skills/.domfiles-<skill-name>` | `<skill-name>` | `true` | Globally exposed as `<skill-name>` through the system established by `domfiles sync`. |
| Global overlay | `skills/.dom-<base-name>` | `dom-<base-name>` | `true` | Same installation surface as the global category. |
| Public | `skills/<skill-name>` | `<skill-name>` | Omitted | Globally exposed through `domfiles sync` and independently installable through `skills` without `domfiles`. |

## Apply Category Contracts

- **Global context:** Global skills may rely on domfiles-managed global instructions and the complete globally exposed skill set.
- **Global overlays:** Inherit global requirements. Their `<base-name>` must exactly match the base skill’s frontmatter `name`, without shortening, rewording, or dropping qualifiers.
- **Public independence:** Public skills must deliver their advertised behavior when independently installed.
- **Script ownership:** Internal and global skills may own scripts. Public skills remain documentation-only.

## Synchronize Skill Links

When task-authorized changes under `skills/` alter the exposed skill set, logical names, or source-to-install mapping, update the [maintained skill inventory](maintained-skills.md) and the applicable provisioning and retired-name mappings, then reconcile the affected links in `~/.agents/skills/` and `~/.claude/skills/` before completing the task. Reconcile those links directly rather than asking the user to run `domfiles sync`, running full synchronization, or adding a separate synchronization command. Content-only edits to sources already linked require no link update.

Keep the inventory complete, with names alone for model-invocable skills and short summaries for command-only skills. Keep summaries aligned when responsibilities change, and add or remove them when invocation modes change.

Identify the provisioned checkout from the installed `domfiles` command or existing managed links before choosing targets. Keep discovery pointed at that checkout, not a disposable task worktree. If the task’s changes are in another checkout or their intended source is not yet available in the provisioned checkout, leave the affected links unchanged and report that reconciliation awaits integration into that checkout.

Use the task’s before-and-after source paths and installed names to identify exactly which links it affects. Inspect their path-level state before mutation, retain existing link targets until verification, and create, replace, or remove only links whose task ownership is established. Preserve unrelated skills, real files and directories, and links owned by another installation. Use the existing execution checkpoint for conflicts or required access grants rather than broadening cleanup.

Verify that each affected installed name resolves to its intended source and that names retired by the task are absent. Confirm both discovery roots without reading unrelated skill contents or configuration.

## Maintain Public Behavior

Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.

When changing a public skill contract or its standalone fallback, consult the **Shared Skill Contracts** section of domfiles’ `.agents/PROJECT.md` to identify related guidance.

## Develop Scripts

Write skill-owned scripts in TypeScript that Node.js runs directly, without a build step. Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate.

Treat every skill-owned script as a [portable command](skill-owned-scripts.md#support-portable-commands), regardless of its skill’s installation category, without a `home/.local/bin` wrapper. Run TypeScript scripts with `node`. Write their adjacent tests with `node:test` and `node:assert`, and run independent tests concurrently through `describe(…, { concurrency: true })`.
