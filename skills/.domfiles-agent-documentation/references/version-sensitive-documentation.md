# Version-Sensitive Documentation

Establish one authoritative behavioral baseline before editing. Use the documented tool, runtime, or language’s most recent stable release unless direct user instruction or authoritative target environment evidence establishes another version.

For newly authored or materially revised security boundary claims, verify behavior against the selected upstream baseline and retain the exact revision and supporting source paths in the conversation or task artifacts. Record verification evidence in the source project’s reference documentation, outside skill directories, only when the user explicitly requests a durable audit trail. Version requirements and explanatory source links remain ordinary documentation, not verification records.

Evaluate conflicting evidence against the selected baseline before changing documentation. Do not combine current documentation, pinned source, and upstream `main` as if they describe one implementation. Write only the interfaces, semantics, and syntax of the selected baseline. Do not add compatibility branches, historical caveats, legacy forms, version detection, or version migration guidance. Report when the baseline cannot be verified rather than guessing.
