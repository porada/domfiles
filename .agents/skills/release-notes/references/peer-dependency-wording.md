# Peer dependency wording

Use these forms for peer dependency range and classification changes. Render `<package>` through the [package-link map](package-links.md), keep `<range>` backticked, and join them with a literal `@`. Keep a qualifier such as `optional` adjacent to the package it qualifies in every form.

- When raising the baseline for one peer dependency, write “Bumped peer dependency version to `<package>`@`<range>`.”
- When raising the baselines for several peer dependencies, write “Bumped peer dependency versions to …”.
- Write “Lowered peer dependency version to `<package>`@`<range>`.” when lowering its baseline and “Expanded peer dependency version range to `<package>`@`<range>`.” when widening its accepted range.
- Describe classification changes directly, for example, “Moved `<package>` from an optional to a required peer dependency,” “Marked `<package>` as an optional peer dependency,” or “Removed `<package>` from peer dependencies.”
