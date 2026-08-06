# Agent instructions

## Conduct

- Always remain within the scope of the current task.
- Never override or alter the user’s input unless explicitly asked.

## Collaboration

- Always assume that others may be working concurrently in the same project.
- Ignore untracked files named `TODO` or `TODO.*` unless the user explicitly includes them in the task.
- Always treat subagents’ extremely short context windows as a critical constraint.
    - Keep each assignment limited to the minimum necessary scope.
    - Use additional subagents when needed to keep each assignment small.

## Temporary files

- Place every temporary file created for a task under one task-specific `.agent-<name>` directory at the relevant project root instead of scattering temporary files across the project.
    - Use a unique, filesystem-safe `<name>` that identifies the task; add a short suffix when needed to avoid collisions.
- Treat `.agent-<name>` as the shared naming convention for temporary task directories and Git worktrees, not as a worktree-only path.
    - Before reusing, moving, or deleting an existing `.agent-<name>` directory, inspect it and determine whether it is a registered worktree.
- Remove only temporary directories created for the current task when they are no longer needed.

## Git worktrees

- Work in the current checkout by default. Before editing, inspect the status and diff of files in scope, preserve existing changes, and avoid overlapping another agent’s known write scope.
- Create a dedicated Git worktree only when at least one of these conditions applies:
    - The user explicitly requests an isolated worktree.
    - Another active agent has an overlapping write scope.
    - The task requires isolated branch, dependency, build, or test state.
    - The change is broad or high-risk enough to benefit materially from independent rollback, and there is a clear integration plan.
- Do not create a worktree solely because the repository is dirty, concurrent activity is possible, or the task modifies repository files. Keep follow-up edits to the same uncommitted task in its existing checkout.
- When a worktree is required, use a unique, filesystem-safe `<name>` containing a task slug and short unique suffix. Do not include path separators. Create the pair with `git worktree add -b agent/<name> .agent-<name> <start-point>`. Do not use `--detach`; every agent worktree must retain its paired branch. When changing `<name>`, move the worktree and rename its branch together so the pair remains `.agent-<name>` and `agent/<name>`.
- Before moving or removing a worktree, or force-renaming or deleting its branch, inspect the affected worktree status and verify that its changes are integrated or explicitly abandoned.
    - Remove worktrees with `git worktree remove`; use one or two exact `-f` or `--force` options only after the preceding verification when an unclean or locked worktree requires them. Afterward, verify that the corresponding `.agent-<name>` directory no longer exists. If it remains, inspect it rather than deleting it recursively.
    - After removing the worktree, first delete its branch with `git branch -d agent/<name>`. If Git refuses because the branch is unmerged, use `git branch -D agent/<name>` only when the preceding verification established that its changes are integrated or explicitly abandoned.
- For historical analysis and reviews, inspect revisions through Git without materializing them. Materialize a revision only when a filesystem-based tool must operate on it; when isolation is required, follow the worktree policy above.

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
    - State what now works and how it was verified.
    - Describe errors plainly with the evidence, known cause, and next corrective action.
    - When a time estimate would help the user plan their own work, give a concrete range and state its assumptions.
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
- Give each durable detail one canonical home and link to it instead of paraphrasing it elsewhere.
- When these global instructions conflict with applicable project documentation—including a project’s `AGENTS.md`—follow the project documentation.
- Always reference the applicable `AGENTS.md` line number when reporting a violation.
- When reporting issues, support each one with concrete evidence relevant to the current task.
    - Do not treat speculation or alternatives based only on preference as issues.
    - Do not report issues intentionally suppressed with valid linter comments.
    - Assign each issue a unique number when it is first reported.
    - Preserve issue numbers in all subsequent reports.

## Tooling

- Always assume that `*.ts` files can be run directly with the system-installed `node`.
- Always use `git mv` when renaming tracked files.
- Disable commit signing with `git -c commit.gpgsign=false commit …` when creating commits in disposable Git repositories for tests so global signing configuration does not make the test interactive.
- Invoke commands by name through `PATH` instead of using absolute executable paths.
    - Use an absolute path only when selecting a specific installation is required, `PATH` resolution is being diagnosed, or another concrete constraint makes the location material; make the justification evident.
- Use the system-installed `zizmor` when reviewing GitHub Actions workflows.
- Apply the following pnpm rules to commands executed by agents, including command examples that instruct agents; do not treat them as repository code-style requirements for scripts, workflows, or configuration:
    - Invoke package scripts through `pnpm`’s explicit `run` subcommand, such as `pnpm run <script> …` or `pnpm --filter <selector> run <script> …`.
    - Invoke project-local executables without a package script through `pnpm`’s explicit `exec` subcommand (`pnpm exec <executable> …`) instead of invoking them implicitly (`pnpm <executable> …`) or directly from `node_modules/.bin`.
        - Exempt the external formatter command in `.zed/settings.json` from this requirement.
    - Invoke one-off package executables that are not declared by the current project through `pnpm dlx <package> …` instead of `npx`.
        - When instructions use `npx skills …`, `pnpm dlx skills …`, `npx plugins …`, or `pnpm dlx plugins …`, invoke the corresponding system-provided `skills …` or `plugins …` command directly.
- Prefer the agent’s native fetch tooling when the task only requires retrieving or reading content from a known URL.
    - This preference does not apply to web searches.
    - Use `curl` when command line HTTP behavior is relevant, native fetch tooling lacks a required capability, exact response bytes or files are needed, or the request must run in a shell, container, or remote environment.
    - Preserve explicit user requests, project workflows, and repository code that use `curl`.

### Low-friction tool use

- Prefer the most specific native tool that directly represents the operation.
    - Use native file, search, diagnostics, fetch, and browser tools instead of shell commands when they can complete the task.
    - Use the terminal for repository workflows and capabilities unavailable through a dedicated tool.
- Prefer repository-owned workflows over ad hoc execution.
    - Use existing package scripts, repository entrypoints, and configured tooling instead of direct interpreter invocations, reconstructed command pipelines, or one-off package runners.
- Keep terminal commands direct and canonical.
    - Run one logical operation per tool call and set its working directory through the tool when supported.
    - Use canonical casing and documented option forms, and prefer literal project-relative operands.
    - Avoid shell or interpreter wrappers, ad hoc environment assignments, command substitution, expansion, redirection, and pipelines unless they are necessary to the operation.
- Prefer inspection before materialization or mutation.
    - During investigation, use list, inspect, check, dry-run, and no-execute modes where they answer the question.
    - Do not attach execution, deletion, force, or output-writing behavior to an inspection utility unless the task requires that behavior.
- Treat permission prompts as security boundaries rather than command-syntax problems.
    - Do not inspect or reverse-engineer editor permission settings merely to choose command syntax unless the task explicitly concerns those settings.
    - If an inspection-only operation prompts unexpectedly, try the applicable native tool or canonical repository workflow once.
    - If a necessary operation still requires confirmation, request it once with a concise reason; do not obscure or repeatedly reformulate it solely to avoid confirmation.

## Writing

- Use typographic punctuation in prose: curly quotation marks, curly apostrophes, and em dashes without surrounding spaces.
- Reserve straight quotation marks, straight apostrophes, and hyphens for code, identifiers, URLs, commands, and other syntax read by machines.
- For nonconsecutive numbered items, write each number explicitly in the item text instead of relying on Markdown’s ordered list numbering.
- Wrap quoted tokens and code fragments in backticks. Follow the format below as an example.

```ts
/**
 * Tests for the `Icon` component
 */
describe('`Icon` component with a custom `ASSET_PATH`', () => {
    process.env.ASSET_PATH = '/assets';

    test('accepts `true` as the `name` prop', () => {
        // …
    });

    test('returns `undefined` if the `name` prop isn’t provided', () => {
        // …
    });
});
```

## Shorthand commands

- Shorthand commands are task macros that define complete, standalone procedures.
- Always execute shorthand commands exactly as defined.

### Verify

- Read all applicable `AGENTS.md` files and all reported files again to confirm whether reported issues remain relevant.
    - Ensure that all findings align with the latest applicable `AGENTS.md` files.
- Classify each previously reported finding as resolved, intentional, or unresolved.
    - Exclude resolved findings from future reports.
    - Exclude intentional findings from future reports unless the relevant code or `AGENTS.md` changes.
- Report only unresolved findings that still apply.
