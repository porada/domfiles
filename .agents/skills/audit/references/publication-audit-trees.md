# Publication Audit Trees

By default, inspect tracked `HEAD` directly rather than materializing an isolated filesystem copy. Capture its exact commit once and use that commit for every read-only Git inspection. Select the repository root through the tool’s working directory parameter or `git -C <repository-root>`, and do not let a moving ref, the current shell subdirectory, or the working tree narrow or alter the evidence.

An explicit user direction may select another route, but it doesn’t establish that an existing tool preserves the selected tree. Don’t design or implement a materializer without authorization for that additional work. When the audit requires an isolated copy, follow [Publication Audit Staging](publication-audit-staging.md).
