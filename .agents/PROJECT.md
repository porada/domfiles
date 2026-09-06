# Project Documentation

This document records durable facts, rationale, constraints, and maintenance decisions that are not obvious from source and configuration. `AGENTS.md` remains authoritative for agent instructions.

## Compatibility

### Node.js Engine Range

The root `engines.node` range intentionally declares the minimum supported Node.js major without mirroring narrower patch-level constraints from individual tools. pnpm and the invoked tools are expected to report when the installed Node.js release does not satisfy a tool’s more specific engine range.

### Supported Environment

`domfiles` actively targets multiple Apple Silicon–based Macs. Bootstrap and synchronization must work on a fresh installation of macOS 26 or newer with Command Line Tools and Homebrew already installed and available through `PATH`.

[`home/README.md`](../home/README.md) documents the repository owner’s configuration workflow rather than a supported onboarding path for other users. It intentionally does not restate the complete supported environment or bootstrap prerequisites. Others may install the repository, but the project makes no compatibility or support commitment for that use. `domfiles sync` reports a missing Homebrew installation before synchronization. Command Line Tools provide the Git used for the initial clone before synchronization installs the managed Git version.

The canonical Apple Silicon location fallback for `brew` is only a convenience for invoking Homebrew itself. It does not relax the `PATH` prerequisite for commands installed through Homebrew.

`fish` is the default interactive shell on every managed machine. Shell behavior and setup logic must not assume that Bash or Zsh is the user’s default shell.

## Security

### GitHub CLI Authentication Boundary

`gh` is provisioned as a supporting agent command, but authentication remains machine-local and user-managed. The supported setup targets `github.com` with credentials stored in the operating system credential store.

GitHub CLI can fall back to storing a token in plaintext when secure credential storage is unavailable. That fallback is outside the supported boundary for agent use.

### Permission Pattern Length Bound

The Zed settings workflow caps decoded permission patterns at 1,000 Unicode scalars as a self-imposed reviewability bound rather than a Zed or regex-engine constraint.

### Zed Agent Permission Model

Zed Agent tool permissions intentionally use `agent.tool_permissions.default: "allow"`. `fetch` is the only tool with tool-specific configuration. A tool that invokes configured permission evaluation and has no tool-specific entry falls back to the global baseline. A tool that bypasses that evaluator receives no decision from this setting.

The terminal intentionally has no configured command patterns. Command patterns classify normalized command text rather than semantic capabilities, so equivalent effects can remain available through another executable, generated code, or a native tool. Always-loaded agent policy governs intent and task authorization. At Zed commit `1662f5f3`, Zed wraps a terminal command in its operating-system sandbox only when the sandboxing feature is enabled, the project is local, the platform has a macOS, Linux, or Windows integration, and persistent `agent.sandbox_permissions.allow_unsandboxed` is false. An approved once-only or thread-wide unsandboxed grant runs the selected command without that wrapper while leaving the sandboxed tool surface available. The tracked settings do not persist `allow_unsandboxed`. Native `fetch` runs in Zed rather than inside the terminal sandbox and separately consumes the same per-host grants that authorize sandboxed terminal networking.

When terminal sandboxing is active, a tool-permission `allow` does not grant an effect outside that sandbox. When sandboxing is unavailable, disabled, or bypassed by an approved unsandboxed grant, the selected command runs with Zed’s ambient process permissions. For native `fetch`, a tool-permission `allow` does not bypass the shared host-grant authorization. Native path tools do not run inside the operating-system sandbox. No permission layer authorizes work prohibited by agent policy. Fetch permissions retain their separate explicit prompt model. External Agents do not run inside Zed Agent’s operating-system sandbox. At Zed commit `1662f5f3`, native Zed Agent tools that call `ToolCallEventStream::authorize` use the configured tool-permission evaluator together with any built-in checks their implementations apply. Other native tools, including `diagnostics`, `find_path`, `grep`, `list_directory`, and `read_file`, do not call `decide_permission_from_settings` and instead use their built-in path, privacy, and safety checks. External Agent permission requests enter `AcpThread::request_tool_call_authorization`, which uses ACP-supplied permission options and does not consult the native evaluator.

### Zed Fetch and Sandbox Host Scope

`agent.tool_permissions.tools.fetch.always_allow` contains one generic HTTPS syntax rule. A path-filtered fetch allowance uses a same-host `always_confirm` complement for every other direct initial path and relies on the generic rule for its approved prefixes. Confirmation precedence makes those prefixes prompt-free at the fetch-tool layer without redundant allow rules. The generic rule excludes URL userinfo and explicit ports.

`agent.sandbox_permissions.network_hosts` is the canonical persistent hostname inventory shared by native fetch and sandboxed terminal actions. Zed consumes those entries as host grants for native fetch and, while terminal sandboxing is active, as the sandbox network floor for terminal processes. It matches the grants case-insensitively without a port constraint, and each grant covers every port. This all-port, whole-host trust is a separate decision from the prompt-free initial prefixes. It is intentional where minimizing prompts outweighs path containment. Terminal actions independently inherit the global tool default and remain subject to task authorization and any active sandbox wrapper.

`*.spec.whatwg.org` supports recurring web-platform research across WHATWG’s complete, changing standards catalog. The current catalog assigns each listed specification a dedicated `<spec-name>.spec.whatwg.org` hostname. Exact enumeration can lag catalog changes, interrupting that workflow with host confirmations whenever research follows a newly listed hostname until settings are updated. The exception accepts every current and future strict subdomain at any depth under `spec.whatwg.org`, including any non-specification host WHATWG might place there. Every WHATWG hostname outside that suffix remains a separate host-grant decision. The [fetch and network permission policy](skills/domfiles-zed-settings/references/fetch-and-network-permissions.md#apply-the-fetch-and-network-permission-policy) owns the wildcard exception gate.

The same-host complement is an initial-fetch prompt filter rather than a path-scoped network boundary. Zed does not re-evaluate a same-host redirect path against fetch patterns, and the complement does not filter sandboxed terminal networking.

### Zed Permission Regex Compatibility

`Cargo.toml` pins the Rust `regex` version used to validate Zed permission patterns. The root `Cargo.lock` may update that crate’s transitive dependencies independently. The [Zed regex compatibility audit](skills/domfiles-zed-settings/references/permission-evaluator.md#audit-zed-regex-compatibility) compares the direct version with current Zed source.

### Zed Worktree Permission Coupling

The global [`git-worktrees` skill](../skills/.domfiles-git-worktrees/SKILL.md) is independent of the [temporary-file namespace](GLOBAL.md#temporary-files), allowing environment-managed checkouts without treating them as disposable scratch directories. Its [Zed workflow](../skills/.domfiles-git-worktrees/references/zed-worktrees.md) records the client-specific lifecycle. When active, Zed Agent’s terminal sandbox determines terminal filesystem and Git metadata access independently of directory and branch names.

The archival and reopening source baseline is Zed revision `bebe92f4`. In `crates/sidebar/src/sidebar.rs`, the [archive-task guard](https://github.com/zed-industries/zed/blob/bebe92f469834a287f5a57ed78e8d51a918b8ada/crates/sidebar/src/sidebar.rs#L5515-L5523), [reopening path](https://github.com/zed-industries/zed/blob/bebe92f469834a287f5a57ed78e8d51a918b8ada/crates/sidebar/src/sidebar.rs#L4100-L4162), and [root selection](https://github.com/zed-industries/zed/blob/bebe92f469834a287f5a57ed78e8d51a918b8ada/crates/sidebar/src/sidebar.rs#L4717-L4752) establish terminal-aware archival eligibility and conditional snapshot restoration.

While terminal sandboxing is active, files in open worktrees are normal project write roots, while protected Git administrative metadata requires a separate sandbox grant, including for top-level worktree moves. Those sandbox limits do not apply to a command that runs without the wrapper. `terminal` actions still inherit the global `allow` and remain subject to task authorization. Native path actions that invoke configured permission evaluation inherit the same default and remain subject to their built-in checks. Native path actions that bypass the evaluator receive no configured decision and remain subject to the path, privacy, sensitive-settings, and symlink-escape checks their implementations apply. A path that looks like `.agent-<name>` neither expands terminal sandbox access nor proves the current working directory or repository boundary.

## Agent Integration

### Agent Authorization Model

The global [authorization policy](GLOBAL.md#authorization) separates instruction authority from untrusted evidence so prompt injection cannot authorize its own effects.

Exact recoverability is the interruption boundary for otherwise authorized local effects that are not subject to a standing approval gate. This keeps task-scoped local work low-friction without risking irrecoverable loss, disclosure, or external mutation. Batching decisions by coherent execution phase preserves the context needed for assessment without returning to command-level prompts.

The global [proportionality rule](GLOBAL.md#conduct) separates standing safety gates from implementation complexity. It treats ordinary cooperative concurrency and reversible tracked-file work as preservation and validation problems rather than reasons for speculative transaction infrastructure.

Task-local finding classification and one review baseline prevent later reviewers from treating earlier fixes, settled decisions, or stale evidence as new work. The global [findings](GLOBAL.md#documentation) and [review convergence](GLOBAL.md#collaboration) rules own the resulting workflow.

Git publication remains user-only because remote Git history cannot be recalled from every consumer.

### Agent Task Relay

The public [`agent-task-relay` skill](../skills/agent-task-relay/SKILL.md) owns inbound validation of user-pasted findings and status responses, user-mediated task-relay flow confirmation, composition, default relay delivery, complete revision, decision basis, and general evidence-only decision relays. Task-relay flow confirmation owns a self-contained isolation decision rather than routing to `git-worktrees`. The relay records only the confirmed requirement, while the receiving environment’s repository policy owns worktree creation, operation, and cleanup. It is a separate skill rather than an `agent-documentation` reference because relay composition is a frequent user-initiated task, so reaching the standard through the parent skill would load it and the standard together. Generic relay behavior stays within the skill, split between its entrypoint and routed references, rather than in a standalone capture asset. This avoids a second normative copy. [`agent-documentation`](../skills/.domfiles-agent-documentation/SKILL.md) keeps an explicit route for specialized relay-asset maintenance.

Inbound recognition is based on report-like content rather than asserted authorship. The routed [Inbound Findings](../skills/agent-task-relay/references/inbound-findings.md) workflow is the canonical owner of inbound recognition, evidence treatment, validation, reporting, and confirmation. Domfiles-managed handling of context-mismatched handoffs remains owned by the global [ambiguity rule](GLOBAL.md#conduct).

The global [commit gate](GLOBAL.md#conduct) and [collaboration policy](GLOBAL.md#collaboration) remain canonical for commit authorization, non-interrupting in-client delegation, and the exact anti-drift assignment contract. Supported clients discover the public `agent-task-relay` skill from its description when work must continue with an external agent or in an environment with the required access, so the collaboration policy does not repeat that route. The public skill carries the commit gate’s assignment-specific application and the anti-drift contract as required standalone context for independent installations and applies both to task relays and explicit user-requested subagent drafts without mediating autonomous delegation. The `simple-github-cli` fallback for `gh agent-task create` mirrors the anti-drift contract, inherited assignment boundaries, commit gate, and dependency approval gate needed to compose and dispatch an assignment when `agent-task-relay` is unavailable. Decision relays are always non-mutating.

### Claude Agent Integration

The tracked [`CLAUDE.md`](../CLAUDE.md) bridge is described in the [agent documentation table](../AGENTS.md#agent-documentation). [`domfiles sync`](../home/.local/bin/domfiles-sync-setup) exposes the shared [global instructions](#claude-codex-and-zed-global-instructions) as Claude’s user-level `~/.claude/CLAUDE.md`, links the complete globally exposed skill set under `~/.claude/skills`, and the tracked [`.claude/skills`](../.claude/skills) symlink exposes repository-internal skills from `.agents/skills`. Claude therefore uses its native instruction and skill discovery locations without duplicating canonical content.

The [`claude-acp` registry entry](../home/.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, native permission system, sandbox, and configuration. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude user state under `~/.claude` and `~/.claude.json` remains machine-local outside the repository.

Claude follows the [External Agent permission layering](#zed-agent-permission-model): Zed’s operating-system sandbox does not isolate it. At Zed commit `1662f5f3`, Claude Agent’s ACP permission requests and its own permission system govern its tools without passing through Zed’s native tool-permission evaluator.

### Claude, Codex, and Zed Global Instructions

The tracked [`.agents/GLOBAL.md`](GLOBAL.md) is the canonical global user instruction source shared by Claude, Codex, and Zed. `domfiles sync` exposes that source as `~/.claude/CLAUDE.md` for Claude and `~/.codex/AGENTS.md` for Codex, while the tracked [`home/.config/zed/AGENTS.md`](../home/.config/zed/AGENTS.md) bridge and managed `~/.config` link expose it as `~/.config/zed/AGENTS.md` for Zed. All three agents therefore load one instruction source across every project. It is not project scoped.

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, always refer to `.agents/GLOBAL.md`.

The [agent-documentation ownership model](../AGENTS.md#agent-documentation) defines the repository-specific instruction surfaces.

### Commit Workflow

The global [`commit` skill](../skills/.domfiles-commit/SKILL.md) owns commit preparation, execution confirmation, and result verification. Its editorial model for newly composed messages comes from the repository owner’s 2026 diff-to-message history, with bodyless output selected explicitly. Inherited cherry-pick messages and Git-generated merge messages retain operation context that this editorial model would otherwise discard. It remains documentation-only because ordinary Git operations can execute the approved batches without a separate staging implementation. The global [commit gate](GLOBAL.md#conduct) and [index-preservation policy](GLOBAL.md#collaboration) remain the authorization and preservation owners.

Conditional message guidance keeps planning and message-preserving operations independent of authored-subject conventions without splitting the shared confirmation and execution lifecycle.

The [unpushed-history update route](../skills/.domfiles-commit/references/update-unpushed-commits.md) is an explicitly selected mode, leaving ordinary `/commit` behavior focused on new commits. Its publication cutoff avoids rewriting shared history, regardless of whether a pull request exists. Fixups preserve the intended boundaries of earlier commits without forcing independently useful additions into them. Post-rebase validation assesses the resulting series rather than temporary fixups and replay states.

### Contribution Flow

The global [`contribution-flow` skill](../skills/.domfiles-contribution-flow/SKILL.md) coordinates preparation of contributions to repositories the user does not own. Its responsibility ends at readiness for the user’s manual browser submission, rather than publication or ongoing maintainer follow-up.

Contribution-level decisions, prewriting context collection, and consistency with previous submissions live in this workflow so `human-facing-writing` remains useful independently of remote retrieval. Existing-work assessment and reference relationships connect the contribution’s justification to its eventual prose. Personal reference preferences remain in this global workflow rather than changing the public writing skill’s defaults. Upstream checkpoints belong to the contribution workflow, while `commit` owns the mechanics and eligibility of history updates. That dependency is one-way and leaves the global authorization rules unchanged.

### Deferred Global Policy

Conditional global policy may move into a global skill when most sessions do not need it, following the [documentation principles](../skills/.domfiles-agent-documentation/SKILL.md#apply-the-documentation-principles). Eligibility depends on invocation mode. A model-invocable deferral requires a discrete trigger the agent can recognize without the deferred content and a safe default when discovery is missed. A command-only deferral requires a complete workflow that applies only when the user invokes its slash command. Conduct that applies continuously stays inline even when it is large.

The `Collaboration` policy is the standing example of what does not move. Its delegation rules shape how much work is done directly on every task rather than at one recognizable decision point, an agent that never loads them cannot notice that evidence has outgrown the main thread, and missing them drops the boundaries a subagent inherits.

`git-worktrees` is the first model-invocable deferral. Its former `Default` bullet was concurrent-work hygiene rather than worktree policy, so preserving existing changes and avoiding another agent’s write scope now lives in the global “Concurrent work” rule. Entry into an existing linked worktree is a discovery trigger so manually created checkouts receive the same guidance as agent-created ones. The global “Shared convention” rule retains only the `.agent-<name>` registration check required before reuse, movement, or deletion, and supported clients discover `git-worktrees` when that check identifies a worktree. The current-checkout rule names the skill so it cannot read as a prohibition on isolation. The worktree workflow remains global while it is tried in daily use, with client-specific lifecycle guidance kept out of the public relay skill.

### GitHub CLI Agent Integration

The public [`simple-github-cli` skill](../skills/simple-github-cli/SKILL.md) owns conditional agent behavior for `gh`. It carries the authentication and remote-mutation rules its workflow needs plus the [command-specific standalone handoff fallback](#agent-task-relay) for `gh agent-task create`, so the skill remains independently usable. Supported clients discover the skill from its description when a task calls for `gh` or direct GitHub work. The global [GitHub CLI policy](GLOBAL.md#github-cli) retains aligned domfiles-managed copies of the authentication and remote-mutation gates so those boundaries remain directly loaded across projects without repeating the route.

`gh agent-task` and the other non-simple families in [Opt-In Operations](../skills/simple-github-cli/SKILL.md#opt-in-operations) are never chosen without a direct user request. The boundary is scope-based rather than tied to preview status. User-requested external task handoffs use `agent-task-relay` for confirmation and assignment composition when it is available, while `simple-github-cli` owns the selected `gh` interface and terminal command delivery for `gh agent-task create` and task-bearing `gh copilot` invocations. `simple-github-cli` declares `agent-task-relay` through one entrypoint route and one bundled [optional-peer reference](../skills/simple-github-cli/references/optional-peer-agent-task-relay.md). `agent-task-relay` carries a generic workflow-owned delivery deferral, and the `simple-github-cli` agent-task fallback preserves standalone behavior without the peer.

### Global System-Available Tooling

The [global system-available tooling list](GLOBAL.md#system-available-tooling) covers non-standard supporting development commands that agents can invoke directly across projects. It mirrors the non-CI development dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../home/.local/bin/domfiles-sync-install), using executable names when package names differ and subject to the inclusions and omissions below.

The list also includes `cargo`, `fish`, `node`, `pnpm`, and `rustc` even though `domfiles-sync-install` classifies their Homebrew formulas as primary dependencies. `cargo` and `rustc` support package-oriented and direct Rust workflows, while `fish`, `node`, and `pnpm` support Fish configuration checks, JavaScript and direct TypeScript execution, and the preferred package-manager workflow, respectively.

The list intentionally omits `claude`, `codex`, `fisher`, `git`, `mole`, and `vim`. `claude` and `codex` are agent runtimes rather than supporting commands. `fisher` is Fish package plumbing. `git` is guaranteed by the [supported environment](#supported-environment) and governed separately. `mole` is a system-maintenance utility outside coding workflows. `vim` is an interactive editor.

`brew` is intentionally absent because it is a supported-environment prerequisite rather than a dependency installed by `domfiles sync`. Companion commands supplied by listed dependencies, including `corepack`, `fish_indent`, `npm`, `npx`, and `rustfmt`, are not listed separately because the list tracks primary tool interfaces rather than every available executable.

In shell sessions configured by `domfiles` after synchronization, direct invocation assumes repository-managed commands are available through `PATH` in addition to the [supported-environment](#supported-environment) prerequisites.

### Package Release-Note Skills

The global [`release-notes` overlay](../skills/.domfiles-release-notes/SKILL.md) is invoked explicitly as `/release-notes` and builds on the public [`release-notes-for-humans` skill](../skills/release-notes-for-humans/SKILL.md). Its `disable-model-invocation: true` frontmatter prevents automatic model invocation. The dependency remains one-way so the public skill stays independently installable.

The global overlay retains `*` through its [presentation conventions](../skills/.domfiles-release-notes/SKILL.md#presentation-conventions) because previously published notes use that marker. This keeps release notes produced through `/release-notes` consistent with earlier releases, even though Markdown accepts other unordered-list markers.

### Protected Skill Mutation

At Zed commit `dd04a229`, native mutation tools force confirmation when a directly named or canonical path contains consecutive `.agents` and `skills` components. Repository-root `AGENTS.md`, `.agents/PROJECT.md`, the root `skills` directory, and other `.agents` paths outside `skills` do not receive that agent-specific classification. Zed also requires the fixed `.agents/skills/<skill>/SKILL.md` layout for project skill discovery, so repository-internal skills retain that canonical location.

The public `skills/human-facing-writing` source does not receive Zed’s agent-specific classification. Its staging boundary applies to every agent because changes to its writing contract can affect every project-authored agent-documentation writing surface composed through it.

The [protected skill mutation policy](../skills/.domfiles-agent-documentation/references/protected-skill-mutation.md) owns the exact workflow. Its `.agents/skills` branch is limited to Zed Agent’s native permission model. Non-Zed writes to `.agents/skills` remain outside this policy, so the policy does not guarantee that they hide intermediate states from concurrent Zed sessions. The staging-host exception prevents a checkout whose name matches the task-directory convention from containing another task staging root.

### Repository Harmonization

The global [`harmonize` skill](../skills/.domfiles-harmonize/SKILL.md) is invoked explicitly as `/harmonize` and owns its change-oriented cross-repository consistency workflow. Its `disable-model-invocation: true` frontmatter prevents automatic model invocation.

### Skill Description Limit

The 1,024-byte figure in the [skill description policy](../skills/.domfiles-agent-documentation/SKILL.md#compose-the-change) is Zed’s limit rather than an intrinsic property of skill descriptions. Each client that receives the global skill set applies its own limit, so the figure requires revalidation whenever a supported client changes one.

### Skill Distribution

The [skill distribution contract](../AGENTS.md#skills) defines project-authored skill categories and installation surfaces. Every tracked skill remains subject to the repository’s public-disclosure boundary.

Authoring and maintenance records stay outside skill directories so installed guidance stays focused on use rather than the history of its creation. The [documentation principles](../skills/.domfiles-agent-documentation/SKILL.md#apply-the-documentation-principles) own this boundary.

[`skills/README.md`](../skills/README.md) targets visitors who install public skills without synchronizing the rest of this repository. Its top-level examples select user-wide installation to enact the README’s recommendation. Its featured `npx skills add … --skill …` examples intentionally preserve the form documented by skills.sh, including the omission of `--global`.

[`home/.local/bin/domfiles-sync-setup`](../home/.local/bin/domfiles-sync-setup) defines the exact source-to-destination mappings for globally exposed skills.

Documentation for global skills is maintained under the assumption that an installation exposing one global skill exposes the complete set. The skills form a complementary ecosystem on top of the same global instructions, allowing one skill to defer an overlapping domain to its canonical sibling instead of repeating fallback guidance.

The public [`posix-shell-scripting`](../skills/posix-shell-scripting/SKILL.md) and [`fish-shell-scripting`](../skills/fish-shell-scripting/SKILL.md) skills respectively own portable POSIX shell and Fish authoring, review, audit, diagnosis, and validation guidance. The repository-internal [`domfiles-shell-integration`](skills/domfiles-shell-integration/SKILL.md) skill retains domfiles-specific shell invariants and integration policy. General wording remains owned by [`human-facing-writing`](../skills/human-facing-writing/SKILL.md), keeping shell semantics separate from editorial guidance.

The public `human-facing-writing` skill applies its [Writing Principles](../skills/human-facing-writing/SKILL.md#writing-principles) standard to every task, then routes connected prose and technical copy to separate references, giving overlapping work one precedence contract while preserving a complete nontechnical path. The global **Numbering** rule exists for Zed-specific behavior, remains owned by [`.agents/GLOBAL.md`](GLOBAL.md#writing), and is intentionally excluded from the public typography contract. Synchronization removes the obsolete managed symlinks rather than retaining aliases, so clients discover the merged skill once.

During source authoring, the `agent-documentation` workflow composes every project-authored agent-documentation writing surface and all human-facing writing in its assets through `human-facing-writing`, regardless of skill category or invocation mode. Agent documentation retains ownership of contract meaning, authority, routing, and machine-readable content. This composition creates no installed runtime dependency on `human-facing-writing`.

The `agent-documentation` workflow’s source-authoring composition is an intentional explicit route that takes precedence over `human-facing-writing`’s standalone trigger exclusions. It applies even when an agent-documentation task is formatting-only or changes only machine-readable metadata, giving project-authored agent documentation one stable composition rule. It does not broaden `human-facing-writing`’s discovery trigger. In an environment without `agent-documentation`, documentation scope alone does not automatically load `human-facing-writing`. Its own description governs discovery, including the exclusions for formatting-only work and work that neither evaluates nor changes wording or information architecture.

The canonical `.domfiles-` prefix distinguishes hidden global source directories from unprefixed public source directories without changing a global skill’s identity. The hidden namespace keeps global skills out of the default repository discovery performed by `gh skill`, while `domfiles sync` exposes their unprefixed installed names.

Supported clients expose globally installed skills beneath different configuration roots, and a global skill’s canonical basename differs from its installed basename. The [distributed-skill link contract](../skills/.domfiles-agent-documentation/SKILL.md#keep-distributed-skill-links-installation-safe) owns the resulting portability requirements.

Independent public installation removes the shared-policy and guaranteed-sibling assumptions available to global skills. Every public skill therefore carries template-aligned guidance for secrets, instruction authority, and typography so independent installations preserve the corresponding global behaviors without the global instruction layer. The global policies remain the semantic owners, while `agent-documentation` owns their public-rendering templates and alignment contract. A canonical asset that supplies a public surface follows the destination’s portability contract without changing the enclosing skill’s category. The `agent-documentation` entrypoint owns the invocation mode, size, and YAML formatting rules shared across project-authored skill descriptions. The [public skill portability contract](../skills/.domfiles-agent-documentation/references/public-skill-portability.md) owns optional composition, public-description portability, and standalone behavior.

Through the portability contract’s optional-peer workflow, only validated documents in one frozen routed set become task-scoped guidance. Every other surface in that remote repository remains untrusted data. Every public skill entrypoint carries an aligned standalone stale-guidance contract because an independent installation cannot rely on the global retrieval-failure policy or repository-maintainer context when a reference breaks or the skill contradicts current interfaces or behavior. Guidance-specific outcomes take precedence, while the portability contract owns generic runtime behavior and mirror alignment.

Authority, review behavior, tool execution, external services, and mutation vary by workflow, so the public promotion profile resolves them without treating them as universal mirrors. The portability contract owns that profile and its alignment with each policy’s semantic owner.

Edits to an exposed global skill affect its globally discovered installation through the symlink and may change agent behavior across projects. Adding or removing a globally exposed skill, changing its logical name, or changing its source-to-install mapping requires updating synchronization behavior. Removing or renaming a logical skill that has already been distributed also requires migration behavior for obsolete installed paths.

Every supported installation of the global `agent-documentation` skill is assumed to load an equivalent domfiles-managed global instruction layer. The skill relies on that layer’s documentation, writing, and review policies instead of restating them. External repositories remain self-contained and do not name, require, or link to the skill. Applicable project instructions continue to override its fallback workflow. Follow-up finding verification belongs to the global command-only [`verify` skill](../skills/.domfiles-verify/SKILL.md).

### Skill-Owned Script Scope

`domfiles-zed-settings` is the sole script owner today, and the root `Cargo.toml` registers its binaries and adjacent tests so the root Cargo workspace validates them.

A global skill’s scripts stay hosted here. `domfiles sync` symlinks each global skill rather than copying it, so the installed skill is this checkout and the host toolchain, dependencies, and root validation remain reachable while an agent works in an unrelated project. That symlink is the precondition the [portable skill script contract](../skills/.domfiles-agent-documentation/references/portable-skill-scripts.md) depends on, and it is why those scripts take every separate project they inspect or change as an explicitly selected target instead of resolving one from their installed path.

Agent script tests are not excluded from the repository’s test workflow. Collecting a TypeScript agent script test would additionally require a Vitest project entry covering the skill tree, which waits until the first such script exists.

The [smallest sufficient contract](../skills/.domfiles-agent-documentation/references/skill-owned-scripts.md#design-the-smallest-sufficient-contract) gate challenges necessity before correctness. Adversarial design review runs before implementation and remains bounded to declared consumers, evidence, and the operating model, so it removes unsupported contract elements instead of hardening a script around speculative requirements.

### Verify Skill

The global [`verify` skill](../skills/.domfiles-verify/SKILL.md) is invoked explicitly as `/verify` and owns rechecking previously reported findings against the current state. Its `disable-model-invocation: true` frontmatter prevents automatic model invocation. Keeping this follow-up procedure out of the global instructions avoids loading it in sessions that do not request verification.

### Version-Sensitive Agent Documentation

Version-sensitive agent documentation uses one authoritative upstream baseline because current documentation, pinned source, and upstream `main` can describe different implementations. The canonical [agent-documentation workflow](../skills/.domfiles-agent-documentation/SKILL.md#compose-the-change) resolves conflicts against that baseline before editing and ties security-boundary claims to exact implementation evidence.

### Zed Selection-to-New-Thread Key Binding

The `ctrl-enter` binding in `home/.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## Synchronization

### Synchronization Checkout State

`__domfiles_is_clean` intentionally compares the tracked working tree with the index and the index with `HEAD`. This keeps index stat metadata alone from making the checkout appear dirty. Untracked files do not affect the result, and paths marked with `git update-index --assume-unchanged` remain excluded so intentional local overrides are respected. This predicate governs synchronization warnings and dependency reconciliation. Repository-update safety handles assume-unchanged entries separately.

Repository updates are skipped when the checkout contains entries marked by `git update-index --assume-unchanged`. While those entries are present, synchronization avoids rebases and hard resets because Git may overwrite their working tree contents.

### Synchronization Workflow

`domfiles sync` is the repository’s canonical update path. It intentionally establishes the repository-managed state, including replacing the initial contents of managed paths. That replacement is expected synchronization behavior rather than accidental data loss.

Synchronization links the repository’s `home/.local/bin` directory to `~/.local/bin`. It does not link `home/.local/share/domlib` into the user’s home because each command resolves its real path before sourcing `../share/domlib` from the repository.

`domfiles sync` is a best-effort workflow that prioritizes completing as much independent work as possible with minimal interruption. An individual failure is recoverable only when the main workflow or a sync stage handles it explicitly, surfaces the result, and can continue later work independently of the failed operation. Source control flow defines the exact recoverable cases.

The workflow can complete with visible, explicitly handled failures. An unhandled error or a nonzero exit from a sync stage stops the broader workflow.

The final dependency status is advisory. Its result remains visible while synchronization continues to completion.

## Tooling

### Claude Code Distribution

`claude` is intentionally installed through Homebrew’s `claude-code` cask rather than declared as an `@anthropic-ai/claude-code` project dependency. This keeps the CLI machine-level, follows Anthropic’s stable Homebrew channel, and excludes it from dependency installation in CI because `claude` is a development Homebrew dependency. The Homebrew CLI installation is separate from the `claude-acp` registry package managed by Zed.

### Codex Distribution

`codex` is intentionally installed through Homebrew rather than declared as an `@openai/codex` project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Cross-Shell Helper Differences

Accepted shell-specific contract differences between paired `domlib` and Fish helpers are recorded here with their rationale:

- **Command routing:** POSIX `__` routes `brew` and `pnpm` through `domlib` wrappers that add fallback or search paths, environment overrides, and custom missing-command diagnostics. Fish `__domfiles_print_and_run` invokes the requested external command directly. Fish startup establishes the supported command paths, and the generic wrapper intentionally adds no per-command routing, environment, or diagnostics.
- **Failure handling:** Argument-validation failures in POSIX `__`, `__confirm`, and `__is_boolean` terminate the running shell. Their Fish peers report the error and return status 1 because terminating from an autoloaded function would close the interactive shell.
- **Quoting:** POSIX `__print_command` renders arguments with Python `shlex.quote`, while Fish `__domfiles_print_command` uses `string escape`. Each produces syntax for its own shell, so equivalent commands do not require byte-identical display text.
- **Suppression lifecycle:** `domlib` normalizes `DOMFILES_SUPPRESSED` once when loaded. Fish `__domfiles_print_command` reads and validates the current value for every command because `home/.config/fish/config.fish` intentionally does not initialize it. An unsupported value therefore fails when `domlib` loads or when Fish attempts to print a command. Fish has no `__suppress` peer because a single-command variable override can limit suppression to one function invocation.

### Dependency Status Labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell-script policy](skills/domfiles-shell-integration/SKILL.md#check-supported-environment-compatibility) owns the row-inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

The `rust` row reports whether both `cargo` and `rustc` are available, matching the managed Homebrew formula rather than either executable name.

`vim` is intentionally omitted from the checklist even though synchronization installs it as a primary Homebrew dependency. Its availability does not affect the command’s output or exit status.

### Development Lint Wrapper Architecture

The language-specific `home/.local/bin/domfiles-dev-lint-*` entrypoints retain their own default scopes and lint commands while sharing discovery and execution through `domlib`. This preserves stable interfaces for pnpm, staged linting, language-specific CI, and targeted agent validation without duplicating the execution pipeline.

Default discovery intentionally uses line-delimited `git ls-files` output. This lets POSIX `sh` preserve discovery failures and call the in-process lint callbacks without temporary files or another language parser. Git can C-quote control characters and, when `core.quotePath` is enabled, non-ASCII bytes. A quoted pathname is skipped because it does not resolve to the original file, so pass that path explicitly when linting it.

### Domlib Helper Documentation

Every `domlib` function has one adjacent contract comment. The uniform surface lets readers compare helpers without reconstructing shell bodies. Comment prose wraps at 80 columns while preserving ordinary sentence flow, so a wrapped line remains a continuation rather than a separate statement. Internal periods may separate sentences, while terminal punctuation remains omitted under the shell prose policy.

Comments describe the semantic contract domfiles adds. They omit ordinary behavior already implied by a command-shaped name, implementation values canonically owned by source, validation and fallback details, and cross-cutting policy owned elsewhere unless the omission would make the contract misleading. The `__touch` comment therefore emphasizes file existence, standard permissions, and parent creation while timestamp updates remain implied by `touch`. The `__print_command` and `__suppress` comments leave the CI exception to [suppressed command output](#suppressed-command-output), which canonically owns that policy.

In helper comments, domfiles is an unformatted plural noun parallel to “dotfiles” when it denotes the repository or managed configuration, while `domfiles` is code-formatted only when it denotes the CLI command. The phrase “domfiles have …” is therefore intentional. The postpositive modifiers in “heading, dimmed” and “text, formatted” preserve the shared base description across related helpers rather than introducing separate terminology for each variant.

`__is_brew_installed` intentionally owns both the no-argument Homebrew installation check and the optional package check. Repeating “returns success” makes the result of each branch explicit. `__git_skipped_files` intentionally describes semantic skipped files while preserving tagged `git ls-files -v` entries because `git-skipped` owns display-path extraction and its other callers only test whether output exists. `__git_diff_list_changed_excluded_paths` lets `--commit` and `--worktree` stand for their complete modes, with the commit reference implied by the `--commit` context.

`__ssh_add`’s comment intentionally relies on the command-shaped name for ordinary success and failure semantics. The helper reports failures before returning nonzero, allowing `domfiles-sync` to tolerate the status without silencing the diagnostic.

The `__symlink` comment states the normal replacement contract and omits source-containment rejection because that rejection is a safety precondition rather than an alternate supported outcome. The helper creates the complete missing destination-parent chain through `__mkdir` and `mkdir -p`. Standard permissions apply to the final parent passed to `__mkdir`, while any ancestors created by `mkdir -p` retain their ordinary creation modes.

### FFmpeg Media Preset Compatibility

Every supplied input and generated output media format, dimension, duration, and other size constraint in `home/.config/fish/functions/ffmpeg-wav-png.fish` is an accepted platform-compatibility constraint for current and future presets. Their compatibility is an accepted project premise rather than an independently verified property.

Each preset owns a complete conversion branch. The repeated discovery loop, image pairing, and output naming across those branches are intentional. Consolidating them into one shared pipeline is a non-goal, so every preset’s container, filter chain, codec options, and constraints stay independent.

`ffmpeg` is an intentionally unmanaged optional runtime dependency for this command. Its availability check defines the supported failure behavior, and bootstrap and synchronization intentionally do not provision it.

The Instagram branch intentionally combines `-t 60` and `-shortest` so output ends at 60 seconds or when shorter audio ends. The hard cap takes precedence over preserving a stream-copied audio packet that crosses the limit.

### Fish Abbreviation Ownership

The managed Fish configuration intentionally erases every existing abbreviation before defining its own set. This keeps abbreviation state deterministic across machines and removes stale universal abbreviations. Abbreviations defined outside domfiles are not preserved across shell startup.

### Fish `clone` Argument Contract

The [`clone`](../home/.config/fish/functions/clone.fish) helper intentionally supports only `clone <repository>` and `clone <repository> <directory>`. It neither parses nor rejects Git options. Use `git clone` directly for option-bearing invocations. An unsupported invocation can reach Git without a reliable follow-up directory change, which is an accepted consequence of keeping the wrapper simple.

For the supported one-argument form, follow-up target derivation intentionally covers only common remote URLs and ordinary local paths. Full parity with Git’s destination naming is a non-goal, including sources addressed through an inner `.git` directory.

### Fish Interactive Configuration

Tracked aliases and colors load only during interactive Fish sessions. `home/.config/fish/config.fish` keeps both sources inside its `status is-interactive` guard so noninteractive Fish invocations do not inherit interactive-only aliases or color configuration.

### Fish Local Configuration

`home/.config/fish/local.fish` is active machine-local Fish configuration when present. Fish sources it through `home/.config/fish/config.fish` during startup without redirecting standard output or standard error.

A bare Fish interpreter invocation can therefore execute machine-local configuration outside the requested command and emit its output. The [global tooling guidance](GLOBAL.md#system-available-tooling) defaults agent invocations to `fish --no-config` unless Fish startup configuration or configured runtime behavior is in scope.

### Git Diff Presentation

The lockfile-aware presentation in `git-d` and `git-view` is consolidated because it forms a substantial shared pipeline whose behavior must remain aligned.

`git-view` intentionally bypasses that split presentation for merge commits that change an excluded lockfile. Git’s native `-m` output keeps every patch within its parent-qualified section, which takes precedence over suppressing lockfile patches.

### Git Fixup Amend Behavior

With `--amend`, `git f` compares its inferred or positional fixup target with the commit currently at `HEAD`. When both resolve to the same commit, it preserves an existing `amend!`, `fixup!`, or `squash!` message with `--no-edit`. For any other subject, it moves the target to the first parent when one exists. If no first parent exists, it leaves the target at `HEAD` so Git can amend a root commit.

### Git Log Search Coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is intentional in favor of implementation simplicity.

### Git Short Status Command

`git s` is a purpose-built view that combines root-relative, short `git status` output with tracked files marked `--assume-unchanged`. It is not an alias for or drop-in replacement for `git status`. It accepts pathspecs with an optional leading `--`. Status options and alternate output formats remain the responsibility of `git status` rather than `git s`.

### Peer Dependency Versions

Every peer dependency in workspace packages intentionally uses the version `"*"`. The workspace catalog, root dependency declarations, and lockfile maintain the concrete compatible versions, so repeating version constraints in individual workspace packages would duplicate the same policy. These ranges are complete declarations rather than missing compatibility constraints and are not intended to mirror the currently resolved version.

### Prettier Formatter Command

[`domfiles-format`](../home/.local/bin/domfiles-format) accepts existing file paths, resolves symlinks, and lists each destination once with `$HOME` abbreviated to `~` before asking for confirmation through `__confirm`. It has no recursive mode or `--write` option. Resolved paths containing control characters are unsupported so the numbered confirmation list stays unambiguous. After confirmation, a non-writing Prettier check runs across the complete selection to catch formatting errors before any writes. Writes follow Prettier’s normal per-file behavior rather than a batch transaction.

Formatting runs from the domfiles checkout with its installed Prettier, explicit configuration, and full plugin set. Target-side Prettier configuration, `.editorconfig`, and ignore files do not supply formatting policy. Prettier’s built-in exclusions still apply. The caller’s relative paths are resolved before changing directories, which also keeps native formatter configuration discovery rooted in domfiles. The command uses the existing workspace installation and does not install dependencies automatically.

### Prettier Formatter Wrappers

`prettier-plugin-fish`, `prettier-plugin-rust`, and `prettier-plugin-toml` are intentionally thin whole-file wrappers around Homebrew-provided `fish_indent`, `rustfmt`, and `taplo`, respectively. Each native formatter’s output is preserved verbatim, and that formatter owns its language’s formatting semantics. Prettier options such as `tabWidth` and `useTabs` intentionally do not affect their output. The Fish and Rust wrappers declare the `fish` and `rust-script` interpreters so Prettier infers their parsers for extensionless files with matching hashbangs.

Markdown files directly under `skills/posix-shell-scripting/references/` use a two-space Prettier indentation override so embedded `sh` examples follow the skill’s POSIX indentation convention. The override intentionally excludes `skills/posix-shell-scripting/SKILL.md` because applying the same indentation setting to the whole file would reindent its YAML frontmatter. The skill’s example-bearing guidance is therefore structured as references, while `SKILL.md` retains the rules and routes needed by every invocation.

The Rust wrapper invokes `rustfmt --edition 2024 --emit stdout`. The explicit edition is required because direct stdin formatting otherwise defaults to Rust 2015. Native `rustfmt` defaults own all remaining Rust formatting policy, so the repository intentionally has no `rustfmt.toml` and exposes no duplicate Prettier options. The TOML wrapper invokes `taplo fmt -` and likewise relies on the native formatter’s defaults, so the repository has no Taplo configuration or duplicate Prettier options. Homebrew’s `fish`, `rust`, and `taplo` formulas provision all three native formatters, while `rustup` and `rust-analyzer` are intentionally unmanaged.

Partial `rangeStart` and `rangeEnd` formatting is intentionally unsupported. None of the native formatters has a range API, and Prettier’s range calculation does not recognize custom parser names, so partial range requests leave the source unchanged. Prettier’s standalone mode is also intentionally unsupported because these wrappers require a Node.js process to execute their external formatter binaries.

Prettier pragma comments—including `@format`, `@prettier`, `@noformat`, and `@noprettier`—are intentionally unsupported. The wrappers omit `hasPragma`, `hasIgnorePragma`, and `insertPragma`, so `requirePragma` and `checkIgnorePragma` do not gate formatting and `insertPragma` does not add a pragma.

Interior cursor mapping is intentionally omitted. The wrappers expose a single whole-file AST node because the native formatters provide neither token locations nor source maps. End-of-input cursor positions remain supported, but interior cursors may not remain attached to the same token after formatting. The wrappers do not implement heuristic source-to-output mapping.

Each `expectTypeOf(plugin).toExtend<Plugin>()` assertion intentionally serves as a forward-compatibility sentinel for Prettier’s plugin contract. It is not intended to prove that currently optional exports exist. Behavioral formatting tests cover the operational `languages`, `parsers`, and `printers` exports. The assertion’s forward-compatibility value remains despite the current `Plugin` properties being optional.

### Repository-Scoped Commands

`plugins` and `skills` intentionally remain in the root `dependencies`. They provide agent-facing or user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

`domfiles-sync-update` intentionally does not invoke `plugins update` because the current CLI treats unknown subcommands as plugin source paths, so the command can exit successfully without updating anything. This decision can be revisited if upstream adds a supported update workflow.

The corresponding scripts in `home/.local/bin/` are the stable command interfaces. They resolve implementations from the domfiles pnpm workspace without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

pnpm 12 persists an exact `packageManager` pin at major 12 or newer in a leading environment document in `pnpm-lock.yaml`. With the default `pmOnFail: download`, every command reconciles its `packageManagerDependencies`, including version output. A frozen install fails when this document is missing or stale instead of updating it. The `packageManager` field and both lockfile documents therefore change together during a pnpm major upgrade.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout-state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `warn`, which reports outdated dependencies and runs the command without installing them. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

### Ripgrep Configuration Isolation

`rg` reads `RIPGREP_CONFIG_PATH` before parsing arguments, and a configuration file can supply `--pre`, which runs another program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](GLOBAL.md#system-available-tooling) requires `--no-config` on every agent invocation.

### String Helper Reuse

The `__string_*` helpers are optional conveniences rather than a mandatory abstraction boundary.

### Suppressed Command Output

`DOMFILES_SUPPRESSED` suppresses the `$ …` command echo emitted by the paired `__print_command` and `__domfiles_print_command` helpers. It defaults to `false`. `domlib` and Fish normalize user-supplied values through the paired `__read_boolean_from_env` and `__domfiles_read_boolean_from_env` helpers, then only normalized `true` enables suppression. Gating the command-printing helpers covers Fish `__domfiles_print_and_run` and every POSIX caller: `__`, and therefore `__chmod`, `__mkdir`, `__touch`, and `__symlink`, plus `__ssh_add` and `__domfiles_exec --print`. Only the echo is suppressed, so a wrapped command’s own output, headings, confirmations, and errors continue to print.

`__is_ci` and `__domfiles_is_ci` override suppression, so automated runs keep the complete command trace regardless of `DOMFILES_SUPPRESSED`. A CI log is the only record of what a run executed and has no interactive reader to spare, so suppression there would remove diagnostic value without providing the benefit it exists for.

`__suppress` overrides `DOMFILES_SUPPRESSED` only inside its own subshell. The variable is runtime control state that Fish configuration intentionally does not initialize. The [`domlib` maintenance policy](skills/domfiles-shell-integration/references/domlib-integration.md#maintain-domlib) requires `domlib` counterparts for Fish-defined `$DOMFILES_*` variables but permits variables defined only in `domlib`, including `DOMFILES_SUPPRESSED`.

A Fish counterpart remains unwanted. Fish does not export `set -g`, which every `DOMFILES_*` entry in Fish configuration uses, so a counterpart in the established form would have no effect on `domlib`, while `set -gx` or `set -x` would suppress command echo for every domfiles command in the session.

An exported value reaches every child script, so `DOMFILES_SUPPRESSED=true domfiles sync` covers an entire synchronization run. `__suppress` applies the same suppression to one command by exporting the variable inside a subshell, which is how `domfiles-sync-setup` keeps the agent-skill linking loop from echoing without affecting later synchronization steps.

That loop intentionally confirms the source skill directory rather than the two destinations it replaces. One source is the unit of work, both destination roots are fixed, and `__symlink` removes and recreates each destination on every run, so naming them would report routine churn rather than the artifact being distributed. The removals stay in the CI trace through `__suppress`.

That subshell is also why `__suppress` rejects `__domfiles_exec`. It would absorb that function’s `exec`, letting the caller resume and run the remainder of `domfiles-sync` a second time. The echo there is suppressed by omitting the opt-in `--print` flag instead.

The prefix form `DOMFILES_SUPPRESSED=true __symlink …` is intentionally unused. POSIX leaves it unspecified whether a variable assignment preceding a function call persists after that function returns, and macOS `/bin/sh` is Bash 3.2 in POSIX mode, where it does persist and suppresses the remainder of the script.

No standardized environment variable covers command-echo suppression. `NO_COLOR` and `DO_NOT_TRACK` address color and telemetry only, so this name follows the prefixed convention of `HOMEBREW_NO_*` rather than an unprefixed `SUPPRESSED`, which any unrelated exported value in the invoking shell could set.
