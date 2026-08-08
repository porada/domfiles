# Peer dependency wording

Use these forms for peer dependency range and classification changes:

- When raising the baseline for one peer dependency, write “Bumped peer dependency version to `package@range`.” Omit the article `the`.
- When raising the baselines for several peer dependencies, write “Bumped peer dependency versions to …” and keep a qualifier such as `optional` adjacent to the package it qualifies.
- Write “Lowered peer dependency version to `package@range`” when lowering its baseline and “Expanded peer dependency version range to `package@range`” when widening its accepted range.
- Describe classification changes directly, for example, “Moved `package` from an optional to a required peer dependency,” “Marked `package` as an optional peer dependency,” or “Removed `package` from peer dependencies.”
