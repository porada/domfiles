# Shell Integration

## Cross-Shell Helper Differences

Accepted shell-specific contract differences between paired `domlib` and Fish helpers are recorded here with their rationale:

- **Caching:** Fish `__domfiles_is_macos` caches its exit status, and `__domfiles_pretty_hostname` caches its output, after successful system lookups. Caching is limited to these two helpers because their results are stable enough to reuse for a shell session and doing so avoids repeated system calls during prompt rendering and shell setup. Fish command substitution preserves these global assignments. POSIX helpers remain uncached because this optimization targets repeated lookups within interactive Fish sessions. In addition, command substitutions such as `$(__pretty_hostname)` discard assignments when their subshell exits. A machine rename appears in a new Fish session.
- **Command routing:** POSIX `__` routes `brew` through its `domlib` wrapper for fallback discovery, environment overrides, and a custom missing-command diagnostic. Other commands run directly, as they do through Fish `__domfiles_print_and_run`.
- **Failure handling:** Argument validation failures in POSIX `__`, `__confirm`, `__continue_on_macos`, and `__is_boolean` terminate the running shell. Their Fish peers report the error and return status 1 because terminating from an autoloaded function would close the interactive shell.
- **Platform guards:** Outside macOS, POSIX `__continue_on_macos` prints a command-specific notice and exits successfully. Fish `__domfiles_continue_on_macos` prints the same notice and returns status 1, leaving the caller to return rather than closing the interactive shell.
- **Quoting:** POSIX `__print_command` uses `__quote_params` to match Python’s `shlex.quote` output. Fish `__domfiles_print_command` uses `string escape`. Each renders syntax for its own shell, so equivalent commands may be displayed differently.
- **Suppression lifecycle:** `domlib` normalizes `DOMFILES_SUPPRESSED` once when loaded. Fish `__domfiles_print_command` reads and validates the current value for every command because `home/.config/fish/config.fish` intentionally [does not initialize it](#suppressed-command-output). An unsupported value therefore fails when `domlib` loads or when Fish attempts to print a command. Fish has no `__suppress` peer because a single-command variable override can limit suppression to one function invocation.

## Dependency Status Labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell script policy](../skills/shell-integration/SKILL.md#check-supported-environment-compatibility) owns the row inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

## Development Lint Wrapper Architecture

The language-specific `home/.local/bin/domfiles-dev-lint-*` entrypoints retain independent default scopes and lint commands, while file-oriented wrappers share discovery and dispatch through `domlib`. ShellCheck runs in a batch. Fish and JSON run per file because Fish treats later operands as script arguments and the JSON check requires exactly one value per file.

File-oriented wrappers’ default discovery intentionally uses line-delimited `git ls-files` output. This lets POSIX `sh` preserve discovery failures and call the in-process lint callbacks without temporary files or another language parser. Git C-quotes backslashes, control characters, and double quotes, as well as non-ASCII bytes when `core.quotePath` is enabled. A quoted pathname is skipped because it does not resolve to the original file.

## `domfiles-format` Command

[`domfiles-format`](../../home/.local/bin/domfiles-format) intentionally has no recursive mode or `--write` option. Resolved paths containing control characters are unsupported so its confirmation list stays unambiguous. Its preflight check does not make subsequent per-file writes a batch transaction.

Formatting policy comes from domfiles, not target-side Prettier configuration, `.editorconfig`, or ignore files. Native formatter configuration discovery also stays rooted in domfiles.

## `domlib` Helper Contracts

`__is_brew_installed` intentionally combines the no-argument Homebrew installation check and the optional package check in one helper rather than separate predicates. Its adjacent source comment owns both contracts.

The `__print_heading_dimmed` comment intentionally uses “heading, dimmed” to keep the same base description as `__print_heading`.

The `__symlink` comment intentionally omits rejection of a destination that equals or contains the source. It describes the normal replacement contract, not the safety preconditions for performing it.

## FFmpeg Media Preset Compatibility

All supplied input and generated output media formats, dimensions, durations, and other size constraints in `home/.config/fish/functions/ffmpeg-wav-png.fish` are accepted platform compatibility constraints for current and future presets. They are project premises, not independently verified claims.

Each preset owns a complete conversion branch. The repeated discovery loop, image pairing, and output naming across those branches are intentional. Consolidating them into one shared pipeline is a non-goal, so every preset’s container, filter chain, codec options, and constraints stay independent.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

## Fish Abbreviation Ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

## Fish `clone` Argument Contract

The [`clone`](../../home/.config/fish/functions/clone.fish) helper intentionally supports only `clone <repository>` and `clone <repository> <directory>`. It neither parses nor rejects Git options. Option-bearing invocations belong to `git clone` itself. An unsupported invocation can reach Git without a reliable follow-up directory change and can return a non-zero status even when Git succeeds. Both are accepted consequences of keeping the wrapper simple.

For the supported one-argument form, the helper intentionally derives the follow-up directory only from common remote URLs and ordinary local paths. Full parity with Git’s destination naming is a non-goal, including sources addressed through an inner `.git` directory.

## Fish `$DOMFILES` Variable

`home/.config/fish/config.fish` intentionally defines `$DOMFILES` as an unexported global variable even though no tracked code reads it. It is available within Fish sessions, including [machine-local configuration](#fish-local-configuration), while domfiles scripts resolve their own value through `domlib`.

## Fish Local Configuration

`home/.config/fish/config.fish` sources the machine-local `home/.config/fish/local.fish` only during interactive Fish startup, without redirecting standard output or standard error. Non-interactive SSH commands skip this file during shell startup.

The [global tooling guidance](../GLOBAL.md#system-available-tooling) covers Fish invocations that need to skip startup configuration entirely.

## Fish `ls` Alias

`home/.config/fish/aliases.fish` intentionally defines `ls` as an alias rather than an abbreviation. Fish implements it as `command ls -A`, so it bypasses Fish’s built-in `ls` function along with that function’s color and `-F` type indicator options.

## Git Diff Presentation

`git-view` intentionally bypasses lockfile-aware split presentation for merge commits that change an excluded lockfile. Git’s native `-m` output keeps every patch within its parent-qualified section, which takes precedence over suppressing lockfile patches.

## Git Log Search Coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is accepted to keep the pipeline simple.

## Git Short Status Command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias or drop-in replacement for `git status`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

## Suppressed Command Output

`DOMFILES_SUPPRESSED` affects only the `$ …` command echo, not command output, confirmations, errors, or headings. Its prefix avoids collisions with unrelated environment variables.

`__is_ci` and `__domfiles_is_ci` override suppression so automated runs retain their complete diagnostic command trace. These runs have no interactive reader to benefit from suppression.

`__suppress` exports the variable inside a subshell to limit suppression to one command. An assignment-prefixed function call is unsuitable because POSIX leaves assignment persistence unspecified and macOS `/bin/sh` retains it after the call.

The subshell also explains why `__suppress` rejects `__domfiles_exec`: it would absorb the function’s `exec`, allowing the caller to resume and run the remaining synchronization twice. Omitting `__domfiles_exec`’s opt-in `--print` flag suppresses its echo instead.

Fish intentionally has no global initialization for this variable. The configuration’s usual `set -g` would not reach `domlib`, while exporting it would suppress every domfiles command in the session.

The skill-linking loop suppresses individual link commands with `__suppress` and prints one confirmation per source skill. Destination removals remain visible in CI.
