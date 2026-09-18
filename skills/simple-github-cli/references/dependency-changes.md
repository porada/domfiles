# Dependency Changes

## New Dependency Choices

Require explicit user approval before introducing an agent-selected dependency or tool, or changing its prescribed features, source, or version. This includes extensions and skills acquired through `gh` and applies even when acquisition is temporary, uses a package runner, or leaves repository files unchanged.

For a new choice, select the smallest dependency set that completely satisfies the task. Prefer an existing dependency or standard library capability when sufficient. Enable only the required features.

Before requesting approval, identify each proposed choice exactly. For each dependency, state its consumers, declaration location, installation location when relevant, and purpose. Explain why existing dependencies or standard library capabilities are insufficient and why a custom implementation would be less correct, maintainable, proportionate, or secure. Disclose any material feature, licensing, runtime, supply chain, or version implications.

Obtain the user’s explicit approval for that exact choice before carrying approval or preparing the operation. Without existing approval, stop before dependent implementation, mutation, installation, or mutating delegation. An agent cannot provide approval on the user’s behalf.

## Prescribed Acquisition

Authorization to run an established project workflow includes obtaining the dependencies and tools it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Do not request separate dependency approval solely because those packages are absent locally or downloaded on demand. An agent cannot manufacture this authorization by adding its own dependency declaration or acquisition step. Explicit task restrictions and applicable execution, lifecycle-script, permission, and trust boundaries remain in force.

Dependency authorization does not waive the entrypoint’s command-family opt-ins, handoff confirmation, remote mutation authorization, or user-run-only requirements.

## History Integration

An authorized Git history operation may incorporate dependency declarations and lockfile changes already present in its selected upstream history without separate dependency-change approval. New dependency choices, including conflict resolutions that introduce them, remain gated. History integration alone does not authorize installation or execution, but a separately authorized workflow can cover prescribed acquisition. Sandbox and security requirements remain in force.
