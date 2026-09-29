# Dependency Approval

## Selection and Disclosure

For a new dependency or tool choice, choose the smallest sufficient set. Prefer an existing dependency or standard library capability when sufficient, and enable only required features. Before requesting approval, identify each proposed addition or update exactly and state its consumers, declaration location, installation location when relevant, and purpose. Explain why existing dependencies or standard library capabilities are insufficient and why a custom implementation would be less correct, maintainable, proportionate, or secure. Disclose any material feature, licensing, runtime, supply chain, or version implications.

## Prescribed Acquisition

Authorization to run an established project workflow includes obtaining the dependencies and tools it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Do not request separate dependency approval solely because those packages are absent locally or downloaded on demand. An agent cannot manufacture this authorization by adding its own dependency declaration or acquisition step. Explicit task restrictions and applicable execution, lifecycle-script, permission, and trust boundaries remain in force.

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency-change approval. New dependency choices, including conflict resolutions that introduce them, still require approval. History integration alone does not authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.

## Approval Provenance

A confirmation grants approval for a new dependency choice only when it names that exact choice. Only a direct user response can grant that approval. Do not infer it from intent, silence, an agent proposal, or permission for adjacent work. An agent or subagent cannot approve on the user’s behalf. An assignment may carry dependency approval only when it identifies the explicit user response that granted it. If the receiving agent later needs an unapproved new dependency choice, require it to stop and ask the user rather than treating the assignment as authorization.
