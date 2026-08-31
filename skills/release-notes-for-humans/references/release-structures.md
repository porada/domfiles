# Release Structures

Start with a flat release-note list. Use one of these structures only when it makes a material consumer decision easier to find or understand.

## Status Items

A status item communicates release state rather than an ordinary consumer outcome. Render its exact italicized text as one unordered-list item using the marker required by the release surface.

- **Initial release:** Use `_Initial release._` only after the user confirms that the release is initial. In an initial package section within a non-initial synchronized release, use it only when the package’s exhaustive inventory contains no more specific consumer-facing item. Do not substitute unbulleted prose or an emoji-only body.
- **No functional changes:** Use `_No functional changes._` only for a non-initial package section in a synchronized release whose exhaustive inventory contains no consumer-relevant item, including a dependency update or similar package change. A required synchronized version bump alone does not count as an item. Keep this status as the section’s only release-note item.

## Aggregate Release-Note Files

For a file that covers one package, use `# <package> Release Notes` as the top-level heading. Put `## Unreleased` first when present. Follow it with linked `## [<version>](<release-url>)` sections in newest-first order, separating releases with `---`. Omit a version link only when no canonical release URL is available.

For a new aggregate file shared by a synchronized package set, use a title established by the supplied context or repository metadata instead of selecting one package name. If neither source establishes a title, ask rather than inventing one.

Apply the [approval gates](approval-gates.md) before normalizing the delimiters in a supplied or previously approved aggregate file.

## Structured Releases

Add a level-3 heading when a release has a clear theme, migration context, or substantial group of related changes. Reserve level 4 for a subsection within that theme:

```text
### The Compatibility Release

- Added compatibility with another ecosystem implementation.
- Improved compatibility with existing plugins.

#### Option Handling

- Preserved explicitly tagged values when applying quoting options.
```

Keep detail lists flat under a neutral subject heading such as `Option Handling`. Do not use nested bullets merely to group details. When a top-level summary points to a later subsection, connect them with a compact cross-reference such as `(outlined below)`, and use the same wording throughout the note.

For a ready-to-paste hosted release body, reserve levels 1 and 2 for the surrounding page and release title.

Do not repeat migration prose in later patch releases unless the user explicitly requests the reminder. When migration guidance intentionally repeats across one major release line, name that major version rather than referring to “this release.”

## Synchronized Multi-Package Releases

Give every package a stable section when one repository publishes several packages under a shared release:

```text
### Breaking Changes

<migration-and-consumer-impact>

### All Packages

- Raised the shared runtime requirement.

### @scope/core

- Added the new core behavior.

### @scope/integration

- _No functional changes._
```

Put `Breaking Changes` first, followed by `All Packages` when needed. Then use the package order established by the supplied draft or repository metadata. If neither establishes an order, alphabetize the package sections.

Use `All Packages` only for a change that applies uniformly to every package in the synchronized set. Do not repeat that change in package-specific sections.

When synchronized packages must publish together, retain one section for every package in the set. Apply the [status-item rules](#status-items) when a package has no consumer-relevant item. Outside a synchronized release, having no item to list normally does not warrant a release.
