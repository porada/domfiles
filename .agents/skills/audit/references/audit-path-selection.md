# Audit Path Selection

Resolve explicit audit scope in two steps: select candidate paths, then apply the exclusions that remain.

## Select Candidate Paths

Classify a path as **tracked** if Git tracks it or it is a directory containing tracked files. Otherwise, classify it as **ignored** if Git ignores it, or **untracked** if it does not.

- **Named paths:** Select each named path. For a directory, also select descendants with the same classification.
- **Path categories:** Select repository paths in an expressly requested group within the requested scope. A language or configuration category, such as Fish scripts and config, selects tracked paths and any ignored paths that applicable `AGENTS.md` scope rules include. Other ignored matches require an expressly ignored category, such as dependency installation directories, generated output, or installed plugins. Audit modes and issue categories do not select path categories.

A request for untracked content alone does not select ignored paths.

## Apply Remaining Exclusions

Explicit selection overrides the corresponding Git status defaults. Apply the entrypoint’s [exhaustive scope exceptions](../SKILL.md#resolve-audit-scope). Otherwise, retain every other non-secret default exclusion unless the user names the excluded path itself or expressly selects its path category. Naming an ancestor directory alone does not lift an exclusion on a descendant.

Path selection does not waive required approvals or authorize changes.

Selecting a symbolic link does not select its target. Dereference it only when the request or applicable policy requires the target.
