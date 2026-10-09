# Project Documentation

This document records constraints, durable facts, maintenance decisions, and rationale that are not obvious from source and configuration. `AGENTS.md` remains authoritative for agent instructions.

## Compatibility

### Node.js Engine Range

The root `engines.node` range intentionally declares the minimum supported Node.js major without mirroring narrower patch-level constraints from individual tools. pnpm and the invoked tools are expected to report when the installed Node.js release does not satisfy a tool’s more specific engine range.

### Supported Environment

Domfiles actively targets multiple Apple Silicon–based Macs. Bootstrap and synchronization must work on a fresh installation of macOS 26 or newer with Command Line Tools and Homebrew already installed and available through `PATH`.

[`home/README.md`](../home/README.md) documents the repository owner’s configuration workflow rather than a supported onboarding path for other users. It intentionally does not restate the complete supported environment or bootstrap prerequisites. Others may install the repository, but the project makes no compatibility or support commitment for that use. `domfiles sync` reports a missing Homebrew installation before synchronization. Command Line Tools provide the Git used for the initial clone before synchronization installs the managed Git version.

The canonical Apple Silicon location fallback for `brew` is only a convenience for invoking Homebrew itself. It does not relax the `PATH` prerequisite for commands installed through Homebrew.

`fish` is the default interactive shell on every managed machine. Shell behavior and setup logic must not assume that Bash or Zsh is the user’s default shell.

## Security

### Cargo Shared State

The [Zed settings](../home/.config/zed/settings.json) grant sandboxed terminal commands write access to the entire default Cargo home directory rather than enumerating cache directories and metadata files. Under the [shared pnpm store’s mutual trust model](#pnpm-shared-store), this intentionally includes Cargo configuration, credentials, and installed executables.

### Dependency Installation Policy

[`pnpm-workspace.yaml`](../pnpm-workspace.yaml) keeps installation build approval separate from publication trust. `esbuild` is explicitly denied installation build scripts because its runtime can use the prebuilt `@esbuild/darwin-arm64` optional dependency on the [supported machines](#supported-environment). This avoids installation time script execution without disabling esbuild’s runtime build API. It gives up the script’s binary version check, fallback download, and launcher optimization, so optional dependencies remain required by this installation model.

`trustPolicy: no-downgrade` checks for weaker publishing evidence than earlier releases provided. The `vercel` and `@vercel/*` exclusions allow matching packages to install despite such downgrades rather than disabling the policy for every package. `trustPolicyIgnoreAfter` bounds how long that comparison can prevent installation: pnpm skips it when the target release’s age in whole minutes exceeds 43,200, or 30 days. This cutoff applies across the workspace, not only to Vercel packages. It is neither a calendar month nor a minimum release age.

### GitHub CLI Authentication Boundary

`gh` is provisioned as a supporting agent command, but authentication remains machine-local and user-managed. The supported setup targets `github.com` with credentials stored in the operating system credential store.

GitHub CLI can fall back to storing a token in plaintext when secure credential storage is unavailable. That fallback is outside the supported boundary for agent use.

### pnpm Shared Store

Local development processes, including agents and their subprocesses, are mutually trusted. The shared pnpm store therefore prioritizes cross-project reuse over per-project cache isolation.

The [synchronization script](../home/.local/bin/domfiles-sync-install) explicitly selects pnpm’s standard macOS store location instead of a relative per-project store. An explicit setting avoids pnpm 12’s default location hard link probes.

The [Zed settings](../home/.config/zed/settings.json) grant every sandboxed terminal command write access to pnpm’s entire home directory, covering both dependency storage and package manager bootstrap state without tracking internal subdirectories. The separate cache grant remains necessary because that cache lives outside pnpm’s home. Under the mutual trust model above, this boundary intentionally permits changes to pnpm-managed executables later run outside the sandbox. Zed requires literal absolute paths without home expansion or glob matching, so the grants include the repository owner’s approved macOS username.

### Zed Agent Permission Model

The terminal intentionally has no configured command patterns because they classify normalized text rather than semantic capabilities. Equivalent effects can remain available through another executable, generated code, or a native tool.

At Zed commit `1662f5f3`, terminal sandboxing requires the feature to be enabled, a local project, a Linux, macOS, or Windows integration, and persistent `agent.sandbox_permissions.allow_unsandboxed` to be false. A once-only or thread-wide unsandboxed grant removes the sandbox wrapper for selected commands without removing the sandboxed tool surface. Commands without the sandbox wrapper run with Zed’s ambient process permissions.

At that revision, native tools calling `ToolCallEventStream::authorize` use configured permission evaluation plus built-in checks. `diagnostics`, `find_path`, `grep`, `list_directory`, and `read_file` bypass `decide_permission_from_settings` and use their built-in checks. External Agents are outside Zed Agent’s operating system sandbox. Their `AcpThread::request_tool_call_authorization` path uses ACP-supplied options, not the native evaluator.

### Zed Fetch and Sandbox Host Scope

Unrestricted host access intentionally replaces the persistent host inventory to avoid network destination prompts and repeated allowlist maintenance. This gives up destination filtering for terminal traffic, including access to loopback and private networks. Filesystem write restrictions, protected Git metadata, and independent fetch URL rules remain separate boundaries.

The [fetch and network permission policy](skills/zed-settings/references/fetch-and-network-permissions.md#apply-fetch-and-network-permission-policy) owns the configuration model and the distinction between unrestricted terminal networking and native fetch’s retained URL and DNS checks.

### Zed Shared Temporary Directory

The [Zed settings](../home/.config/zed/settings.json) grant sandboxed terminal commands write access to `/private/tmp`, macOS’s canonical target for `/tmp`, rather than enumerating individual tools’ temporary directories. This avoids repeated permission requests for tools that use fixed `/tmp` paths instead of Zed’s per-thread `TMPDIR`.

Under the [shared pnpm store’s mutual trust model](#pnpm-shared-store), this deliberately allows sandboxed terminal commands to create, modify, or delete entries throughout the shared temporary directory, including unrelated applications’ temporary files owned by the same user. Normal macOS permissions and Zed’s Git metadata protection still apply.

### Zed Worktree Permission Coupling

While Zed Agent’s terminal sandbox is active, open worktrees are normal project write roots, independent of directory and branch names. Protected Git administrative metadata requires a separate grant, including for top-level worktree moves. These sandbox limits do not apply to commands run without the sandbox wrapper.

## Agent Integration

### Agent Authorization Model

Exact recoverability is the interruption boundary for otherwise authorized local effects that are not subject to a standing approval gate. This keeps task-scoped local work low-friction without risking irrecoverable loss, disclosure, or external mutation. Batching related decisions where their approval gates permit it preserves the context needed for assessment without returning to command-level prompts.

Git publication and contribution submission require explicit authorization rather than remaining categorically user-only. History replacement retains an expected-head lease because authorization to publish does not establish that unseen remote work may be discarded. The global [authorization policy](GLOBAL.md#authorization) owns these boundaries.

### Agent Documentation Composition

Separating [`agent-documentation`](../skills/.domfiles-agent-documentation/SKILL.md) from [`skill-development`](../skills/.domfiles-skill-development/SKILL.md) keeps ordinary instruction maintenance independent of skill packaging and scripting guidance. Both rely on the domfiles-managed global documentation, review, and writing policies rather than restating them. External repositories remain self-contained and do not link to, name, or require these skills. The [repository independence contract](../skills/.domfiles-agent-documentation/references/repository-independence.md) distinguishes intentional local copies from competing policy definitions. Applicable project instructions override their fallback workflows.

The [writing composition route](../skills/.domfiles-agent-documentation/SKILL.md#apply-documentation-principles) gives every project-authored agent documentation surface and human-facing asset one writing standard, regardless of skill category or invocation mode.

The explicit route also covers formatting-only and machine-readable metadata tasks, overriding the writing skill’s standalone trigger exclusions without broadening its discovery description. Without `agent-documentation`, the writing skill’s own description determines discovery, including its exclusions for formatting-only work and work that neither evaluates nor changes wording or information architecture.

### Agent Task Relay

[`agent-task-relay`](../skills/agent-task-relay/SKILL.md) is a separate skill rather than an `agent-documentation` reference because relay composition is a frequent user-initiated task. Reaching it through the parent skill would load both for work that needs only the relay workflow.

### Browser Automation

The [global browser automation policy](GLOBAL.md#browser-automation) owns browser eligibility and tool choice. [Retrieval Boundaries](GLOBAL.md#retrieval-boundaries) distinguishes browser-backed tools from purpose-built non-browser MCP tools and owns general failure handling. This keeps browser restrictions tied to browser capabilities rather than the MCP transport. Keeping browser tool preferences in the shared instruction layer avoids modifying externally owned skills and preserves them across skill updates. Browser launch configuration remains client-owned, so the policy alone does not change how an existing MCP server starts.

### Checkout Workflow

The user creates disposable worktrees through Zed’s UI and starts agents bound to those checkouts. This makes worktree lifecycle management a user responsibility rather than an agent workflow. The global [checkout boundary](GLOBAL.md#collaboration) applies independently of contribution preparation, whose branch selection and upstream setup remain owned by [`sensible-contribution-flow`](../skills/sensible-contribution-flow/SKILL.md).

The `worktree_directory` setting in [Zed’s configuration](../home/.config/zed/settings.json) places these worktrees under `~/Projects/.zed` for repositories directly under `~/Projects`. This shared directory is a worktree container rather than an independent project.

### Claude Agent Integration

Claude Code and Zed’s Claude Agent load the project’s [`AGENTS.md`](../AGENTS.md) directly in the default project instruction mode, without a repository `CLAUDE.md` bridge. Claude’s global instruction setup is described under [global instructions](#global-agent-instructions). [`domfiles sync`](../home/.local/bin/domfiles-sync-setup) links the complete globally exposed skill set under `~/.claude/skills`, while the tracked [`.claude/skills`](../.claude/skills) symlink exposes repository-internal skills from `.agents/skills`. These native discovery locations avoid duplicating canonical content.

`domfiles sync` also links [`home/.claude/settings.json`](../home/.claude/settings.json) to `~/.claude/settings.json`. The tracked file defines the shared, non-secret preference set. Claude Code [uses this user settings path for configuration updates](https://code.claude.com/docs/en/settings), and the file is managed as mutable public configuration. Credentials and private machine or account values are excluded from this settings surface.

The [`claude-acp` registry entry](../home/.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, native permission system, sandbox, and configuration. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude’s remaining runtime state under `~/.claude`, together with `~/.claude.json`, remains machine-local outside the repository.

### Commit Workflow

The public [`sensible-commit-flow` skill](../skills/sensible-commit-flow/SKILL.md) owns commit mechanics and their standalone safety boundaries. Promotion separates the reusable workflow from personal message presentation, allowing independent installation without domfiles-managed policy or sibling skills. It remains documentation-only because native Git operations can execute approved batches without a separate staging implementation.

The global [`dom-sensible-commit-flow` overlay](../skills/.dom-sensible-commit-flow/SKILL.md) co-applies and routes global callers through the public workflow. Its editorial model comes from the repository owner’s 2026 diff-to-message history, with bodyless authored messages selected explicitly. Inherited and Git-generated messages retain operation context under the public [message constraints](../skills/sensible-commit-flow/SKILL.md#preserve-message-constraints), rather than being normalized to that personal model. The global [**Commit gate**](GLOBAL.md#conduct) and [**Index preservation**](GLOBAL.md#collaboration) policies remain applicable in managed installations.

Temporary human review markers separate preparation state from intended contribution content. Keeping them outside automatic commits avoids cleanup commits without treating the human checkpoint as completed. The [marker preservation route](../skills/sensible-commit-flow/references/preserve-human-review-markers.md) owns eligibility for excluding marker additions from commits and their preservation. The global [**Outcomes**](GLOBAL.md#communication) policy has a separate, self-contained trigger, so reporting a pending human review step does not require loading the commit workflow or classifying an edit for exclusion.

The [early Git access checkpoint](../skills/sensible-commit-flow/references/early-git-access.md) surfaces capability requirements before implementation so a requested commit does not introduce a late permission interruption.

Git 2.55.0 at [`e9019fca`](https://github.com/git/git/tree/e9019fcafe0040228b8631c30f97ae1adb61bcdc) is the recorded behavioral baseline. The history update handoff uses an explicit expected-object-ID [push lease](https://github.com/git/git/blob/e9019fcafe0040228b8631c30f97ae1adb61bcdc/Documentation/git-push.adoc#L230-L291) so background fetches cannot silently refresh its lease expectation.

### Contribution Flow

Contribution research and preferences live in [`sensible-contribution-flow`](../skills/sensible-contribution-flow/SKILL.md) so `human-facing-writing` remains useful independently of remote retrieval. The existing commit overlay supplies managed message conventions without a separate contribution overlay.

The personal [worktree lifecycle restriction](GLOBAL.md#collaboration) and [**External skills**](GLOBAL.md#documentation) policy remain global rather than constraining independently installed contribution workflows. The public skill preserves the supplied checkout default, existing state, and consuming project protections without imposing those personal gates.

The global [contribution authorization policy](GLOBAL.md#contribution-preparation-authorization) delegates authority to named, domfiles-managed workflows. Renaming the installed skill therefore requires alignment of the policy’s named delegate as well as the installation mapping and retired-name migration.

### Deferred Global Policy

Conditional global policy may move into a globally exposed skill when most sessions do not need it, following the [documentation principles](../skills/.domfiles-agent-documentation/SKILL.md#apply-documentation-principles). Eligibility depends on invocation mode. A model-invocable deferral requires a discrete trigger the agent can recognize without the deferred content and a safe default when discovery is missed. A command-only deferral requires a complete workflow that applies only when the user invokes its slash command. Conduct that applies continuously stays inline even when it is large.

The **Collaboration** policy is the standing example of what does not move. Its delegation rules shape how much work is done directly on every task rather than at one recognizable decision point. An agent that never loads them cannot notice that evidence has outgrown the main thread. Missing them also drops the boundaries a subagent inherits.

### Global Agent Instructions

The tracked [`.agents/GLOBAL.md`](GLOBAL.md) is the canonical global user instruction source shared by Claude and Zed. `domfiles sync` exposes that source as `~/.claude/CLAUDE.md` for Claude, while the tracked [`home/.config/zed/AGENTS.md`](../home/.config/zed/AGENTS.md) bridge and managed `~/.config` link expose it as `~/.config/zed/AGENTS.md` for Zed. Both agents therefore load one instruction source across every project. It is not project scoped.

The ChatGPT app still uses `~/.codex`, where synchronization continues to link this source as `AGENTS.md`. The Codex CLI is no longer provisioned, and the [migration stage](../home/.local/bin/domfiles-sync-migrate) removes its Homebrew installation.

### Global System-Available Tooling

The [global system-available tooling list](GLOBAL.md#system-available-tooling) covers non-standard supporting development commands available across projects. It mirrors the non-CI development dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../home/.local/bin/domfiles-sync-install), using executable names when package names differ and subject to the inclusions and omissions below.

The list also includes `cargo`, `fish`, `node`, `pnpm`, and `rustc` even though `domfiles-sync-install` classifies their Homebrew formulas as primary dependencies. `cargo` and `rustc` support package-oriented and direct Rust workflows, while `fish`, `node`, and `pnpm` support Fish configuration checks, JavaScript and direct TypeScript execution, and the preferred package manager workflow, respectively.

The list intentionally omits `claude`, `fisher`, `git`, `mole`, and `vim`. `claude` is an agent runtime rather than a supporting command. `fisher` is Fish package plumbing. `git` is guaranteed by the [supported environment](#supported-environment) and governed separately. `mole` is a system maintenance utility outside coding workflows. `vim` is an interactive editor.

`brew` is intentionally absent because it is a supported environment prerequisite rather than a dependency installed by `domfiles sync`. Companion commands supplied by listed dependencies, including `corepack`, `fish_indent`, `npm`, `npx`, and `rustfmt`, are not listed separately because the list tracks primary tool interfaces rather than every available executable.

### Package Release Note Skills

The [`dom-release-notes-for-humans` overlay](../skills/.dom-release-notes-for-humans/SKILL.md#presentation-conventions) retains `*` as its unordered list marker to stay consistent with previously published release notes.

### Protected Skill Mutation

Zed’s classification, described by the [protected skill mutation policy](../skills/.domfiles-skill-development/references/protected-skill-mutation.md), was checked at commit `dd04a229`. Zed requires the fixed `.agents/skills/<skill>/SKILL.md` layout for project skill discovery, so repository-internal skills retain that canonical location.

Non-Zed writes to `.agents/skills` remain outside this policy, so it does not guarantee that they hide intermediate states from concurrent Zed sessions.

### Public Skill Fallback Families

Each skill’s peer declarations own its declared fallback relationships, including the [contribution workflow’s peer table](../skills/sensible-contribution-flow/SKILL.md#compose-with-peers). The index below records selected cross-file families beyond those skill-level declarations. It is non-exhaustive and supplements the [complete-scope alignment checks](../skills/.domfiles-agent-documentation/SKILL.md#run-complete-scope-checks). An absent row does not establish that no related guidance exists.

| Canonical Contract | Standalone Guidance |
| --- | --- |
| [`intentional-dependency-choice`](../skills/intentional-dependency-choice/SKILL.md) and its routed contracts | [`skills/agent-task-relay/references/dependency-approval.md`](../skills/agent-task-relay/references/dependency-approval.md#selection-and-approval-routing), [`skills/sensible-contribution-flow/references/execution-boundaries.md`](../skills/sensible-contribution-flow/references/execution-boundaries.md#handle-dependencies-and-protected-content) |
| Requirements for governing instruction sources in [Authorization](GLOBAL.md#authorization), under **Instruction provenance** and **Approval provenance** | [`skills/agent-task-relay/references/continuing-approval.md`](../skills/agent-task-relay/references/continuing-approval.md#workflow-approval-modes), [`skills/sensible-commit-flow/references/alternative-approval-modes.md`](../skills/sensible-commit-flow/references/alternative-approval-modes.md), [`skills/sensible-contribution-flow/references/review-findings.md`](../skills/sensible-contribution-flow/references/review-findings.md#preserve-independently-governed-authority) |
| [`skills/sensible-commit-flow/references/preserve-human-review-markers.md`](../skills/sensible-commit-flow/references/preserve-human-review-markers.md) | [`skills/sensible-contribution-flow/references/preserve-human-review-markers.md`](../skills/sensible-contribution-flow/references/preserve-human-review-markers.md) |
| [`skills/sensible-commit-flow/references/update-commit-history.md`](../skills/sensible-commit-flow/references/update-commit-history.md) | [`skills/sensible-contribution-flow/references/update-commit-history.md`](../skills/sensible-contribution-flow/references/update-commit-history.md) |

Alignment is semantic rather than whole-file equality. Fallbacks and mirrors adapt contribution-specific scope and submission terminology, links into their own lifecycle, and each consuming workflow’s approval, delivery, and execution boundaries. The approval family shares requirements for governing instruction sources, not one grant scope or lifetime. These are source maintenance relationships, not installation dependencies.

### Shell Skill Composition

The [POSIX terminal presentation compatibility paragraph](../skills/posix-shell-scripting/references/functions-and-interfaces.md#terminal-destinations) is canonical. [Fish’s copy](../skills/fish-shell-scripting/references/functions-and-wrappers.md#wrapper-selection) supplies required standalone context for independent installation. They form one documentation family under the [complete-scope alignment checks](../skills/.domfiles-agent-documentation/SKILL.md#run-complete-scope-checks).

### Skill Catalogs

[`skills/README.md`](../skills/README.md) targets visitors installing public skills without synchronizing the repository. Its examples select user-wide installation to enact its recommendation. The root [`README.md`](../README.md) and individual skill READMEs intentionally use minimum viable `gh skill install` and `npx skills add` commands, omitting scope flags to keep the focus on the skills.

### Skill Description Limit

The conservative byte cap in the [skill description policy](../skills/.domfiles-skill-development/references/skill-descriptions.md#encoding-and-size) is an authoring constraint, not Zed’s unit of measurement. The [Agent Skills specification](https://agentskills.io/specification#description-field) allows 1–1,024 characters. At Zed v1.22.0 (`76659a55`), the [limit uses Unicode scalar values](https://github.com/zed-industries/zed/blob/76659a55a8c10ed355a070f8764a0b1733e3c115/crates/agent_skills/agent_skills.rs#L420-L423), [loading warns](https://github.com/zed-industries/zed/blob/76659a55a8c10ed355a070f8764a0b1733e3c115/crates/agent_skills/agent_skills.rs#L325-L341), and [strict validation rejects](https://github.com/zed-industries/zed/blob/76659a55a8c10ed355a070f8764a0b1733e3c115/crates/agent_skills/agent_skills.rs#L526-L537) overlong descriptions. The authoring cap requires revalidation whenever a supported client changes its limit.

### Skill Distribution

None of the `skills/` source namespaces is a project-local discovery surface. Client-specific project discovery remains backed by `.agents/skills/*`. Hidden source directories keep global skills, including personal overlays, out of the default repository discovery performed by `gh skill`.

Client-specific installation roots and differing canonical and installed basenames motivate the [distributed skill link contract](../skills/.domfiles-skill-development/references/skill-installation.md#distributed-links).

Edits to an exposed global skill affect its installation through the symlink and may change agent behavior across projects. Changes to the exposed set, logical names, or source-to-install mapping require synchronization changes. [`domfiles-sync-migrate`](../home/.local/bin/domfiles-sync-migrate) removes retired names rather than retaining compatibility aliases, so clients discover each logical skill once.

### Skill-Owned Script Scope

`domfiles sync` creates installed global skill symlinks that point to their source directories in this checkout rather than copying them. Their host toolchain, dependencies, and root validation remain reachable from unrelated projects, satisfying the [portable skill script contract](../skills/.domfiles-skill-development/references/portable-skill-scripts.md)’s reachable host prerequisite.

### Verify Findings Skill

The user-selected [`verify-findings`](../skills/verify-findings/SKILL.md) name favors an intuitive action over avoiding an existing descriptive name on skills.sh.

### Zed Selection-to-New-Thread Key Binding

The `ctrl-enter` binding in `home/.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## Synchronization

### Synchronization Checkout State

`__domfiles_is_clean` intentionally compares the tracked working tree with the index and the index with `HEAD`. This keeps index stat metadata alone from making the checkout appear dirty. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs dependency reconciliation, synchronization warnings, and whether synchronization preserves tracked changes in a stash for later restoration. Repository update safety handles assume-unchanged entries separately.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. While those entries are present, synchronization avoids rebases and hard resets because Git may overwrite their working tree contents.

### Synchronization Workflow

`domfiles sync` is the repository’s canonical update path. It intentionally establishes the repository-managed state, including replacing the initial contents of managed paths. That replacement is expected synchronization behavior rather than accidental data loss.

Synchronization links the repository’s `home/.local/bin` directory to `~/.local/bin`. It does not link `home/.local/share/domlib` into the user’s home because each command resolves its real path before sourcing `../share/domlib` from the repository.

`domfiles sync` is a best-effort workflow that prioritizes completing as much independent work as possible with minimal interruption. An individual failure is recoverable only when the main workflow or a sync stage handles it explicitly, surfaces the result, and can continue later work independently of the failed operation. The source’s control flow defines the exact recoverable cases.

The workflow can complete with visible, explicitly handled failures. An unhandled error or a nonzero exit from a sync stage stops the broader workflow.

The final dependency status is advisory. Its result remains visible while synchronization continues to completion.

## Tooling

### Claude Code Distribution

`claude` is intentionally installed through Homebrew’s `claude-code` cask rather than declared as an `@anthropic-ai/claude-code` project dependency. This keeps the CLI machine-level, follows Anthropic’s stable Homebrew channel, and excludes it from dependency installation in CI because `claude` is a development Homebrew dependency. The Homebrew CLI installation is separate from the `claude-acp` registry package managed by Zed.

### Cross-Shell Helper Differences

Accepted shell-specific contract differences between paired `domlib` and Fish helpers are recorded here with their rationale:

- **Command routing:** POSIX `__` routes `brew` and `pnpm` through `domlib` wrappers that add fallback or search paths, environment overrides, and custom missing-command diagnostics. Fish `__domfiles_print_and_run` invokes the requested external command directly. Fish startup establishes the supported command paths, and the generic wrapper intentionally adds no per-command routing, environment, or diagnostics.
- **Failure handling:** Argument validation failures in POSIX `__`, `__confirm`, and `__is_boolean` terminate the running shell. Their Fish peers report the error and return status 1 because terminating from an autoloaded function would close the interactive shell.
- **Quoting:** POSIX `__print_command` renders arguments with Python `shlex.quote`, while Fish `__domfiles_print_command` uses `string escape`. Each produces syntax for its own shell, so equivalent commands do not require byte-identical display text.
- **Suppression lifecycle:** `domlib` normalizes `DOMFILES_SUPPRESSED` once when loaded. Fish `__domfiles_print_command` reads and validates the current value for every command because `home/.config/fish/config.fish` intentionally does not initialize it. An unsupported value therefore fails when `domlib` loads or when Fish attempts to print a command. Fish has no `__suppress` peer because a single-command variable override can limit suppression to one function invocation.

### Dependency Status Labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell script policy](skills/shell-integration/SKILL.md#check-supported-environment-compatibility) owns the row inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

`vim` is intentionally omitted from the checklist even though synchronization installs it as a primary Homebrew dependency. Its availability does not affect the command’s output or exit status.

### Development Lint Wrapper Architecture

The language-specific `home/.local/bin/domfiles-dev-lint-*` entrypoints retain their own default scopes and lint commands. File-oriented wrappers share discovery, filtering, headings, and callback dispatch through `domlib`. ShellCheck and Tombi use native batch invocations. Fish and JSON retain per-file execution because Fish treats later operands as script arguments and the JSON check requires exactly one value per file. The [Rust wrapper](../home/.local/bin/domfiles-dev-lint-rs) invokes Clippy once for the Cargo workspace. This preserves stable interfaces for pnpm, staged linting, language-specific CI, and targeted agent validation without duplicating the execution pipeline.

File-oriented wrappers’ default discovery intentionally uses line-delimited `git ls-files` output. This lets POSIX `sh` preserve discovery failures and call the in-process lint callbacks without temporary files or another language parser. Git C-quotes backslashes, control characters, and double quotes, as well as non-ASCII bytes when `core.quotePath` is enabled. A quoted pathname is skipped because it does not resolve to the original file.

### `domlib` Helper Documentation

In helper comments, domfiles is an unformatted plural noun parallel to “dotfiles” when it denotes the repository or managed configuration, while `domfiles` is code-formatted only when it denotes the CLI command. The phrase “domfiles have …” is therefore intentional. The postpositive modifier in “heading, dimmed” preserves the shared base description across related helpers.

`__is_brew_installed` intentionally owns both the no-argument Homebrew installation check and the optional package check. Repeating “returns success” makes the result of each branch explicit. `__git_skipped_files` intentionally describes semantic skipped files while preserving tagged `git ls-files -v` entries because `git-skipped` owns display path extraction and its other callers only test whether output exists. `__git_diff_list_changed_excluded_paths` lets `--commit` and `--worktree` stand for their complete modes, with the commit reference implied by the `--commit` context.

The `__symlink` comment states the normal replacement contract and omits source containment rejection because that rejection is a safety precondition rather than an alternate supported outcome.

### FFmpeg Media Preset Compatibility

Every supplied input and generated output media format, dimension, duration, and other size constraint in `home/.config/fish/functions/ffmpeg-wav-png.fish` is an accepted platform compatibility constraint for current and future presets. Their compatibility is an accepted project premise rather than an independently verified property.

Each preset owns a complete conversion branch. The repeated discovery loop, image pairing, and output naming across those branches are intentional. Consolidating them into one shared pipeline is a non-goal, so every preset’s container, filter chain, codec options, and constraints stay independent.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish Abbreviation Ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

### Fish `clone` Argument Contract

The [`clone`](../home/.config/fish/functions/clone.fish) helper intentionally supports only `clone <repository>` and `clone <repository> <directory>`. It neither parses nor rejects Git options. Option-bearing invocations belong to `git clone` itself. An unsupported invocation can reach Git without a reliable follow-up directory change and can return a nonzero status even when Git succeeds. Both are accepted consequences of keeping the wrapper simple.

For the supported one-argument form, follow-up target derivation intentionally covers only common remote URLs and ordinary local paths. Full parity with Git’s destination naming is a non-goal, including sources addressed through an inner `.git` directory.

### Fish `$DOMFILES` Variable

`home/.config/fish/config.fish` intentionally defines `$DOMFILES` as an unexported global variable without a tracked reader. It is available within Fish sessions, including [machine-local configuration](#fish-local-configuration), while domfiles scripts resolve their own value through `domlib`.

### Fish Local Configuration

`home/.config/fish/local.fish` is active machine-local Fish configuration when present. Fish sources it through `home/.config/fish/config.fish` during startup without redirecting standard output or standard error.

A bare Fish interpreter invocation can therefore execute machine-local configuration outside the requested command and emit its output. The [global tooling guidance](GLOBAL.md#system-available-tooling) owns invocation isolation.

### Fish `ls` Alias

`home/.config/fish/aliases.fish` intentionally defines `ls` as an alias rather than an abbreviation. Fish implements it as `command ls -A`, so it bypasses Fish’s built-in `ls` function along with that function’s color and `-F` type indicator options.

### Git Diff Presentation

The lockfile-aware presentation in `git-d` and `git-view` is consolidated because it forms a substantial shared pipeline whose behavior must remain aligned.

`git-view` intentionally bypasses that split presentation for merge commits that change an excluded lockfile. Git’s native `-m` output keeps every patch within its parent-qualified section, which takes precedence over suppressing lockfile patches.

### Git Fixup Amend Behavior

The `git f --amend` fallback to `HEAD` when no first parent exists preserves support for amending a root commit.

### Git Log Search Coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is intentional in favor of implementation simplicity.

### Git Short Status Command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias or drop-in replacement for `git status`. It accepts pathspecs with an optional leading `--`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

### Metal Toolchain

The Metal Toolchain is optional provisioning for non-CI macOS environments with full Xcode. First-launch setup, license acceptance, and Xcode selection remain user-managed, preserving the [Command Line Tools–only baseline](#supported-environment).

### Peer Dependency Versions

Every peer dependency in workspace packages intentionally uses the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. These ranges are complete declarations rather than missing compatibility constraints and are not intended to mirror the currently resolved version.

### Prettier Formatter Command

[`domfiles-format`](../home/.local/bin/domfiles-format) intentionally has no recursive mode or `--write` option. Resolved paths containing control characters are unsupported so its confirmation list stays unambiguous. Its preflight check does not make subsequent per-file writes a batch transaction.

Formatting policy comes from domfiles, not target-side Prettier configuration, `.editorconfig`, or ignore files. Native formatter configuration discovery also stays rooted in domfiles.

### Prettier Formatter Wrappers

The Fish, Rust, and TOML Prettier plugins are thin whole-file wrappers. Their native formatters own formatting semantics, so output is preserved verbatim rather than reinterpreted through Prettier options.

The Rust wrapper requires an explicit edition because direct stdin formatting otherwise defaults to Rust 2015. Native defaults own the remaining policy, so there is no `rustfmt.toml`. TOML indentation matches Rust, and formatting and linting run offline to avoid remote schema fetching.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. None of the native formatters has a range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged. Prettier’s standalone mode is also intentionally unsupported because these wrappers require a Node.js process to execute their external formatter binaries.

Prettier pragma comments—including `@format`, `@prettier`, `@noformat`, and `@noprettier`—are intentionally unsupported. The wrappers omit `hasPragma`, `hasIgnorePragma`, and `insertPragma`, so `requirePragma` and `checkIgnorePragma` do not gate formatting and `insertPragma` does not add a pragma.

Interior cursor mapping is intentionally omitted. The wrappers expose a single whole-file AST node because the native formatters provide neither token locations nor source maps. End-of-input cursor positions remain supported, but interior cursors may not remain attached to the same token after formatting. The wrappers do not implement heuristic source-to-output mapping.

Each `expectTypeOf(plugin).toExtend<Plugin>()` assertion intentionally serves as a forward compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist. Behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports.

### Repository-Scoped Commands

`skills` and `vercel` intentionally remain in the root `dependencies`. They provide agent-facing or user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

The corresponding scripts in `home/.local/bin/` are the stable command interfaces. They resolve implementations from the domfiles pnpm workspace without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

pnpm 12 persists an exact `packageManager` pin at major 12 or newer in a leading environment document in `pnpm-lock.yaml`. With the default `pmOnFail: download`, every command reconciles its `packageManagerDependencies`, including version output. A frozen install fails when this document is missing or stale instead of updating it. The `packageManager` field and both lockfile documents therefore change together during a pnpm major upgrade.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `warn`, which reports outdated dependencies and runs the command without installing them. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

The [repository validation policy](../AGENTS.md#validation) separates agent checks from wrapper reconciliation through command-local `PNPM_CONFIG_*` overrides. They preserve package manager version enforcement while refusing automatic project dependency installation and environment lockfile changes. The environment form also takes precedence over the `--config.verify-deps-before-run` flag and pnpm’s inherited dependency check recursion guard. Native pnpm 12 dotted CLI configuration keys use kebab-case, and pnpm silently ignores camelCase spellings such as `--config.verifyDepsBeforeRun`.

### ripgrep Configuration Isolation

`rg` parses command line arguments before deciding whether to read the file selected by `RIPGREP_CONFIG_PATH`. Unless `--no-config` suppresses that read, it combines the configuration arguments with the command line arguments and parses them again. A configuration file can supply `--pre`, which runs another program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](GLOBAL.md#system-available-tooling) requires `--no-config` on every agent invocation.

### String Helper Reuse

The `__string_*` helpers are optional conveniences rather than a mandatory abstraction boundary.

### Suppressed Command Output

`DOMFILES_SUPPRESSED` affects only the `$ …` command echo, not command output, confirmations, errors, or headings.

`__is_ci` and `__domfiles_is_ci` override suppression, so automated runs keep the complete command trace regardless of `DOMFILES_SUPPRESSED`. A CI log is the only record of what a run executed and has no interactive reader to spare, so suppression there would remove diagnostic value without providing the benefit it exists for.

A Fish counterpart to the `DOMFILES_SUPPRESSED` initialization remains unwanted. Fish does not export `set -g`, which every `DOMFILES_*` entry in Fish configuration uses, so a counterpart in the established form would have no effect on `domlib`, while `set -gx` or `set -x` would suppress command echo for every domfiles command in the session.

An exported value reaches every child script, so `DOMFILES_SUPPRESSED=true domfiles sync` covers an entire synchronization run. `__suppress` applies the same suppression to one command by exporting the variable inside a subshell, which is how `domfiles-sync-setup` keeps the agent skill linking loop from echoing without affecting later synchronization steps.

That loop intentionally confirms the source skill directory rather than the two destinations it replaces. One source is the unit of work, both destination roots are fixed, and `__symlink` removes and recreates each destination on every run, so naming them would report routine churn rather than the artifact being distributed. The removals stay in the CI trace through `__suppress`.

That subshell is also why `__suppress` rejects `__domfiles_exec`. It would absorb that function’s `exec`, letting the caller resume and run the remainder of `domfiles-sync` a second time. The echo there is suppressed by omitting the opt-in `--print` flag instead.

The prefix form `DOMFILES_SUPPRESSED=true __symlink …` is intentionally unused. POSIX leaves it unspecified whether a variable assignment preceding a function call persists after that function returns, and macOS `/bin/sh` is Bash 3.2 in POSIX mode, where it does persist and suppresses the remainder of the script.

No standardized environment variable covers command echo suppression. `NO_COLOR` and `DO_NOT_TRACK` address color and telemetry only, so this name follows the prefixed convention of `HOMEBREW_NO_*` rather than an unprefixed `SUPPRESSED`, which any unrelated exported value in the invoking shell could set.

### Zed CLI Open Behavior

`cli_default_open_behavior` remains explicit in [the user settings](../home/.config/zed/settings.json) to avoid repeating [CLI open behavior setup](https://github.com/zed-industries/zed/blob/v1.21.0/crates/zed/src/zed/open_listener.rs#L743-L768). When the setting is absent and a CLI request reaches that setup, Zed prompts for the preferred behavior and writes the selected value back to the user settings file. The [Zed settings policy](skills/zed-settings/SKILL.md#apply-general-policy) owns the redundancy criterion.
