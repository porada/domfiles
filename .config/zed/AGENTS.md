# Agent Instructions

- Always remain within the scope of the current task.
- Never override or alter the user’s input unless explicitly asked.
- Always assume that others may be working concurrently in the same project.
- Always run project-local commands through `pnpm`’s implicit executable form (`pnpm <executable> ...`), an existing `pnpm` script, or `pnpm exec` instead of invoking executables from `node_modules/.bin` directly.
    - Prefer the implicit form over `pnpm exec` when both work; for example, use `pnpm prettier` instead of `pnpm exec prettier`.
    - Exempt the external formatter command in `.zed/settings.json` from this requirement.
- Always use `git mv` when renaming tracked files.
- Always treat subagents’ short context windows as a critical constraint.
    - Keep each assignment limited to the minimum necessary scope.
    - Use additional subagents when needed to keep each assignment small.
- Always reference the applicable `AGENTS.md` line number when reporting a violation.
- Enclose all tokens and code fragments in `backticks` when quoting them in strings or comments.
- When listing non-consecutively numbered items, include each number in the item text instead of using ordered-list markers.
