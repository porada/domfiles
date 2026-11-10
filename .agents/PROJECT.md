# Project Documentation

This document records constraints, durable facts, maintenance decisions, and rationale that are not obvious from source and configuration. `AGENTS.md` remains authoritative for agent instructions.

## Compatibility

### Node.js Engine Range

The root `engines.node` range intentionally declares the minimum supported Node.js major without mirroring narrower patch-level constraints from individual tools. pnpm and the invoked tools are expected to report when the installed Node.js release does not satisfy a tool’s more specific engine range.

### Supported Environment

Domfiles targets macOS 27 or newer on Apple Silicon and headless Ubuntu LTS with glibc on ARM64 and x86_64. The Ubuntu scope does not extend to other Linux distributions, musl-based environments, or a Mac-equivalent desktop setup. ARM64 is the first Ubuntu acceptance target. Tested support for each architecture requires its own validation.

For the Home Assistant deployment, Ubuntu runs inside the app container. Home Assistant OS remains the host and is not provisioned by domfiles. Ubuntu uses the normal `domfiles sync` workflow and shared home and agent configuration, not a reduced agent-only profile.

Bootstrap requires Git, Homebrew, and `jq` to be installed and available through `PATH`. macOS also requires Command Line Tools, which provide Git for the initial clone. Ubuntu needs Git provisioned separately before synchronization installs the managed version.

The fallback to Homebrew’s canonical Apple Silicon or Linux location is only a convenience for invoking `brew` itself. It does not relax the `PATH` prerequisite for commands installed through Homebrew.

`fish` is the intended default interactive shell on managed machines. Bootstrap and non-interactive agent processes may use another shell, without Fish startup configuration or a defined `SHELL` variable.

[`home/README.md`](../home/README.md) documents the owner’s configuration workflow, not general onboarding or the complete bootstrap prerequisites. Other users may install domfiles without a compatibility or support commitment.

## Security

### Dependency Installation Policy

[`pnpm-workspace.yaml`](../pnpm-workspace.yaml) keeps installation build approval separate from publication trust. `esbuild` is explicitly denied installation build scripts because its runtime can use the prebuilt optional binary package matching the operating system and architecture on the [supported machines](#supported-environment). This avoids running scripts during installation without disabling esbuild’s runtime build API. It forgoes the script’s binary version check, fallback download, and launcher optimization. Optional dependencies therefore remain required by this installation model.

`trustPolicy: no-downgrade` checks for weaker publishing evidence than earlier releases provided. The `vercel` and `@vercel/*` exclusions allow matching packages to install despite such downgrades rather than disabling the policy for every package. `trustPolicyIgnoreAfter` stops that comparison from blocking releases older than 30 days. This cutoff applies across the workspace, not only to Vercel packages, and is not a minimum release age.

### GitHub CLI Authentication Boundary

The supported GitHub CLI setup for agent use targets `github.com` with credentials stored in the operating system credential store.

### T3 Code MCP Authentication

The MCP URLs under `t3coderelay.com` in [`home/.config/zed/settings.json`](../home/.config/zed/settings.json) are public routing addresses, not secrets or bearer credentials. Knowing a URL does not grant access. Connections require OAuth authentication and user approval in T3 Code. Zed stores OAuth credentials in the operating system credential store, outside the tracked settings file.

### Zed Agent Permissions

Zed Agent’s terminal sandbox is deliberately disabled through `allow_unsandboxed`, and native `fetch` deliberately has no permission rules. With the terminal unsandboxed, fetch rules would only restrict destinations that terminal commands can already reach.

## Agent Integration

### Agent CLI Distribution

`claude` and `codex` are intentionally installed through Homebrew’s `claude-code` and `codex` casks rather than declared as `@anthropic-ai/claude-code` or `@openai/codex` project dependencies. Both CLIs remain machine-level development Homebrew dependencies, so dependency installation in CI excludes them. For Claude, the cask also follows Anthropic’s stable Homebrew channel. The Homebrew CLI installations are separate from the agent packages Zed manages, such as the `claude-acp` registry package.

### Agent Discovery

[`.agents/GLOBAL.md`](GLOBAL.md) is the shared global instruction source. [`domfiles sync`](../home/.local/bin/domfiles-sync-setup) exposes canonical instructions, skills, and preferences through each client’s discovery locations without duplicating content.

| Surface | Canonical Source | Client Location | Mechanism |
| --- | --- | --- | --- |
| Claude global instructions | `.agents/GLOBAL.md` | `~/.claude/CLAUDE.md` | Synchronization symlink |
| Claude project instructions | [`AGENTS.md`](../AGENTS.md) | Repository `AGENTS.md` | Direct loading by Claude Code and Zed’s Claude Agent in the default project instruction mode |
| Claude project skills | `.agents/skills/` | [`.claude/skills`](../.claude/skills) | Tracked directory symlink |
| Claude settings | [`home/.claude/settings.json`](../home/.claude/settings.json) | `~/.claude/settings.json` | Synchronization symlink |
| Codex global instructions | `.agents/GLOBAL.md` | `~/.codex/AGENTS.md` | Synchronization symlink |
| Globally exposed skills | [Globally exposed categories](../skills/.domfiles-skill-development/references/domfiles-skill-policy.md#classify-skills) under `skills/` | `~/.agents/skills/<skill-name>` and `~/.claude/skills/<skill-name>` | Synchronization symlinks using installed names |
| Zed global instructions | `.agents/GLOBAL.md` | `~/.config/zed/AGENTS.md` | Tracked [`home/.config/zed/AGENTS.md`](../home/.config/zed/AGENTS.md) bridge through the managed `~/.config` link |

None of the `skills/` source namespaces is a project-local discovery surface. Project discovery remains backed by `.agents/skills/*`. Hidden source directories keep global skills, including personal overlays, out of `gh skill`’s default repository discovery. Client-specific roots and differing source and installed names motivate the [distributed skill link contract](../skills/.domfiles-skill-development/references/skill-installation.md#distributed-links).

[`domfiles-sync-migrate`](../home/.local/bin/domfiles-sync-migrate) retains retired-name cleanup rather than compatibility aliases so future synchronization leaves each logical skill discoverable once.

### Claude Agent Integration

The [Claude user settings](../home/.claude/settings.json) are shared, non-secret preferences managed as mutable public configuration. Claude Code [writes configuration updates to this user settings path](https://code.claude.com/docs/en/settings), exposed through [agent discovery](#agent-discovery).

The [`claude-acp` registry entry](../home/.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, native permission system, sandbox, and configuration. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude’s remaining runtime state under `~/.claude`, together with `~/.claude.json`, remains machine-local outside the repository.

### Global System-Available Tooling

The [global system-available tooling list](GLOBAL.md#system-available-tooling) names non-standard commands that agents use for development work across projects. The list draws from the dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../home/.local/bin/domfiles-sync-install). It names each tool’s primary executable interfaces rather than every executable a package supplies. Agent runtimes, plugin managers, interactive editors, and system maintenance utilities stay off the list, as do Git and Homebrew, which the [supported environment](#supported-environment) guarantees.

### Zed Selection-to-New-Thread Key Binding

The `ctrl-enter` binding in `home/.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## User Approval Checkpoints

This non-exhaustive index records intentionally retained user approval checkpoints and their rationale. The global [**Execution checkpoints** policy](GLOBAL.md#execution-checkpoints) owns checkpoint placement.

### Contribution Publication

The contribution workflow owns [final publication approval](../skills/sensible-contribution-flow/references/hand-off-contributions.md#confirm-pull-request-publication), [fork selection and naming](../skills/sensible-contribution-flow/references/hand-off-contributions.md#resolve-publication-destination), and [approved publication execution](../skills/sensible-contribution-flow/references/hand-off-contributions.md#publish-approved-pull-requests). The final checkpoint preserves the user’s control over what is submitted on their behalf.

### Harmonization Destinations

The harmonization workflow owns [repository and role selection](skills/harmonize/references/repository-harmonization.md#resolve-repositories). This initial checkpoint preserves the user’s control over which repositories may receive changes, rather than letting discovery alone select write destinations.

### Prepush History Plan

The [`prepush` workflow](../skills/.domfiles-prepush/references/prepare-commit-proposal.md#obtain-proposal-approval) deliberately retains approval of its proposed commit series before mutation. The checkpoint lets the user judge grouping, messages, and the placement of pending changes before history is rewritten. Standing commit authority does not replace this review of the concrete proposal. Publication remains a separate decision.

## Skill Maintenance

### Agent Documentation Composition

Separating [`agent-documentation`](../skills/.domfiles-agent-documentation/SKILL.md) from [`skill-development`](../skills/.domfiles-skill-development/SKILL.md) keeps ordinary instruction maintenance independent of skill packaging and scripting guidance. Both rely on the domfiles-managed global documentation, review, and writing policies rather than restating them.

The [conditional writing composition](../skills/.domfiles-agent-documentation/SKILL.md#apply-documentation-principles) avoids loading editorial guidance for mechanical maintenance while leaving `human-facing-writing`’s discovery description unchanged.

### Agent Task Relay

[`agent-task-relay`](../skills/agent-task-relay/SKILL.md) is a separate skill rather than an `agent-documentation` reference because relay composition is a frequent user-initiated task. Reaching it through the parent skill would load both for work that needs only the relay workflow.

### Contribution Flow

Contribution research and preparation live in [`sensible-contribution-flow`](../skills/sensible-contribution-flow/SKILL.md) so `human-facing-writing` remains useful independently of remote retrieval.

The public workflow’s [command presentation rule](../skills/sensible-contribution-flow/references/execution-boundaries.md#deliver-commands) keeps separate operations individually copyable in installations without global presentation defaults.

The global [contribution authorization policy](GLOBAL.md#contribution-preparation-authorization) delegates authority to named, domfiles-managed workflows. Renaming the installed skill therefore requires updating the policy’s named delegate, the installation mapping, and the retired-name migration together.

### Human Review Markers

Keeping eligible human review markers outside automatic commits avoids cleanup commits without treating the checkpoint as completed. The [marker preservation route](../skills/sensible-commit-flow/references/preserve-human-review-markers.md) owns eligibility and preservation. Reporting pending human review under the global [**Outcomes**](GLOBAL.md#communication) policy does not depend on the commit workflow or marker eligibility.

### Package Release Note Skills

The [`dom-release-notes-for-humans` overlay](../skills/.dom-release-notes-for-humans/SKILL.md#presentation-conventions) retains `*` as its unordered list marker to stay consistent with previously published release notes.

### Shared Skill Contracts

This non-exhaustive index supplements each skill’s peer declarations and the [complete-scope alignment checks](../skills/.domfiles-agent-documentation/SKILL.md#run-complete-scope-checks). Related guidance can exist without a row here. The relationships concern source maintenance, not installation dependencies.

| Canonical Contract | Related Guidance | Relationship |
| --- | --- | --- |
| Adjacent opportunity handling under [**Findings**](GLOBAL.md#findings) | [Review findings policies](../skills/subagent-review-flow/SKILL.md#apply-findings-policies) and [relay findings](../skills/agent-task-relay/SKILL.md#findings-and-opportunities) | Standalone guidance |
| Commit authority under [**Commit gate**](GLOBAL.md#git-changes) | [Commit approval modes](../skills/sensible-commit-flow/references/alternative-approval-modes.md), [relay authority](../skills/agent-task-relay/references/inbound-findings.md#resolve-fix-authority), [review coordination](../skills/subagent-review-flow/SKILL.md#coordinate-fixes-and-re-review), and [GitHub agent tasks](../skills/simple-github-cli/references/agent-task-creation.md) | Standalone guidance |
| **Committed review state** under [Communication](GLOBAL.md#communication) | [Named-commit reviews](../skills/subagent-review-flow/SKILL.md#prepare-the-review) | Standalone guidance |
| Contribution review checkpoints, criteria, and readiness in [`sensible-contribution-flow`](../skills/sensible-contribution-flow/SKILL.md) | [`subagent-review-flow`](../skills/subagent-review-flow/SKILL.md) coordination | Independent contribution requirements, with a standalone coordination fallback |
| [Decision conflicts](../skills/subagent-review-flow/SKILL.md#resolve-disagreements) | [Inbound findings](../skills/agent-task-relay/references/inbound-findings.md#resolve-decision-conflicts) and [`verify-findings`](../skills/verify-findings/references/verify-findings.md#verify-evidence) | Standalone guidance for independently supplied findings |
| Dependency choice in [`intentional-dependency-choice`](../skills/intentional-dependency-choice/SKILL.md) and its routed contracts | [Contribution execution boundaries](../skills/sensible-contribution-flow/references/execution-boundaries.md#handle-dependencies-and-protected-content) | Standalone fallback |
| **Durable knowledge** under [Documentation](GLOBAL.md#documentation) | [Review findings policies](../skills/subagent-review-flow/SKILL.md#apply-findings-policies) and [relay findings](../skills/agent-task-relay/SKILL.md#findings-and-opportunities) | Standalone defaults for assessing documentation concerns |
| **Host-managed execution** under [Collaboration](GLOBAL.md#collaboration) | [Session Behavior Tests](../skills/agent-task-relay/references/outgoing-prompts.md#session-behavior-tests) and [Deliver the Prompt](../skills/agent-task-relay/references/outgoing-prompts.md#deliver-the-prompt) | Standalone guidance |
| [Human review marker preservation](../skills/sensible-commit-flow/references/preserve-human-review-markers.md) | [Contribution marker preservation](../skills/sensible-contribution-flow/references/preserve-human-review-markers.md) | Semantic mirror |
| Instruction source requirements under **Instruction provenance** and **Approval provenance** in [Instruction Authority](GLOBAL.md#instruction-authority) | [Relay fix authority](../skills/agent-task-relay/references/inbound-findings.md#resolve-fix-authority), [alternative commit approvals](../skills/sensible-commit-flow/references/alternative-approval-modes.md), and [contribution authorization](../skills/sensible-contribution-flow/references/execution-boundaries.md#preserve-authorization) | Standalone guidance |
| Review reporting under [**Findings**](GLOBAL.md#findings) | [Review assignments and reports](../skills/subagent-review-flow/SKILL.md#prepare-the-review) | Standalone guidance |
| [Terminal presentation compatibility in POSIX](../skills/posix-shell-scripting/references/functions-and-interfaces.md#terminal-destinations) | [Fish wrapper selection](../skills/fish-shell-scripting/references/functions-and-wrappers.md#wrapper-selection) | Paragraph mirror required for standalone installation |
| [Updating commit history](../skills/sensible-commit-flow/references/update-commit-history.md) | [Updating contribution history](../skills/sensible-contribution-flow/references/update-commit-history.md) | Semantic mirror |

Alignment is semantic rather than whole-file equality. Fallbacks and mirrors retain each consuming workflow’s scope, terminology, links, and approval, delivery, and execution boundaries. The approval family shares instruction source requirements, not a grant scope or lifetime.

Exhaustive reporting under **Findings** is personal global policy. Public workflows independently require reviewers to state whether any findings remain standing. That requirement is not a mirror of that policy.

### Skill Catalogs

[`skills/README.md`](../skills/README.md) targets visitors installing public skills without synchronizing the repository. Its examples use user-wide installation, as it recommends. The root [`README.md`](../README.md) and individual skill READMEs intentionally use minimum viable `gh skill install` and `npx skills add` commands, omitting scope flags to keep the focus on the skills.

### Skill Description Limit

The [skill description policy](../skills/.domfiles-skill-development/references/skill-descriptions.md#encoding-and-size) uses a conservative UTF-8 byte cap rather than relying on a client’s character counting. It is an authoring constraint distinct from the [Agent Skills specification’s character limit](https://agentskills.io/specification#description-field).

### Skill Security Scans

The [`human-facing-writing`](../skills/human-facing-writing/SKILL.md) description and its [GitHub Collaboration route](../skills/human-facing-writing/SKILL.md#specialized-routes) intentionally frame GitHub material as copy within the authorized writing unit to avoid Snyk Agent Scan’s [W011 warning](https://github.com/snyk/agent-scan/blob/main/docs/issue-codes.md#W011) on [skills.sh](https://www.skills.sh/porada/domfiles). This framing preserves the skill’s applicability, but its effect on the scan remains unverified.

### Writing Skill Organization

[`human-facing-writing`](../skills/human-facing-writing/SKILL.md) keeps general writing, including connected prose and substantive revision, self-contained in `SKILL.md`. Length, multiple reading units, and revision difficulty alone do not justify loading references. References serve specialized requirements, such as technical accuracy grounded in implementation evidence.

This design prioritizes reducing the context and retrieval cost of a complete writing task rather than minimizing the entrypoint’s size. It keeps substantive writing guidance and revision checks complete in `SKILL.md` rather than reducing them to summaries of their aims, preserving the workflow’s expected rigor and outcomes.

## Synchronization

### Synchronization Checkout State

`__domfiles_is_clean` intentionally compares the tracked working tree with the index and the index with `HEAD`. This keeps index stat metadata alone from making the checkout appear dirty. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs dependency reconciliation, synchronization warnings, and whether synchronization preserves tracked changes in a stash for later restoration. Repository update safety handles assume-unchanged entries separately.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. While those entries are present, synchronization avoids rebases and hard resets because Git may overwrite their working tree contents.

### Synchronization Workflow

`domfiles sync` is the repository’s canonical update path. It intentionally establishes the repository-managed state, including replacing the initial contents of managed paths. That replacement is expected synchronization behavior rather than accidental data loss.

Synchronization links the repository’s `home/.local/bin` directory to `~/.local/bin`. It does not link `home/.local/share/domlib` into the user’s home because each command resolves its real path before sourcing `../share/domlib` from the repository.

`domfiles sync` is a best-effort workflow that prioritizes completing as much independent work as possible with minimal interruption. It can finish despite visible failures only when the main workflow or a sync stage explicitly handles them and later work does not depend on the failed operation. The final dependency status is advisory.

## Shell Integration

### Cross-Shell Helper Differences

Accepted shell-specific contract differences between paired `domlib` and Fish helpers are recorded here with their rationale:

- **Caching:** Fish `__domfiles_is_macos` caches its exit status, and `__domfiles_pretty_hostname` caches its output, after successful system lookups. Caching is limited to these two helpers because their results are stable enough to reuse for a shell session and doing so avoids repeated system calls during prompt rendering and shell setup. Fish command substitution preserves these global assignments. POSIX helpers remain uncached because this optimization targets repeated lookups within interactive Fish sessions. In addition, command substitutions such as `$(__pretty_hostname)` discard assignments when their subshell exits. A machine rename appears in a new Fish session.
- **Command routing:** POSIX `__` routes `brew` through its `domlib` wrapper for fallback discovery, environment overrides, and a custom missing-command diagnostic. Other commands run directly, as they do through Fish `__domfiles_print_and_run`.
- **Failure handling:** Argument validation failures in POSIX `__`, `__confirm`, `__continue_on_macos`, and `__is_boolean` terminate the running shell. Their Fish peers report the error and return status 1 because terminating from an autoloaded function would close the interactive shell.
- **Platform guards:** Outside macOS, POSIX `__continue_on_macos` prints a command-specific notice and exits successfully. Fish `__domfiles_continue_on_macos` prints the same notice and returns status 1, leaving the caller to return rather than closing the interactive shell.
- **Quoting:** POSIX `__print_command` uses `__quote_params` to match Python’s `shlex.quote` output. Fish `__domfiles_print_command` uses `string escape`. Each renders syntax for its own shell, so equivalent commands may be displayed differently.
- **Suppression lifecycle:** `domlib` normalizes `DOMFILES_SUPPRESSED` once when loaded. Fish `__domfiles_print_command` reads and validates the current value for every command because `home/.config/fish/config.fish` intentionally [does not initialize it](#suppressed-command-output). An unsupported value therefore fails when `domlib` loads or when Fish attempts to print a command. Fish has no `__suppress` peer because a single-command variable override can limit suppression to one function invocation.

### Dependency Status Labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell script policy](skills/shell-integration/SKILL.md#check-supported-environment-compatibility) owns the row inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

### Development Lint Wrapper Architecture

The language-specific `home/.local/bin/domfiles-dev-lint-*` entrypoints retain independent default scopes and lint commands, while file-oriented wrappers share discovery and dispatch through `domlib`. ShellCheck runs in a batch. Fish and JSON run per file because Fish treats later operands as script arguments and the JSON check requires exactly one value per file.

File-oriented wrappers’ default discovery intentionally uses line-delimited `git ls-files` output. This lets POSIX `sh` preserve discovery failures and call the in-process lint callbacks without temporary files or another language parser. Git C-quotes backslashes, control characters, and double quotes, as well as non-ASCII bytes when `core.quotePath` is enabled. A quoted pathname is skipped because it does not resolve to the original file.

### `domfiles-format` Command

[`domfiles-format`](../home/.local/bin/domfiles-format) intentionally has no recursive mode or `--write` option. Resolved paths containing control characters are unsupported so its confirmation list stays unambiguous. Its preflight check does not make subsequent per-file writes a batch transaction.

Formatting policy comes from domfiles, not target-side Prettier configuration, `.editorconfig`, or ignore files. Native formatter configuration discovery also stays rooted in domfiles.

### `domlib` Helper Contracts

`__is_brew_installed` intentionally combines the no-argument Homebrew installation check and the optional package check in one helper rather than separate predicates. Its adjacent source comment owns both contracts.

The `__print_heading_dimmed` comment intentionally uses “heading, dimmed” to keep the same base description as `__print_heading`.

The `__symlink` comment intentionally omits rejection of a destination that equals or contains the source. It describes the normal replacement contract, not the safety preconditions for performing it.

### FFmpeg Media Preset Compatibility

All supplied input and generated output media formats, dimensions, durations, and other size constraints in `home/.config/fish/functions/ffmpeg-wav-png.fish` are accepted platform compatibility constraints for current and future presets. They are project premises, not independently verified claims.

Each preset owns a complete conversion branch. The repeated discovery loop, image pairing, and output naming across those branches are intentional. Consolidating them into one shared pipeline is a non-goal, so every preset’s container, filter chain, codec options, and constraints stay independent.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish Abbreviation Ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

### Fish `clone` Argument Contract

The [`clone`](../home/.config/fish/functions/clone.fish) helper intentionally supports only `clone <repository>` and `clone <repository> <directory>`. It neither parses nor rejects Git options. Option-bearing invocations belong to `git clone` itself. An unsupported invocation can reach Git without a reliable follow-up directory change and can return a non-zero status even when Git succeeds. Both are accepted consequences of keeping the wrapper simple.

For the supported one-argument form, the helper intentionally derives the follow-up directory only from common remote URLs and ordinary local paths. Full parity with Git’s destination naming is a non-goal, including sources addressed through an inner `.git` directory.

### Fish `$DOMFILES` Variable

`home/.config/fish/config.fish` intentionally defines `$DOMFILES` as an unexported global variable even though no tracked code reads it. It is available within Fish sessions, including [machine-local configuration](#fish-local-configuration), while domfiles scripts resolve their own value through `domlib`.

### Fish Local Configuration

`home/.config/fish/local.fish` is active machine-local Fish configuration when present. Fish sources it through `home/.config/fish/config.fish` during startup without redirecting standard output or standard error.

A bare Fish interpreter invocation can therefore execute machine-local configuration outside the requested command and emit its output. The [global tooling guidance](GLOBAL.md#system-available-tooling) owns invocation isolation.

### Fish `ls` Alias

`home/.config/fish/aliases.fish` intentionally defines `ls` as an alias rather than an abbreviation. Fish implements it as `command ls -A`, so it bypasses Fish’s built-in `ls` function along with that function’s color and `-F` type indicator options.

### Git Diff Presentation

`git-view` intentionally bypasses lockfile-aware split presentation for merge commits that change an excluded lockfile. Git’s native `-m` output keeps every patch within its parent-qualified section, which takes precedence over suppressing lockfile patches.

### Git Log Search Coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is accepted to keep the pipeline simple.

### Git Short Status Command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias or drop-in replacement for `git status`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

### Suppressed Command Output

`DOMFILES_SUPPRESSED` affects only the `$ …` command echo, not command output, confirmations, errors, or headings. Its prefix avoids collisions with unrelated environment variables.

`__is_ci` and `__domfiles_is_ci` override suppression so automated runs retain their complete diagnostic command trace. These runs have no interactive reader to benefit from suppression.

`__suppress` exports the variable inside a subshell to limit suppression to one command. An assignment-prefixed function call is unsuitable because POSIX leaves assignment persistence unspecified and macOS `/bin/sh` retains it after the call.

The subshell also explains why `__suppress` rejects `__domfiles_exec`: it would absorb the function’s `exec`, allowing the caller to resume and run the remaining synchronization twice. Omitting `__domfiles_exec`’s opt-in `--print` flag suppresses its echo instead.

Fish intentionally has no global initialization for this variable. The configuration’s usual `set -g` would not reach `domlib`, while exporting it would suppress every domfiles command in the session.

The skill-linking loop uses `__suppress` and confirms each source skill rather than routine replacement of its two fixed destinations. Destination removals remain visible in CI.

## Tooling

### Fish Prettier Plugin

The Fish Prettier plugin intentionally exposes a whole-file native formatter with these boundaries:

- **Cursor mapping:** End-of-input cursor positions are supported. Interior positions may move between tokens because the native formatter supplies no token locations or source maps, and the wrapper adds no heuristic mapping.
- **Formatter output:** Native output is preserved verbatim rather than reinterpreted through Prettier options.
- **Partial ranges:** Partial `rangeEnd` or `rangeStart` requests leave source unchanged. The native formatter has no range API, and Prettier’s range calculation does not recognize custom parser names.
- **Pragmas:** `checkIgnorePragma` and `requirePragma` do not gate formatting, and `insertPragma` adds no pragma. The corresponding optional plugin hooks are absent.
- **Standalone mode:** Unsupported because the wrapper needs Node.js to execute the formatter binary.

### Metal Toolchain

The Metal Toolchain is optionally provisioned in non-CI macOS environments with full Xcode. Users manage first-launch setup, license acceptance, and Xcode selection, preserving the [Command Line Tools–only baseline](#supported-environment).

### Peer Dependency Versions

Every peer dependency in workspace packages intentionally uses the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. These ranges are complete declarations rather than missing compatibility constraints.

### pnpm Execution and Lockfile Behavior

With an exact `packageManager` pin at major 12 or newer, pnpm 12 stores that pin in the leading environment document in `pnpm-lock.yaml`. With the default `pmOnFail: download`, every command, including a version query, reconciles its `packageManagerDependencies`. A frozen install fails when this document is missing or stale instead of updating it. The `packageManager` field and both lockfile documents therefore change together during a pnpm major upgrade.

The [repository-scoped command wrappers](#repository-scoped-commands) rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `warn`, which reports outdated dependencies and runs the command without installing them. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

### Repository-Scoped Commands

`doclint`, `skills`, and `vercel` intentionally remain in the root `dependencies`. They provide agent-facing or user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

`doclint` is an ESLint-backed Markdown CLI implemented in [`packages/doclint`](../packages/doclint), not a skill-owned portable script. It uses the domfiles Node.js and pnpm installation rather than dependencies in the repository being checked. Native ESLint configuration and plugins provide its extension mechanism rather than a custom rule runner. Its [usage and coverage](../skills/.domfiles-agent-documentation/references/check-markdown.md) belong to the `agent-documentation` skill.

The corresponding scripts in `home/.local/bin/` are the stable command interfaces. They resolve implementations from the domfiles pnpm workspace without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

### ripgrep Configuration Isolation

Without `--no-config`, `rg` combines its command line arguments with those in the file selected by `RIPGREP_CONFIG_PATH`. A configuration file can supply `--pre`, which runs another program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](GLOBAL.md#system-available-tooling) requires `--no-config` on every agent invocation.

### Worktree Location

Zed rejects absolute `git.worktree_directory` values and requires the resolved directory to stay within the repository or its parent, so the shared root lives in `~/Projects`. For repositories directly under `~/Projects`, the [Zed setting](../home/.config/zed/settings.json) places new worktrees in a per-repository directory under `~/Projects/.worktrees`. Repositories elsewhere resolve it relative to their main checkout.

### Zed CLI Open Behavior

`cli_default_open_behavior` remains explicit in [the user settings](../home/.config/zed/settings.json) to avoid repeating [CLI open behavior setup](https://github.com/zed-industries/zed/blob/v1.21.0/crates/zed/src/zed/open_listener.rs#L743-L768). When the setting is absent and a CLI request reaches that setup, Zed prompts for the preferred behavior and writes the selected value back to the user settings file. The [**Zed settings**](../AGENTS.md#general) policy owns the redundancy criterion.
