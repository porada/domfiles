# Publication Audit Staging

The default publication audit route inspects tracked `HEAD` directly rather than materializing an isolated filesystem copy. An explicit user direction may select another route, but it does not establish that an existing tool preserves the selected tree. Do not design or implement a materializer without authorization for that additional work.

Do not assume that `git archive`, a checkout, a linked worktree, or a before-and-after attribute check provides an exact isolated copy. `git archive` can omit or rewrite tracked content through `export-ignore` and `export-subst`, including attributes read from live repository metadata. A checkout or worktree can apply filters and introduce state outside the selected tree.

When isolation is unnecessary, inspect one captured commit directly through read-only Git operations. Select the repository root explicitly through the tool’s working directory parameter or `git -C <repository-root>`, name the exact commit rather than a moving ref, and do not let the current shell subdirectory or working tree narrow or alter the evidence.

For an explicitly requested isolated route, establish that the selected mechanism preserves the required evidence and fits the authorized effects. If no supported mechanism can do so, report that limitation and continue independent direct-tree inspection where useful. A procedural override cannot substitute for evidence of fidelity.
