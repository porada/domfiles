# Preserve Human Review Markers

## Establish the Marker’s Role

Treat a pending addition as a temporary human review marker only when applicable instructions or the user establish that its sole purpose is to keep a human-only checkpoint visible until that action is completed, and that it is not intended for the committed or submitted result. Broad commit authorization does not establish that classification.

Do not infer this role from agent origin, filenames, or warning-like wording. Being required for a check does not make generated files, permanent review or compliance records, regression tests, or release notes temporary markers. Do not exclude a marker that applicable instructions require in committed history or the submitted result. Ask only when its role or the governing requirements are unclear or conflicting.

## Select Only the Contribution

Record the exact marker addition, the evidence for its role, and the pending human action in the shared [proposal](../SKILL.md#confirm-commits). Exclude only that addition from the proposed commits, preserving substantive changes in the same file or diff hunk. If it cannot be separated safely, stop for the user’s decision.

Retain the entrypoint’s [index preservation safeguards](../SKILL.md#create-approved-commits), including for an already staged marker. Do not strip existing committed content under this pending-addition rule.

## Keep the Checkpoint Visible

Keep the marker visible and unchanged in the working tree until the required human action. Do not hide, remove, or stash it, and do not commit it merely to enable an operation. Never substitute an agent-produced acknowledgment for the human action. If required tooling or replay cannot proceed while it remains, pause that operation for the user’s action instead of bypassing the checkpoint. At the shared verification checkpoint, confirm that recorded commits exclude the marker addition and that the working-tree marker remains intact.

At any handoff or stop while a marker remains, report its location, whether it was excluded from commits, and the exact remaining human action. Do this even when no commit was made. A completed implementation or agent review does not complete the human checkpoint.
