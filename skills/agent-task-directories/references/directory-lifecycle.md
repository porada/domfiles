# Directory Lifecycle

Directory reuse and relocation rules govern task roots. Moving or renaming individual artifacts or subdirectories within owned task storage does not create new task roots. Cleanup, durable promotion, and retention apply to the selected artifacts or directory within the task’s authority.

## Reuse Directories

Before reusing a directory, establish task continuity and write ownership, then verify that its `.gitignore` still ignores all contents. Don’t overwrite unrelated existing content to restore the ignore rule. If you can’t meet these conditions for reuse, leave the directory intact and resolve the conflict or use a fresh task-owned destination.

## Move or Rename Directories

Move or rename a directory only when the task authorizes it. Preserve its contents, including `.gitignore`, without replacing existing destination state. Follow the usual placement rules except for [temporary relocation during project checks](#handle-interference-with-project-checks). Update affected task references and coordinate with active consumers before they continue using the new path.

## Handle Interference with Project Checks

Before changing an interfering path, establish who can act on it. Within your assigned scope, remove disposable artifacts once their consumers have finished. For another task’s files, have its owner handle them or agree with the owner or coordinator on the files, who will restore them, and when consumers can resume. Pause consumers before disturbing paths they use.

You may archive files at a released path if the user identifies that path as abandoned, its owning task releases it for cleanup, or an applicable project rule designates it as reclaimable. Leave paths outside your write scope intact until the owner or coordinator agrees or release is confirmed. If the required command still cannot run and other authorized options are exhausted, identify the command and path and ask the user to decide.

Prefer packing only the interfering files in a tar archive inside the ignored task directory that holds those files. Keep that directory’s `.gitignore` in place. Check that every step of the normal invocation skips the archive. Verify that it can restore the selected files before removing their originals. If the directory itself interferes, an authorized temporary move to an unused path outside all relevant scan roots can clear it when packing cannot.

Run the normal project command. Even if it fails, return every relocated directory and restore packed files at their original paths before consumers resume. Files at a released path may stay packed when that resolves the interference. Preserve intervening changes during restoration and keep the archive or relocated copy until restoration succeeds. Report any retained archive or copy by path. If interference remains, identify the required command and the path it discovers without bypassing the check.

## Retain and Clean Up

Retain artifacts while they’re needed for an agreed handoff, current work, recovery, or expected reuse that makes keeping them more efficient than creating them again. Continued need has no automatic expiration.

By default, remove only artifacts and directories created for the current task, and only after their consumers no longer need them. An explicit cleanup request may select other non-secret targets, but doesn’t authorize unrelated deletion. Inspect the selected subtree before cleanup and preserve unexpected or unrelated state for reconciliation. Don’t perform blanket namespace cleanup or infer abandonment from a directory’s name, age, apparent inactivity, a closed thread, or a missing ownership marker. Remove newly empty task-owned subdirectories without extending cleanup to the project root.

When an artifact becomes a durable deliverable, place it in its authorized durable location rather than leaving its only copy in ignored scratch storage. Directory retention is not a backup guarantee.
