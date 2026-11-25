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

T3 Code manages the Antigravity CLI and runs it with an isolated profile. The profile links selected rules and skills from `~/.gemini/`, including the locations used by domfiles below.

`claude` and `codex` are intentionally installed through Homebrew’s `claude-code@latest` and `codex` casks rather than declared as `@anthropic-ai/claude-code` or `@openai/codex` project dependencies. Both CLIs remain machine-level development Homebrew dependencies, so dependency installation in CI excludes them. Claude Code follows Anthropic’s latest release channel. Homebrew’s unversioned `claude-code` cask tracks the stable channel. The Homebrew CLI installations are separate from the agent packages Zed manages, such as the `claude-acp` registry package.

### Agent Discovery

[`.agents/GLOBAL.md`](GLOBAL.md) is the shared global instruction source. [`domfiles sync`](../home/.local/bin/domfiles-sync-setup) exposes canonical instructions, skills, and preferences through each client’s discovery locations without duplicating content.

| Surface | Canonical Source | Client Location | Mechanism |
| --- | --- | --- | --- |
| Antigravity global instructions | `.agents/GLOBAL.md` | `~/.gemini/AGENTS.md` | Synchronization symlink to the [bridge](ANTIGRAVITY.md), which tells Antigravity to read the canonical file through the `~/.agents/GLOBAL.md` symlink |
| Antigravity project instructions and skills | [`AGENTS.md`](../AGENTS.md) and `.agents/skills/` | Repository root | Native workspace discovery |
| Claude global instructions | `.agents/GLOBAL.md` | `~/.claude/CLAUDE.md` | Synchronization symlink |
| Claude project instructions | [`AGENTS.md`](../AGENTS.md) | Repository `AGENTS.md` | Direct loading by Claude Code and Zed’s Claude Agent in the default project instruction mode |
| Claude project skills | `.agents/skills/` | [`.claude/skills`](../.claude/skills) | Tracked directory symlink |
| Claude settings | [`home/.claude/settings.json`](../home/.claude/settings.json) | `~/.claude/settings.json` | Synchronization symlink |
| Codex global instructions | `.agents/GLOBAL.md` | `~/.codex/AGENTS.md` | Synchronization symlink |
| Globally exposed skills | [Globally exposed categories](../skills/.domfiles-skill-development/references/domfiles-skill-policy.md#classify-skills) under `skills/` | `~/.agents/skills/<skill-name>`, `~/.claude/skills/<skill-name>`, and `~/.gemini/config/skills/<skill-name>` | Synchronization symlinks using installed names |
| Zed global instructions | `.agents/GLOBAL.md` | `~/.config/zed/AGENTS.md` | Tracked [`home/.config/zed/AGENTS.md`](../home/.config/zed/AGENTS.md) bridge through the managed `~/.config` link |

Antigravity’s [24,000-byte rule limit](https://antigravity.google/docs/rules#size-limits-and-token-budgets) applies after includes expand. Reading the global instructions separately avoids truncation and may [require approval](https://antigravity.google/docs/permissions?tab=cli) when the file is outside the workspace.

None of the `skills/` source namespaces is a project-local discovery surface. Project discovery remains backed by `.agents/skills/*`. Hidden source directories keep global skills, including personal overlays, out of `gh skill`’s default repository discovery. Client-specific roots and differing source and installed names motivate the [distributed skill link contract](../skills/.domfiles-skill-development/references/skill-installation.md#distributed-links).

[`domfiles-sync-migrate`](../home/.local/bin/domfiles-sync-migrate) retains retired-name cleanup rather than compatibility aliases so future synchronization leaves each logical skill discoverable once.

### Claude Agent Integration

The [Claude user settings](../home/.claude/settings.json) are shared, non-secret preferences managed as mutable public configuration. Claude Code [writes configuration updates to this user settings path](https://code.claude.com/docs/en/settings), exposed through [agent discovery](#agent-discovery).

The [`claude-acp` registry entry](../home/.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, native permission system, sandbox, and configuration. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude’s remaining runtime state under `~/.claude`, together with `~/.claude.json`, remains machine-local outside the repository.

### Global System-Available Tooling

The [global system-available tooling list](GLOBAL.md#system-available-tooling) names non-standard commands that agents use for development work across projects. The list draws from the dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../home/.local/bin/domfiles-sync-install). It names each tool’s primary executable interfaces rather than every executable a package supplies. Agent runtimes, plugin managers, interactive editors, and system maintenance utilities stay off the list, as do Git and Homebrew, which the [supported environment](#supported-environment) guarantees.

### Zed Selection-to-New-Thread Key Binding

The `ctrl-enter` binding in `home/.config/zed/keymap.json` uses `workspace::SendKeystrokes` because Zed exposes separate actions for creating an agent thread and adding the active selection, but no single action that combines them. The `cmd-? cmd-n cmd-? cmd->` sequence is intentional: it focuses the agent panel, creates a new thread, returns focus to the selected editor text, then invokes `agent::AddSelectionToThread`, which refocuses the panel and inserts the reference. The focus round-trip preserves the source context and adds dispatch yields around asynchronous thread creation.

## Intentional Workflow Stops

For tasks that change or evaluate workflow interruptions, the [Intentional Workflow Stops](project/intentional-workflow-stops.md) reference document records why particular stops were retained and any conditions for reconsidering them. It is in a separate file so other tasks can skip it.

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

### Package Release Note Skills

The [`dom-release-notes-for-humans` overlay](../skills/.dom-release-notes-for-humans/references/release-note-defaults.md#presentation-conventions) retains `*` as its unordered list marker to stay consistent with previously published release notes.

### Shared Skill Contracts

The [Shared Skill Contracts](project/shared-skill-contracts.md) reference document links to related guidance for tasks that change or evaluate a global rule or a shared skill contract. It is in a separate file so other tasks can skip it.

### Skill Catalogs

[`skills/README.md`](../skills/README.md) targets visitors installing public skills without synchronizing the repository. Its examples use user-wide installation, as it recommends. The root [`README.md`](../README.md) and individual skill READMEs intentionally use minimum viable `gh skill install` and `npx skills add` commands, omitting scope flags to keep the focus on the skills.

### Skill Description Limit

The [skill description policy](../skills/.domfiles-skill-development/references/skill-descriptions.md#encoding-and-size) uses a conservative UTF-8 byte cap rather than relying on a client’s character counting. It is an authoring constraint distinct from the [Agent Skills specification’s character limit](https://agentskills.io/specification#description-field).

### Skill Security Scans

The [`human-facing-writing`](../skills/human-facing-writing/SKILL.md) description and its [GitHub Collaboration route](../skills/human-facing-writing/SKILL.md#specialized-routes) intentionally frame GitHub material as copy within the authorized writing unit to avoid Snyk Agent Scan’s [W011 warning](https://github.com/snyk/agent-scan/blob/main/docs/issue-codes.md#W011) on [skills.sh](https://www.skills.sh/porada/domfiles). This framing preserves the skill’s applicability, but its effect on the scan remains unverified.

### Subagent Review Corrections

In a sample of 30 review runs, 27 had follow-up reports. Corrections accounted for 156 of the 204 follow-up findings by introducing a defect or leaving an earlier finding unresolved. By position in a reviewer’s sequence, the share of follow-up reports with at least one new finding was 27/54 for the first report, 16/41 for the second, 13/24 for the third, 10/17 for the fourth, and 29/65 for the fifth and later reports. New findings persisted across the sequence, supporting the same depth of [correction verification](../skills/subagent-review-flow/SKILL.md#coordinate-fixes-and-re-review) throughout a review.

A retrospective test recorded where two consecutive qualifying reports from one reviewer would have triggered the [stall rule](../skills/subagent-review-flow/SKILL.md#apply-the-stall-rule) in this sample. It used the 149 attributed findings that coordinators later corrected as a proxy for validated findings. Each run had two reviewers, and run length was their combined number of follow-up reports assessing a correction. In all seven runs with ten or more such reports, the rule fired by a reviewer’s fourth follow-up report. It fired by that point in three of the other 20 runs with follow-ups. Counting all 156 attributed findings gave the same result. Both counts excluded findings of uncertain origin. Runs with few follow-up reports had fewer opportunities to trigger the rule, and the test examined only this threshold.

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

The [Shell Integration](project/shell-integration.md) reference document records design decisions and constraints for tasks that change or evaluate the repository’s shell code or command entrypoints. It is in a separate file so other tasks can skip it.

## Tooling

### Fish Prettier Plugin

The Fish Prettier plugin intentionally exposes a whole-file native formatter with these boundaries:

- **Cursor mapping:** End-of-input cursor positions are supported. Interior positions may move between tokens because the native formatter supplies no source map, and the wrapper adds no heuristic mapping.
- **Formatter output:** The plugin runs `fish_indent`, inserts an empty line after a shebang and any `#!` lines right below it when code or a comment follows them directly, and reflows eligible full-line comments to target `printWidth`. Trailing comments retain their native formatting. `fish_indent` controls indentation regardless of `tabWidth` and `useTabs`.
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

The corresponding scripts in `home/.local/bin/` are the stable command interfaces. They resolve implementations and dependencies from the domfiles checkout containing the launcher’s resolved location without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

The [checkout selection procedure](../AGENTS.md#checkout-command-selection) exercises worktree launchers without changing the provisioned installation or introducing another command interface.

### ripgrep Configuration Isolation

Without `--no-config`, `rg` combines its command line arguments with those in the file selected by `RIPGREP_CONFIG_PATH`. A configuration file can supply `--pre`, which runs another program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](GLOBAL.md#system-available-tooling) requires `--no-config` on every agent invocation.

### Worktree Location

Zed rejects absolute `git.worktree_directory` values and requires the resolved directory to stay within the repository or its parent, so the shared root lives in `~/Projects`. For repositories directly under `~/Projects`, the [Zed setting](../home/.config/zed/settings.json) places new worktrees in a per-repository directory under `~/Projects/.worktrees`. Repositories elsewhere resolve it relative to their main checkout.

### Zed CLI Open Behavior

`cli_default_open_behavior` remains explicit in [the user settings](../home/.config/zed/settings.json) to avoid repeating [CLI open behavior setup](https://github.com/zed-industries/zed/blob/v1.21.0/crates/zed/src/zed/open_listener.rs#L743-L768). When the setting is absent and a CLI request reaches that setup, Zed prompts for the preferred behavior and writes the selected value back to the user settings file. The [**Zed settings**](../AGENTS.md#general) policy owns the redundancy criterion.
