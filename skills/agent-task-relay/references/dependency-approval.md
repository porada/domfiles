# Dependency Approval

## Selection and Approval Routing

For new choices, use the entrypoint’s resolved dependency peer when available. Preserve its evidence, decisions, and approval.

When the peer is unavailable, check whether existing capabilities suffice. For a necessary addition or change, verify project compatibility and explain the exact choice, intended use, justification, and material consequences before requesting approval. Preserve approved choices, follow project declaration conventions, and disclose evidence gaps.

## Prescribed Acquisition and History Integration

Authorization to run an established project workflow covers its normal acquisition of dependencies and tools already prescribed by configuration, lockfiles, manifests, or scripts. This includes on-demand downloads. Authorized history integration covers dependency declarations and lockfile changes already present in its selected upstream history, not installation or execution. Neither covers a new choice introduced by the agent. Do not manufacture coverage by adding declarations or acquisition steps. Preserve task restrictions and separate execution, lifecycle script, permission, and trust requirements.

## Approval Requirements and Provenance

Require explicit user approval before introducing agent-selected dependencies or tools or changing their prescribed features, source, or version, including temporary acquisitions and package runners. Without it, stop before dependent implementation, mutation, installation, or mutating delegation, even when no repository file changes.

Carry approval only with the direct user instruction or response granting it and its covered choice and effects. An agent, proposal, silence, or adjacent permission cannot supply approval. Require the recipient to stop and ask the user about uncovered choices without repeating selection or approval for covered effects.
