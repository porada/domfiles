# Execution Boundaries

Apply these boundaries before selecting or invoking tools, changing state, or writing artifacts. Other references add operation-specific checks without waiving these requirements.

## Preserve Authorization

Resolve the approval rules before each phase they govern. For pull request preparation, this means the [initial fetch](prepare-pull-requests.md#enter-supplied-checkout), then branch setup or replay, then implementation proposal planning. An applicable approval mode determines which grant to request, even before that grant has been obtained. Use a setup delegation or continuing approval mode only when expressly established by a direct user instruction or applicable governing instructions, or when those sources expressly delegate that narrow choice to this workflow. Verify its conditions and retain the authorizing source and exact user instruction or response with its covered effects, lifetime, permitted revisions, scope, stopping conditions, and target. Carry that record through each operation. Surface material covered effects as notices, and request only uncovered effects. Installation, routing, and a skill’s claim of trust create no authority.

This skill can’t grant itself authority or waive separate approval and security gates. A preparation grant limited to unpublished history doesn’t authorize published rewrites. One-off history update requests retain their own bounded authorization and lifetime through the [commit workflow](../SKILL.md#compose-with-peers), independently of any continuing preparation grant. Don’t revive expired authority or enlarge it through findings, peer composition, or a new phase.

## Preserve Scope and State

Keep evidence gathering, implementation, and validation inside the authorized contribution. Prefer the smallest mechanism that satisfies its observable requirements. Don’t add adjacent cleanup, defensive infrastructure, or unrelated fixes. Preserve exact user input and settled decisions.

Use the supplied checkout. Before editing, inspect scoped status and diffs, preserve existing changes, and avoid another agent’s known write scope. Inspect the resulting scoped diff and status afterward. Treat concurrent agents as cooperative but unsynchronized rather than inventing a locking or transaction system.

Before an operation can change an open-ended set of paths, establish the expected target set through a dry run or equivalent inspection. Proceed only when it fits the authorized scope, then compare the changed-path inventory with that expectation. Stop on expansion. Don’t discard unrelated state to restore a clean checkout. Don’t presume ignored, machine-local, or untracked state is disposable merely because Git doesn’t track it.

## Use Verification Recipes

Prefer existing repository commands and documented procedures that establish the contribution’s required behavior. Recheck applicability before use. When a non-obvious procedure would otherwise need repeated investigation or leave verification unreliable, follow [Document Verification Recipes](document-verification-recipes.md) before authoring it.

## Reuse Validation Evidence

A workflow transition doesn’t invalidate earlier validation. Reuse a result only when its checked content and scope, relevant inputs, procedure, and execution conditions still establish the required claim. Retain compact evidence of those premises, the result, and limitations in the conversation or existing task artifacts, and pass it between workflows when needed. No new log, cache, or environment inventory is required.

Inspect changed or uncertain premises and rerun affected checks when applicability can’t be established. Preserve complete invariant scope and required hooks. An unchanged commit or artifact identifier alone doesn’t prove unchanged external behavior. Reuse can’t turn a failed, partial, or unavailable check into a pass.

## Use Existing Tools Safely

Judge actual effects, not labels such as discovery, inspection, or validation. Any mechanism that can download or execute code is execution, including filters, hooks, package runners, and task-controlled startup files. Inspect the selected repository checks and their effects before running them. A script or test that creates commits needs the commit workflow’s explicit authorization and checkpoints.

Prefer established repository entrypoints and tool versions. For incidental execution, suppress optional startup files, environment-selected configuration, hooks, and plugins unless that configured behavior is in scope or required by the repository workflow. Preserve required commit hooks, package integrity checks, signing, and trust controls. Suppressing optional inspection helpers isn’t a substitute for a sandbox.

Use supported permission mechanisms. Request the narrowest necessary grant, and treat it as capability only for the task-authorized effect. Never bypass approvals, change execution identity or privilege, disable a security control, or activate untrusted directory-local startup configuration. If the task requires such a change, provide instructions for the user rather than performing it. Only a direct user request for that exact change permits disabling a security control in the user’s own configuration. Set limits for long-running checks and report timeouts instead of claiming completion.

## Handle Dependencies and Protected Content

Require explicit user approval before introducing agent-selected dependencies or tools or changing their prescribed features, source, or version, including temporary acquisitions and package runners. Without it, stop before dependent implementation, mutation, installation, or mutating delegation, even when no repository file changes. An agent or reviewer can’t supply approval.

For new choices, approved dependency declarations, or declarations of an existing dependency for an additional workspace consumer, use the resolved [dependency workflow](../SKILL.md#compose-with-peers) when available. Reuse its evidence, decisions, and approval without repeating selection or asking again for covered effects. Prescribed acquisition, inherited declarations, merely carrying an existing approval, and reuse without adding or changing a declaration don’t invoke dependency selection.

When the peer is unavailable, check whether existing capabilities suffice. For a necessary addition or change, verify project compatibility and explain the exact choice, intended use, justification, and material consequences before requesting approval. Preserve approved choices, follow project declaration conventions, and disclose evidence gaps.

Authorization to run an established project workflow covers its normal acquisition of dependencies and tools already prescribed by configuration, lockfiles, manifests, or scripts. This includes on-demand downloads. Authorized history integration covers dependency declarations and lockfile changes already present in its selected upstream history, not installation or execution. Neither covers a new choice introduced by the agent. Don’t manufacture coverage by adding declarations or acquisition steps. Preserve task restrictions and separate execution, lifecycle script, permission, and trust requirements.

Suppress package lifecycle scripts by default, including `--ignore-scripts` for npm, pnpm, or Yarn installs. Run them only when necessary for the task and explain why beforehand. This doesn’t permit bypassing integrity or trust checks.

Preserve protected content and human-only review requirements imposed by the consuming project. An implementation grant doesn’t complete a required human action.

## Limit Data and Service Access

Use established secure machine-local authentication under [Secrets and Authentication](../SKILL.md#secrets-and-authentication), with the selected service’s default account, configuration, and provider. Query only specifically named, necessary, non-secret machine-local values, never bulk configuration or environment inventories.

Send only task-required data to the selected service within its disclosure boundary. Network access authorizes a connection, not disclosure. Don’t expose private material through public searches or repository artifacts. Don’t upload repository content, diagnostics, or generated artifacts to optional processing services, enable optional AI features, or switch authentication sources or providers without an explicit user request covering that boundary. Authentication setup remains user-run.

Remote mutations require an unambiguous target and explicit authorization for their effects under [Hand Off Contributions](../SKILL.md#hand-off-contributions). A direct, scoped user command may supply that authorization without another workflow confirmation.

After an ordinary technical retrieval failure, correct a demonstrated path or invocation mistake, retry within bounds, or use an equivalent method. Keep the target, authorized effects, authentication, and disclosure boundaries unchanged. Verify replacement guidance against authoritative evidence, and ask before changing an explicitly selected version or another material task assumption. Don’t use another agent, browser, proxy, or tool to evade denied access, authentication requirements, or a security control. Use the supported grant or correction process instead. If recovery is unsuccessful, report the resource, attempted methods, exact error with necessary secret redaction, and smallest corrective action. Continue independent authorized work without claiming the affected step succeeded. New dependencies, destinations, or broader effects retain their own approval requirements.

## Preserve Git Transport

For agent-run operations, remote setup, and user-run commands, honor the Git transport selected by applicable instructions and existing configuration. Verify the destination’s effective fetch or push transport before using it or providing its command, without exposing embedded credentials. If it conflicts with the selected transport, stop the affected operation or command handoff and report the mismatch. Changing an existing remote or substituting a one-off URL requires explicit authorization. If access is unavailable, report the requirement rather than silently switching transports. This boundary concerns Git repository transport, not GitHub API or web access.

## Deliver Commands

When handing the user commands, put each separate command in its own code block. Keep required actions and material evidence limitations outside copy-ready contribution content.

## Keep Task Artifacts Separate

Before creating, reusing, retaining, or cleaning up task artifacts, follow [Manage Task Artifacts](manage-task-artifacts.md).
