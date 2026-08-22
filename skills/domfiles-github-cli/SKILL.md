---
name: github-cli
description: Use GitHub CLI for bounded discovery, retrieval, and explicitly authorized GitHub operations. Use whenever a task calls for `gh`, GitHub CLI, `gh search`, `gh api`, or reading or changing GitHub state through the CLI. Covers interface selection, read bounds, API method classification, unsupported command families, and failure handling. Do not use for local Git state, directly addressable public URLs, rendered or interactive browser state, agent skills, or agent plugins.
metadata:
    internal: true
---

# GitHub CLI

## Select the interface

- Use `gh` only for bounded GitHub state and operations.
- Use native fetch for directly addressable public URLs, native browser tools for rendered or interactive state, `git` and the applicable worktree policy for local repository state, `skills` for agent skills, and `plugins` for agent plugins.
- For GitHub repository and code discovery, prefer a dedicated native GitHub search tool, then bounded `gh search repos` or `gh search code`, rather than `curl`, browser scraping, or hand-written API requests.

## Read bounded state

- Select the repository and object explicitly when context is ambiguous. Request only needed JSON fields, apply concrete limits, and avoid account-wide inventories, unbounded pagination, log following, and bulk output.
- Treat `gh api` as a raw API boundary. Use explicit `GET` for REST reads with fields. Allow GraphQL only for bounded `query` operations under read-only authority, and classify every GraphQL `mutation` or other method by its actual effect.

## Reject unsupported operations

Do not use `gh agent-task` or its aliases. `gh` availability does not authorize `gh copilot`, `gh skill`, `gh extension`, `gh alias`, persistent `gh config` changes, Codespaces, authentication or key management, secrets or variables, or local Git mutation.

## Stop at capability boundaries

If `gh` lacks authentication, scope, network access, or a required capability, report the exact boundary and stop. Do not use aliases or extensions to approximate unavailable behavior. Route an explicitly requested exception through its established owner and approval boundary.
