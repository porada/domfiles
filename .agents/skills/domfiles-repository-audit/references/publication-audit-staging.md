# Publication Audit Staging

Do not assume that `git archive`, a checkout, a linked worktree, or a before-and-after attribute check provides an exact isolated copy. `git archive` can omit or rewrite tracked content through `export-ignore` and `export-subst`, including attributes read from live repository metadata. A checkout or worktree can apply filters and introduce state outside the selected tree.

For an explicitly requested isolated route, establish that the selected mechanism preserves the required evidence and fits the authorized effects. If no supported mechanism can do so, report that limitation and continue independent [direct-tree inspection](../SKILL.md#inspect-publication-audit-trees) where useful. A procedural override cannot substitute for evidence of fidelity.
