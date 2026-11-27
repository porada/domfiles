# Intentional Workflow Stops

This non-exhaustive index records deliberate decisions to retain workflow stops despite their interruption cost, with rationale and any conditions for reconsideration. The linked policies and workflows own the requirements. The global [**Execution checkpoints** policy](../GLOBAL.md#execution-checkpoints) owns checkpoint placement.

## Recovery and Completion

These stops preserve required evidence, safe execution, or completion conditions. They can leave independent work available without requiring a user decision.

### Audit Evidence

The audit workflow’s [check eligibility rules](../skills/audit/references/repository-audit.md#choose-automated-checks) preserve the contents under review even when this limits validation. Optional checks can be omitted with a disclosed limitation. Required evidence can leave the affected conclusion pending, with a user decision needed only when additional authority or a material choice can resolve the gap.

### Existing Work Preservation

The [history eligibility rules](../../skills/sensible-commit-flow/references/update-commit-history.md#establish-rewrite-eligibility), [commit execution safeguards](../../skills/sensible-commit-flow/references/commit-execution.md#create-approved-commits), [personal history checks](../../skills/.dom-sensible-commit-flow/SKILL.md#corrections-and-history-updates), and global [**Concurrent work**](../GLOBAL.md#working-with-other-agents), [**History coordination**](../GLOBAL.md#gates), and [**High-impact local mutations**](../GLOBAL.md#execution-checkpoints) policies preserve existing work when eligibility, expected targets, or concurrent state changes. These stops affect the unsafe mutation. The global [**Progress and recovery** policy](../GLOBAL.md#task-conduct) permits evidence-backed correction within current authority without making every discrepancy a new approval request.

### Harmonization Dependencies

The harmonization workflow’s [**Atomicity** rule](../skills/harmonize/references/apply-harmonization.md#apply-changes) prevents partial adoption from violating a shared contract. A required source, unavailable destination, or overlapping work can therefore defer an indivisible change while independently valid changes proceed.

### Optional Peers

The [optional public peer fallback](../../skills/.domfiles-skill-development/references/optional-public-peers.md) preserves the originating workflow when remote guidance cannot be used. Invalid snapshots end remote-peer use rather than the task. This branch intentionally falls back locally instead of turning an optional enhancement into a new approval prerequisite.

### Required Capabilities and Evidence

The [review execution requirements](../../skills/.domfiles-super-review/references/review-execution.md), [reviewer continuity contract](../../skills/subagent-review-flow/SKILL.md#establish-reviewer-continuity), and [handoff delivery requirements](../../skills/agent-task-relay/references/outgoing-prompts.md#deliver-the-prompt) preserve the selected environment, assessment context, and observable result. Required capabilities cannot be silently replaced by a different review or handoff. The review workflow permits [verified equivalent interfaces](../../skills/.domfiles-super-review/references/review-execution.md#verify-an-equivalent-interface) within existing authority while retaining explicit interface selections and every required capability. The [evidence validation](../../skills/verify-findings/references/verify-findings.md) and [guidance recovery](../../skills/agent-task-relay/references/guidance-recovery.md) workflows retain unsupported conclusions or dependent work as limitations when authorized alternatives and complete fallbacks are exhausted.

### Required Review

The contribution workflow’s [review prerequisites](../../skills/sensible-contribution-flow/references/prepare-pull-requests.md#review-contributions) make the applicable assessment and clearance part of the promised result, including for small changes. A permitted, disclosed self-review remains distinct from a workflow that requires independent clearance. The [personal contribution overlay](../../skills/.dom-sensible-contribution-flow/SKILL.md#implementation-review) selects the required implementation review stages.

### Retry Limits

The global [**Retry limit** policy](../GLOBAL.md#task-conduct) limits repeated failure of one approach so repetition cannot substitute for progress. The [recovery policy](../GLOBAL.md#task-conduct) leaves room for a genuinely different, evidence-backed approach within existing authority without resetting an exhausted limit by renaming the same attempt.

### Review Recovery

The review workflow’s [dispute limits](../../skills/subagent-review-flow/SKILL.md#resolve-disagreements) and [reviewer recovery allowances](../../skills/subagent-review-flow/references/reviewer-continuity.md#recover-reviewer-continuity) prevent repeated arguments or replacement chains from manufacturing agreement. Separate allowances distinguish recoverable infrastructure failures from lost assessment context. Productive corrections and continuation in fresh sessions required by the host remain possible. A limited review preserves standing findings and evidence gaps instead of claiming clearance.

### Review Thread Settlement

The [coordinating thread settlement rules](../../skills/.domfiles-super-review/references/super-review.md#settle-the-coordinating-thread) distinguish review clearance from coordinating thread settlement. [Reviewer session cleanup](../../skills/.domfiles-super-review/references/review-execution.md#settle-reviewer-sessions) permits subsequent review work during verified transient cleanup failures when assessments are complete, evidence is retained, and execution state is safe. Keeping reported optional opportunities outstanding until the owner’s disposition requirements are satisfied prevents them from disappearing at handoff without making acceptance a prerequisite for review clearance.

## Security Boundaries

The global [security requirements](../GLOBAL.md#security) preserve protected execution and credential handling independently of task approval. The [**Security boundaries** policy](../GLOBAL.md#security) prevents a substitute tool or agent from becoming a route around denied access. Secret-bearing evidence therefore calls for sanitized material or protected user-run inspection, not broader approval to expose it.

The [public repository boundary](../../AGENTS.md#public-repository-boundary) makes an established public-safe credential route a prerequisite for authentication tools. The contribution workflow’s [private vulnerability reporting boundary](../../skills/sensible-contribution-flow/SKILL.md#assess-contribution-fit) preserves confidentiality when an authorized private channel is unavailable, and [security report guidance](../../skills/human-facing-writing/references/security-reports.md) excludes real secrets from reports. These protections remain distinct from user choices about authorization, disclosure destinations, or credential storage.

## User Checkpoints

These stops preserve a user decision, authorization, or human-only action. An existing grant can satisfy an authorization checkpoint within its scope, while a required human action remains distinct from permission to proceed. The global [**Approval provenance** policy](../GLOBAL.md#authority) preserves the user as the source of approval even when a coordinator relays it to another agent.

### Commit Offers

The commit-offer checkpoint is intentionally retained so the user decides whether to commit changes outside existing commit authority, including work without the independent review required for automatic committing. This decision is due for reconsideration when T3 Code restores continuation with the same reviewer context. The global [**Automatic commits**](../GLOBAL.md#standing-grants) and [**Commit offers**](../GLOBAL.md#gates) policies own the requirements.

### Contribution Implementation

The contribution workflow’s [execution authority checkpoint](../../skills/sensible-contribution-flow/references/propose-pull-requests.md#resolve-execution-authority) preserves the user’s choice when scope or design requires a decision or an effect needs approval. The [**Straightforward pull requests** grant](../GLOBAL.md#contribution-preparation-authorization) lets preparation proceed on the original request when its conditions hold.

### Dependency Choices

The [dependency selection checkpoint](../../skills/intentional-dependency-choice/SKILL.md#explain-your-choice) preserves the user’s control over new dependencies, tools, and changes to prescribed choices. Its stop applies before dependent work, with [prescribed acquisition and governing allowances](../GLOBAL.md#standing-grants) retaining their established coverage.

### Disclosure

The global [**External services** and **Agent communication** policies](../GLOBAL.md#gates) preserve the user’s control over recipients, processing services, and disclosed material. The [relay workflow](../../skills/agent-task-relay/references/outgoing-prompts.md#assign-work) permits a concrete assignment to be prepared within existing drafting authority while dispatch approval remains pending, so the user can review what would be sent.

### GitHub Opt-In Operations

The GitHub workflow’s [opt-in operations](../../skills/simple-github-cli/SKILL.md#opt-in-operations) deliberately reserve the owner’s listed command families and effects for a direct user request. Ordinary GitHub access does not imply consent to those additional operations.

### Harmonization Scope

The harmonization workflow owns [project selection, repository roles, and branch update approval](../skills/harmonize/references/repository-harmonization.md#resolve-repositories). This initial checkpoint preserves the user’s control over the participating checkouts, their source-only or destination roles, and the upstream updates performed before harmonization, rather than treating discovery as permission to change a checkout.

### Human Review Markers

Keeping eligible human review markers outside automatic commits avoids cleanup commits without treating the checkpoint as completed. The [marker preservation route](../../skills/sensible-commit-flow/references/preserve-human-review-markers.md) owns eligibility and preservation. Tooling that cannot proceed while the marker remains intentionally waits for the human action. Reporting pending human review under the global [**Outcomes**](../GLOBAL.md#reporting) policy does not depend on the commit workflow or marker eligibility.

### Implementation Language

The POSIX shell workflow’s [language boundary](../../skills/posix-shell-scripting/SKILL.md#keep-posix-targets-in-posix-shell) preserves the user’s architectural choice. An implementation difficulty does not itself authorize a different language or interpreter, even when the alternative runtime is already installed.

### Prepush History Plan

The [`prepush` proposal checkpoint](../../skills/.domfiles-prepush/references/prepush.md) gives the user a chance to judge grouping, messages, and the placement of pending changes before history is rewritten. It applies to every direct invocation and to routed use unless the calling workflow carries the user’s explicit waiver. Publication remains a separate decision.

### Preserved Wording

The release note workflow’s [editorial approval gate](../../skills/release-notes-for-humans/references/approval-gates.md) preserves meaning, wording, and structure the user expressly selected. Ordinary authorized editing remains available without approval of individual sentences, while a conflict with those choices requires a decision.

### Publication

The global [**Remote mutation** and **Git publication** policies](../GLOBAL.md#gates) preserve the user’s control over pushes, submissions, and other remote changes beyond local preparation. Initial contribution publication also retains a distinct approval of the finished package. The contribution workflow owns [final publication approval](../../skills/sensible-contribution-flow/references/hand-off-contributions.md#confirm-pull-request-publication), [fork selection and naming](../../skills/sensible-contribution-flow/references/hand-off-contributions.md#resolve-publication-destination), and [approved publication execution](../../skills/sensible-contribution-flow/references/hand-off-contributions.md#publish-approved-pull-requests).

The [personal contribution queue](../../skills/.dom-sensible-contribution-flow/SKILL.md#queue-publication) allows local preparation to finish while holding a new pull request at the user’s limit for that upstream.

### Release Scope

The release note workflow’s [scope clarification](../../skills/release-notes-for-humans/references/write-and-review-release-notes.md#release-scope) prevents guessed package ownership, grouping, or release boundaries from silently omitting changes or including earlier releases. The decision pauses only affected release units, leaving independently established scopes available for drafting.

### Scope Changes

The global [task authorization](../GLOBAL.md#task-authorization), [checkout boundaries](../GLOBAL.md#working-with-other-agents), and [**External skills** policy](../GLOBAL.md#gates) preserve the user’s choice when a task needs another checkout or a change to an external skill beyond its existing grants. Routine implementation decisions remain within existing authority. The [cross-shell helper contract](../skills/shell-integration/references/shared-helper-design.md#port-helpers-between-shells) likewise requires mismatches to be resolved without making every resolution a user question. Expired grants do not silently authorize another task, while [returned findings](../GLOBAL.md#standing-grants) can establish a new bounded correction request.

### Sensitive Operations

The global [**Client-mediated operations** policy](../GLOBAL.md#security) permits protected client execution while retaining explicit authorization for sensitive effects. Secret entry satisfies an input requirement, not approval of an unspecified operation. This distinction lets authorized work continue without a terminal handoff while preserving control over authentication, key management, secret updates, and elevation. Temporary delivery is transport, not persistent credential storage. The [GitHub CLI storage boundary](../PROJECT.md#github-cli-authentication-boundary) remains applicable.

### Skill Activation

The [skill activation boundary](../../skills/.domfiles-skill-development/references/domfiles-skill-policy.md#synchronize-skill-links) preserves the stable global installation while changes are prepared in worktrees. Source validation does not authorize integration or activation. An existing grant covering both permits continuation without another approval, while a task ending before integration leaves those effects for a separately authorized task.

### Thread Archival

The global [**Thread cleanup** policy](../GLOBAL.md#working-with-other-agents) distinguishes completion from archival when the host supports both. Explicit archival preserves the user’s choice about conversation organization after the work is complete.

### Workflow Refinement

The refinement workflow’s [implementation handoff](../../skills/refine-agent-workflow/references/refine-agent-workflow.md#apply-or-hand-off-improvements) keeps recommendations distinct from accepted changes. Acceptance can cover implementation and creation of its thread together, while existing authority for either remains effective.
