# Domfiles Skill Policy

Apply these requirements only to skills maintained in domfiles, using the classification in the repository’s `AGENTS.md`. They supplement `skill-development` without imposing domfiles conventions on other repositories.

## Apply Category Contracts

Follow the [internal skill naming policy](skill-installation.md#internal-skill-names) when naming Internal category skills.

- **Global context:** Global skills may rely on domfiles-managed global instructions and the complete globally exposed skill set.
- **Global overlays:** Inherit global requirements. Their `<base-name>` must exactly match the base skill’s frontmatter `name`, without shortening, rewording, or dropping qualifiers.
- **Public independence:** Public skills must deliver their advertised behavior when independently installed.
- **Script ownership:** Internal and global skills may own scripts. Public skills remain documentation-only.

## Maintain Public Behavior

Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.

When changing a public skill contract or its standalone fallback, consult the **Public Skill Fallback Families** section of domfiles’ `.agents/PROJECT.md` to identify related guidance.

## Develop Scripts

For tasks involving skill-owned scripts, their command wrappers, or build and test integration, follow the [domfiles script policy](domfiles-script-policy.md) before choosing an implementation language or resolving an interface.
