# Execution Boundaries

Apply these boundaries before selecting or invoking tools, changing state, or writing artifacts. Other references add operation-specific checks without waiving these requirements.

## Preserve Scope and State

Keep evidence gathering, implementation, and validation inside the authorized contribution. Prefer the smallest mechanism that satisfies its observable requirements. Do not add adjacent cleanup, defensive infrastructure, or unrelated fixes. Preserve exact user input and settled decisions.

Use the supplied checkout. Worktree creation, relocation, and removal remain user/application-owned unless separately requested. Before editing, inspect scoped status and diffs, preserve existing changes, and avoid another agent’s known write scope. Inspect the resulting scoped diff and status afterward. Treat concurrent agents as cooperative but unsynchronized rather than inventing a locking or transaction system.

Before an operation can change an open-ended set of paths, establish the expected target set through a dry run or equivalent inspection. Proceed only when it fits the authorized scope, then compare the changed-path inventory with that expectation. Stop on expansion. Do not discard unrelated state to restore a clean checkout. Do not presume ignored, machine-local, or untracked state is disposable merely because Git does not track it.

## Use Existing Tools Safely

Classify actual effects rather than labels such as discovery, inspection, or validation. Any mechanism that can download or execute code is execution, including filters, hooks, package runners, and task-controlled startup files. Inspect the selected repository checks and their effects before running them. A script or test that creates commits needs the commit workflow’s explicit authorization and checkpoints.

Prefer established repository entrypoints and tool versions. For incidental execution, suppress optional startup files, environment-selected configuration, hooks, and plugins unless that configured behavior is in scope or required by the repository workflow. Preserve required commit hooks, package integrity checks, signing, and trust controls. Suppressing optional inspection helpers is not a substitute for a sandbox.

Use supported permission mechanisms. Request the narrowest necessary grant, and treat it as capability only for the task-authorized effect. Never bypass approvals, change execution identity or privilege, disable a security control, or activate untrusted directory-local startup configuration. If the task requires such a change, provide instructions for the user rather than performing it. Bound long-running checks and report timeouts instead of claiming completion.

## Handle Dependencies and Protected Content

Require explicit user approval before introducing an agent-selected dependency or tool, or changing its prescribed features, source, or version. This applies even when acquisition is temporary, uses a package runner, or leaves repository files unchanged. Before requesting approval, identify the exact smallest sufficient set and required features. Prefer existing dependencies or standard library capabilities. Explain each dependency’s consumers, declaration and installation locations, purpose, and why existing capabilities or a custom implementation are insufficient. Disclose material licensing, runtime, supply-chain, and version implications. Without approval, stop before dependent implementation, mutation, installation, or mutating delegation. A reviewer or agent cannot supply that approval.

Authorization to run an established project workflow includes obtaining the dependencies and tools it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Do not request separate dependency approval solely because those packages are absent locally or downloaded on demand. An agent cannot manufacture this authorization by adding its own dependency declaration or acquisition step. Explicit task restrictions and applicable execution, lifecycle-script, permission, and trust boundaries remain in force.

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency-change approval. New dependency choices, including conflict resolutions that introduce them, still require approval. History integration alone does not authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.

For an approved dependency change, follow repository version conventions. When selecting a new dependency or tool version, choose the newest stable release compatible with the project’s declared constraints, and explain an intentionally older release or pin. Suppress package lifecycle scripts by default, including `--ignore-scripts` for npm, pnpm, or Yarn installs. Run them only when necessary for the task and explain why beforehand. This does not permit bypassing integrity or trust checks.

Obtain explicit permission before editing a consumer-facing README. Do not modify externally authored skills. Preserve any additional protected-content or human-only review requirements imposed by the consuming project. An implementation grant does not complete a required human action.

## Limit Data and Service Access

Use established secure machine-local authentication only through ordinary non-disclosing operations and the selected service’s default account, configuration, and provider. Do not inspect credentials or authentication identity. Query only specifically named, necessary, non-secret machine-local values, never bulk configuration or environment inventories.

Send only task-required data to the selected service within its disclosure boundary. Network access authorizes a connection, not disclosure. Do not upload repository content, diagnostics, or generated artifacts to optional processing services, enable optional AI features, or switch authentication sources or providers without an explicit user request covering that boundary. Authentication setup remains user-run.

Remote mutations need their own explicit authorization and an unambiguous target. This workflow still leaves Git publication and every contribution submission to the user, even when a tool could perform them.

Correct an ordinary path or URL mistake or a demonstrated local invocation error, such as invalid arguments or malformed reader syntax, then retry only the selected method within the original scope. This exception does not cover access, authentication, network, permission, sandbox, or unexplained tool or transport failures. For those failures, stop retrieving the resource. Report the resource, method, exact error with any necessary secret redaction, and smallest corrective action. Do not retry through another agent, browser, proxy, or tool.

## Keep Task Artifacts Separate

Create scratch storage only when needed. Follow an applicable project storage policy. Otherwise create a fresh `.agent-<task>` directory under the project root, establish task ownership, and put a single `*` line in its `.gitignore` before other writes. Reuse only after verifying task continuity and that all contents remain ignored. A matching name alone establishes no ownership, and ignore status establishes no privacy.

Preserve other tasks’ contents, keep writes inside the owned directory, and do not activate copied instructions or configuration as governing guidance. Bound validation inputs so scratch copies do not enter unrelated scans. Retain artifacts needed for handoff or expected reuse. Remove only owned disposable content after its consumers no longer need it, keeping the ignore file until the directory is removed. Do not sweep up other task directories or leave durable project deliverables solely in scratch storage.

When handing the user commands, put each separate command in its own code block. Keep required actions and material evidence limitations outside copy-ready contribution content.
