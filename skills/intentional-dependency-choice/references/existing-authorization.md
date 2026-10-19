# Existing Authorization

## Approved Choices

Identify the direct user instruction that selected or approved the choice and the effects it covers. Preserve its scope, including any allowed feature, source, or version changes. Verify facts that still need checking without requesting the same approval again. A scope change needs approval only for the effects not already covered. An agent or another workflow can’t approve a choice on the user’s behalf.

Apply the [declaration convention](../SKILL.md#return-to-implementation) before returning an approved change to implementation. Do not repeat selection for the covered choice.

## Prescribed Acquisition

Authorization to run an established project workflow includes obtaining the dependencies and tools it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Don’t request separate dependency approval solely because those dependencies and tools are absent locally or downloaded on demand. An agent can’t create this authorization by adding its own declaration or acquisition step. Explicit task restrictions and applicable execution, lifecycle script, permission, and trust boundaries remain in force.

When those conditions are satisfied and no new choice is needed, return to the owning workflow without a selection exercise.

## History Integration

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency change approval. New dependency choices still require approval, including those introduced while resolving conflicts. History integration alone doesn’t authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.

Return inherited declarations to the history workflow. Bring a newly introduced choice back through the skill’s selection and approval steps without expanding the history operation’s authority.
