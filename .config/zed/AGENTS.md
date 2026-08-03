# Agent instructions

## Conduct

- Always remain within the scope of the current task.
- Never override or alter the user’s input unless explicitly asked.

## Collaboration

- Always assume that others may be working concurrently in the same project.
- Ignore untracked files named `TODO` or `TODO.md` unless the user explicitly includes them in the task.
- Always treat subagents’ extremely short context windows as a critical constraint.
    - Keep each assignment limited to the minimum necessary scope.
    - Use additional subagents when needed to keep each assignment small.

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

- When adding or updating a dependency, select the newest stable release permitted by all applicable project and package manager policies, runtime and platform requirements, and dependency compatibility constraints.
    - Do not select an older release without a documented reason.
- Follow the project’s established versioning convention.
    - If none exists, use the ecosystem’s conventional declaration for accepting compatible updates.
    - Document intentional pins and other deviations from that convention.

## Documentation

- Give each durable detail one canonical home and link to it instead of paraphrasing it elsewhere.
- When these global instructions conflict with applicable project documentation—including a project’s `AGENTS.md`—follow the project documentation.
- Always reference the applicable `AGENTS.md` line number when reporting a violation.
- When reporting issues, support each one with concrete evidence relevant to the current task.
    - Do not treat speculation or alternatives based only on preference as issues.
    - Do not report issues intentionally suppressed with valid linter comments.
    - Assign each issue a unique number when it is first reported.
    - Preserve issue numbers in all subsequent reports.

## Tooling

- Always use `git mv` when renaming tracked files.
- Disable commit signing with `git -c commit.gpgsign=false commit ...` when creating commits in disposable Git repositories for tests so global signing configuration does not make the test interactive.
- Invoke commands by name through `PATH` instead of using absolute executable paths.
    - Use an absolute path only when selecting a specific installation is required, `PATH` resolution is being diagnosed, or another concrete constraint makes the location material; make the justification evident.
- Always invoke package scripts through `pnpm`’s explicit `run` subcommand, such as `pnpm run <script> ...` or `pnpm --filter <selector> run <script> ...`.
- Always invoke project-local executables without a package script through `pnpm`’s explicit `exec` subcommand (`pnpm exec <executable> ...`) instead of invoking them implicitly (`pnpm <executable> ...`) or directly from `node_modules/.bin`.
    - Exempt the external formatter command in `.zed/settings.json` from this requirement.
- Prefer the agent’s native fetch tooling when the task only requires retrieving or reading content from a known URL.
    - This preference does not apply to web searches.
    - When another dedicated native tool exists for the task, use it instead.
    - Use `curl` when command line HTTP behavior is relevant, native fetch tooling lacks a required capability, exact response bytes or files are needed, or the request must run in a shell, container, or remote environment.
    - Preserve explicit user requests, project workflows, and repository code that use `curl`.

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
