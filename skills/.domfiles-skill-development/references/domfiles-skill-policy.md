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

When task-authorized changes under `skills/` alter the exposed skill set, logical names, or source-to-install mapping, update the [maintained skill inventory](maintained-skills.md) and the applicable provisioning and retired-name mappings together. Validate the proposed source before activating it through global discovery.

Keep the inventory complete, with names alone for model-invocable skills and short summaries for command-only skills. Keep summaries aligned when responsibilities change, and add or remove them when invocation modes change.

For worktree validation, check the proposed frontmatter names, inventory, provisioning mappings, and retired names against that worktree’s source. Apply [Distributed Links](skill-installation.md#distributed-links) using the proposed installed names and directory layout. Resolve relative Markdown links against the worktree sources, not the currently installed skill contents. When verifying that resolution requires an installation layout on disk, create discovery symlinks only in task-owned temporary roots without registering them with a client. These checks validate source and installation layout, not live client discovery.

Identify the provisioned checkout from the installed `domfiles` command or existing managed links before choosing activation targets. Keep discovery pointed at that checkout, not a disposable task worktree. If the intended source is not yet available there, leave global links unchanged. When integration is outside the task’s scope, the worktree task may complete after validation, reporting the pending integration and the affected installed names, source paths, and retired names. Content-only edits require no link changes, but worktree edits become active only after integration into the linked source.

The task changing the provisioned checkout, through direct edits or integration, owns activation. Within its authorization, reconcile the affected links in `~/.agents/skills/` and `~/.claude/skills/` directly before completing that task. Don’t ask the user to run `domfiles sync`, run full synchronization, or add a separate synchronization command for this purpose. A handoff records remaining work, not permission for a later task. When integration or link mutation is outside the active authority, report the remaining action and obtain coverage before performing it.

Use the source change’s before-and-after paths and installed names, including any handoff from the worktree task, to identify exactly which links it affects. Inspect their path-level state before mutation, retain existing link targets until verification, and create, replace, or remove only links whose task ownership is established. Preserve unrelated skills, real files and directories, and links owned by another installation. Use the existing execution checkpoint for conflicts or required access grants rather than broadening cleanup.

Verify that each affected installed name resolves to its intended source and that names retired by the source change are absent. Confirm both discovery roots without reading unrelated skill contents or configuration.

## Maintain Public Behavior

Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.

When changing a public skill contract or its standalone fallback, consult the **Shared Skill Contracts** section of domfiles’ `.agents/PROJECT.md` to identify related guidance.

## Develop Scripts

Write skill-owned scripts in TypeScript that Node.js runs directly, without a build step. Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate.

Treat every skill-owned script as a [portable command](skill-owned-scripts.md#support-portable-commands), regardless of its skill’s installation category, without a `home/.local/bin` wrapper. Run TypeScript scripts with `node`. Write their adjacent tests with `node:test` and `node:assert`, and run independent tests concurrently through `describe` with a bounded `concurrency` value suited to the available execution capacity.
