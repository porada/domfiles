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

### Zed terminal permission limitations

Zed applies `terminal.always_confirm` ahead of `terminal.always_allow`, and its Rust-compatible regular expressions do not support lookarounds. A narrower allowance therefore cannot exempt a command that a broader confirmation rule already matches. Keep trusted local package-manager `exec` binaries inside each manager’s positive command-family allowance, using an `exec`-specific option grammar, and keep ordinary npm, pnpm, and Yarn workflows in positive command alternatives. Unlisted executable names then fall through the terminal’s confirm-by-default boundary; broad `exec` confirmation overrides would make the allowlist require brittle complement expressions. Exact informational forms can be allowed separately.

Terminal patterns cover wrapper ordering and option forms verified in the supported environment rather than every shell-equivalent permutation or speculative abbreviation. Command casing variants intentionally fall through the terminal’s confirm-by-default boundary. Recheck version-dependent option abbreviations when a package-manager major changes instead of widening a pattern speculatively.

Git inspection allowances intentionally accept arbitrary uppercase `GIT_*` assignments before `git`, while state-changing Git allowances retain their narrower prefix set. This avoids maintaining a variable inventory, but Git environment variables can redirect repositories and indexes, select helpers or pagers, and write trace output. Treat the broad inspection prefix as an intentional convenience boundary, and reassess environment-selected behavior when extending a Git command family.

### Zed xargs permission mirroring

The `xargs` terminal allowance intentionally mirrors the executable alternatives in the consolidated general terminal allowance so agents can batch the same baseline commands. Update both lists together and keep `xargs`’s own options limited to bounded, noninteractive argument splitting and batching controls.

Zed authorizes the `xargs` shell segment before standard input becomes child-command arguments, so it cannot apply the child command’s normal confirmation overrides to injected options. Require confirmation for the complete nested command family whenever standard input could activate a code-execution hook, file-writing option, destructive operation, or other hazardous form.

## Tooling

### Codex distribution

Keep `codex` installed through Homebrew rather than declaring `@openai/codex` as a project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Dependency status labels

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available; keep the concise `ssh` label for consistency with the adjacent dependency rows.

### FFmpeg media preset compatibility

When auditing `bin/ffmpeg-wav-png`, assume that every supplied input and generated output media format, dimension, duration, and other size constraint is compliant with every platform targeted by the script. Treat this as one platform-agnostic rule for current and future presets; do not question, independently verify, or report those compatibility choices unless the user explicitly requests it.

`ffmpeg` and `ffprobe` are intentionally unmanaged optional runtime dependencies for this command. Their availability checks define the supported failure behavior; do not report their omission from bootstrap or synchronization provisioning unless the user explicitly asks to change that dependency policy.

The Instagram branch intentionally probes each audio duration before choosing `-t` or `-shortest`. Applying both options together changes stream-copied audio at the duration boundary because `-shortest` trims the packet that crosses the limit; retain the separate paths unless equivalent behavior is demonstrated with actual media.

### Fish abbreviation ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations; abbreviations defined outside domfiles are not preserved across shell startup.

### Fish formatter plugin

Keep `prettier-plugin-fish` a thin whole-file wrapper around `fish_indent`. Preserve `fish_indent` output verbatim and let it own Fish formatting semantics; Prettier options such as `tabWidth` and `useTabs` intentionally do not affect Fish output.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. `fish_indent` has no range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged.

The `expectTypeOf(pluginFish).toExtend<Plugin>()` assertion intentionally serves as a forward-compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist; behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports. Do not report the assertion as vacuous solely because the current `Plugin` properties are optional.

### Fish local configuration

Sourcing `.config/fish/local.fish` intentionally suppresses both stdout and stderr. Do not report this redirection as hidden diagnostics; inspect or validate `local.fish` directly when its behavior is in scope.

### Peer dependency versions

Declare every peer dependency in workspace packages with the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. Do not flag `"*"` peer ranges as missing compatibility constraints or narrow them solely to mirror the currently resolved version.

### Repository-scoped commands

Keep `plugins` and `skills` in the root `dependencies`. They provide user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

Do not invoke `plugins update` from `domfiles-sync-update`: the current CLI treats unknown subcommands as plugin source paths, so the command can exit successfully without updating anything. Re-evaluate this only if upstream adds a supported update workflow.

The corresponding scripts in `bin/` are the stable command interfaces. Their implementations are resolved from the domfiles pnpm workspace so `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Do not install parallel copies through global pnpm state.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization when Git-visible tracked files differ from `HEAD`, `domfiles-sync-update` overrides this behavior with `error` so commands can run only when dependencies are already current. Revalidate these assumptions when changing the pinned pnpm major version or overriding `verifyDepsBeforeRun`.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### Synchronization checkout state

`__domfiles_is_clean` intentionally checks only whether Git-visible tracked files differ from `HEAD`. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs synchronization warnings and dependency reconciliation; repository-update safety handles assume-unchanged entries separately.

### Synchronization completion

`domfiles sync` prioritizes completing as much independent synchronization work as possible with minimal interruption. Recoverable issues must be reported, but fixing them is not a prerequisite for running or completing unrelated sync stages.

Synchronization scripts otherwise fail fast. An unhandled error or a nonzero exit from a sync stage stops the broader workflow; the best-effort policy does not suppress script failures.

Repository fetch, rebase, stashing, and stash-restoration failures are recoverable. They are reported without aborting the broader synchronization workflow, which continues against the available checkout so the remaining setup, installation, update, and cleanup stages can still run.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. Synchronization must not rebase or perform a hard reset while those entries are present because Git may overwrite their working tree contents.

The final dependency status is advisory. Its failures remain visible but do not invalidate that the broader workflow reached completion.

`.lastsync` records that the broader workflow reached completion. It is intentionally write-only for now and reserved for a possible future feature.

### Zed and Codex global instructions

The tracked `.config/zed/AGENTS.md` is the canonical global `AGENTS.md` shared by Zed and Codex. `domfiles sync` links that source to `~/.config/zed/AGENTS.md` for Zed and `~/.codex/AGENTS.md` for Codex. Both agents therefore load one instruction source across every project; it is not project scoped.

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, always refer to `.config/zed/AGENTS.md`.

Keep guidance specific to this repository in the root `AGENTS.md` or applicable project skills instead.

### Zed project scan exclusions

The repository-level `.zed/settings.json` restates the installed Zed version’s complete default `file_scan_exclusions` together with the project-specific exclusions. Zed replaces the default array whenever this property is configured, so refresh the restated defaults against the installed version when changing this setting.
