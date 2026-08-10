# Agent instructions

## Conduct

- Always remain within the scope of the current task.
- Never override or alter the user’s input unless explicitly asked.
- When user input appears inconsistent with the current task, accidentally pasted from another context, or mistyped, proceed only when the intended request can be inferred confidently from the conversation and project evidence without changing its material scope or outcome. Otherwise stop before acting and ask one focused clarification instead of following the input literally or silently choosing among plausible interpretations.

## Collaboration

- Always assume that others may be working concurrently in the same project.
- Ignore untracked files named `TODO` or `TODO.*`. Do not read or modify them unless the user explicitly requests work on them.
- Use subagents as context-isolation boundaries for evidence gathering whose source count or output size cannot be bounded safely before execution.
    - Delegate exploratory online research before the first potentially unbounded search, fetch, or open-ended navigation. Treat searches across GitHub issues, pull requests, discussions, release histories, and similar collections as exploratory.
    - For local investigation, first narrow the scope and use native pagination or output limits. Delegate broad command output, large Git ranges, or independent repository audit scopes when those controls are insufficient and the main thread needs only a concise result.
    - Treat subagents’ extremely short context windows as a critical constraint. Give each one a narrow question, explicit source constraints and stop conditions, and a concise output contract. Default to the five strongest findings with URLs or project-relative evidence, and split independent questions across subagents instead of broadening one assignment.
    - Keep final synthesis and decisions in the main thread. Save substantial supporting evidence in the [task-specific temporary directory](#temporary-files) and read only targeted portions back into the main thread.
    - If a subagent fails or exhausts its context, continue in the original thread and start a new, narrower subagent rather than repeating the unbounded investigation in the main thread.
- When online research depends on authenticated or otherwise nontransferable browser state, keep it in the main thread but define a narrow source count and stopping point before browsing.
- Do not delegate known bounded retrieval, targeted diagnostics or file reads, reliably bounded commands, or the `Verify` shorthand solely for context isolation. Do not delegate mutating work merely because its validation may be verbose.

## Temporary files

- Place temporary files managed directly by the agent under one task-specific `.agent-<name>` directory at the relevant project root instead of scattering them across the project, unless applicable project instructions require another approved temporary namespace.
    - Use a unique, filesystem-safe `<name>` that identifies the task. Add a short suffix when needed to avoid collisions.
- Treat `.agent-<name>` as the shared naming convention for temporary task directories and Git worktrees, not as a worktree-only path.
    - Before reusing, moving, or deleting an existing `.agent-<name>` directory, inspect it and use `git --no-pager worktree list --porcelain` to determine whether it is registered. Add `-z` only when a parser consumes the output.
- Helper scripts may remain in their task-specific `.agent-<name>` directory across sessions when likely reuse makes retaining them more efficient than recreating them. Treat that expected reuse as continued need under the cleanup rule.
- Remove only temporary directories created for the current task when they are no longer needed.

## Git worktrees

- Work in the current checkout by default. Before editing, inspect the status and diff of files in scope, preserve existing changes, and avoid overlapping another agent’s known write scope.
- Create a dedicated Git worktree only when at least one of these conditions applies:
    - The user explicitly requests an isolated worktree.
    - Another active agent has an overlapping write scope.
    - The task requires isolated branch, dependency, build, or test state.
    - The change is broad or high-risk enough to benefit materially from independent rollback, and there is a clear integration plan.
- Do not create a worktree solely because the repository is dirty, concurrent activity is possible, or the task modifies repository files. Keep follow-up edits to the same uncommitted task in its existing checkout.
- When a worktree is required, use a unique, filesystem-safe `<name>` containing a task slug and short unique suffix. Do not include path separators. Create the pair with `git worktree add -b agent/<name> .agent-<name> <start-point>`. Do not use `--detach`. Every agent worktree must retain its paired branch. When changing `<name>`, move the worktree and rename its branch together so the pair remains `.agent-<name>` and `agent/<name>`.
- Before moving or removing a worktree, or force-renaming or deleting its branch, inspect the affected worktree status and verify that its changes are integrated or explicitly abandoned.
    - Remove worktrees with `git worktree remove`. Use one or two exact `-f` or `--force` options only after the preceding verification when an unclean or locked worktree requires them. Afterward, verify that the corresponding `.agent-<name>` directory no longer exists. If it remains, inspect it rather than deleting it recursively.
    - After removing the worktree, first delete its branch with `git branch -d agent/<name>`. If Git refuses because the branch is unmerged, use `git branch -D agent/<name>` only when the preceding verification established that its changes are integrated or explicitly abandoned.
- For historical analysis and reviews, inspect revisions through Git without materializing them. Materialize a revision only when a filesystem-based tool must operate on it. When isolation is required, follow the worktree policy above.

## Communication

- Make every response immediately actionable.
    - Begin final responses with the result or the smallest useful next action.
    - Put a requested command, path, or snippet before its supporting explanation.
    - When user action remains, end with one small, concrete action they can take immediately.
- Structure work with multiple steps so its state remains visible.
    - Use the shortest complete numbered sequence, with one bounded action per item.
    - When a plan tool is available, keep one item in progress and use the plan to preserve state instead of repeating it in prose.
    - Across turns, state what finished, what is current, and what comes next.
- Protect focus.
    - Resolve incidental questions without involving the user when possible and incorporate the answers into the current work.
    - Defer unrelated observations until the current task is complete, then mention them separately and briefly.
    - After three consecutive failed attempts, stop repeating the approach, identify the assumption that may be wrong, and ask one focused diagnostic question.
- Make progress and failures explicit.
    - State outcomes as positive conditions rather than as the absence of a negative condition. Treat negating a negative-state term such as `blocked`, `failed`, `incomplete`, `missing`, or `unresolved` as a double negative, and name the resulting state directly.
    - State what now works. Include verification evidence only when it materially establishes the result.
    - Describe errors plainly with the evidence, known cause, and next corrective action.
    - When a time estimate would help the user plan their own work, give a concrete range and state its assumptions.
- Keep routine validation silent.
    - Do not report passing diagnostics, formatting, linting, typechecking, or whitespace checks individually.
    - Treat each applicable unmentioned check as run and passed. Never use silence for a check that was skipped, unavailable, incomplete, or not run.
    - Report failures, warnings, and validation limitations directly.
    - Mention a successful check only when it materially demonstrates the requested behavior or the user asks for validation details.
    - Do not report routine preservation of unrelated or protected files unless a conflict affected the task or the user asks about them.
- Keep output easy to scan without removing needed substance.
    - Keep lists to five items or fewer. Split longer lists into immediate and later or optional groups.
    - Avoid generic preambles, redundant recaps, closing pleasantries, figurative language, and hedging that adds no information.
    - Give full explanations when requested. Safety, real ambiguity, task requirements, and higher priority instructions override brevity.

## Dependencies

- Add `--ignore-scripts` to `npm install`, `pnpm install`, and `yarn install` by default so package lifecycle scripts do not run.
    - Run an install without `--ignore-scripts` only when package lifecycle scripts are necessary for the current task, and state the reason before running it.
- When adding or updating a dependency, select the newest stable release permitted by all applicable project and package manager policies, runtime and platform requirements, and dependency compatibility constraints.
    - Do not select an older release without a documented reason.
- Follow the project’s established versioning convention.
    - If none exists, use the ecosystem’s conventional declaration for accepting compatible updates.
    - Document intentional pins and other deviations from that convention.

## Documentation

- Never edit a consumer-facing `README` file without the user’s explicit permission.
- As an editorial rule for Markdown, keep standalone block elements such as tables and fenced code blocks aligned to the document’s left edge. Restructure surrounding lists or other blocks to reference them rather than nesting them.
- Give each durable detail one canonical home and link to it instead of paraphrasing it elsewhere.
- When these global instructions conflict with applicable project agent instructions, follow the project agent instructions.
- Always reference the applicable `AGENTS.md` line number when reporting a violation.
- When reporting issues, support each one with concrete evidence relevant to the current task.
    - Do not treat speculation or alternatives based only on preference as issues.
    - Do not report issues intentionally suppressed with valid linter comments.
    - Assign each issue a unique number when it is first reported.
    - Preserve issue numbers in all subsequent reports.

## Tooling

- Always use `git mv` when renaming tracked files.
- Disable commit signing with `git -c commit.gpgsign=false commit …` when creating commits in disposable Git repositories for tests so global signing configuration does not make the test interactive.
- Invoke commands by name through `PATH` instead of using absolute executable paths.
    - Use an absolute path only when selecting a specific installation is required, `PATH` resolution is being diagnosed, or another concrete constraint makes the location material. Make the justification evident.

### System-available tooling

Assume the following non-standard development commands are system-installed and available through `PATH`:

| Command | Purpose | Guidance |
| --- | --- | --- |
| `actionlint` | GitHub Actions correctness | — |
| `cargo` | Rust package management and workspace workflows | — |
| `fd` | Filesystem path search | — |
| `fish` | Fish shell and configuration checks | — |
| `jq` | JSON querying and transformation | — |
| `node` | JavaScript and TypeScript execution | Run `*.ts` files directly |
| `plugins` | Agent plugin installation | Invoke directly—not through `npx` or `pnpm dlx` |
| `pnpm` | JavaScript package management | Preferred. Use `run` for scripts, `exec` for local binaries, and `dlx` for undeclared one-offs |
| `rg` | File-content search | — |
| `rustc` | Direct Rust compilation | Use for standalone source files |
| `shellcheck` | Shell-script analysis | — |
| `skills` | Agent skill management | Invoke directly—not through `npx` or `pnpm dlx` |
| `taplo` | TOML formatting and validation | — |
| `yarn` | JavaScript package management | Use for Yarn-based projects |
| `zizmor` | GitHub Actions security reviews | — |

Command guidance applies to agent invocations and command examples, not repository scripts, workflows, or configuration.

### Low-friction tool use

- Prefer the most specific native tool that directly represents the operation.
    - Use native file, search, diagnostics, fetch, and browser tools instead of shell commands when they can complete the task.
    - Use the terminal for repository workflows and capabilities unavailable through a dedicated tool.
- Access ordinary local files and directly addressable URLs without MCP indirection.
    - Use native file tools for ordinary local files and native fetch tooling for known URLs. This preference does not apply to web searches.
    - Use `cat` or `curl` when exact bytes, delivery through standard input, a necessary shell pipeline, command-line HTTP behavior, exact response files, or execution in a shell, container, or remote environment is material to the task. Prefer passing a file path directly when the downstream command supports it. Preserve explicit user requests, project workflows, and repository code that use `curl`.
    - Do not use MCP servers as generic filesystem or HTTP proxies. Do not open the same file or URL through Chrome MCP, browser automation, a subagent, or another indirect proxy merely because direct access failed.
    - If direct access fails because of a tool, permission, sandbox, network, or unexplained transport error, stop retrieving that resource and report the resource, attempted method, exact error, and smallest corrective action. Correct ordinary path or URL mistakes through direct discovery. Do not submit an external tooling issue unless the user explicitly requests it.
    - Use MCP or a browser when the task depends on server-owned semantics, remote-only state, rendered DOM state, user interaction, browser-managed downloads, nontransferable authentication, or an explicit user or project workflow. Select that route for the required capability, not as a retrieval fallback.
- Prefer repository-owned workflows over ad hoc execution.
    - Use existing package scripts, repository entrypoints, and configured tooling instead of direct interpreter invocations, reconstructed command pipelines, or one-off package runners.
- Keep terminal commands direct and canonical.
    - Run one logical operation per tool call and set its working directory through the tool when supported.
    - Use canonical casing and documented option forms, and prefer literal project-relative operands.
    - When terminal copying is necessary, default to `cp -n --`. Do not treat an existing destination as updated, and leave link-creating, link-preserving, overwriting, or recursive forms confirmable.
    - Avoid shell or interpreter wrappers, ad hoc environment assignments, command substitution, expansion, redirection, and pipelines unless they are necessary to the operation.
- Prefer inspection before materialization or mutation.
    - During investigation, use list, inspect, check, dry-run, and no-execute modes where they answer the question.
    - Do not attach execution, deletion, force, or output-writing behavior to an inspection utility unless the task requires that behavior.
- Treat permission prompts as security boundaries rather than command-syntax problems.
    - Do not inspect or reverse-engineer editor permission settings merely to choose command syntax unless the task explicitly concerns those settings.
    - If an inspection-only operation prompts unexpectedly, try the applicable native tool or canonical repository workflow once.
    - If a necessary operation still requires confirmation, request it once with a concise reason. Do not obscure or repeatedly reformulate it solely to avoid confirmation.

## Writing

- Write every JSDoc comment as a multiline block with `/**` and `*/` on separate lines, including one-sentence comments.
- Write suppression directives as `/* … */` block comments when both the language and relevant tool accept that form, including `/* oxlint-disable-next-line rule/name */`, `/* prettier-ignore */`, and `/* @ts-expect-error */`. Use the tool-required syntax otherwise. Do not add explanatory text unless applicable repository or linter policy requires it.
- In prose, avoid semicolons, use typographic “quotation marks” and apostrophes, and write em dashes without surrounding spaces. Preserve literal punctuation where syntax requires it.
- In prose, use the ellipsis character `…` for placeholders and ordinary ellipses. Preserve language-required spread and rest syntax, exact literals, and quoted source text.
- For nonconsecutive numbered items, write each number explicitly in the item text instead of relying on Markdown’s ordered list numbering.
- Wrap identifiers, paths, commands, and quoted code tokens in backticks.

## Shorthand commands

- Shorthand commands are task macros that define complete, standalone procedures.
- Always execute shorthand commands exactly as defined.

### Verify

- Reread every applicable `AGENTS.md` file and previously reported file, then align each finding with the latest instructions and contents.
- Reclassify each previously reported finding as resolved, intentional, or unresolved.
- Report only findings classified as unresolved when present. When every finding is resolved or intentional, state the resulting status directly.
