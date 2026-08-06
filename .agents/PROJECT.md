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

### Zed fetch and sandbox host scope

URL patterns in `agent.tool_permissions.tools.fetch.always_allow` that require a path after the hostname may intentionally omit that hostname from `agent.sandbox_permissions.network_hosts`. A `network_hosts` entry would persistently grant the entire host, broadening trust beyond the path-qualified fetch allowance. Preserve the extra sandbox boundary; do not report this documented divergence as accidental.

### Zed generated-output deletion

Directories named `.pnpm-store`, `build`, `coverage`, `dist`, or `node_modules` are treated as disposable generated output at any path depth. Native `delete_path` may remove either a directory root or its descendants. Terminal `rm` may do the same with `-d`, `-f`, `-R`, `-r`, `-v`, and `-x`, while `rmdir` may remove empty directories with only `-v`. Both accept an optional `--`, multiple operands, safe concrete path segments, and simple `*` or `?` globs.

Brace expansion, broader `rm` options, command substitution, parent-removing `rmdir -p`, path traversal, paths outside those named trees, similarly named directories, and variable expansion remain confirmable. Zed’s built-in sensitive-path and symlink-escape checks remain additional confirmation gates.

### Zed npm `--all` option

Treat npm’s exact `--all` as an ordinary scope option rather than a lifecycle-script override. It is safe for allowed npm command families such as `npm ls`, where it includes transitive dependencies. Keep the ambiguous `--a` and `--al` forms and exact `--allow-scripts` behind confirmation; `npm approve-scripts --all` remains confirmable because `approve-scripts` is intentionally absent from the npm positive command alternatives and terminal defaults to confirmation.

### Zed temporary archive staging

Publication audits may automatically stage the tracked `HEAD` tree only by writing Git’s built-in `tar` format to a descendant of a Zed agent terminal temporary directory, then extracting it beneath such a directory. The exact allowances are `git archive --format=tar --output=<temporary-path> HEAD` and `tar -xf <temporary-archive> -C <temporary-directory>`. The exact help (`-h` and `--help`) and list (`-l` and `--list`) forms are also allowed.

Both `/private/var/folders/.../T/zed-agent-terminal-*` and `/var/folders/.../T/zed-agent-terminal-*` roots are accepted. Every path must contain at least one safe descendant segment, and path traversal remains excluded. The Rust-compatible permission engine cannot require the archive and destination to share one generated terminal-directory identifier, so each path is constrained independently to the same temporary namespace.

Additional archive operands, alternate formats, broader extraction flags, extra operands, non-`HEAD` refs, non-temporary paths, and other archive options remain confirmable.

### Zed terminal permission limitations

Zed applies `terminal.always_confirm` ahead of `terminal.always_allow`, and its Rust-compatible regular expressions do not support lookarounds. A narrower allowance therefore cannot exempt a command that a broader confirmation rule already matches. Keep trusted local package-manager `exec` binaries inside each manager’s positive command-family allowance, using an `exec`-specific option grammar, and keep ordinary npm, pnpm, and Yarn workflows in positive command alternatives. Unlisted executable names then fall through the terminal’s confirm-by-default boundary; broad `exec` confirmation overrides would make the allowlist require brittle complement expressions. Exact informational forms can be allowed separately.

Terminal patterns cover wrapper ordering and option forms verified in the supported environment rather than every shell-equivalent permutation or speculative abbreviation. Command casing variants intentionally fall through the terminal’s confirm-by-default boundary. Recheck version-dependent option abbreviations when a package-manager major changes instead of widening a pattern speculatively.

Every Git allowance and matching confirmation override uses the same optional, repeated, fixed-value environment-assignment grammar; the exact positive list in [Zed settings](../.config/zed/settings.json) is canonical. It deliberately contains only values used by recurring approved workflows to disable optional writes, neutralize runtime config, or suppress interaction or output. `MANPAGER=cat` and `PAGER=cat` remain part of the shared prefix.

A Git variable is not eligible merely because a documented value appears safe. Add a name and value only after recurring approved use demonstrates that automatic permission is useful, prefer disabling or noninteractive values over default-restoring or enabling values, and keep every prefix copy byte-identical. Re-audit the retained semantics whenever Git changes.

Unlisted assignments remain confirmable. This includes alternate attribute, index, object, reference, repository, or worktree locations; alternate config paths or config injection; arbitrary diff, editor, helper, pager, proxy, or SSH executables; author, committer, and reflog metadata; CA, certificate, credential, key, and proxy-path selection; every value of `GIT_SSL_NO_VERIFY`, including `0`; generated, internal, test-only, or unknown variables; and uncommon compatibility, debug, format, network-tuning, pathspec, or trace controls.

### Zed worktree permission coupling

The global [agent instructions](../.config/zed/AGENTS.md#git-worktrees) pair the project-relative `.agent-<name>` namespace with the branch namespace `agent/<name>`. The corresponding allowances in [Zed settings](../.config/zed/settings.json) deliberately couple creation, cleanup, and maintenance permission to those namespaces for scoped `git worktree add` forms—including force, `-B`, `--lock`, `--no-checkout`, and `--orphan`; native `delete_path` for `.agent-*` roots and descendants; native `move_path` between strict descendants of `.agent-*` worktrees; destructive `git -C .agent-<name>` checkout, switch, reset, and clean forms; `git worktree lock`, `unlock`, dry-run `prune`, repair, and `move` or `remove` with up to two force options; `rm`; `rmdir`; and non-forced or forced `git branch` copy, deletion, reset, or rename.

The tracked [global excludes file](../.config/git/ignore) owns the `.agent-*` ignore so this namespace stays out of status output in every repository. [Git configuration](../.config/.gitconfig) selects it through `core.excludesFile`.

Keep the naming convention and permission patterns synchronized. After the required status and integration-or-abandonment verification, `.agent-<name>` directories and `agent/<name>` branches are disposable. Forced worktree move or removal is permitted only for exact paths in the worktree namespace, while forced branch deletion or rename is permitted only for branches in the agent namespace. Non-forced branch deletion retains Git’s fully-merged check; forced deletion bypasses that check but remains namespace-constrained.

Worktree lock, unlock, and repair forms accept only exact `.agent-<name>` targets, and worktree moves require both source and destination to use that namespace. Unlock remains automatic because the disposable namespace already permits forced move and removal. Worktree creation and branch copy or rename patterns enforce both namespaces but cannot compare their `<name>` suffixes, so the global instructions require preserving each worktree-branch pair. `--detach` remains confirmable because it breaks that pairing.

Zed’s native `move_path` evaluates its source and destination together and uses the most restrictive permission result. Its automatic allowance therefore matches only strict descendants of `.agent-*`: both endpoints must remain inside an agent worktree, while moving a top-level `.agent-*` directory still confirms. Use `git worktree move` for top-level worktree moves so Git’s administrative metadata stays synchronized. Zed’s sensitive-settings and symlink-escape checks remain additional confirmation gates.

Prune is automatically allowed only with `-n` or `--dry-run`; those forms may add `-v`, `--verbose`, or a simple `--expire` value. Worktree list may use the same simple expiration value. Complex lock reasons or expiration values, actual pruning, out-of-namespace paths or branches, remote operations, shell globs, path traversal, parent-removing `rmdir -p`, and other broader deletion mechanisms intentionally remain subject to confirmation.

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

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior; do not report its omission from bootstrap or synchronization provisioning unless the user explicitly asks to change that dependency policy.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish abbreviation ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations; abbreviations defined outside domfiles are not preserved across shell startup.

### Fish formatter plugin

Keep `prettier-plugin-fish` a thin whole-file wrapper around `fish_indent`. Preserve `fish_indent` output verbatim and let it own Fish formatting semantics; Prettier options such as `tabWidth` and `useTabs` intentionally do not affect Fish output.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. `fish_indent` has no range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged.

The `expectTypeOf(pluginFish).toExtend<Plugin>()` assertion intentionally serves as a forward-compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist; behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports. Do not report the assertion as vacuous solely because the current `Plugin` properties are optional.

### Fish local configuration

Sourcing `.config/fish/local.fish` intentionally suppresses both stdout and stderr. Do not report this redirection as hidden diagnostics; inspect or validate `local.fish` directly when its behavior is in scope.

### Git short status command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias for or drop-in replacement for `git status`. It accepts pathspecs with an optional leading `--`; use `git status` directly for status options or alternate output formats.

### Peer dependency versions

Declare every peer dependency in workspace packages with the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. Do not flag `"*"` peer ranges as missing compatibility constraints or narrow them solely to mirror the currently resolved version.

### Repository-scoped commands

Keep `plugins` and `skills` in the root `dependencies`. They provide user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

Do not invoke `plugins update` from `domfiles-sync-update`: the current CLI treats unknown subcommands as plugin source paths, so the command can exit successfully without updating anything. Re-evaluate this only if upstream adds a supported update workflow.

The corresponding scripts in `bin/` are the stable command interfaces. Their implementations are resolved from the domfiles pnpm workspace so `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Do not install parallel copies through global pnpm state.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization when Git-visible tracked files differ from `HEAD`, `domfiles-sync-update` overrides this behavior with `error` so commands can run only when dependencies are already current. Revalidate these assumptions when changing the pinned pnpm major version or overriding `verifyDepsBeforeRun`.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### Shell wrapper duplication

Keep the Fish and POSIX discovery and lint wrappers separate even though their orchestration overlaps. They are short, language-specific entrypoints, and direct repetition is preferable to a parameterized abstraction that exists only to remove those similarities. Do not report their shared traversal, argument handling, or heading setup as duplication.

Consolidate shell implementations when they duplicate a substantial, virtually identical behavior pipeline that must remain aligned, as with the lockfile-aware presentation in `git-d` and `git-view`.

### String helper reuse

Do not report the `__string_*` helpers themselves or equivalent inline string operations anywhere in this repository as reimplementations. Treat these helpers as optional conveniences rather than mandatory shared abstractions.

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

The repository-level `.zed/settings.json` intentionally replaces Zed’s complete default `file_scan_exclusions` array with the narrower tracked list because no other entries from the original default exclusion set are needed in this repository context. Do not restate or refresh the installed defaults. The short `.git` and `.DS_Store` entries are intentional rather than recursive `**/.git` and `**/.DS_Store` patterns.

### Zed selection-to-new-thread key binding

The `ctrl-enter` binding in `.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. Preserve the `cmd-? cmd-n cmd->` sequence: focusing the agent panel first makes `cmd-n` resolve to `agent::NewThread` instead of the editor’s `workspace::NewFile`, and the final keystroke invokes `agent::AddSelectionToThread` for the active editor selection.
