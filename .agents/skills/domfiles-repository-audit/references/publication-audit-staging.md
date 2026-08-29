# Publication audit staging

An isolated filesystem copy of tracked `HEAD` is not a supported publication-audit route. When an audit requires one, stop and report that limitation. Do not design or implement a materializer as part of the ordinary audit workflow.

Do not substitute `git archive`, a checkout, a linked worktree, or a before-and-after attribute check. `git archive` can omit or rewrite tracked content through `export-ignore` and `export-subst`, including attributes read from live repository metadata. A checkout or worktree can apply filters and introduce state outside the selected tree.

When isolation is unnecessary, inspect one captured commit directly through read-only Git operations. Select the repository root explicitly through the tool’s working-directory parameter or `git -C <repository-root>`, name the exact commit rather than a moving ref, and do not let the current shell subdirectory or working tree narrow or alter the evidence.

Reconsider isolated materialization only in a separate user-authorized task after a concrete audit demonstrates that direct tree inspection is insufficient.
