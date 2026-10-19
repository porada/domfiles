# Preserve Human Review Markers

## Establish Marker Roles

Treat a pending addition as a temporary human review marker only when the user or applicable instructions establish both that its sole purpose is to keep a human-only checkpoint visible until that action is completed and that it isn’t intended for the committed or submitted contribution. Broad commit authorization doesn’t establish that role.

Don’t infer it from agent origin, filenames, or warning-like wording. Generated files, permanent review or compliance records, regression tests, and release notes are not temporary markers merely because a check requires them. Don’t exclude content required in committed history or the submitted result. Ask when the role or governing requirements are unclear or conflicting.

## Select Contribution Changes

Record the exact marker addition, the evidence for its role, and the pending human action in the shared [execution baseline](prepare-commits.md#confirm-commits). Exclude only that eligible addition, preserving substantive changes in the same file or hunk. If they can’t be separated safely, pause for the user’s decision.

Retain the shared [index preservation safeguards](prepare-commits.md#create-approved-commits), including for an already staged marker. The pending addition rule doesn’t authorize stripping existing committed content.

## Keep Checkpoints Visible

Keep the marker visible and unchanged in the working tree until the required human action. Don’t hide, remove, or stash it, and don’t commit it merely to enable an operation. Never substitute an agent-produced acknowledgment for the human action. If required tooling or replay can’t proceed while it remains, pause that operation for the user’s action rather than bypassing the checkpoint.

At the shared verification checkpoint, confirm that recorded commits exclude the marker addition and that the working tree marker remains intact. At every handoff or stop while it remains, report its location, whether it was excluded from commits, and the exact remaining human action. Do this even when no commit was made. Completed implementation, commits, and agent review don’t complete the human checkpoint or establish submission readiness.
