# Domfiles Skill Policy

Apply these requirements only to skills maintained in domfiles, using the classification in the repository’s `AGENTS.md`. They supplement `skill-development` without imposing domfiles conventions on other repositories.

## Apply Category Contracts

Follow the [internal skill naming policy](skill-installation.md#internal-skill-names) when naming Internal category skills.

- **Global context:** Global skills may rely on domfiles-managed global instructions and the complete globally exposed skill set.
- **Global overlays:** Inherit global requirements. Their `<base-name>` must exactly match the base skill’s frontmatter `name`, without shortening, rewording, or dropping qualifiers.
- **Public independence:** Public skills must deliver their advertised behavior when independently installed.
- **Script ownership:** Internal and global skills may own scripts. Public skills remain documentation-only.

## Synchronize Skill Links

After task-authorized changes under `skills/` alter the exposed skill set, logical names, or source-to-install mapping, update the applicable provisioning and retired-name mappings, then reconcile the affected links in `~/.agents/skills/` and `~/.claude/skills/` before completing the task. Do this directly rather than asking the user to run `domfiles sync`, running full synchronization, or adding a separate synchronization command. Content-only edits to already linked sources require no link update.

Establish the provisioned checkout from the installed `domfiles` command or existing managed links before choosing targets. Keep discovery pointed at that checkout, not a disposable task worktree. If the task’s changes are in another checkout or their intended source is not yet available in the provisioned checkout, leave the affected links unchanged and report reconciliation pending integration there.

Use the task’s before-and-after source paths and installed names to bound the affected links. Inspect their path-level state before mutation, retain existing link targets until verification, and create, replace, or remove only links whose task ownership is established. Preserve unrelated skills, real files and directories, and links owned by another installation. Return conflicts or required access grants through the existing execution checkpoint rather than broadening cleanup.

Verify that each affected installed name resolves to its intended source and that task-retired names are absent. Confirm both discovery roots without reading unrelated skill contents or configuration.

## Maintain Public Behavior

Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.

When changing a public skill contract or its standalone fallback, consult the **Public Skill Fallback Families** section of domfiles’ `.agents/PROJECT.md` to identify related guidance.

## Develop Scripts

For tasks involving skill-owned scripts, their command wrappers, or build and test integration, follow the [domfiles script policy](domfiles-script-policy.md) before choosing an implementation language or resolving an interface.
