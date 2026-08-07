# Release structures

Choose the smallest structure that communicates each material consumer decision and follows the repository’s established release history.

## Concise single-package release

Use a flat bullet list when every change belongs to one package and no thematic section improves comprehension:

```text
* Added support for the new input form.
* Fixed handling of empty values.
* Lowered the `engines` baseline to `node@>=20` (from `node@>=22`).
```

## Structured release

Use a level-3 heading when a release has a clear theme, migration context, or substantial group of related changes. Use level 4 only for a subsection nested under that theme:

```text
### The Compatibility Release

* Added support for another ecosystem implementation.
* Improved compatibility with existing plugins.

#### Option Handling

* Preserved explicitly tagged values when applying quoting options.
```

For a ready-to-paste hosted release body, reserve levels 1 and 2 for the surrounding page and release title. If the user instead requests an aggregate changelog, follow that file’s existing heading hierarchy.

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

Put `Breaking Changes` first, followed by `All Packages` when needed, then package-specific sections in the repository’s established order.

When synchronized packages must publish together, retain every package section required by the established structure. Use exactly `* _No functional changes._` only when the package’s exhaustive inventory yields no other consumer-relevant item to list, including no dependency update or similar package change. A required synchronized version bump alone does not count as an item. Never combine the label with another release-note bullet. Outside a synchronized release, having no item to list normally does not warrant a release.

Do not repeat migration prose in later patch releases unless the user or established repository policy intentionally requires that reminder.
