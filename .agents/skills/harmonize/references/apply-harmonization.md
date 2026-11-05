# Apply Harmonization

## Establish Repository Documentation

When the user requests setup or connection of a new repository, use the same confirmed repository set and baseline rather than requiring a preliminary documentation migration. Follow the [repository independence contract](../../../../skills/.domfiles-agent-documentation/references/repository-independence.md) to select the minimum local instruction surfaces and supported client bridges.

Connection means selecting the repository and its authoring baseline for this pass. Future passes rediscover its project-authored surfaces through the same eligibility rules. Do not add a repository registry, synchronization manifest, generator, or runtime dependency on domfiles merely to record that relationship. An explicit future selection can include a repository outside default discovery.

## Apply Changes

- **Edit matrix:** Before mutation, record one repository-and-file matrix complete for the destinations being changed and the sources or related destinations needed for their validity. Identify each requirement’s canonical source, local destination, proposed change or creation, and any permission or access requirement. List deferred destinations with their reason and pending scope without inventing uninspected file details. Keep unresolved choices and intentional variants distinct from approved edits.
- **Atomicity:** Apply each family across authorized, safely writable destinations when those changes remain independently valid. Defer unavailable destinations or those with overlapping work, and report the remaining alignment gaps without requiring separate approval for that partial pass. Use a coordinated family-wide change only when partial adoption would violate an accepted cross-repository contract or leave a destination incorrect. In that case, leave the dependent family unchanged until all required destinations can be updated safely or the user authorizes a different outcome. Preserve complete local invariants and don’t overwrite overlapping work.
- **Coordination and boundaries:** When a repository is unavailable to its required tools or protected-path workflow, relay its edit pass to an agent running there or defer its edits under **Atomicity**.

Complete [validation and reporting](repository-harmonization.md#validate-and-report) after the change pass.
