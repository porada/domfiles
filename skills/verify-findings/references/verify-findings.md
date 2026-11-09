# Verify Findings

## Resolve Scope

Verify the findings already established in the conversation or explicitly supplied by the user. Honor a selected subset and preserve each finding’s identifier. If you can’t establish the finding set or target, ask for that context rather than reconstructing it. Don’t expand verification into a new audit.

Keep this workflow read-only unless the user expressly requests additional effects under [Agent Conduct](../SKILL.md#agent-conduct). Without that direction, don’t apply fixes, change configuration, create report files, install dependencies, stage or commit changes, or submit results to an external service. Verification itself doesn’t authorize another effect. Route separately authorized work through its owning workflow.

## Verify Evidence

Re-read applicable instructions and the previously reported files for the selected findings. Reuse content only when evidence establishes that it hasn’t changed since the current task read it. Check each finding against the current target, governing requirements, and settled user decisions, not merely the original reviewer’s conclusion.

When conclusions conflict, compare their target revisions, governing requirements, settled decisions, and material assumptions. When a finding conflicts with an established decision, assess whether the decision still applies and whether materially new evidence challenges it. Intent alone isn’t a rebuttal, and a prior decision doesn’t excuse a real defect. As the agent verifying the findings, you own their disposition. Weigh evidence, not reviewer counts, and use the smallest decisive inspection or check within the selected scope, authority, and existing limits. Don’t add reviewers merely to obtain agreement or classify preference-only alternatives as required fixes. Ask the user only for a material choice that the evidence and current authority can’t settle, or a separately required approval. Retain the decisive reason with the finding’s classification, and follow **Report Results** below when deciding which findings to include in the response.

Verify claims that depend on evidence outside the inspected files separately, including ignored files, runtime state, and upstream behavior. Unchanged files don’t establish unchanged external behavior. Inspect only relevant evidence, excluding known secret-bearing sources. Don’t reopen a finding previously classified as requiring no change unless relevant content changed or materially new evidence is available.

Use the consuming environment’s established tools. In the read-only workflow, run only checks whose effects are known to be non-mutating. Inspection and validation can execute code. Disable unrelated executable startup configuration, hooks, and plugins unless their behavior is in scope or required by an established project workflow. Don’t bypass required controls. If a check requires an ungranted permission or unauthorized state change, leave the affected claim unverified and identify what’s needed rather than running it.

Use established access under [Secrets and Authentication](../SKILL.md#secrets-and-authentication) for task-required external evidence. Don’t select other credentials or services, upload local evidence, or invoke optional remote processing. When required evidence is unavailable, continue only the independent checks that remain possible.

For ordinary technical failures, correct a demonstrated path or invocation mistake, make bounded retries, or use an equivalent method while the target, authorized effects, authentication, and disclosure boundaries remain unchanged. Don’t evade denied access, authentication requirements, or a security control through tool substitution or delegation. Use the supported grant or correction process instead. Ask before changing an explicitly selected source or version or another material task assumption. New dependencies, destinations, or broader effects retain their own approval requirements. Never infer unavailable evidence.

## Classify Findings

Assign each selected finding one outcome based on the evidence:

| Outcome | Meaning |
| --- | --- |
| Intentional | Applicable instructions or a settled user decision establish that no change is needed, and current evidence identifies no defect requiring correction in the reported behavior. |
| Not supported | Current evidence contradicts the reported issue. |
| Requires change | Current evidence establishes an issue that still requires a change. |
| Resolved | Current evidence establishes that the reported issue has been addressed. |
| Unverified | Required evidence is unavailable or insufficient to determine whether a change is needed. |

A proposed fix or changed file alone doesn’t establish resolution. Don’t classify uncertainty as either confirmation or dismissal.

## Report Results

Report findings classified as **Requires change** or **Unverified**, retaining their identifiers. For a retrieval failure, identify the resource, attempted methods, and exact error without exposing sensitive values.

Make each reported finding understandable without re-reading the earlier report. Pair its identifier with a descriptive title naming the concrete issue. Briefly explain the affected behavior and consequence, the current evidence or verification limit, and the smallest correction, check, or decision still needed. Include earlier decisions only when they materially explain the current state.

Default to one compact paragraph per finding, normally two to four sentences. Use the decisive evidence rather than replaying the investigation, logs, or rejected alternatives. Link supporting detail where useful, and present an actionable command separately when needed. Expand only when a material distinction would otherwise be lost or the user requests more detail.

Don’t repeat findings classified as **Intentional**, **Not supported**, or **Resolved**. When no findings require a change or remain unverified, state the resulting status directly. Preserve mandatory reporting requirements from applicable instructions, including any pending human-only review step. Stop after reporting unless the user has separately authorized continuation through another workflow.
