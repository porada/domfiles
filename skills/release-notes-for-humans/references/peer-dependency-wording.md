# Peer Dependency Wording

Make the direction of a peer dependency change clear from the opening verb. Format `<package>` for the release surface, keep `<range>` backticked, and join them with a literal `@`. Keep qualifiers such as `optional` beside the package they describe.

- **Higher minimum for one peer:** Write “Bumped peer dependency version to `<package>`@`<range>`.”
- **Higher minimums for several peers:** Write “Bumped peer dependency versions to …”
- **Lower minimum:** Write “Lowered peer dependency version to `<package>`@`<range>`.”
- **Wider accepted range:** Write “Expanded peer dependency version range to `<package>`@`<range>`.”
- **Classification change:** Name the change directly. For example, write “Moved `<package>` from an optional to a required peer dependency,” “Marked `<package>` as an optional peer dependency,” or “Removed `<package>` from peer dependencies.”
