# Remote Changes

Require explicit user authorization and an unambiguous target before any operation whose actual effects can close, comment on, create, delete, dispatch, edit, fork, merge, publish, reconfigure, review, or synchronize GitHub resources or a remote repository. Treat `gh repo sync <destination-repository>` as a remote mutation of the named destination. Treat the no-argument form as a local Git mutation under [Opt-In Operations](../SKILL.md#opt-in-operations). The `--force` form hard-resets the selected destination branch. Any remote history replacement, including synchronization, must satisfy the publication safeguards below.

Publish local Git commits, tags, or refs only under direct, scoped user authorization covering the destination and effects. Preparation, a local commit request, or permission for local history rewrites alone does not authorize publication. Without publication authorization, provide the exact command for the user to run. For published history replacement, verify the destination and current remote head, preserve the work already present there, and use an explicit `--force-with-lease=<remote-ref>:<expected-remote-oid>`. A lease alone does not establish that the prepared result preserves remote work. If the selected interface cannot enforce the expected-head lease, stop before replacement and report the need for a supported route. Preserve required hooks, signing, and other security controls.

Inspect existing state first when a read-only operation can establish what already exists or prevent a duplicate change.

Use noninteractive flags. Pass substantial bodies through a task-local temporary file rather than a command literal.
