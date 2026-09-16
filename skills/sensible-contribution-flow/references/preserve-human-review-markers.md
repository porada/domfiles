# Preserve Human Review Markers

## Establish the Marker’s Role

Treat a pending addition as a temporary human review marker only when applicable instructions or the user establish that its sole purpose is to keep a human-only checkpoint visible until that action is completed, and that it is not intended for the committed or submitted contribution. Broad commit authorization does not establish that role.

Do not infer it from agent origin, filenames, or warning-like wording. Generated files, permanent review or compliance records, regression tests, and release notes are not temporary markers merely because a check requires them. Do not exclude content required in committed history or the submitted result. Ask when the role or governing requirements are unclear or conflicting.

## Select Only the Contribution

Record the exact marker addition, the evidence for its role, and the pending human action in the shared [commit proposal](prepare-commits.md#confirm-commits). Exclude only that eligible addition, preserving substantive changes in the same file or hunk. If they cannot be separated safely, pause for the user’s decision.

Retain the shared [index preservation safeguards](prepare-commits.md#create-approved-commits), including for an already staged marker. The pending-addition rule does not authorize stripping existing committed content.

## Keep the Checkpoint Visible

Keep the marker visible and unchanged in the working tree until the required human action. Do not hide, remove, or stash it, and do not commit it merely to enable an operation. Never substitute an agent-produced acknowledgment for the human action. If required tooling or replay cannot proceed while it remains, pause that operation for the user’s action rather than bypassing the checkpoint.

At the shared verification checkpoint, confirm that recorded commits exclude the marker addition and that the working-tree marker remains intact. At every handoff or stop while it remains, report its location, whether it was excluded from commits, and the exact remaining human action. Do this even when no commit was made. Completed implementation, commits, and agent review do not complete the human checkpoint or establish submission readiness.
