# Execution Boundaries

Apply these boundaries before selecting or invoking tools, changing state, or writing artifacts. Other references add operation-specific checks without waiving these requirements.

## Preserve Authorization

Resolve the approval rules before each phase they govern. For pull request preparation, this means the [initial fetch](prepare-pull-requests.md#enter-supplied-checkout), then branch setup or replay, then implementation proposal planning. An applicable approval mode determines which grant to request, even before that grant has been obtained. Use a setup delegation or continuing approval mode only when expressly established by a direct user instruction or applicable governing instructions, or when those sources expressly delegate that narrow choice to this workflow. Verify its conditions and retain the authorizing source and exact user instruction or response with its covered effects, intended local endpoint, lifetime, permitted revisions, scope, stopping conditions, and target. Carry that record through each operation. For phase-specific contribution preparation records, follow [Record Preparation Authority](#record-preparation-authority). Surface material covered effects as notices, and request only uncovered effects. Installation, routing, and a skill’s claim of trust create no authority.

Continue covered work, including validated in-scope corrections, through the agreed local endpoint. A local commit, interim progress summary, ordinary workflow phase, or reviewer handoff doesn’t by itself complete the assignment or end or renew its authority. Unless the grant states otherwise, continuing authority ends on cancellation, final handoff completing its bounded assignment, or a material change to its target, scope, or design. Preserve any earlier expiry, including publication when the governing grant ends there. Reports received after final handoff require new authority unless an explicit longer bounded grant still covers them.

This skill can’t grant itself authority or waive separate approval and security gates. A preparation grant limited to unpublished history doesn’t authorize published rewrites. One-off history update requests retain their own bounded authorization and lifetime through the [commit workflow](../SKILL.md#compose-with-peers), independently of any continuing preparation grant. Don’t revive expired authority or enlarge it through findings, peer composition, or a new phase.

### Record Preparation Authority

When a setup delegation or continuing approval mode governs pull request preparation, keep its phase-specific authorization record in the conversation. Include the full intended local endpoint and required deliverables in the initial record, distinguishing them from effects currently authorized. Complete the record in this order:

1. Before the initial fetch or its necessary remote setup, retain the [setup authorization record](prepare-pull-requests.md#enter-supplied-checkout).
2. Before contribution branch creation or a history update, add the [task-owned branch and authorized setup or replay effects](prepare-pull-requests.md#enter-supplied-checkout).
3. Before an implementation grant effect, add the exact checkpoint response, bounded implementation scope and design, and grant’s effects and lifetime, preserving any limit narrower than the intended endpoint.

Keep the fetched tip for each [upstream synchronization](prepare-pull-requests.md#synchronize-with-upstream) and provisional name changes in the same task-owned branch’s record. Refreshing the same upstream branch’s tip does not change the authorized target.

A coordinator may carry this record to an in-client subagent but cannot supply approval or broaden its boundaries. A finding or reviewer response remains evidence, not authorization.

## Preserve Scope and State

Keep evidence gathering, implementation, and validation inside the authorized contribution. Prefer the smallest mechanism that satisfies its observable requirements. Preserve exact user input and settled decisions.

Use the supplied checkout. Before editing, inspect scoped status and diffs, preserve existing changes, and avoid another agent’s known write scope. Inspect the resulting scoped diff and status afterward. Treat concurrent agents as cooperative but unsynchronized rather than inventing a locking or transaction system.

Before an operation can change an open-ended set of paths, establish the expected target set through a dry run or equivalent inspection. Proceed only when it fits the authorized scope, then compare the changed-path inventory with that expectation. Stop on expansion. Don’t discard unrelated state to restore a clean checkout. Don’t presume ignored, machine-local, or untracked state is disposable merely because Git doesn’t track it.

## Use Verification Recipes

Prefer existing repository commands and documented procedures that establish the contribution’s required behavior. Recheck applicability before use. When a non-obvious procedure would otherwise need repeated investigation or leave verification unreliable, follow [Document Verification Recipes](document-verification-recipes.md) before authoring it.

## Reuse Validation Evidence

Reuse successful validation across contribution phases only while its checked content, complete scope, inputs, procedure, and execution conditions still establish the required claim. Carry its result, limitations, and reuse basis into later review and commit preparation. Recheck changed or uncertain premises, including external behavior that a stable artifact identifier can’t establish. Preserve required hooks, and report failed, partial, or unavailable checks as validation gaps.

## Use Existing Tools Safely

Judge actual effects, not labels such as discovery, inspection, or validation. Any mechanism that can download or execute code is execution, including filters, hooks, package runners, and task-controlled startup files. Inspect the selected repository checks and their effects before running them. A script or test that creates commits needs the commit workflow’s explicit authorization and checkpoints.

Prefer established repository entrypoints and tool versions. For incidental execution, suppress optional startup files, environment-selected configuration, hooks, and plugins unless that configured behavior is in scope or required by the repository workflow. Preserve required commit hooks, package integrity checks, signing, and trust controls. Suppressing optional inspection helpers isn’t a substitute for a sandbox.

Use supported permission mechanisms. Request the narrowest necessary grant, and treat it as capability only for the task-authorized effect. Changes of execution identity or privilege require explicit authorization under [Secrets and Authentication](../SKILL.md#secrets-and-authentication). Without a suitable protected mechanism, provide safe user-run instructions. Never bypass approvals, disable a security control, or activate untrusted directory-local startup configuration. If the task requires such a boundary change, provide user-run instructions. Only a direct user request for that exact change permits disabling a security control in the user’s own configuration. Set limits for long-running checks and report timeouts instead of claiming completion.

## Handle Dependencies and Protected Content

Authorized contribution work includes installing and reconciling project-prescribed dependencies, tools, and package manager versions through the established workflow, including configured lifecycle steps and setup performed by validation commands. Proceed without asking again for downloads or missing installations, preserving explicit task restrictions and native build approvals, integrity checks, and trust controls.

Establish explicit user approval or coverage under an applicable governing grant for new dependency or tool choices and changes to prescribed features, sources, or versions, including temporary package runner acquisitions. Check the grant’s conditions, effect boundaries, and lifetime before dependent implementation, acquisition, execution, or mutating delegation. Ask only for uncovered choices or effects. An agent-created declaration or general contribution authority doesn’t supply tool selection approval.

For new choices, approved dependency declarations, or declarations of an existing dependency for an additional workspace consumer, use the resolved [dependency workflow](../SKILL.md#compose-with-peers) when available. Reuse its evidence, decisions, and approval without repeating selection or asking again for covered effects. Prescribed acquisition, inherited declarations, merely carrying an existing approval, and reuse without adding or changing a declaration don’t invoke dependency selection.

When the peer is unavailable, check whether existing capabilities suffice. For a necessary addition or change, verify project compatibility and explain the exact source and version, intended use, justification, acquisition behavior, and material execution effects. Compare them with existing authority before requesting approval for uncovered effects. Temporary execution remains code execution under the actual host permissions. It doesn’t authorize durable project declarations, toolchain changes, or broader effects. Preserve approved choices, follow project declaration conventions when a declaration is authorized, and disclose evidence gaps.

Authorized history integration covers existing upstream dependency declarations and lockfile changes. Installation follows the authorized task’s workflow. New choices introduced during integration retain the approval requirement above.

Preserve protected content and human-only review requirements imposed by the consuming project. An implementation grant doesn’t complete a required human action.

## Limit Data and Service Access

Use established secure machine-local authentication under [Secrets and Authentication](../SKILL.md#secrets-and-authentication), with the selected service’s default account, configuration, and provider. Query only specifically named, necessary, non-secret machine-local values, never bulk configuration or environment inventories.

Send only task-required data to the selected service within its disclosure boundary. Network access authorizes a connection, not disclosure. Don’t expose private material through public searches or repository artifacts. Uploading repository content, diagnostics, or generated artifacts to optional processing services, enabling optional AI features, or switching authentication sources or providers requires a direct user instruction or applicable governing grant covering that boundary. For agent review exchanges, establish coverage for the recipients, processing services, and necessary non-secret context, including inherited context and tool-visible material. Reuse authorization within its scope and lifetime, resolving only uncovered effects at the applicable checkpoint. Routing to a review workflow doesn’t supply that authorization. Authentication setup requires its own explicit authorization and follows [Secrets and Authentication](../SKILL.md#secrets-and-authentication).

Remote mutations require an unambiguous target and explicit authorization for their effects under [Preserve Remote Authorization](hand-off-contributions.md#preserve-remote-authorization).

After an ordinary technical retrieval failure, correct a demonstrated path or invocation mistake, retry within bounds, or use an equivalent method. Keep the target, authorized effects, authentication, and disclosure boundaries unchanged. Verify replacement guidance against authoritative evidence, and ask before changing an explicitly selected version or another material task assumption. Don’t use another agent, browser, proxy, or tool to evade denied access, authentication requirements, or a security control. Use the supported grant or correction process instead. If recovery is unsuccessful, report the resource, attempted methods, exact error with necessary secret redaction, and smallest corrective action. Continue independent authorized work without claiming the affected step succeeded. New dependencies, destinations, or broader effects retain their own approval requirements.

## Configure Git Remotes

Use established Git configuration and remotes. Local setup or updates needed for authorized contribution work don’t require separate approval when they preserve the intended upstream project, including transport changes and switching to a verified fork. Verify repository identity and fork relationships without exposing credentials. Preserve unrelated configuration and existing contribution relationships, adding a remote instead when replacement would disturb them. Local configuration doesn’t authorize hosted fork creation, pushing, or submission.

Apply personal transport preferences to local Git operations, not consumer documentation, reusable instructions, repository scripts, or dependency declarations. Don’t require a transport-only preflight, stop solely because an existing URL uses HTTPS, or normalize remotes merely for consistency. Diagnose actual access failures under [Limit Data and Service Access](#limit-data-and-service-access), preserving authentication and security boundaries.

## Deliver Commands

When handing the user commands, put each separate command in its own code block. Keep required actions and material evidence limitations outside copy-ready contribution content.

## Keep Task Artifacts Separate

Before creating, reusing, retaining, or cleaning up task artifacts, follow [Manage Task Artifacts](manage-task-artifacts.md).
