# Package Links

Use this table as the complete allowlist and destination map for package hyperlinks in release-note prose outside headings.

| Package | Destination |
| --- | --- |
| `@standard-config/oxlint` | `https://github.com/standard-config/oxlint` |
| `@standard-config/oxlint-react` | `https://github.com/standard-config/oxlint/tree/main/packages/oxlint-react` |
| `@standard-config/oxlint-stylistic` | `https://github.com/standard-config/oxlint/tree/main/packages/oxlint-stylistic` |
| `@standard-config/prettier` | `https://github.com/standard-config/prettier` |
| `@standard-config/template` | `https://github.com/standard-config/template` |
| `@standard-config/tsconfig` | `https://github.com/standard-config/tsconfig` |
| `prettier-plugin-expand-json` | `https://github.com/porada/prettier-plugin-expand-json` |
| `prettier-plugin-markdown-html` | `https://github.com/porada/prettier-plugin-markdown-html` |
| `prettier-plugin-yaml` | `https://github.com/porada/prettier-plugin-yaml` |
| `vitest-react-serializer` | `https://github.com/porada/vitest-react-serializer` |

Apply these release-scope exclusions before using a mapping:

- Keep the package currently being documented and every package in the current synchronized release set backticked and unlinked.
- When the resolved release scope contains an `@standard-config/*` package, keep every `@standard-config/*` package name backticked and unlinked.

For every other listed package, always render its name as a code-formatted link to the exact mapped destination: ``[`<package>`](<destination>)``.

Keep every unlisted package name backticked and unlinked. Do not infer or search for an unlisted mapping during the release-note task.

This map governs package-name links only. Continue to link advisories, migrations, specifications, pull requests, and other non-package evidence when the link helps consumers understand the change.
