# Project documentation

This document records durable facts, rationale, constraints, and maintenance decisions that are not obvious from source and configuration. `AGENTS.md` remains authoritative for agent instructions.

## Compatibility

### Node.js engine range

The root `engines.node` range intentionally declares the minimum supported Node.js major without mirroring narrower patch-level constraints from individual tools. pnpm and the invoked tools are expected to report when the installed Node.js release does not satisfy a tool’s more specific engine range.

### Supported environment

`domfiles` actively targets multiple Apple Silicon–based Macs. Bootstrap and synchronization must work on a fresh installation of macOS 26 or newer with Command Line Tools and Homebrew already installed and available through `PATH`.

The README’s “only prerequisite” statement is intentionally scoped to third-party bootstrap software rather than an exhaustive restatement of the supported environment. Its omission of platform, operating-system, `PATH`, and vendor-tool guarantees is intentional. Command Line Tools provide the Git used for the initial clone before synchronization installs the managed Git version.

The canonical Apple Silicon location fallback for `brew` is only a convenience for invoking Homebrew itself. It does not relax the `PATH` prerequisite for commands installed through Homebrew.

`fish` is the default interactive shell on every managed machine. Shell behavior and setup logic must not assume that Bash or Zsh is the user’s default shell.

## Security

### Zed agent permission model

Agent tool permissions intentionally use an allow-by-default baseline. The terminal tool overrides that baseline with confirm-by-default behavior, using explicit allowances for accepted forms and confirmation overrides for hazardous forms.

### Zed automatic terminal denials

Configured `terminal.always_deny` rules cover forms whose purpose or verified behavior exposes ambient credentials or authentication capabilities, transports literal credentials, disables agent or operating-system security boundaries, or loads authentication identities and providers. The exact command inventory remains canonical in [Zed settings](../.config/zed/settings.json). Configured denials cannot be approved manually.

Information-looking forms receive the same treatment when their actual behavior is more privileged than their spelling suggests:

- `corepack` manager selectors can download the selected pnpm or Yarn release before displaying its help or version output.
- Direct `git credential-* … get` calls invoke credential helpers and can print stored usernames and passwords.
- Home Assistant CLI API-token, alternate-endpoint, and debug-logging forms can expose, transmit, or log Supervisor credentials. macOS `security` password-output and decrypted-dump flags print Keychain secrets.
- Package-runner operands named like discovery commands can install and execute packages, while option-looking first arguments to known mutating pnpm or Yarn script shorthands are forwarded to those scripts instead of producing package-manager help.
- `sort --compress-program` and its accepted abbreviated long forms execute an arbitrary compression helper.

These forms are denied rather than left confirmable so a discovery-looking command cannot be approved under a false premise. Custom Home Assistant configuration selection remains confirmable because it changes credential and endpoint sources without inherently disclosing them.

### Zed bulk configuration output

Bulk value listings through `git config list`, its legacy `--list` and `-l` forms, `git var -l`, and `yarn config list` remain user-confirmable because configuration can contain credentials. Their higher-precedence confirmation overrides intentionally cover display options, including name-only output, rather than relying on a brittle complement expression. Targeted configuration reads retain their existing permission treatment.

### Zed fetch and sandbox host scope

URL patterns in `agent.tool_permissions.tools.fetch.always_allow` that require a path after the hostname may intentionally omit that hostname from `agent.sandbox_permissions.network_hosts`. A `network_hosts` entry would persistently grant the entire host, broadening trust beyond the path-qualified fetch allowance.

### Zed fixture repository permissions

Strict descendants of project-relative `.agent-<name>` directories are disposable fixture repository scope, distinct from top-level agent worktrees.

Git’s repository discovery can walk from a descendant into an enclosing worktree when the descendant lacks its own repository. Permission regexes cannot verify repository boundaries, resolve gitfiles or symlinks, or neutralize user-managed Git configuration. The automatic boundary therefore treats existing descendant state and configuration-driven filters, hooks, lazy fetching, submodule behavior, and URL rewrites as trusted. Explicit command-line forms that select external or remote behavior remain outside the boundary. Zed’s sandbox, sensitive-path, and symlink-escape checks remain additional boundaries.

### Zed generated-output deletion

Entries named `.pnpm-store`, `build`, `coverage`, `dist`, or `node_modules` and paths beneath them are treated as disposable generated output at any path depth. Because permissions match paths lexically, a matching root is intentionally allowed whether it is a directory or a regular file. Native `delete_path` may remove a matching root or its descendants. Terminal `rm` may do the same with `-d`, `-f`, `-R`, `-r`, `-v`, and `-x`, while `rmdir` may remove empty directories with only `-v`. Both accept an optional `--`, multiple operands, safe concrete path segments, and simple `*` or `?` globs.

Brace expansion, broader `rm` options, parent-removing `rmdir -p`, path traversal, paths outside those named roots, and similarly named entries remain confirmable. Shell substitutions and interpolations are instead denied by the [permission evaluator](skills/domfiles-zed-settings/references/permission-evaluator.md#evaluate-permission-behavior) before configured patterns are considered. Zed’s built-in sensitive-path and symlink-escape checks remain additional confirmation gates.

### Zed npm `--all` option

npm’s exact `--all` is an ordinary scope option rather than a lifecycle-script override. It is safe for allowed npm command families such as `npm ls`, where it includes transitive dependencies. The ambiguous `--a` and `--al` forms and exact `--allow-scripts` remain behind confirmation. `npm approve-scripts --all` remains confirmable because `approve-scripts` is intentionally absent from the npm positive command alternatives and terminal defaults to confirmation.

### Zed permission regex compatibility

The exact `regex` crate version pinned in `Cargo.toml` was verified against Zed as of commit `9e23609`. The root `Cargo.lock` may update that crate’s transitive dependencies independently. The [Zed regex compatibility audit](skills/domfiles-zed-settings/references/permission-evaluator.md#audit-zed-regex-compatibility) revalidates the direct version baseline against current Zed source.

### Zed `printenv` exposure

The automatic `printenv` allowance is limited to the explicit, alphabetized non-secret variable names in [Zed settings](../.config/zed/settings.json). Unlisted names outside the denial categories below remain confirmable because agent environments can contain credentials and capability-bearing endpoints.

The automatic denial covers exact `PASSWORD` and `SSH_AUTH_SOCK` lookups, names ending case-insensitively in `_KEY`, `_PASSPHRASE`, `_PASSWORD`, `_PROXY`, `_SECRET`, or `_TOKEN`, wildcard-bearing variable operands, and zero-name output from either a bare invocation or exact `--`. This denial remains necessary even though the positive allowlist excludes those forms so neither explicit approval nor a future allowance can expose them.

### Zed command discovery defaults

Terminal discovery forms require verified exit-only behavior regardless of spelling. Long and single-dash options can be operational flags or ordinary operands, so an exact, end-anchored form qualifies only when it exits without entering an interactive mode, mutating state, reading input, or starting normal execution. A verified unsupported form may qualify when it terminates without prompting. This fail-closed boundary lets each executable own its discovery forms without treating option-like spelling as evidence of safety.

### Zed temporary archive staging

[Zed settings](../.config/zed/settings.json) provide publication audits a narrow terminal temporary namespace for staging tracked `HEAD` without modifying the checkout. The allowance covers Git’s built-in tar archive and extraction beneath that namespace. Alternate refs, formats, paths, and broader extraction options remain confirmable.

macOS can expose the same terminal temporary area through canonical and noncanonical root spellings, so both are accepted. Each path requires a safe descendant and excludes traversal. The Rust-compatible permission engine cannot require the archive and destination to share one generated terminal-directory identifier, so each is constrained independently to the same temporary namespace.

### Zed terminal permission limitations

Zed applies `terminal.always_confirm` ahead of `terminal.always_allow`, and its Rust-compatible regular expressions do not support lookarounds. A narrower allowance therefore cannot exempt a command that a broader confirmation rule already matches. Trusted local package-manager `exec` binaries remain inside each manager’s positive command-family allowance with an `exec`-specific option grammar, while ordinary npm, pnpm, and Yarn workflows remain in positive command alternatives. Unlisted executable names then fall through the terminal’s confirm-by-default boundary. Broad `exec` confirmation overrides would make the allowlist require brittle complement expressions. Exact informational forms can be allowed separately.

Terminal patterns cover wrapper ordering and option forms verified in the supported environment rather than every shell-equivalent permutation or speculative abbreviation. Command casing variants intentionally fall through the terminal’s confirm-by-default boundary. Version-dependent option abbreviations require revalidation when a package-manager major changes rather than speculative widening.

Every `rust-lldb` invocation containing `--local-lldbinit` is denied. Zed’s regex engine cannot express the option’s ordering-sensitive `--no-lldbinit` neutralization without a brittle complement, so the intentional false positive preserves LLDB’s current-directory initialization boundary.

Package-manager subcommand arguments are version-specific, so invocation tests rather than option-like spelling determine their behavior. pnpm 11 treats an option-looking first operand after `pnpm exec` as an executable name rather than a pnpm flag. Bare `--` remains a separator. Yarn Classic 1 treats unknown `dlx` as a package-script invocation and passes an option-looking first operand to that script. Those option-leading forms are denied so they cannot be approved by mistake. The supported discovery forms are `pnpm help exec` and `yarn help <name>`.

Fixed Git prefix values are limited to disabling optional behavior or interaction because arbitrary environment values can select alternate repositories, configuration, executables, credentials, or network behavior. The [Git prefix policy](skills/domfiles-zed-settings/references/git-permissions.md#apply-the-git-permission-policy) owns their selection and synchronized application across command groups.

Unlisted assignments remain confirmable. This includes alternate attribute, index, object, reference, repository, or worktree locations, alternate config paths or config injection, and arbitrary diff, editor, helper, pager, proxy, or SSH executables. It also includes author, committer, and reflog metadata, CA, certificate, credential, key, and proxy-path selection, and every value of `GIT_SSL_NO_VERIFY`, including `0`. Generated, internal, test-only, or unknown variables and uncommon compatibility, debug, format, network-tuning, pathspec, or trace controls remain confirmable as well.

Git subcommand discovery is restricted to compiled command names so aliases and external `git-*` helpers remain confirmable. Exact `--help` is allowed for the alphabetized names returned by `git --list-cmds=builtins`. Exact `-h` is allowed for the alphabetized names returned by `git --list-cmds=parseopt`, whose built-ins use Git’s parse-options framework, plus the explicitly verified `credential` exception. `git credential -h` prints usage and exits before reading credential protocol input. Both inventory commands are informational allowances. The lists require refresh when Git changes.

### Zed worktree permission coupling

The global [agent instructions](../.config/zed/AGENTS.md#git-worktrees) pair the project-relative `.agent-<name>` namespace with the branch namespace `agent/<name>`. [Zed settings](../.config/zed/settings.json) use those namespaces as the security boundary for native path tools and terminal Git and filesystem operations. This permits automated creation, maintenance, integration, and cleanup inside disposable agent scope without granting equivalent operations elsewhere.

Terminal permission matching evaluates normalized command inputs without exposing the invocation’s current working directory to configured regexes. Bare commands therefore cannot inherit agent-worktree trust from their execution directory. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) owns the resulting permission-pattern namespace requirement.

Automatic task integration permits explicit staging, commit-time staging of tracked changes, and bounded noninteractive amendments inside agent worktrees, while merges from agent branches remain fast-forward-only. Because Zed strips ordinary shell quotes before permission matching, the commit grammar cannot distinguish normalized message words from non-option relative pathspec tokens. Both are trusted only inside agent worktrees, while option-looking tokens and broader operations remain confirmable. The allowed forms also trust repository-defined clean filters and commit or post-merge hooks within the user-managed repository.

Permission patterns can require the worktree and branch namespaces independently but cannot compare their `<name>` suffixes, so pair equality remains an agent-level invariant. Forced operations remain namespace-bound. Non-forced branch deletion retains Git’s fully-merged check, while forced deletion bypasses it.

Native `move_path`’s [multi-path permission evaluation](skills/domfiles-zed-settings/references/permission-evaluator.md#evaluate-permission-behavior) enables automatic strict-descendant moves within agent worktrees. Permission regexes constrain only lexical operands and cannot detect a permitted-looking parent symlink that resolves elsewhere inside an open worktree. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) leaves direct symbolic-link creation confirmable, so native path allowances treat existing worktree-internal symlinks as previously trusted repository state. Top-level worktree moves must also update Git’s administrative metadata, while Zed’s sensitive-settings and outside-worktree symlink-escape checks remain additional confirmation gates.

Worktree pruning remains confirmable because it can mutate shared Git administrative state beyond the bounded agent namespaces. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) owns the dry-run exception.

### Zed xargs command ownership

Per-command `xargs` ownership avoids a second pooled child inventory whose membership could drift from direct command owners. The [terminal command-owner policy](skills/domfiles-zed-settings/references/terminal-permissions.md#apply-the-terminal-permission-policy) owns the exact partition and repeated wrapper grammar.

Zed authorizes the `xargs` shell segment before standard input becomes child-command arguments, so it cannot apply the child command’s normal confirmation overrides to injected options. Standard input can therefore activate hazardous child behavior after the shell segment has already been allowed. Complete nested `jq` and `ps` families require confirmation rather than denial so legitimate batching remains available with explicit approval.

## Agent integration

### Global agent skills

Skills tracked under `.agents/skills` without the `domfiles-` prefix are portable global skills whose canonical sources live in this repository. They are not scoped to `domfiles`. `domfiles sync` exposes selected portable skills through the user’s global skill directory. [`bin/domfiles-sync-setup`](../bin/domfiles-sync-setup) defines the exact links and destinations.

Portable skill documentation is maintained under the assumption that an installation exposing one portable skill exposes the complete set. The skills form a complementary ecosystem on top of the same global instructions, allowing one skill to defer an overlapping domain to its canonical sibling instead of repeating fallback guidance.

Edits to an exposed portable skill affect its globally discovered installation through the symlink and may change agent behavior across projects. Adding, removing, or renaming a portable skill requires updating synchronization behavior. Removing or renaming a skill that has already been distributed also requires migration behavior for obsolete installed paths.

Every installation of the portable `agent-documentation` skill is assumed to use the tracked global `.config/zed/AGENTS.md`. The skill relies on that document’s documentation, writing, review, and `Verify` policies instead of restating them. External repositories remain self-contained and do not name, require, or link to the skill. Applicable project instructions continue to override its fallback workflow.

### Prompt relays

The [global agent instructions](../.config/zed/AGENTS.md#prompt-relays) define prompt relay delivery and complete-revision defaults. The portable [`agent-documentation` skill](skills/agent-documentation/SKILL.md) owns the complementary [relay composition and evidence standard](skills/agent-documentation/references/prompt-relays.md) and a [generic task-relay prompt](skills/agent-documentation/assets/task-relay-prompt.md). The portable `release-notes` and `technical-copy` skills and the repository-scoped `domfiles-zed-settings` skill provide standalone decision-capture profiles for completed work ([release notes](skills/release-notes/assets/decision-capture-prompt.md), [technical copy](skills/technical-copy/assets/decision-capture-prompt.md), [Zed settings](skills/domfiles-zed-settings/assets/decision-capture-prompt.md)). These are maintainer assets rather than runtime guidance, so ordinary skill invocations do not load them.

### Global system-available tooling

The [global system-available tooling list](../.config/zed/AGENTS.md#system-available-tooling) covers non-standard supporting development commands that agents can invoke directly across projects. It mirrors the non-CI development dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../bin/domfiles-sync-install), using executable names when package names differ and subject to the inclusions and omissions below.

The list also includes `cargo`, `fish`, `node`, `pnpm`, and `rustc` even though `domfiles-sync-install` classifies their Homebrew formulas as primary dependencies. `cargo` and `rustc` support package-oriented and direct Rust workflows, while `fish`, `node`, and `pnpm` support Fish configuration checks, JavaScript and direct TypeScript execution, and the preferred package-manager workflow, respectively.

The list intentionally omits `codex`, `fisher`, `git`, `mole`, and `vim`. `codex` is an agent runtime rather than a supporting command. `fisher` is Fish package plumbing. `git` is guaranteed by the [supported environment](#supported-environment) and governed separately. `mole` is a system-maintenance utility outside coding workflows. `vim` is an interactive editor.

`brew` is intentionally absent because it is a supported-environment prerequisite rather than a dependency installed by `domfiles sync`. Companion commands supplied by listed dependencies, including `corepack`, `fish_indent`, `npm`, `npx`, and `rustfmt`, are not listed separately because the list tracks primary tool interfaces rather than every available executable.

### Package release-note bullet marker

The [release-note bullet-marker policy](skills/release-notes/SKILL.md#write-concise-consumer-facing-prose) preserves `*` because previously published notes use that marker. This keeps new and revised release notes consistent even though Markdown accepts other unordered-list markers.

### Zed and Codex global instructions

The tracked `.config/zed/AGENTS.md` is the canonical global `AGENTS.md` shared by Zed and Codex. `domfiles sync` links that source to `~/.config/zed/AGENTS.md` for Zed and `~/.codex/AGENTS.md` for Codex. Both agents therefore load one instruction source across every project. It is not project scoped.

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, always refer to `.config/zed/AGENTS.md`.

The [agent-documentation ownership model](../AGENTS.md#agent-documentation) defines the repository-specific instruction surfaces.

### Zed project scan exclusions

The repository-level `.zed/settings.json` intentionally replaces Zed’s complete default `file_scan_exclusions` array with the narrower tracked list because no other entries from the original default exclusion set are needed in this repository context. The short `.git` and `.DS_Store` entries are intentional rather than recursive `**/.git` and `**/.DS_Store` patterns.

### Zed selection-to-new-thread key binding

The `ctrl-enter` binding in `.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## Synchronization

### Synchronization checkout state

`__domfiles_is_clean` intentionally checks only whether Git-visible tracked files differ from `HEAD`. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs synchronization warnings and dependency reconciliation. Repository-update safety handles assume-unchanged entries separately.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. While those entries are present, synchronization avoids rebases and hard resets because Git may overwrite their working tree contents.

### Synchronization workflow

`domfiles sync` is the repository’s canonical update path. It intentionally establishes the repository-managed state, including replacing the initial contents of managed paths. That replacement is expected synchronization behavior rather than accidental data loss.

`domfiles sync` is a best-effort workflow that prioritizes completing as much independent work as possible with minimal interruption. An individual failure is recoverable only when the main workflow or a sync stage handles it explicitly, surfaces the result, and can continue later work independently of the failed operation. Source control flow defines the exact recoverable cases.

The workflow can complete with visible, explicitly handled failures. An unhandled error or a nonzero exit from a sync stage stops the broader workflow.

The final dependency status is advisory. Its result remains visible while synchronization continues to completion.

`.lastsync` records only that the broader workflow reached its end. Command output remains the record of individual operation outcomes. The file remains intentionally write-only until a consumer is introduced.

## Tooling

### Codex distribution

`codex` is intentionally installed through Homebrew rather than declared as an `@openai/codex` project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Dependency status labels

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

### FFmpeg media preset compatibility

Every supplied input and generated output media format, dimension, duration, and other size constraint in `bin/ffmpeg-wav-png` is an accepted platform-compatibility constraint for current and future presets. Their compatibility is an accepted project premise rather than an independently verified property.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish abbreviation ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

### Fish local configuration

`.config/fish/local.fish` is active machine-local Fish configuration when present. Its sourcing intentionally suppresses both stdout and stderr so local setup does not add shell-startup output.

### Git short status command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias for or drop-in replacement for `git status`. It accepts pathspecs with an optional leading `--`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

### Peer dependency versions

Every peer dependency in workspace packages intentionally uses the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. These ranges are complete declarations rather than missing compatibility constraints and are not intended to mirror the currently resolved version.

### Prettier formatter wrappers

`prettier-plugin-fish`, `prettier-plugin-rust`, and `prettier-plugin-toml` are intentionally thin whole-file wrappers around Homebrew-provided `fish_indent`, `rustfmt`, and `taplo`, respectively. Each native formatter’s output is preserved verbatim, and that formatter owns its language’s formatting semantics. Prettier options such as `tabWidth` and `useTabs` intentionally do not affect their output. The Fish and Rust wrappers declare the `fish` and `rust-script` interpreters so Prettier infers their parsers for extensionless files with matching hashbangs.

The Rust wrapper invokes `rustfmt --edition 2024 --emit stdout`. The explicit edition is required because direct stdin formatting otherwise defaults to Rust 2015. Native `rustfmt` defaults own all remaining Rust formatting policy, so the repository intentionally has no `rustfmt.toml` and exposes no duplicate Prettier options. The TOML wrapper invokes `taplo fmt -` and likewise relies on the native formatter’s defaults, so the repository has no Taplo configuration or duplicate Prettier options. Homebrew’s `fish`, `rust`, and `taplo` formulas provision all three native formatters, while `rustup` and `rust-analyzer` are intentionally unmanaged.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. None of the native formatters has a range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged. Prettier’s standalone mode is also intentionally unsupported because these wrappers require a Node.js process to execute their external formatter binaries.

Prettier pragma comments—including `@format`, `@prettier`, `@noformat`, and `@noprettier`—are intentionally unsupported. The wrappers omit `hasPragma`, `hasIgnorePragma`, and `insertPragma`, so `requirePragma` and `checkIgnorePragma` do not gate formatting and `insertPragma` does not add a pragma.

Interior cursor mapping is intentionally omitted. The wrappers expose a single whole-file AST node because the native formatters provide neither token locations nor source maps. End-of-input cursor positions remain supported, but interior cursors may not remain attached to the same token after formatting. The wrappers do not implement heuristic source-to-output mapping.

Each `expectTypeOf(plugin).toExtend<Plugin>()` assertion intentionally serves as a forward-compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist. Behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports. The assertion’s forward-compatibility value remains despite the current `Plugin` properties being optional.

### Repository-scoped commands

`plugins` and `skills` intentionally remain in the root `dependencies`. They provide user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

`domfiles-sync-update` intentionally does not invoke `plugins update` because the current CLI treats unknown subcommands as plugin source paths, so the command can exit successfully without updating anything. This decision can be revisited if upstream adds a supported update workflow.

The corresponding scripts in `bin/` are the stable command interfaces. Their implementations are resolved from the domfiles pnpm workspace so `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout-state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `error`, requiring dependencies to be current before commands run. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### Shell wrapper duplication

The Fish and POSIX discovery and lint wrappers are short, language-specific entrypoints. Parameterizing them would add indirection solely to remove surface similarities.

The lockfile-aware presentation in `git-d` and `git-view` is consolidated because it forms a substantial shared pipeline whose behavior must remain aligned.

### String helper reuse

The `__string_*` helpers are optional conveniences rather than a mandatory abstraction boundary.
