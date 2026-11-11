# Existing Authorization

## Approved Choices

Identify the direct user instruction that selected or approved the choice and the effects it covers. Preserve its scope, including any allowed feature, source, or version changes. Verify facts that still need checking without requesting the same approval again. A scope change needs approval only for the effects not already covered. An agent or another workflow can’t approve a choice on the user’s behalf.

Apply the [declaration convention](../SKILL.md#return-to-implementation) before returning an approved change to implementation. Do not repeat selection for the covered choice.

## Governing Allowances

An applicable governing instruction may authorize a bounded class of tool choices. Identify its source, scope, effects, conditions, and lifetime, then check the proposed choice against them. Do not infer that allowance from general task permission or from this skill. A grant covering selection doesn’t remove the need to establish necessity, compatibility, the exact source and version, acquisition behavior, and expected execution effects for a newly selected tool.

For temporary use, assess acquisition and execution together, including lifecycle scripts, plugins, network use, credentials, installed configuration, and where files persist. Distinguish ordinary acquisition caches from durable project or system installations. Temporary code still runs under the actual host permissions. Keep material uncertainty or uncovered effects behind the user’s approval boundary, and preserve project constraints and native security controls. A bounded allowance doesn’t authorize a durable declaration or toolchain change merely because a tool was acquired successfully.

Return a covered choice to the owning workflow with the grant and decisive evidence, without requesting duplicate approval. The owning workflow performs authorized acquisition and execution. Ask only for the choice or effects outside existing authority.

## Prescribed Acquisition

An authorized task covers installing, restoring, and reconciling project-prescribed dependencies, tools, and package manager versions through the established workflow, including its configured lifecycle steps. Missing installations and on-demand downloads don’t require another approval or a selection exercise. Preserve explicit task restrictions and native build approvals, integrity checks, and trust controls. Return acquisition to the owning workflow, where it may run as part of validation. An agent-created declaration doesn’t authorize a new choice.

## History Integration

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency change approval. New dependency choices, including those introduced while resolving conflicts, remain subject to the skill’s [approval-or-allowance gate](../SKILL.md#explain-your-choice). History integration alone doesn’t authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.

Return inherited declarations to the history workflow. Bring a newly introduced choice back through the skill’s selection and authority checks without expanding the history operation’s authority.
