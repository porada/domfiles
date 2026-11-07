# Apply Harmonization

## Establish Repository Documentation

When the user requests setup or connection of a new repository, use the same confirmed repository set and baseline rather than requiring a preliminary documentation migration. Follow the [repository independence contract](../../../../skills/.domfiles-agent-documentation/references/repository-independence.md) to select the minimum local instruction surfaces and supported client bridges.

Connection means selecting the repository and its authoring baseline for this pass. Future passes rediscover its project-authored surfaces through the same eligibility rules. Do not add a repository registry, synchronization manifest, generator, or runtime dependency on domfiles merely to record that relationship. An explicit future selection can include a repository outside default discovery.

## Apply Changes

- **Coordination record:** Use a repository-and-file matrix when dependencies, partial adoption, or differing permissions make coordination difficult. Include the affected sources, destinations, proposed changes, and material boundaries. Straightforward propagation needs no separate matrix. Keep deferred destinations, unresolved choices, and intentional variants distinct from approved edits.
- **Atomicity:** Apply each family across authorized, safely writable destinations when those changes remain independently valid. Defer unavailable destinations or those with overlapping work, and report the remaining alignment gaps without requiring separate approval for that partial pass. Use a coordinated family-wide change only when partial adoption would violate an accepted cross-repository contract or leave a destination incorrect. In that case, leave the dependent family unchanged until all required destinations can be updated safely or the user authorizes a different outcome. Preserve complete local invariants and don’t overwrite overlapping work.
- **Coordination and boundaries:** When a repository is unavailable to its required tools or protected-path workflow, relay its edit pass to an agent running there or defer its edits under **Atomicity**.

Complete [validation and reporting](repository-harmonization.md#validate-and-report) after the change pass.
