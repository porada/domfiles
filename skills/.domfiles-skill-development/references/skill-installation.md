# Skill Installation

Use the applicable project’s skill classification to resolve each skill’s category, canonical source, logical name, metadata, supported installation, and whether the category permits bundled scripts before applying these rules. Evaluate changes, reviews, and audits against the complete supported installation model rather than inferring reach from the source location.

## Skill Identity

Keep each project-authored skill’s frontmatter `name` identical to its discovery name. For internal and public skills, this is the canonical directory basename. For global skills, including overlays, use the project’s mapping from canonical source directories to installed names so the name matches every final symlink basename.

Use action-based names for non-overlay skills that set `disable-model-invocation: true`.

## Internal Skill Names

Use unprefixed names for project-authored internal skills unless the repository bundles third-party skills. When it does, prefix project-authored internal skill names with the project name and preserve third-party names. Globally available skills outside the repository do not count as bundled third-party skills.

## Category Changes

Before moving content in a promotion that retains an overlay or another source owner, classify every existing rule and reference as promoted, retained under a named owner, or removed.

Before moving or rewriting content in a promotion to the public category, complete the [public promotion profile](public-skill-promotion.md#build-public-promotion-profiles).

For every category change, update the canonical location, metadata, documentation links, and installation behavior together.

Bring a skill’s scripts into conformance with the [portable skill script contract](portable-skill-scripts.md) before promoting it from internal to global. Remove or relocate them before promoting the skill into a public category that does not support bundled scripts.

## Global Reach

Write a global skill for every consuming project supported by its globally exposed installation. Treat any canonical source prefix used only for distribution as a namespace rather than a repository scope. Name or depend on the source repository only when the behavior requires repository-managed infrastructure.

## Distributed Links

For a skill with a supported installation outside its canonical repository:

- Keep relative links within the installed skill directory. Link to a sibling only when every supported installation guarantees that sibling and the same relative path resolves from the canonical source and every installation. Otherwise refer to the sibling by its frontmatter `name` without a Markdown link.
- Do not use relative links that leave the installed skills root or target a client-specific global instruction path. Refer to an already loaded global policy by its stable policy or section name instead.

Verify every relevant skill frontmatter name. Resolve every relative link from each supported installation root and reject links that escape the installed skills tree or target an unavailable peer.
