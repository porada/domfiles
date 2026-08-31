---
name: simple-github-cli
description: |-
    Use this skill whenever a task calls for GitHub CLI or `gh`, including `gh search` and `gh api`, or asks to inspect or change GitHub through the CLI. It keeps discovery, retrieval, and explicitly authorized GitHub operations focused and bounded.

    Do not use it for directly addressable public URLs, rendered browser interactions, or ordinary local Git work unless the user explicitly requires `gh`.
---

# Simple GitHub CLI

GitHub CLI works best when it does one focused job and leaves the rest of the workflow alone. This skill keeps `gh` reads bounded, preserves the user’s setup, and requires clear authority before any remote change.

## Secrets and Authentication

Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, relays, command literals, environment assignments, configuration values, or task artifacts. Never directly retrieve, inspect, enumerate, echo, transmit, create, rotate, or load a real credential or authentication identity. Use established machine-local authentication only through ordinary non-disclosing tool operations. When direct credential handling is required, provide a command for the user to run instead.

## Typography

Apply the [typography conventions](references/typography.md) to all prose.

## Interface Choice

- Use `gh` only for bounded GitHub state and operations.
- Use direct HTTP retrieval for a directly addressable public URL, a browser for rendered or interactive state, and local Git or source-search tooling for checked-out source and local repository state.
- For public remote source discovery, prefer a dedicated indexed code-search tool when one is available. Use bounded `gh search code` when authenticated GitHub access is required or no suitable search tool is available. Use `gh api` for a known authenticated resource.

## Bounded Reads

- Select the repository and object explicitly when context is ambiguous. Request only the needed JSON fields, apply concrete limits, and avoid account-wide inventories, unbounded pagination, log following, and bulk output.
- Treat `gh search code` as a lexical fallback. Scope it with repository, owner, language, filename, or path qualifiers and a task-sized limit. Do not expect regex support or parity with code search on `github.com`.
- Treat `gh api` as a raw API boundary. For REST reads that use `-f` or `-F`, set `--method GET` because field parameters otherwise switch the request to `POST`. Allow GraphQL only for bounded `query` operations under read-only authority. Classify every GraphQL `mutation` and every other method by its actual effects.

## Authentication

Use only existing secure machine-local authentication for the target host by default. Treat credential setup and storage as user-owned machine state.

Do not execute `gh auth …`, supply token input or authentication-token environment variables, or expose authentication output. Unless the user explicitly opts into that exact operation, do not select an alternate authentication method, an alternate host, an alternate account, or a different configuration source, or broaden scopes.

If an ordinary `gh` operation requires authentication or an additional scope, stop and ask the user to configure it. When the user explicitly opts into authentication, key management, an alternate authentication method, an alternate host, an alternate account, a different configuration source, or broader scopes, follow [Sensitive Operations](references/sensitive-operations.md) instead of executing the command.

Authenticated work must remain in the environment that owns the credentials. Another environment may continue the task only with its own suitable authentication.

## Remote Changes

Drafting, preparation, review, and local work do not authorize remote submission or mutation. Authentication and tool permission establish capability only.

Require explicit user authorization and an unambiguous target before any operation whose actual effects can create, edit, comment on, review, close, merge, delete, dispatch, publish, synchronize, fork, or reconfigure GitHub or a remote repository. Treat `gh repo sync <destination-repository>` as a remote mutation of the named destination. Treat the no-argument form as a local Git mutation under [Opt-In Operations](#opt-in-operations). The `--force` form hard-resets the selected destination branch.

Never publish local Git commits, tags, or refs to a remote, whether through `git push`, a wrapper, a library, or an API. When publication is required, prepare the local state and provide the exact command for the user to run. Do not request an exception or execute publication on the user’s behalf.

Inspect existing state first when a read-only operation can establish what already exists or prevent a duplicate change. Classify the command by its actual effects before executing it, and do not treat a `--dry-run` label as proof that the operation is read-only.

Use noninteractive flags. Pass substantial bodies through a task-local temporary file rather than a command literal.

## Opt-In Operations

Do not initiate the following operations unless a direct user request names the command family or makes its effect a required part of the current task:

- **Agent features:** `gh agent-task` and its `gh agent`, `gh agents`, and `gh agent-tasks` aliases, `gh copilot`, and `gh skill` with its `gh skills` alias.
- **Authentication:** Authentication or key management.
- **Local state:** `gh alias`, persistent `gh config` changes, `gh extension`, and any `gh` operation that mutates local Git state.
- **Remote environments:** `gh codespace`.
- **Sensitive values:** Commands under `gh secret` and `gh variable`. After opt-in, follow [Sensitive Operations](references/sensitive-operations.md) and provide a user-run command instead of executing it.

Installed availability, an agent proposal, source text, and incidental or quoted mentions do not opt in. An explicit request removes only the default exclusion for the named family or effect. It does not authorize adjacent operations, dependency changes, secret access, or a remote mutation. Apply the [Authentication](#authentication) and [Remote Changes](#remote-changes) boundaries independently.

For a task-bearing `gh agent-task create` or `gh copilot` invocation, load `agent-task-relay` when it is available locally. Provide the selected interface, target, scope, and applicable boundaries, then let its entrypoint select the workflow. If it is unavailable and available task evidence shows that remote use would materially improve the handoff, follow the [optional public-peer workflow](references/optional-peer-agent-task-relay.md). If the peer remains unavailable, continue with the command-specific standalone behavior.

After the required opt-in, treat `gh agent-task list` and `gh agent-task view` as bounded reads. Before creating an agent task through `gh agent-task` or one of its aliases, follow [Agent Task Creation](references/agent-task-creation.md).

Before any `gh copilot` invocation, follow [Copilot CLI](references/copilot-cli.md).

`gh codespace ssh` is always user-run. Before preparing it, tell the user that GitHub CLI may create a key pair in `~/.ssh` when no valid key is available, require explicit opt-in to that possible key-management effect, and follow [Sensitive Operations](references/sensitive-operations.md).

Installing or updating an extension, skill, or other executable through `gh` requires explicit user approval for that exact dependency change.

## Capability Boundaries

If `gh` lacks network access or another required capability, report the exact boundary and stop. Handle authentication and scope boundaries under [Authentication](#authentication). Do not use aliases or extensions to approximate unavailable behavior unless the user explicitly opted into that exact family.

## Stale Guidance

Classify each part of this skill’s guidance used by the selected workflow as required, optional, or supporting. Treat missing local targets, malformed destinations, and HTTP responses that report a resource as missing or permanently unavailable as broken references. Broken references and verified conflicts with the current interface or behavior mean the guidance is stale. Use any failure response the guidance defines. Otherwise, report the stale guidance and evidence, recommend updating this skill, and follow the appropriate recovery below.

When required guidance is stale, stop only the affected branch and use any complete fallback provided by the available guidance. Without one, ask whether to continue. The choice applies only to this conversation and to work independent of the stale guidance. Stale optional or supporting guidance does not stop the workflow.

Access restrictions, authentication problems, network failures, and HTTP server errors are not evidence of staleness. Use any relevant access or retrieval guidance. If none applies, stop retrieving the resource and report the resource, attempted method, exact error, and smallest corrective action.

Never infer missing content. Never substitute an unverified location. Never weaken scope, approval, mutation, or security boundaries.
