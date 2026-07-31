# Project documentation

This document records durable architecture, tooling decisions, compatibility constraints, and maintenance procedures that are not obvious from the source alone. `AGENTS.md` remains authoritative for agent instructions; use this file for project rationale and operational context.

Organize future entries under broad second-level sections so this document can grow without becoming a flat list of unrelated notes.

## Compatibility

### Node.js engine range

The root `engines.node` range intentionally declares the minimum supported Node.js major without mirroring narrower patch-level constraints from individual tools. pnpm and the invoked tools are expected to report when the installed Node.js release does not satisfy a tool’s more specific engine range.

### Supported environment

`domfiles` actively targets multiple Apple Silicon–based Macs. Bootstrap and synchronization must work on a fresh installation of macOS 26 or newer with Command Line Tools and Homebrew already installed and available through `PATH`.

The canonical Apple Silicon location fallback for `brew` is only a convenience for invoking Homebrew itself. It does not relax the `PATH` prerequisite for commands installed through Homebrew.

`fish` is the default interactive shell on every managed machine. Shell behavior and setup logic must not assume that Bash or Zsh is the user’s default shell.

## Security

### Zed agent permission model

Treat Zed’s agent sandbox, tool defaults, command allowances, and confirmation overrides as separate security boundaries.

Agent tool permissions intentionally use an allow-by-default baseline. The terminal tool overrides that baseline with confirm-by-default behavior, using explicit allowances for accepted forms and confirmation overrides for hazardous forms.

## Tooling

### Codex distribution

Keep `codex` installed through Homebrew rather than declaring `@openai/codex` as a project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Fish abbreviation ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations; abbreviations defined outside domfiles are not preserved across shell startup.

### Fish formatter plugin

Keep `prettier-plugin-fish` a thin whole-file wrapper around `fish_indent`. Preserve `fish_indent` output verbatim and let it own Fish formatting semantics; Prettier options such as `tabWidth` and `useTabs` intentionally do not affect Fish output.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. `fish_indent` has no range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged.

### Peer dependency versions

Declare every peer dependency in workspace packages with the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. Do not flag `"*"` peer ranges as missing compatibility constraints or narrow them solely to mirror the currently resolved version.

### `pnpm` executable invocation

`pnpm <executable> ...` is an accepted form for project-local binaries that do not have a package script. `pnpm` intentionally falls back to the workspace binary when no built-in command or script has that name, keeping invocations concise while resolving through workspace dependencies. Forms such as `pnpm vp ...` and `pnpm prettier ...` are intentional and must not be flagged as direct `node_modules/.bin` execution; `pnpm exec <executable> ...` is also valid but is not required.

### Repository-scoped commands

Keep `plugins` and `skills` in the root `dependencies`. They provide user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

`domfiles-sync-update` intentionally invokes `plugins update`. The `plugins` help does not advertise `update` as a named subcommand, but the invocation succeeds in the supported environment; do not infer failure from the help output alone.

The corresponding scripts in `bin/` are the stable command interfaces. Their implementations are resolved from the domfiles pnpm workspace so `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Do not install parallel copies through global pnpm state.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization of a dirty checkout, `domfiles-sync-update` overrides this behavior with `error` so commands can run only when dependencies are already current. Revalidate these assumptions when changing the pinned pnpm major version or overriding `verifyDepsBeforeRun`.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### Synchronization completion

`domfiles sync` prioritizes completing as much independent synchronization work as possible with minimal interruption. Recoverable issues must be reported, but fixing them is not a prerequisite for running or completing unrelated sync stages.

Synchronization scripts otherwise fail fast. An unhandled error or a nonzero exit from a sync stage stops the broader workflow; the best-effort policy does not suppress script failures.

Repository fetch, rebase, stashing, and stash-restoration failures are recoverable. They are reported without aborting the broader synchronization workflow, which continues against the available checkout so the remaining setup, installation, update, and cleanup stages can still run.

The final dependency status is advisory. Its failures remain visible but do not invalidate that the broader workflow reached completion.

`.lastsync` records that the broader workflow reached completion. It is intentionally write-only for now and reserved for a possible future feature.

### Zed personal instructions

The tracked `.config/zed/AGENTS.md` maps to Zed’s global personal instruction file at `~/.config/zed/AGENTS.md`. Zed’s native Agent loads it for every project; it is not a project-scoped instruction file, and external agents and terminal threads do not generally consume it.

Keep repository-specific guidance in the root `AGENTS.md` or applicable project skills instead.
