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

### Agent-browser configuration isolation

The [`agent-browser` shim](../bin/agent-browser) sets `AGENT_BROWSER_CONFIG` to the managed [`.config/agent-browser.json`](../.config/agent-browser.json) before resolving the repository-scoped CLI. Selecting one file suppresses upstream automatic discovery of `~/.agent-browser/config.json` and `./agent-browser.json`. A caller-supplied `AGENT_BROWSER_CONFIG` is rejected, while an explicit CLI `--config <path>` retains upstream’s higher precedence and is the supported route for project-local configuration.

The managed configuration defaults to the `domfiles` namespace so daemon sockets and restore-state directories do not collide with direct or non-domfiles use. It also enables nonce-marked content boundaries around untrusted page output.

Before exporting the managed configuration path, the shim rejects inherited `AGENT_BROWSER_*` variables that can select external state or hidden execution inputs. It preserves standard proxy variables and `AGENT_BROWSER_ENCRYPTION_KEY`, along with diagnostic, output, rendering, restore, session, and timeout controls.

### GitHub CLI authentication boundary

`gh` is provisioned as a supporting agent command, but authentication remains machine-local and user-managed. The supported setup targets `github.com` with credentials stored in the operating system credential store.

GitHub CLI can fall back to storing a token in plaintext when secure credential storage is unavailable. That fallback is outside the supported boundary for agent use.

### Permission pattern length bound

The Zed settings workflow caps decoded permission patterns at 1,000 Unicode scalars as a self-imposed reviewability bound rather than a Zed or regex-engine constraint.

### Zed agent permission model

Zed Agent tool permissions intentionally use `agent.tool_permissions.default: "allow"`. `fetch` is the only tool with tool-specific configuration. A tool that invokes configured permission evaluation and has no tool-specific entry falls back to the global baseline. A tool that bypasses that evaluator receives no decision from this setting.

The terminal intentionally has no configured command patterns. Command patterns classify normalized command text rather than semantic capabilities, so equivalent effects can remain available through another executable, generated code, or a native tool. Always-loaded agent policy governs intent and task authorization. At Zed commit `1662f5f3`, Zed wraps a terminal command in its operating-system sandbox only when the sandboxing feature is enabled, the project is local, the platform has a macOS, Linux, or Windows integration, and persistent `agent.sandbox_permissions.allow_unsandboxed` is false. An approved once-only or thread-wide unsandboxed grant runs the selected command without that wrapper while leaving the sandboxed tool surface available. The tracked settings do not persist `allow_unsandboxed`. Native `fetch` runs in Zed rather than inside the terminal sandbox and separately consumes the same per-host grants that authorize sandboxed terminal networking.

When terminal sandboxing is active, a tool-permission `allow` does not grant an effect outside that sandbox. When sandboxing is unavailable, disabled, or bypassed by an approved unsandboxed grant, the selected command runs with Zed’s ambient process permissions. For native `fetch`, a tool-permission `allow` does not bypass the shared host-grant authorization. Native path tools do not run inside the operating-system sandbox. No permission layer authorizes work prohibited by agent policy. Fetch permissions retain their separate explicit prompt model. External Agents do not run inside Zed Agent’s operating-system sandbox. At Zed commit `1662f5f3`, native Zed Agent tools that call `ToolCallEventStream::authorize` use the configured tool-permission evaluator together with any built-in checks their implementations apply. Other native tools, including `diagnostics`, `find_path`, `grep`, `list_directory`, and `read_file`, do not call `decide_permission_from_settings` and instead use their built-in path, privacy, and safety checks. External Agent permission requests enter `AcpThread::request_tool_call_authorization`, which uses ACP-supplied permission options and does not consult the native evaluator.

### Zed fetch and sandbox host scope

`agent.tool_permissions.tools.fetch.always_allow` contains one generic HTTPS syntax rule. A path-filtered fetch allowance uses a same-host `always_confirm` complement for every other direct initial path and relies on the generic rule for its approved prefixes. Confirmation precedence makes those prefixes prompt-free at the fetch-tool layer without redundant allow rules. The generic rule excludes URL userinfo and explicit ports.

`agent.sandbox_permissions.network_hosts` is the canonical persistent hostname inventory shared by native fetch and sandboxed terminal actions. Zed consumes those entries as host grants for native fetch and, while terminal sandboxing is active, as the sandbox network floor for terminal processes. It matches the grants case-insensitively without a port constraint, and each grant covers every port. This all-port, whole-host trust is a separate decision from the prompt-free initial prefixes. It is intentional where minimizing prompts outweighs path containment. Terminal actions independently inherit the global tool default and remain subject to task authorization and any active sandbox wrapper.

The same-host complement is an initial-fetch prompt filter rather than a path-scoped network boundary. Zed does not re-evaluate a same-host redirect path against fetch patterns, and the complement does not filter sandboxed terminal networking.

### Zed permission regex compatibility

`Cargo.toml` pins the Rust `regex` version used to validate Zed permission patterns. The root `Cargo.lock` may update that crate’s transitive dependencies independently. The [Zed regex compatibility audit](skills/domfiles-zed-settings/references/permission-evaluator.md#audit-zed-regex-compatibility) compares the direct version with current Zed source.

### Zed worktree permission coupling

The global [temporary-file policy](../.config/zed/AGENTS.md#temporary-files) defines the project-relative `.agent-<name>` directory namespace. The global [`git-worktrees` skill](../skills/domfiles-git-worktrees/SKILL.md) applies that namespace to isolated worktrees and owns the paired `agent/<name>` branch namespace, isolation criteria, administration, and lifecycle. When active, Zed Agent’s terminal sandbox determines terminal filesystem and Git metadata access independently of those names.

While terminal sandboxing is active, files in open worktrees are normal project write roots, while protected Git administrative metadata requires a separate sandbox grant, including for top-level worktree moves. Those sandbox limits do not apply to a command that runs without the wrapper. `terminal` actions still inherit the global `allow` and remain subject to task authorization. Native path actions that invoke configured permission evaluation inherit the same default and remain subject to their built-in checks. Native path actions that bypass the evaluator receive no configured decision and remain subject to the path, privacy, sensitive-settings, and symlink-escape checks their implementations apply. A path that looks like `.agent-<name>` neither expands terminal sandbox access nor proves the current working directory or repository boundary.

## Agent integration

### Agent authorization model

The global [authorization policy](../.config/zed/AGENTS.md#authorization) separates instruction authority from untrusted evidence so prompt injection cannot authorize its own effects.

Exact recoverability is the interruption boundary for otherwise authorized local effects that are not subject to a standing approval gate. This keeps task-scoped local work low-friction without risking irrecoverable loss, disclosure, or external mutation. Batching decisions by coherent execution phase preserves the context needed for assessment without returning to command-level prompts.

The global [proportionality rule](../.config/zed/AGENTS.md#conduct) separates standing safety gates from implementation complexity. It treats ordinary cooperative concurrency and reversible tracked-file work as preservation and validation problems rather than reasons for speculative transaction infrastructure.

Task-local finding classification and one review baseline prevent later reviewers from treating earlier fixes, settled decisions, or stale evidence as new work. The global [findings](../.config/zed/AGENTS.md#documentation) and [review convergence](../.config/zed/AGENTS.md#collaboration) rules own the resulting workflow.

Git publication remains user-only because remote Git history cannot be recalled from every consumer.

### Agent task relay

The public [`agent-task-relay` skill](../skills/agent-task-relay/SKILL.md) owns inbound validation of user-pasted findings and status responses, user-mediated task-relay flow confirmation, composition, default relay delivery, complete revision, decision basis, and general evidence-only decision relays. Task-relay flow confirmation owns a self-contained isolation decision rather than routing to `git-worktrees`. The relay records only the confirmed requirement, while the receiving environment’s repository policy owns worktree creation, operation, and cleanup. It is a separate skill rather than an `agent-documentation` reference because relay composition is a frequent user-initiated task, so reaching the standard through the parent skill would load it and the standard together. Generic relay behavior stays within the skill, split between its entrypoint and routed references, rather than in a standalone capture asset. This avoids a second normative copy. [`agent-documentation`](../skills/domfiles-agent-documentation/SKILL.md) keeps an explicit route for specialized relay-asset maintenance.

Inbound recognition is based on report-like content rather than asserted authorship. The routed [Inbound Findings](../skills/agent-task-relay/references/inbound-findings.md) workflow is the canonical owner of inbound recognition, evidence treatment, validation, reporting, and confirmation. Domfiles-managed handling of context-mismatched handoffs remains owned by the global [ambiguity rule](../.config/zed/AGENTS.md#conduct).

The global [commit gate](../.config/zed/AGENTS.md#conduct) and [collaboration policy](../.config/zed/AGENTS.md#collaboration) remain canonical for commit authorization, domfiles-managed automatic external-agent routing, non-interrupting in-client delegation, and the exact anti-drift assignment contract. The public `agent-task-relay` skill carries the commit gate’s assignment-specific application and the anti-drift contract as required standalone context for independent installations and applies both to task relays and explicit user-requested subagent drafts without mediating autonomous delegation. The `simple-github-cli` fallback for `gh agent-task create` mirrors the anti-drift contract, inherited assignment boundaries, commit gate, and dependency approval gate needed to compose and dispatch an assignment when `agent-task-relay` is unavailable. Decision relays are always non-mutating.

### Claude Agent integration

The tracked [`CLAUDE.md`](../CLAUDE.md) bridge is described in the [agent documentation table](../AGENTS.md#agent-documentation). [`domfiles sync`](../bin/domfiles-sync-setup) exposes the shared [global instructions](#claude-codex-and-zed-global-instructions) as Claude’s user-level `~/.claude/CLAUDE.md`, links the complete globally exposed skill set under `~/.claude/skills`, and the tracked [`.claude/skills`](../.claude/skills) symlink exposes repository-internal skills from `.agents/skills`. Claude therefore uses its native instruction and skill discovery locations without duplicating canonical content.

The [`claude-acp` registry entry](../.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, native permission system, sandbox, and configuration. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude user state under `~/.claude` and `~/.claude.json` remains machine-local outside the repository.

Claude follows the [External Agent permission layering](#zed-agent-permission-model): Zed’s operating-system sandbox does not isolate it. At Zed commit `1662f5f3`, Claude Agent’s ACP permission requests and its own permission system govern its tools without passing through Zed’s native tool-permission evaluator.

### Claude, Codex, and Zed global instructions

The tracked `.config/zed/AGENTS.md` is the canonical global instruction source shared by Claude, Codex, and Zed. `domfiles sync` exposes that source as `~/.claude/CLAUDE.md` for Claude and `~/.codex/AGENTS.md` for Codex, while the managed `~/.config` link exposes it as `~/.config/zed/AGENTS.md` for Zed. All three agents therefore load one instruction source across every project. It is not project scoped.

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, always refer to `.config/zed/AGENTS.md`.

The [agent-documentation ownership model](../AGENTS.md#agent-documentation) defines the repository-specific instruction surfaces.

### Deferred global policy

Conditional global policy may move into a global skill when most sessions do not need it, following the [documentation principles](../skills/domfiles-agent-documentation/SKILL.md#apply-the-documentation-principles). Eligibility requires a discrete trigger the agent can recognize without the deferred content, and a safe default when the route is missed. Conduct that applies continuously stays inline even when it is large.

The `Collaboration` policy is the standing example of what does not move. Its delegation rules shape how much work is done directly on every task rather than at one recognizable decision point, an agent that never loads them cannot notice that evidence has outgrown the main thread, and missing them drops the boundaries a subagent inherits.

`git-worktrees` is the first such deferral. Its former `Default` bullet was concurrent-work hygiene rather than worktree policy, so preserving existing changes and avoiding another agent’s write scope now lives in the global “Concurrent work” rule. Its route lives with the global temporary-file `.agent-<name>` convention, which the two namespaces share, and the current-checkout rule names the skill so it cannot read as a prohibition on isolation.

### GitHub CLI agent integration

The public [`simple-github-cli` skill](../skills/simple-github-cli/SKILL.md) owns conditional agent behavior for `gh`. It carries the authentication and remote-mutation rules its workflow needs plus the [command-specific standalone handoff fallback](#agent-task-relay) for `gh agent-task create`, so the skill remains independently usable. The global [GitHub CLI policy](../.config/zed/AGENTS.md#github-cli) retains the route and aligned domfiles-managed copies of those gates so they remain directly loaded across projects.

`gh agent-task` and the other non-simple families in [Opt-In Operations](../skills/simple-github-cli/SKILL.md#opt-in-operations) are never chosen without a direct user request. The boundary is scope-based rather than tied to preview status. User-requested external task handoffs use `agent-task-relay` for confirmation and assignment composition when it is available, while `simple-github-cli` owns the selected `gh` interface and terminal command delivery for `gh agent-task create` and task-bearing `gh copilot` invocations. `simple-github-cli` declares `agent-task-relay` through one entrypoint route and one bundled [optional-peer reference](../skills/simple-github-cli/references/optional-peer-agent-task-relay.md). `agent-task-relay` carries a generic workflow-owned delivery deferral, and the `simple-github-cli` agent-task fallback preserves standalone behavior without the peer.

### Global system-available tooling

The [global system-available tooling list](../.config/zed/AGENTS.md#system-available-tooling) covers non-standard supporting development commands that agents can invoke directly across projects. It mirrors the non-CI development dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../bin/domfiles-sync-install), using executable names when package names differ and subject to the inclusions and omissions below.

The list also includes `cargo`, `fish`, `node`, `pnpm`, and `rustc` even though `domfiles-sync-install` classifies their Homebrew formulas as primary dependencies. `cargo` and `rustc` support package-oriented and direct Rust workflows, while `fish`, `node`, and `pnpm` support Fish configuration checks, JavaScript and direct TypeScript execution, and the preferred package-manager workflow, respectively.

The list intentionally omits `claude`, `codex`, `fisher`, `git`, `mole`, and `vim`. `claude` and `codex` are agent runtimes rather than supporting commands. `fisher` is Fish package plumbing. `git` is guaranteed by the [supported environment](#supported-environment) and governed separately. `mole` is a system-maintenance utility outside coding workflows. `vim` is an interactive editor.

`brew` is intentionally absent because it is a supported-environment prerequisite rather than a dependency installed by `domfiles sync`. Companion commands supplied by listed dependencies, including `corepack`, `fish_indent`, `npm`, `npx`, and `rustfmt`, are not listed separately because the list tracks primary tool interfaces rather than every available executable.

In shell sessions configured by `domfiles` after synchronization, direct invocation assumes repository-managed commands are available through `PATH` in addition to the [supported-environment](#supported-environment) prerequisites.

### Package release-note bullet marker

The [release-note bullet-marker policy](../skills/domfiles-release-notes/SKILL.md#write-concise-consumer-facing-prose) preserves `*` because previously published notes use that marker. This keeps new and revised release notes consistent even though Markdown accepts other unordered-list markers.

### Protected skill mutation

At Zed commit `dd04a229`, native mutation tools force confirmation when a directly named or canonical path contains consecutive `.agents` and `skills` components. Repository-root `AGENTS.md`, `.agents/PROJECT.md`, the root `skills` directory, and other `.agents` paths outside `skills` do not receive that agent-specific classification. Zed also requires the fixed `.agents/skills/<skill>/SKILL.md` layout for project skill discovery, so repository-internal skills retain that canonical location.

The public `skills/human-facing-writing` source does not receive Zed’s agent-specific classification. Its staging boundary applies to every agent because changes to its writing contract can affect every public skill composed through it.

The [protected skill mutation policy](../skills/domfiles-agent-documentation/references/protected-skill-mutation.md) owns the exact workflow. Its `.agents/skills` branch is limited to Zed Agent’s native permission model. Non-Zed writes to `.agents/skills` remain outside this policy, so the policy does not guarantee that they hide intermediate states from concurrent Zed sessions. Registered `.agent-<name>` worktrees and task staging roots are peer uses of the shared namespace, which rules out nested staging roots.

### Repository harmonization

The global [`repository-harmonization` skill](../skills/domfiles-repository-harmonization/SKILL.md) owns the `Harmonize` shorthand and its change-oriented cross-repository consistency workflow.

### Shorthand command routing

A shorthand owned by a skill is routed by that skill’s description, which declares the bare command. The global [shorthand-command policy](../.config/zed/AGENTS.md#shorthand-commands) therefore carries no per-skill route headings. A heading would duplicate a trigger the description already provides and would require maintenance for every current and future shorthand. `Verify` remains defined inline because no skill owns it.

### Skill description limit

The 1,024-byte figure in the [skill description policy](../skills/domfiles-agent-documentation/SKILL.md#compose-the-change) is Zed’s limit rather than an intrinsic property of skill descriptions. Each client that receives the global skill set applies its own limit, so the figure requires revalidation whenever a supported client changes one.

### Skill distribution

The [skill distribution contract](../AGENTS.md#skills) defines project-authored skill categories and installation surfaces. Every tracked skill remains subject to the repository’s public-disclosure boundary.

[`bin/domfiles-sync-setup`](../bin/domfiles-sync-setup) defines the exact source-to-destination mappings for globally exposed skills. The current `skills/domfiles-*` set is global, while `skills/agent-task-relay`, `skills/fish-shell-scripting`, `skills/human-facing-writing`, `skills/posix-shell-scripting`, and `skills/simple-github-cli` are public.

Documentation for global skills is maintained under the assumption that an installation exposing one global skill exposes the complete set. The skills form a complementary ecosystem on top of the same global instructions, allowing one skill to defer an overlapping domain to its canonical sibling instead of repeating fallback guidance.

The public [`posix-shell-scripting`](../skills/posix-shell-scripting/SKILL.md) and [`fish-shell-scripting`](../skills/fish-shell-scripting/SKILL.md) skills respectively own portable POSIX shell and Fish authoring, review, audit, diagnosis, and validation guidance. The repository-internal [`domfiles-shell-integration`](skills/domfiles-shell-integration/SKILL.md) skill retains domfiles-specific shell invariants and integration policy. General wording remains owned by [`human-facing-writing`](../skills/human-facing-writing/SKILL.md), keeping shell semantics separate from editorial guidance.

The public `human-facing-writing` skill applies its [Writing Principles](../skills/human-facing-writing/SKILL.md#writing-principles) standard to every task, then routes connected prose and technical copy to separate references, giving overlapping work one precedence contract while preserving a complete nontechnical path. The global **Numbering** rule exists for Zed-specific behavior, remains owned by [`.config/zed/AGENTS.md`](../.config/zed/AGENTS.md#writing), and is intentionally excluded from the public typography contract. Synchronization removes the obsolete managed symlinks rather than retaining aliases, so clients discover the merged skill once.

The canonical `domfiles-` prefix distinguishes global source directories from unprefixed public source directories without changing a global skill’s identity.

Supported clients expose globally installed skills beneath different configuration roots, and a global skill’s canonical basename differs from its installed basename. The [distributed-skill link contract](../skills/domfiles-agent-documentation/SKILL.md#keep-distributed-skill-links-installation-safe) owns the resulting portability requirements.

Independent public installation removes the shared-policy and guaranteed-sibling assumptions available to global skills. Every project-authored writing surface in a public skill remains agent documentation and is also composed through `human-facing-writing` as human-facing installation, evaluation, or maintenance content. The [public skill portability contract](../skills/domfiles-agent-documentation/references/public-skill-portability.md) owns that composition boundary alongside standalone behavior, optional composition, and descriptions. Every public skill entrypoint carries an aligned standalone stale-guidance contract because an independent installation cannot rely on the global retrieval-failure policy or repository-maintainer context when a reference breaks or the skill contradicts current interfaces or behavior. Guidance-specific outcomes take precedence, while the portability contract owns generic runtime behavior and mirror alignment. This source-authoring composition creates no installed sibling dependency.

Edits to an exposed global skill affect its globally discovered installation through the symlink and may change agent behavior across projects. Adding or removing a globally exposed skill, changing its logical name, or changing its source-to-install mapping requires updating synchronization behavior. Removing or renaming a logical skill that has already been distributed also requires migration behavior for obsolete installed paths.

Every supported installation of the global `agent-documentation` skill is assumed to load an equivalent domfiles-managed global instruction layer. The skill relies on that layer’s documentation, writing, review, and `Verify` policies instead of restating them. External repositories remain self-contained and do not name, require, or link to the skill. Applicable project instructions continue to override its fallback workflow.

### Skill-owned script scope

`domfiles-zed-settings` is the sole script owner today, and the root `Cargo.toml` registers its binaries and adjacent tests so the root Cargo workspace validates them.

A global skill’s scripts stay hosted here. `domfiles sync` symlinks each global skill rather than copying it, so the installed skill is this checkout and the host toolchain, dependencies, and root validation remain reachable while an agent works in an unrelated project. That symlink is the precondition the [portable skill script contract](../skills/domfiles-agent-documentation/references/portable-skill-scripts.md) depends on, and it is why those scripts take every separate project they inspect or change as an explicitly selected target instead of resolving one from their installed path.

Agent script tests are not excluded from the repository’s test workflow. Collecting a TypeScript agent script test would additionally require a Vitest project entry covering the skill tree, which waits until the first such script exists.

The [smallest sufficient contract](../skills/domfiles-agent-documentation/references/skill-owned-scripts.md#design-the-smallest-sufficient-contract) gate challenges necessity before correctness. Adversarial design review runs before implementation and remains bounded to declared consumers, evidence, and the operating model, so it removes unsupported contract elements instead of hardening a script around speculative requirements.

### Version-sensitive agent documentation

Version-sensitive agent documentation uses one authoritative upstream baseline because current documentation, pinned source, and upstream `main` can describe different implementations. The canonical [agent-documentation workflow](../skills/domfiles-agent-documentation/SKILL.md#compose-the-change) resolves conflicts against that baseline before editing and ties security-boundary claims to exact implementation evidence.

### Zed selection-to-new-thread key binding

The `ctrl-enter` binding in `.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## Synchronization

### Synchronization checkout state

`__domfiles_is_clean` intentionally compares the tracked working tree with the index and the index with `HEAD`. This keeps index stat metadata alone from making the checkout appear dirty. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs synchronization warnings and dependency reconciliation. Repository-update safety handles assume-unchanged entries separately.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. While those entries are present, synchronization avoids rebases and hard resets because Git may overwrite their working tree contents.

### Synchronization workflow

`domfiles sync` is the repository’s canonical update path. It intentionally establishes the repository-managed state, including replacing the initial contents of managed paths. That replacement is expected synchronization behavior rather than accidental data loss.

`domfiles sync` is a best-effort workflow that prioritizes completing as much independent work as possible with minimal interruption. An individual failure is recoverable only when the main workflow or a sync stage handles it explicitly, surfaces the result, and can continue later work independently of the failed operation. Source control flow defines the exact recoverable cases.

The workflow can complete with visible, explicitly handled failures. An unhandled error or a nonzero exit from a sync stage stops the broader workflow.

The final dependency status is advisory. Its result remains visible while synchronization continues to completion.

`.lastsync` records only that the broader workflow reached its end. Command output remains the record of individual operation outcomes. The file remains intentionally write-only until a consumer is introduced.

## Tooling

### Claude Code distribution

`claude` is intentionally installed through Homebrew’s `claude-code` cask rather than declared as an `@anthropic-ai/claude-code` project dependency. This keeps the CLI machine-level, follows Anthropic’s stable Homebrew channel, and excludes it from dependency installation in CI because `claude` is a development Homebrew dependency. The Homebrew CLI installation is separate from the `claude-acp` registry package managed by Zed.

### Codex distribution

`codex` is intentionally installed through Homebrew rather than declared as an `@openai/codex` project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Cross-shell helper differences

Accepted shell-specific contract differences between paired `domlib` and Fish helpers are recorded here with their rationale. None are currently established.

### Dependency status labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell-script policy](skills/domfiles-shell-integration/SKILL.md#check-supported-environment-compatibility) owns the row-inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

The `rust` row reports whether both `cargo` and `rustc` are available, matching the managed Homebrew formula rather than either executable name.

`vim` is intentionally omitted from the checklist even though synchronization installs it as a primary Homebrew dependency. Its availability does not affect the command’s output or exit status.

### Development lint wrapper architecture

The language-specific `bin/domfiles-dev-lint-*` entrypoints retain their own default scopes and lint commands while sharing discovery and execution through `domlib`. This preserves stable interfaces for pnpm, staged linting, language-specific CI, and targeted agent validation without duplicating the execution pipeline.

Default discovery intentionally uses line-delimited `git ls-files` output. This lets POSIX `sh` preserve discovery failures and call the in-process lint callbacks without temporary files or another language parser. Git can C-quote control characters and, when `core.quotePath` is enabled, non-ASCII bytes. A quoted pathname is skipped because it does not resolve to the original file, so pass that path explicitly when linting it.

The lockfile-aware presentation in `git-d` and `git-view` is consolidated because it forms a substantial shared pipeline whose behavior must remain aligned.

`git-view` intentionally bypasses that split presentation for merge commits that change an excluded lockfile. Git’s native `-m` output keeps every patch within its parent-qualified section, which takes precedence over suppressing lockfile patches.

### Domlib helper documentation

Every `domlib` function has one adjacent contract comment. The uniform surface lets readers compare helpers without reconstructing shell bodies. Comment prose wraps at 80 columns while preserving ordinary sentence flow, so a wrapped line remains a continuation rather than a separate statement. Internal periods may separate sentences, while terminal punctuation remains omitted under the shell prose policy.

Comments describe the semantic contract domfiles adds. They omit ordinary behavior already implied by a command-shaped name, implementation values canonically owned by source, validation and fallback details, and cross-cutting policy owned elsewhere unless the omission would make the contract misleading. The `__touch` comment therefore emphasizes file existence, standard permissions, and parent creation while timestamp updates remain implied by `touch`. The `__print_command` and `__suppress` comments leave the CI exception to [suppressed command output](#suppressed-command-output), which canonically owns that policy.

In helper comments, domfiles is an unformatted plural noun parallel to “dotfiles” when it denotes the repository or managed configuration, while `domfiles` is code-formatted only when it denotes the CLI command. The phrase “domfiles have …” is therefore intentional. The postpositive modifiers in “heading, dimmed” and “text, formatted” preserve the shared base description across related helpers rather than introducing separate terminology for each variant.

`__is_brew_installed` intentionally owns both the no-argument Homebrew installation check and the optional package check. Repeating “returns success” makes the result of each branch explicit. `__git_skipped_files` intentionally describes semantic skipped files while preserving tagged `git ls-files -v` entries because `git-skipped` owns display-path extraction and its other callers only test whether output exists. `__git_diff_list_changed_excluded_paths` lets `--commit` and `--worktree` stand for their complete modes, with the commit reference implied by the `--commit` context.

`__ssh_add`’s comment intentionally relies on the command-shaped name for ordinary success and failure semantics. The helper reports failures before returning nonzero, allowing `domfiles-sync` to tolerate the status without silencing the diagnostic.

The `__symlink` comment states the normal replacement contract and omits source-containment rejection because that rejection is a safety precondition rather than an alternate supported outcome. The helper creates the complete missing destination-parent chain through `__mkdir` and `mkdir -p`. Standard permissions apply to the final parent passed to `__mkdir`, while any ancestors created by `mkdir -p` retain their ordinary creation modes.

### FFmpeg media preset compatibility

Every supplied input and generated output media format, dimension, duration, and other size constraint in `bin/ffmpeg-wav-png` is an accepted platform-compatibility constraint for current and future presets. Their compatibility is an accepted project premise rather than an independently verified property.

Each preset owns a complete conversion branch. The repeated discovery loop, image pairing, and output naming across those branches are intentional. Consolidating them into one shared pipeline is a non-goal, so every preset’s container, filter chain, codec options, and constraints stay independent.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish abbreviation ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

### Fish `clone` argument contract

The [`clone`](../.config/fish/functions/clone.fish) helper intentionally supports only `clone <repository>` and `clone <repository> <directory>`. It neither parses nor rejects Git options. Use `git clone` directly for option-bearing invocations. An unsupported invocation can reach Git without a reliable follow-up directory change, which is an accepted consequence of keeping the wrapper simple.

For the supported one-argument form, follow-up target derivation intentionally covers only common remote URLs and ordinary local paths. Full parity with Git’s destination naming is a non-goal, including sources addressed through an inner `.git` directory.

### Fish interactive configuration

Tracked aliases and colors load only during interactive Fish sessions. `.config/fish/config.fish` keeps both sources inside its `status is-interactive` guard so noninteractive Fish invocations do not inherit interactive-only aliases or color configuration.

### Fish local configuration

`.config/fish/local.fish` is active machine-local Fish configuration when present. Its sourcing intentionally suppresses both stdout and stderr so local setup does not add shell-startup output.

Fish sources this file through `.config/fish/config.fish` during noninteractive startup. A bare Fish interpreter invocation can therefore execute machine-local configuration outside the requested command while suppressing its output. The [global tooling guidance](../.config/zed/AGENTS.md#system-available-tooling) defaults agent invocations to `fish --no-config` unless Fish startup configuration or configured runtime behavior is in scope.

### Git log search coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is intentional in favor of implementation simplicity.

### Git short status command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias for or drop-in replacement for `git status`. It accepts pathspecs with an optional leading `--`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

### Peer dependency versions

Every peer dependency in workspace packages intentionally uses the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. These ranges are complete declarations rather than missing compatibility constraints and are not intended to mirror the currently resolved version.

### Prettier formatter wrappers

`prettier-plugin-fish`, `prettier-plugin-rust`, and `prettier-plugin-toml` are intentionally thin whole-file wrappers around Homebrew-provided `fish_indent`, `rustfmt`, and `taplo`, respectively. Each native formatter’s output is preserved verbatim, and that formatter owns its language’s formatting semantics. Prettier options such as `tabWidth` and `useTabs` intentionally do not affect their output. The Fish and Rust wrappers declare the `fish` and `rust-script` interpreters so Prettier infers their parsers for extensionless files with matching hashbangs.

Markdown files directly under `skills/posix-shell-scripting/references/` use a two-space Prettier indentation override so embedded `sh` examples follow the skill’s POSIX indentation convention. The override intentionally excludes `skills/posix-shell-scripting/SKILL.md` because applying the same indentation setting to the whole file would reindent its YAML frontmatter. The skill’s example-bearing guidance is therefore structured as references, while `SKILL.md` retains the rules and routes needed by every invocation.

The Rust wrapper invokes `rustfmt --edition 2024 --emit stdout`. The explicit edition is required because direct stdin formatting otherwise defaults to Rust 2015. Native `rustfmt` defaults own all remaining Rust formatting policy, so the repository intentionally has no `rustfmt.toml` and exposes no duplicate Prettier options. The TOML wrapper invokes `taplo fmt -` and likewise relies on the native formatter’s defaults, so the repository has no Taplo configuration or duplicate Prettier options. Homebrew’s `fish`, `rust`, and `taplo` formulas provision all three native formatters, while `rustup` and `rust-analyzer` are intentionally unmanaged.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. None of the native formatters has a range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged. Prettier’s standalone mode is also intentionally unsupported because these wrappers require a Node.js process to execute their external formatter binaries.

Prettier pragma comments—including `@format`, `@prettier`, `@noformat`, and `@noprettier`—are intentionally unsupported. The wrappers omit `hasPragma`, `hasIgnorePragma`, and `insertPragma`, so `requirePragma` and `checkIgnorePragma` do not gate formatting and `insertPragma` does not add a pragma.

Interior cursor mapping is intentionally omitted. The wrappers expose a single whole-file AST node because the native formatters provide neither token locations nor source maps. End-of-input cursor positions remain supported, but interior cursors may not remain attached to the same token after formatting. The wrappers do not implement heuristic source-to-output mapping.

Each `expectTypeOf(plugin).toExtend<Plugin>()` assertion intentionally serves as a forward-compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist. Behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports. The assertion’s forward-compatibility value remains despite the current `Plugin` properties being optional.

### Repository-scoped commands

`agent-browser`, `plugins`, and `skills` intentionally remain in the root `dependencies`. They provide agent-facing or user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

`domfiles-sync-update` intentionally does not invoke `plugins update` because the current CLI treats unknown subcommands as plugin source paths, so the command can exit successfully without updating anything. This decision can be revisited if upstream adds a supported update workflow.

The corresponding scripts in `bin/` are the stable command interfaces. They resolve implementations from the domfiles pnpm workspace without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

pnpm 12 persists an exact `packageManager` pin at major 12 or newer in a leading environment document in `pnpm-lock.yaml`. With the default `pmOnFail: download`, every command reconciles its `packageManagerDependencies`, including version output. A frozen install fails when this document is missing or stale instead of updating it. The `packageManager` field and both lockfile documents therefore change together during a pnpm major upgrade.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout-state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `warn`, which reports outdated dependencies and runs the command without installing them. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### Ripgrep configuration isolation

`rg` reads `RIPGREP_CONFIG_PATH` before parsing arguments, and a configuration file can supply `--pre`, which runs another program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](../.config/zed/AGENTS.md#system-available-tooling) requires `--no-config` on every agent invocation.

### String helper reuse

The `__string_*` helpers are optional conveniences rather than a mandatory abstraction boundary.

### Suppressed command output

`DOMFILES_SUPPRESSED` suppresses the `$ …` command echo emitted by `__print_command`. It defaults to `false`. `domlib` parses user-supplied values through `__read_boolean_from_env`, then only normalized `true` enables suppression. Gating that one function covers every caller—`__`, and therefore `__chmod`, `__mkdir`, `__touch`, and `__symlink`, plus `__ssh_add` and `__domfiles_exec --print`. Only the echo is suppressed, so a wrapped command’s own output, headings, confirmations, and errors continue to print.

`__is_ci` overrides suppression, so automated runs keep the complete command trace regardless of `DOMFILES_SUPPRESSED`. A CI log is the only record of what a run executed and has no interactive reader to spare, so suppression there would remove diagnostic value without providing the benefit it exists for.

`__suppress` overrides `DOMFILES_SUPPRESSED` only inside its own subshell. The variable is runtime control state rather than a path mirrored into Fish. The [`domlib` maintenance policy](skills/domfiles-shell-integration/references/domlib-integration.md#maintain-domlib) therefore exempts it from the `$DOMFILES_*` parity set.

A `.config/fish/config.fish` counterpart remains unwanted for a different reason than the other exemptions. Fish does not export `set -g`, which every `DOMFILES_*` entry in that file uses, so a counterpart in the established form would have no effect on `domlib`, while `set -gx` or `set -x` would suppress command echo for every domfiles command in the session.

An exported value reaches every child script, so `DOMFILES_SUPPRESSED=true domfiles sync` covers an entire synchronization run. `__suppress` applies the same suppression to one command by exporting the variable inside a subshell, which is how `domfiles-sync-setup` keeps the agent-skill linking loop from echoing without affecting later synchronization steps.

That loop intentionally confirms the source skill directory rather than the two destinations it replaces. One source is the unit of work, both destination roots are fixed, and `__symlink` removes and recreates each destination on every run, so naming them would report routine churn rather than the artifact being distributed. The removals stay in the CI trace through `__suppress`.

That subshell is also why `__suppress` rejects `__domfiles_exec`. It would absorb that function’s `exec`, letting the caller resume and run the remainder of `domfiles-sync` a second time. The echo there is suppressed by omitting the opt-in `--print` flag instead.

The prefix form `DOMFILES_SUPPRESSED=true __symlink …` is intentionally unused. POSIX leaves it unspecified whether a variable assignment preceding a function call persists after that function returns, and macOS `/bin/sh` is Bash 3.2 in POSIX mode, where it does persist and suppresses the remainder of the script.

No standardized environment variable covers command-echo suppression. `NO_COLOR` and `DO_NOT_TRACK` address color and telemetry only, so this name follows the prefixed convention of `HOMEBREW_NO_*` rather than an unprefixed `SUPPRESSED`, which any unrelated exported value in the invoking shell could set.
