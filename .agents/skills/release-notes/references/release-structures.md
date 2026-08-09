# Release structures

The exact italicized status bullets in this reference are status markers rather than ordinary outcome prose. Their leading `*` is intentional and follows the [release-note bullet-marker policy](../SKILL.md#write-concise-consumer-facing-prose). Preserve it instead of normalizing it.

When an initial release or initial package section needs no more specific consumer-facing item, use exactly `* _Initial release._`. Do not use unbulleted prose or an emoji-only body.

Do not repeat migration prose in later patch releases unless the user explicitly requests that reminder. When migration prose is intentionally repeated across one major release line, name that major version explicitly instead of referring to “this release.”

## Aggregate release-note file

Use `# <package> Release Notes` as the canonical top-level heading. Put `## Unreleased` first when present, then list linked `## [<version>](<release-url>)` sections newest-first and separate them with `---`. Omit the link only when no canonical release URL is available.

For a new aggregate file shared by a synchronized package set, use a top-level title established by supplied context or repository metadata instead of selecting one package name. If neither establishes a title, ask rather than inventing one.

When a supplied or previously approved aggregate file differs, preserve its heading hierarchy and inter-release delimiters unless the user approves normalization under the [approval gates](approval-gates.md).

## Structured release

Use a level-3 heading when a release has a clear theme, migration context, or substantial group of related changes. Use level 4 only for a subsection nested under that theme:

```text
### The Compatibility Release

* Added compatibility with another ecosystem implementation.
* Improved compatibility with existing plugins.

#### Option Handling

* Preserved explicitly tagged values when applying quoting options.
```

Do not use nested bullets merely to group details. Keep detail lists flat under a neutral subject heading such as `Option Handling`. When a top-level summary points to a later detail subsection, connect it with a compact cross-reference such as `(outlined below)` and keep the chosen wording consistent.

For a ready-to-paste hosted release body, reserve levels 1 and 2 for the surrounding page and release title.

## Synchronized multi-package release

Use stable package sections when one repository publishes several packages under a shared release:

```text
### Breaking Changes

Describe the migration and its consumer impact.

### All Packages

* Raised the shared runtime requirement.

### @scope/core

* Added the new core behavior.

### @scope/integration

* _No functional changes._
```

Put `Breaking Changes` first, followed by `All Packages` when needed, then package-specific sections in the stable order established by the supplied draft or repository metadata. If no stable order is defined, alphabetize the package sections.

Use `All Packages` only for a change that applies uniformly to every package in the synchronized set. Do not repeat that change in package-specific sections.

When synchronized packages must publish together, retain one section for every package in the synchronized set. For a non-initial package section, use exactly `* _No functional changes._` only when the package’s exhaustive inventory yields no other consumer-relevant item to list, including no dependency update or similar package change. A required synchronized version bump alone does not count as an item. Never combine the label with another release-note bullet. Outside a synchronized release, having no item to list normally does not warrant a release.
