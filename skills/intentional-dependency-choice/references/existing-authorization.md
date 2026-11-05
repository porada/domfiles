# Existing Authorization

## Approved Choices

Identify the direct user instruction that selected or approved the choice and the effects it covers. Preserve its scope, including any allowed feature, source, or version changes. Verify facts that still need checking without requesting the same approval again. A scope change needs approval only for the effects not already covered. An agent or another workflow can’t approve a choice on the user’s behalf.

Apply the [declaration convention](../SKILL.md#return-to-implementation) before returning an approved change to implementation. Do not repeat selection for the covered choice.

## Prescribed Acquisition

An authorized task covers installing, restoring, and reconciling project-prescribed dependencies, tools, and package manager versions through the established workflow, including its configured lifecycle steps. Missing installations and on-demand downloads don’t require another approval or a selection exercise. Preserve explicit task restrictions and native build approvals, integrity checks, and trust controls. Return acquisition to the owning workflow, where it may run as part of validation. An agent-created declaration doesn’t authorize a new choice.

## History Integration

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency change approval. New dependency choices still require approval, including those introduced while resolving conflicts. History integration alone doesn’t authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.

Return inherited declarations to the history workflow. Bring a newly introduced choice back through the skill’s selection and approval steps without expanding the history operation’s authority.
