# Directory Lifecycle

## Reuse a Directory

Before reusing a directory, establish task continuity and write ownership, then verify that its `.gitignore` still ignores all contents. Preserve unrelated existing content rather than overwriting it to satisfy this convention. If reuse cannot meet these conditions, leave the directory intact and resolve the conflict or use a fresh task-owned destination.

## Move or Rename a Directory

Move or rename a directory only when the task authorizes it. Preserve its contents, including `.gitignore`, without replacing existing destination state. Keep the destination within the established placement contract, update affected task references, and coordinate with active consumers before they continue using the new path.

## Retain and Clean Up

Retain artifacts while needed for an agreed handoff, current work, expected reuse, or recovery. Helper scripts may remain when likely reuse makes retention more efficient than recreation. Treat expected reuse as continued need, not as an exception requiring automatic expiration.

By default, remove only directories created for the current task, and only after their consumers no longer need them. An explicit cleanup request may select other non-secret targets, but does not authorize unrelated deletion. Inspect the selected subtree before cleanup and preserve unexpected or unrelated state for reconciliation. Do not perform blanket namespace cleanup or infer abandonment from a directory’s name, age, or apparent inactivity. Keep `.gitignore` until the directory itself is removed, and remove newly empty task-owned subdirectories without extending cleanup to the project root.

When an artifact becomes a durable deliverable, place it in its authorized durable location rather than leaving its only copy in ignored scratch storage. Directory retention is not a backup guarantee.
