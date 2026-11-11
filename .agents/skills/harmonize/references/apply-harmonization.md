# Apply Harmonization

## Establish Repository Documentation

When the user requests setup or connection of a new repository, use the same confirmed repository set and baseline rather than requiring a preliminary documentation migration. Follow the [repository independence contract](../../../../skills/.domfiles-agent-documentation/references/repository-independence.md) to select the minimum local instruction surfaces and supported client bridges.

Connection means selecting the repository and its authoring baseline for this pass. Future passes rediscover its project-authored surfaces through the same eligibility rules. Do not add a repository registry, synchronization manifest, generator, or runtime dependency on domfiles merely to record that relationship. An explicit future selection can include a repository outside default discovery.

## Apply Changes

- **Coordination record:** Use a repository-and-file matrix when dependencies, partial adoption, or differing permissions make coordination difficult. Include the affected sources, destinations, proposed changes, and material boundaries. Straightforward propagation needs no separate matrix. Keep deferred destinations, unresolved choices, and intentional variants distinct from approved edits.
- **Atomicity:** Apply each family across authorized, safely writable destinations when those changes remain independently valid. Defer unavailable destinations or those with overlapping work, and report the remaining alignment gaps without requiring separate approval for that partial pass. Use a coordinated family-wide change only when partial adoption would violate an accepted cross-repository contract or leave a destination incorrect. In that case, leave the dependent family unchanged until all required destinations can be updated safely or the user authorizes a different outcome. Preserve complete local invariants and don’t overwrite overlapping work.
- **Coordination and boundaries:** For a remote destination, use the confirmed environment and checkout through the selected [**Remote access**](repository-harmonization.md#resolve-repositories) route. When required tools or a protected-path workflow need an agent at the destination, apply global **Host-managed execution** within the confirmed repository role and covered effects, or defer under **Atomicity**. When dispatching through a host interface, establish its environment, project, and exact checkout binding first. A directory instruction in a prompt doesn’t establish that host binding. Carry the selected documentation family, applicable boundaries, and required validation into the assignment. Require the destination’s instruction and working-state checks before edits and collect the resulting changes, validation evidence, and remaining limitations before reporting completion.

Complete [validation and reporting](repository-harmonization.md#validate-and-report) after the change pass.
