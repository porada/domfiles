# Script Artifact Boundaries

## Bound Artifact Locations

Every filesystem write must remain within one of these authorized categories:

- A declared repository artifact at its canonical path, whether intentionally tracked or stored in an established repository-owned output location.
- A short-lived sibling used exclusively to atomically replace an otherwise authorized destination.
- An explicit file or directory supplied by the caller.

Do not repurpose a location merely because it is ignored. Never modify `.gitignore` while running the script, and do not add an ignore rule solely to accommodate script-specific output. If repository-wide toolchain output exposes a missing ignore policy, handle that as a separate repository configuration change under the current task’s authorization.

For ephemeral artifacts, have the caller establish storage through `agent-task-directories` before invoking the script. Accept the resolved task-specific destination instead of establishing a separate temporary output convention.

Before writing:

- Apply the global **Concurrent work** preservation rule to repository destinations.
- Confirm that the resolved destination remains within the authorized location. Reject traversal or symlink redirection outside it.
- Do not write files unrelated to the declared artifact contract. Keep host and target repository Git metadata read-only. Limit Git metadata writes to [declared disposable fixture setup](skill-owned-scripts.md#test-contracts).
- Leave byte-identical output unchanged.
- Replace an existing path only when it is a declared generated artifact or the current request explicitly authorizes overwriting it.

## Write Artifacts Safely

- Do not provide a generic `--force` or `--fix` escape hatch that bypasses ownership or destination checks.
- Remove stale paths only when they belong to a declared script-owned output set and exact synchronization is part of the documented contract.
- Report every artifact created, updated, unchanged, or removed.
- When atomic replacement requires a same-filesystem temporary file, use a short-lived sibling as an internal implementation detail and remove it after success or a handled failure.
