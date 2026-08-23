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

### GitHub CLI authentication boundary

`gh` is provisioned as a supporting agent command, but authentication remains machine-local and user-managed. The supported setup targets `github.com` with credentials stored in the operating system credential store.

GitHub CLI can fall back to storing a token in plaintext when secure credential storage is unavailable. That fallback is outside the supported boundary for agent use.

### Permission pattern length bound

The 1,000-scalar cap on decoded permission patterns in the [terminal permission policy](skills/domfiles-zed-settings/references/terminal-permissions.md#apply-the-terminal-permission-policy) is a self-imposed reviewability bound rather than a Zed or regex-engine constraint. It tracks no external limit and changes only by decision. The `domfiles-zed-settings-permission-owner-audit` binary enforces the same bound, so the policy figure and that enforcement change together.

### Ripgrep configuration isolation

`rg` reads `RIPGREP_CONFIG_PATH` before parsing arguments, and a configuration file can supply `--pre`, which runs an arbitrary program against every searched file. A bare invocation is therefore an execution surface rather than a read-only search, so the [global tooling guidance](../.config/zed/AGENTS.md#system-available-tooling) requires `--no-config` on every agent invocation.

[Zed settings](../.config/zed/settings.json) enforce the same boundary independently, because every ripgrep search allowance requires the literal `--no-config` token and unflagged invocations fall through to the terminal’s confirm-by-default boundary. The permission layer matches command text and cannot observe the environment, so requiring the flag is the only reliable way to establish that no configuration file participates.

### Zed agent permission model

Agent tool permissions intentionally use an allow-by-default baseline. The terminal tool overrides that baseline with confirm-by-default behavior, using explicit allowances for accepted forms and confirmation overrides for hazardous forms.

### Zed agent-directory allowance scope

Project-relative task-owned `.agent-<name>` directories are a standing user-approved namespace for operation-specific terminal-allowance variants. This lets permission work evaluate a scoped variant without asking the user to reapprove the namespace for each command family.

The namespace neither authorizes a command family nor changes effective permissions on its own. The [agent-directory allowance policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#apply-the-agent-directory-allowance-policy) owns exact eligibility and preserves confirmation or denial for effects that path scope cannot contain, while [Zed settings](../.config/zed/settings.json) remain canonical for configured behavior.

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

### Zed Cargo target directory

`CARGO_TARGET_DIR` has an automatic terminal allowance only when it names the exact project-relative `.agent-<name>/target` directory and precedes an otherwise allowed Cargo form. This operation-specific use of the [agent-directory allowance scope](#zed-agent-directory-allowance-scope) confines generated artifacts to task-owned state without authorizing a Cargo command family or broadening its accepted options.

Operational `cargo clean` and nightly or experimental execution remain confirmable when prefixed by the assignment. Credential-denial patterns intentionally accept any literal no-space `CARGO_TARGET_DIR` value so an unsupported target cannot turn a denied Cargo form into an approvable one. [Zed settings](../.config/zed/settings.json) remain canonical for exact command grammar.

### Zed command discovery defaults

Terminal discovery forms require verified exit-only behavior regardless of spelling because long and single-dash options can be operational flags or ordinary operands. This fail-closed boundary lets each executable own its discovery forms, with the exact qualifying test owned by the [terminal permission policy](skills/domfiles-zed-settings/references/terminal-permissions.md#apply-the-terminal-permission-policy).

### Zed fetch and sandbox host scope

An explicit domain or hostname allowance authorizes the corresponding persistent `agent.sandbox_permissions.network_hosts` scope. Zed matches those grants by case-insensitive hostname without a port constraint, and every grant becomes part of the sandbox network floor available to later sandboxed terminal processes. This all-port persistence is intentional. Terminal commands remain subject to their independent terminal permissions, while explicit-port fetch URLs remain outside the canonical hostname fetch pattern unless separately allowed at the fetch-tool layer.

URL patterns in `agent.tool_permissions.tools.fetch.always_allow` that require a path after the hostname intentionally omit that hostname from `network_hosts`. A hostname grant would broaden trust beyond the path-qualified fetch allowance.

### Zed fixture repository permissions

Strict descendants of project-relative `.agent-<name>` directories are disposable fixture repository scope, distinct from top-level agent worktrees.

Git’s repository discovery can walk from a descendant into an enclosing worktree when the descendant lacks its own repository. Permission regexes cannot verify repository boundaries, resolve gitfiles or symlinks, or neutralize user-managed Git configuration. The automatic boundary therefore treats existing descendant state and configuration-driven filters, hooks, lazy fetching, submodule behavior, and URL rewrites as trusted. Explicit command-line forms that select external or remote behavior remain outside the boundary. Zed’s sandbox, sensitive-path, and symlink-escape checks remain additional boundaries.

### Zed generated-output deletion

Entries named `.pnpm-store`, `build`, `coverage`, `dist`, or `node_modules` and paths beneath them are treated as disposable generated output at any path depth. Because permissions match paths lexically, a matching root is intentionally allowed whether it is a directory or a regular file. Native `delete_path` and bounded terminal `rm` and `rmdir` forms may remove a matching root or its descendants, with the exact accepted option and glob grammar canonical in [Zed settings](../.config/zed/settings.json).

Brace expansion, broader deletion grammar, path traversal, paths outside those named roots, and similarly named entries remain confirmable. Shell substitutions and interpolations are instead denied by the [permission evaluator](skills/domfiles-zed-settings/references/permission-evaluator.md#evaluate-permission-behavior) before configured patterns are considered. Zed’s built-in sensitive-path and symlink-escape checks remain additional confirmation gates.

### Zed Git permission ordering

Git patterns for one subcommand can occupy several independently ordered sections. If an [owner-audit manifest](skills/domfiles-zed-settings/references/permission-evaluator.md#audit-permission-ownership) uses only the subcommand as its section key, the audit collapses those sections and reports entries beyond the first boundary. The [Git permission policy](skills/domfiles-zed-settings/references/git-permissions.md#apply-the-git-permission-policy) owns section membership and ordering, while the owner-audit workflow owns the manifest contract.

### Zed Git remote URL scope

`ls-remote` is the only broadly allowed Git subcommand that reaches the network with an unconstrained destination, while `fetch` and `clone` remain confirmable by default. [Zed settings](../.config/zed/settings.json) keep its broad allowance and add a confirmation for operands carrying a URL scheme or an scp-style `<host>:<path>`, so `git ls-remote <remote-name>` stays automatic while an explicit destination requires approval. A separate denial covers URLs that embed credentials.

The confirmation recognizes the operand shape rather than the host, so an explicit `github.com` URL confirms alongside any other. Exempting one host would require a complement expression the Rust-compatible engine cannot express, and the remote-name form already covers ordinary use.

### Zed npm `--all` option

npm’s exact `--all` is an ordinary scope option rather than a lifecycle-script override. It is safe for allowed npm command families such as `npm ls`, where it includes transitive dependencies. The ambiguous `--a` and `--al` forms and exact `--allow-scripts` remain behind confirmation. `npm approve-scripts --all` remains confirmable because `approve-scripts` is intentionally absent from the npm positive command alternatives and terminal defaults to confirmation.

### Zed permission regex compatibility

The exact `regex` crate version pinned in `Cargo.toml` was verified against Zed as of commit `9e236090`. The root `Cargo.lock` may update that crate’s transitive dependencies independently. The [Zed regex compatibility audit](skills/domfiles-zed-settings/references/permission-evaluator.md#audit-zed-regex-compatibility) revalidates the direct version baseline against current Zed source.

### Zed `printenv` exposure

The automatic `printenv` allowance is limited to the explicit, alphabetized non-secret variable names in [Zed settings](../.config/zed/settings.json). Unlisted names outside the denial categories below remain confirmable because agent environments can contain credentials and capability-bearing endpoints.

The automatic denial covers known credential-exposing names, secret-suffixed name patterns, wildcard-bearing variable operands, and zero-name environment dumps, with the exact denied names and suffixes canonical in the same settings. This denial remains necessary even though the positive allowlist excludes those forms so neither explicit approval nor a future allowance can expose them.

### Zed `stty` device scope

[Zed settings](../.config/zed/settings.json) allow `stty` only in its reporting forms. The bare invocation, `-a`, `-e`, `-g`, and the `all`, `everything`, and `size` operands display current characteristics, while every other operand modifies terminal state the agent and the user share. `stty` has no help or version option, and it opens its target before rejecting an unknown one, so this owner carries no discovery entry.

The `-f` operand retargets the report at a named terminal, which is the only way to read settings when standard input is not a tty. Because `stty` opens that path, and opening a serial device can assert modem control even under `O_NONBLOCK`, the operand is held to `/dev/stdin`, `/dev/tty`, and numbered `/dev/ttys` pseudo-terminals. The allowance fixes `-f` ahead of the reporting option so the grammar stays one ordered sequence. The reverse order remains confirmable.

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

Git subcommand discovery is restricted to compiled command names so aliases and external `git-*` helpers remain confirmable. Exact `--help` is allowed for the alphabetized names returned by `git --list-cmds=builtins`. Exact `-h` is allowed for the alphabetized names returned by `git --list-cmds=parseopt`, whose built-ins use Git’s parse-options framework, plus the explicitly verified `credential` exception. `git credential -h` prints usage and exits before reading credential protocol input. Both inventory commands are informational allowances. The [global Git inventory form](../.config/zed/AGENTS.md#tooling) cannot supply either list because it includes shipped scripts and guide names that are not compiled built-ins and omits some compiled names, while only `parseopt` identifies which built-ins accept `-h`. The lists require refresh when Git changes.

Outside agent worktrees, an allowed commit that supplies a message takes it from a file rather than the command line. Because [quote stripping leaves message words indistinguishable from pathspec tokens](#zed-worktree-permission-coupling), an unscoped `-m` grammar would also permit committing matching working-tree paths while ignoring the index. A `-F` operand carries no message words, so its only variable token is a path, held to a traversal-free descendant of the [standing user-approved namespace](#zed-agent-directory-allowance-scope) by the terminal policy’s [lexical path-namespace rule](skills/domfiles-zed-settings/references/terminal-permissions.md#translate-terminal-behavior-into-regex). That namespace bounds an input here rather than containing the commit’s effect, so the [agent-directory eligibility rule](skills/domfiles-zed-settings/references/agent-repository-permissions.md#apply-the-agent-directory-allowance-policy) does not supply the basis. Message contents never reach the permission grammar, so punctuation and multiple lines remain available. Standard-input `-F -` and the editor-opening `-e` combination stay confirmable because each waits on input an automated commit cannot supply.

Autosquash commit forms are classified by editor behavior rather than by their effect on later history. Plain `git commit --fixup=<commit>` composes its message from the target commit without an editor, while `--fixup=amend:<commit>`, `--fixup=reword:<commit>`, and `--squash=<commit>` open one and block an automated commit, so only the plain form is allowed.

Neither general-scope commit form admits `--no-verify`, so each runs the target repository’s `commit-msg`, `post-commit`, `pre-commit`, and `prepare-commit-msg` hooks. Terminal matching does not expose the working directory, so that residual execution trust reaches every repository opened in the editor rather than this one alone.

Commit signature verification runs `gpg.program` or `gpg.ssh.program`. Neither key is restricted to protected configuration, so a repository’s own config selects the executable. `--show-signature`, the `%G` pretty placeholders, and `for-each-ref`’s `%(signature)` fields all reach that verification, so each is confirmable across every command that interprets it. The guard is lexical and a `pretty.<name>` alias defined in the same repository config defeats it, because `git log --pretty=<name>` expands to a `%G` format string the normalized command never contains.

### Zed `tput` capname namespace

[Zed settings](../.config/zed/settings.json) allow `tput` to query any terminfo capability name rather than a finite list. A capname query reads the terminfo database and writes the capability’s value to standard output, so an unknown or misspelled name produces a diagnostic and a nonzero exit rather than a state change. Enumerating capnames would leave ordinary formatting and geometry queries confirmable while adding no safety, so this is the user-approved bounded namespace the [positive-branch rule](skills/domfiles-zed-settings/references/terminal-permissions.md#translate-terminal-behavior-into-regex) requires before an allowance may accept unknown future names.

`init` and `reset` are subtracted from that namespace because they set tty driver delays and tab expansion in addition to writing strings. Zed’s regex engine has no lookarounds, so the allowance spells the exclusion as an explicit complement over the capname alphabet, accepting every differing or longer name such as `inits` and `rese` while rejecting the two exact operands. `-S` stays outside the namespace because it reads capability names from standard input and waits.

Capability parameters are limited to non-negative integers. The few capabilities that take string parameters remain confirmable so no unconstrained operand reaches the allowance.

### Zed worktree permission coupling

The global [`git-worktrees` skill](../skills/domfiles-git-worktrees/SKILL.md) pairs the project-relative `.agent-<name>` namespace with the branch namespace `agent/<name>`. [Zed settings](../.config/zed/settings.json) use those namespaces as the security boundary for native path tools and terminal Git and filesystem operations. This permits automated creation, maintenance, integration, and cleanup inside disposable agent scope without granting equivalent operations elsewhere.

Terminal permission matching evaluates normalized command inputs without exposing the invocation’s current working directory to configured regexes. Bare commands therefore cannot inherit agent-worktree trust from their execution directory. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) owns the resulting permission-pattern namespace requirement.

Automatic task integration permits explicit staging, commit-time staging of tracked changes, and bounded noninteractive amendments inside agent worktrees, while merges from agent branches remain fast-forward-only. Because Zed strips ordinary shell quotes before permission matching, the commit grammar cannot distinguish normalized message words from non-option relative pathspec tokens. Both are trusted only inside agent worktrees, while option-looking tokens and broader operations remain confirmable. The allowed forms also trust repository-defined clean filters and commit or post-merge hooks within the user-managed repository.

Permission patterns can require the worktree and branch namespaces independently but cannot compare their `<name>` suffixes, so pair equality remains an agent-level invariant. Forced operations remain namespace-bound. Non-forced branch deletion retains Git’s fully-merged check, while forced deletion bypasses it.

Native `move_path`’s [multi-path permission evaluation](skills/domfiles-zed-settings/references/permission-evaluator.md#evaluate-permission-behavior) enables automatic strict-descendant moves within agent worktrees. Permission regexes constrain only lexical operands and cannot detect a permitted-looking parent symlink that resolves elsewhere inside an open worktree. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) leaves direct symbolic-link creation confirmable, so native path allowances treat existing worktree-internal symlinks as previously trusted repository state. Top-level worktree moves must also update Git’s administrative metadata, while Zed’s sensitive-settings and outside-worktree symlink-escape checks remain additional confirmation gates.

Worktree pruning remains confirmable because it can mutate shared Git administrative state beyond the bounded agent namespaces. The [worktree permission policy](skills/domfiles-zed-settings/references/agent-repository-permissions.md#maintain-agent-worktree-permissions) owns the dry-run exception.

### Zed xargs command ownership

Per-command `xargs` ownership avoids a second pooled child inventory whose membership could drift from direct command owners. The [terminal command-owner policy](skills/domfiles-zed-settings/references/terminal-permissions.md#apply-the-terminal-permission-policy) owns the exact partition and repeated wrapper grammar.

Zed authorizes the `xargs` shell segment before standard input becomes child-command arguments, so it cannot apply the child command’s normal confirmation overrides to injected options. Standard input can therefore activate hazardous child behavior after the shell segment has already been allowed. Complete nested `jq` and `ps` families require confirmation rather than denial so legitimate batching remains available with explicit approval.

## Agent integration

### Claude Agent integration

The tracked [`CLAUDE.md`](../CLAUDE.md) bridge is described in the [agent documentation table](../AGENTS.md#agent-documentation). [`domfiles sync`](../bin/domfiles-sync-setup) exposes the shared [global instructions](#claude-codex-and-zed-global-instructions) as Claude’s user-level `~/.claude/CLAUDE.md`, links the complete globally exposed skill set under `~/.claude/skills`, and the tracked [`.claude/skills`](../.claude/skills) symlink exposes repository-internal skills from `.agents/skills`. Claude therefore uses its native instruction and skill discovery locations without duplicating canonical content.

The [`claude-acp` registry entry](../.config/zed/settings.json) registers Claude Agent as a Zed External Agent. Claude Agent owns its authentication, model selection, tools, permissions, sandbox, and native configuration independently of Zed Agent. When subscription-backed Claude Code authentication is selected, `/login` acquires credentials interactively and stores them in macOS Keychain without placing them in tracked files. Claude user state under `~/.claude` and `~/.claude.json` remains machine-local outside the repository.

Zed’s OS sandbox applies only to Zed Agent and does not isolate Claude Agent. The tracked Zed sandbox and terminal permission settings therefore do not govern Claude Agent tools.

### Claude, Codex, and Zed global instructions

The tracked `.config/zed/AGENTS.md` is the canonical global instruction source shared by Claude, Codex, and Zed. `domfiles sync` exposes that source as `~/.claude/CLAUDE.md` for Claude and `~/.codex/AGENTS.md` for Codex, while the managed `~/.config` link exposes it as `~/.config/zed/AGENTS.md` for Zed. All three agents therefore load one instruction source across every project. It is not project scoped.

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, always refer to `.config/zed/AGENTS.md`.

The [agent-documentation ownership model](../AGENTS.md#agent-documentation) defines the repository-specific instruction surfaces.

### Deferred global policy

Conditional global policy may move into a global skill when most sessions do not need it, following the [documentation principles](../skills/domfiles-agent-documentation/SKILL.md#apply-the-documentation-principles). Eligibility requires a discrete trigger the agent can recognize without the deferred content, and a safe default when the route is missed. Conduct that applies continuously stays inline even when it is large.

The `Collaboration` policy is the standing example of what does not move. Its delegation rules shape how much work is done directly on every task rather than at one recognizable decision point, an agent that never loads them cannot notice that evidence has outgrown the main thread, and missing them drops the boundaries a subagent inherits.

`git-worktrees` is the first such deferral. Its former `Default` bullet was concurrent-work hygiene rather than worktree policy, so preserving existing changes and avoiding another agent’s write scope now lives in the global “Concurrent work” rule. Its route lives with the global temporary-file `.agent-<name>` convention, which the two namespaces share, and the current-checkout rule names the skill so it cannot read as a prohibition on isolation.

### GitHub CLI agent integration

The global [`github-cli` skill](../skills/domfiles-github-cli/SKILL.md) owns conditional agent behavior for `gh`. The global [GitHub CLI policy](../.config/zed/AGENTS.md#github-cli) retains the machine-local authentication and remote-mutation authorization gates so they remain directly loaded across projects.

`gh agent-task` remains [unsupported](../skills/domfiles-github-cli/SKILL.md#reject-unsupported-operations) because its preview flags and side effects can change without notice. User-requested external task relays use the [prompt-relays workflow](../skills/domfiles-prompt-relays/SKILL.md) with a separately selected delivery mechanism.

[Zed settings](../.config/zed/settings.json) remain canonical for exact command permissions. The [shared permission-layering policy](skills/domfiles-zed-settings/references/permissions.md#apply-the-shared-permission-policy) records that those permissions track verified CLI inventory and prompt behavior rather than mirroring agent policy. Keeping the layers independent lets policy remain intentionally stricter without coupling documentation changes to version-sensitive regex maintenance. Permission revalidation follows changes to `gh` syntax or behavior instead.

### Global system-available tooling

The [global system-available tooling list](../.config/zed/AGENTS.md#system-available-tooling) covers non-standard supporting development commands that agents can invoke directly across projects. It mirrors the non-CI development dependencies and [repository-scoped commands](#repository-scoped-commands) installed by [`domfiles sync`](../bin/domfiles-sync-install), using executable names when package names differ and subject to the inclusions and omissions below.

The list also includes `cargo`, `fish`, `node`, `pnpm`, and `rustc` even though `domfiles-sync-install` classifies their Homebrew formulas as primary dependencies. `cargo` and `rustc` support package-oriented and direct Rust workflows, while `fish`, `node`, and `pnpm` support Fish configuration checks, JavaScript and direct TypeScript execution, and the preferred package-manager workflow, respectively.

The list intentionally omits `claude`, `codex`, `fisher`, `git`, `mole`, and `vim`. `claude` and `codex` are agent runtimes rather than supporting commands. `fisher` is Fish package plumbing. `git` is guaranteed by the [supported environment](#supported-environment) and governed separately. `mole` is a system-maintenance utility outside coding workflows. `vim` is an interactive editor.

`brew` is intentionally absent because it is a supported-environment prerequisite rather than a dependency installed by `domfiles sync`. Companion commands supplied by listed dependencies, including `corepack`, `fish_indent`, `npm`, `npx`, and `rustfmt`, are not listed separately because the list tracks primary tool interfaces rather than every available executable.

In shell sessions configured by `domfiles` after synchronization, direct invocation assumes repository-managed commands are available through `PATH` in addition to the [supported-environment](#supported-environment) prerequisites.

### Package release-note bullet marker

The [release-note bullet-marker policy](../skills/domfiles-release-notes/SKILL.md#write-concise-consumer-facing-prose) preserves `*` because previously published notes use that marker. This keeps new and revised release notes consistent even though Markdown accepts other unordered-list markers.

### Prompt relays

The global [`prompt-relays` skill](../skills/domfiles-prompt-relays/SKILL.md) owns the relay delivery, complete-revision, composition, and evidence standard and a [generic task-relay prompt](../skills/domfiles-prompt-relays/assets/task-relay-prompt.md). It is a separate skill rather than an `agent-documentation` reference because relay composition is a frequent user-initiated task, so reaching the standard through the parent skill would load it and the standard together. [`agent-documentation`](../skills/domfiles-agent-documentation/SKILL.md) keeps an explicit route for relay-asset maintenance. The global [collaboration policy](../.config/zed/AGENTS.md#collaboration) remains canonical for subagent delegation and the anti-drift prompt contract. The global `release-notes` skill and repository-scoped `domfiles-zed-settings` skill provide standalone decision-capture profiles for completed work ([release notes](../skills/domfiles-release-notes/assets/decision-capture-prompt.md), [Zed settings](skills/domfiles-zed-settings/assets/decision-capture-prompt.md)). These are maintainer assets rather than runtime guidance, so ordinary skill invocations do not load them.

### Protected skill mutation

At Zed commit `dd04a229`, native mutation tools force confirmation when a directly named or canonical path contains consecutive `.agents` and `skills` components. Repository-root `AGENTS.md`, `.agents/PROJECT.md`, the root `skills` directory, and other `.agents` paths outside `skills` do not receive that agent-specific classification. Zed also requires the fixed `.agents/skills/<skill>/SKILL.md` layout for project skill discovery, so repository-internal skills retain that canonical location.

The [protected skill mutation policy](../skills/domfiles-agent-documentation/references/protected-skill-staging.md) preserves this boundary through staging and reviewed promotion when native file tools expose the target repository as a current project root. Outside those roots, scoped terminal write approval is the operative boundary, so the policy requires guarded direct mutation without staged copies.

### Repository harmonization

The global [`repository-harmonization` skill](../skills/domfiles-repository-harmonization/SKILL.md) owns the `Harmonize` shorthand and its change-oriented cross-repository consistency workflow.

### Shorthand command routing

A shorthand owned by a skill is routed by that skill’s description, which declares the bare command. The global [shorthand-command policy](../.config/zed/AGENTS.md#shorthand-commands) therefore carries no per-skill route headings. A heading would duplicate a trigger the description already provides and would require maintenance for every current and future shorthand. `Verify` remains defined inline because no skill owns it.

### Skill description limit

The 1,024-byte figure in the [skill description policy](../skills/domfiles-agent-documentation/SKILL.md#compose-the-change) is Zed’s limit rather than an intrinsic property of skill descriptions. Each client that receives the global skill set applies its own limit, so the figure requires revalidation whenever a supported client changes one.

### Skill distribution

The [skill distribution contract](../AGENTS.md#skills) defines project-authored skill categories and installation surfaces. Every tracked skill remains subject to the repository’s public-disclosure boundary.

[`bin/domfiles-sync-setup`](../bin/domfiles-sync-setup) defines the exact source-to-destination mappings for globally exposed skills. The current `skills/domfiles-*` set is global, while `skills/fish-shell-scripting` and `skills/human-facing-writing` are public.

Documentation for global skills is maintained under the assumption that an installation exposing one global skill exposes the complete set. The skills form a complementary ecosystem on top of the same global instructions, allowing one skill to defer an overlapping domain to its canonical sibling instead of repeating fallback guidance.

The public [`fish-shell-scripting`](../skills/fish-shell-scripting/SKILL.md) skill owns portable Fish authoring, review, audit, diagnosis, and validation guidance across scripts and configuration. The repository-internal [`domfiles-shell-scripts`](skills/domfiles-shell-scripts/SKILL.md) skill retains domfiles-specific shell invariants and integration policy. General wording remains owned by [`human-facing-writing`](../skills/human-facing-writing/SKILL.md), keeping Fish semantics separate from editorial guidance.

The public `human-facing-writing` skill’s entrypoint gives every task a prose baseline, then routes connected prose and technical copy to separate references, giving overlapping work one precedence contract while preserving a complete nontechnical path. Synchronization removes the obsolete managed symlinks rather than retaining aliases, so clients discover the merged skill once.

The canonical `domfiles-` prefix distinguishes global source directories from unprefixed public source directories without changing a global skill’s identity.

Supported clients expose globally installed skills beneath different configuration roots, and a global skill’s canonical basename differs from its installed basename. The [distributed-skill link contract](../skills/domfiles-agent-documentation/SKILL.md#keep-distributed-skill-links-installation-safe) owns the resulting portability requirements.

Independent public installation removes the shared-policy and guaranteed-sibling assumptions available to global skills, while public skill descriptions serve as human-facing discovery surfaces as well as routing metadata. The [public skill portability contract](../skills/domfiles-agent-documentation/references/public-skill-portability.md) owns the resulting requirements for standalone behavior, optional composition, and descriptions.

Edits to an exposed global skill affect its globally discovered installation through the symlink and may change agent behavior across projects. Adding or removing a globally exposed skill, changing its logical name, or changing its source-to-install mapping requires updating synchronization behavior. Removing or renaming a logical skill that has already been distributed also requires migration behavior for obsolete installed paths.

Every supported installation of the global `agent-documentation` skill is assumed to load an equivalent domfiles-managed global instruction layer. The skill relies on that layer’s documentation, writing, review, and `Verify` policies instead of restating them. External repositories remain self-contained and do not name, require, or link to the skill. Applicable project instructions continue to override its fallback workflow.

### Skill-owned script scope

`domfiles-zed-settings` is the sole script owner today, and the root `Cargo.toml` registers its binaries and adjacent tests so the root Cargo workspace validates them.

A global skill’s scripts stay hosted here. `domfiles sync` symlinks each global skill rather than copying it, so the installed skill is this checkout and the host toolchain, dependencies, and root validation remain reachable while an agent works in an unrelated project. That symlink is the precondition the [portable skill script contract](../skills/domfiles-agent-documentation/references/portable-skill-scripts.md) depends on, and it is why those scripts take every separate project they inspect or change as an explicitly selected target instead of resolving one from their installed path.

Agent script tests are not excluded from the repository’s test workflow. Collecting a TypeScript agent script test would additionally require a Vitest project entry covering the skill tree, which waits until the first such script exists.

### Zed project scan exclusions

The repository-level `.zed/settings.json` intentionally replaces Zed’s complete default `file_scan_exclusions` array with the narrower tracked list because no other entries from the original default exclusion set are needed in this repository context. The short `.git` and `.DS_Store` entries are intentional rather than recursive `**/.git` and `**/.DS_Store` patterns.

The `.claude/skills` entry prevents Zed from scanning the [Claude project skill bridge](#claude-agent-integration) as a second path to `.agents/skills`.

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

### Claude Code distribution

`claude` is intentionally installed through Homebrew’s `claude-code` cask rather than declared as an `@anthropic-ai/claude-code` project dependency. This keeps the CLI machine-level, follows Anthropic’s stable Homebrew channel, and excludes it from dependency installation in CI because `claude` is a development Homebrew dependency. The Homebrew CLI installation is separate from the `claude-acp` registry package managed by Zed.

### Codex distribution

`codex` is intentionally installed through Homebrew rather than declared as an `@openai/codex` project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Dependency status labels

`domfiles dependencies` is a user-facing readiness check for the synchronized dotfiles environment, not an inventory of every managed or installed tool. The [shell-script policy](skills/domfiles-shell-scripts/SKILL.md#check-supported-environment-compatibility) owns the row-inclusion rule.

`domfiles dependencies` intentionally uses compact checklist labels shared by success and error output. The `ssh` row reports whether the expected SSH key pair is configured, not whether the `ssh` executable is available. The concise `ssh` label is retained for consistency with the adjacent dependency rows.

The `rust` row reports whether both `cargo` and `rustc` are available, matching the managed Homebrew formula rather than either executable name.

`mole` and `vim` are intentionally omitted from the checklist even though synchronization installs them as primary Homebrew dependencies. Their availability does not affect the command’s output or exit status.

### Development lint wrapper architecture

The language-specific `bin/domfiles-dev-lint-*` entrypoints retain their own default scopes and lint commands while sharing discovery and execution through `domlib`. This preserves stable interfaces for pnpm, staged linting, language-specific CI, and targeted agent validation without duplicating the execution pipeline.

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

### Fish `clone` target derivation

The `clone` helper in [`.config/fish/aliases.fish`](../.config/fish/aliases.fish) derives the directory for its follow-up `cd` with a heuristic scoped to common remote URL forms and ordinary local paths. Full parity with `git clone`’s own destination naming is a non-goal. A source that addresses a repository through its inner `.git` directory, such as `/path/to/repo/.git`, clones successfully while the follow-up `cd` reports an error because the derived name stays `.git`. This gap is accepted to keep the helper free of special cases for inputs outside its practical use.

### Fish local configuration

`.config/fish/local.fish` is active machine-local Fish configuration when present. Its sourcing intentionally suppresses both stdout and stderr so local setup does not add shell-startup output.

### Git log search coloring

`git l` intentionally filters the ANSI-colored formatted log directly so Git’s field colors and `grep`’s match highlighting remain a simple pipeline. Because `grep` treats ANSI escape sequences as input bytes, an expression that crosses a color boundary—for example, from the hash into the subject or from the subject into the date—does not match even though the displayed text is contiguous. This limitation is intentional in favor of implementation simplicity.

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

The corresponding scripts in `bin/` are the stable command interfaces. They resolve implementations from the domfiles pnpm workspace without changing the caller’s working directory, so relative operands and project-scoped operations retain their upstream path semantics. `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Parallel copies through global pnpm state are intentionally unsupported.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. During synchronization, the [checkout-state predicate](#synchronization-checkout-state) determines whether `domfiles-sync-update` overrides this behavior with `warn`, which reports outdated dependencies and runs the command without installing them. These assumptions require revalidation when the pinned pnpm major version changes or `verifyDepsBeforeRun` is overridden.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.

### String helper reuse

The `__string_*` helpers are optional conveniences rather than a mandatory abstraction boundary.

### Suppressed command output

`DOMFILES_SUPPRESSED` suppresses the `$ …` command echo emitted by `__print_command`. It defaults to `false`. `domlib` parses user-supplied values through `__read_boolean_from_env`, then only normalized `true` enables suppression. Gating that one function covers every caller—`__`, and therefore `__chmod`, `__mkdir`, `__touch`, and `__symlink`, plus `__ssh_add` and `__domfiles_exec --print`. Only the echo is suppressed, so a wrapped command’s own output, headings, confirmations, and errors continue to print.

`__is_ci` overrides suppression, so automated runs keep the complete command trace regardless of `DOMFILES_SUPPRESSED`. A CI log is the only record of what a run executed and has no interactive reader to spare, so suppression there would remove diagnostic value without providing the benefit it exists for.

`__suppress` overrides `DOMFILES_SUPPRESSED` only inside its own subshell. The variable is runtime control state rather than a path mirrored into Fish. The [`domlib` maintenance policy](skills/domfiles-shell-scripts/SKILL.md#maintain-domlib) therefore exempts it from the `$DOMFILES_*` parity set.

A `.config/fish/config.fish` counterpart remains unwanted for a different reason than the other exemptions. Fish does not export `set -g`, which every `DOMFILES_*` entry in that file uses, so a counterpart in the established form would have no effect on `domlib`, while `set -gx` or `set -x` would suppress command echo for every domfiles command in the session.

An exported value reaches every child script, so `DOMFILES_SUPPRESSED=true domfiles sync` covers an entire synchronization run. `__suppress` applies the same suppression to one command by exporting the variable inside a subshell, which is how `domfiles-sync-setup` keeps the agent-skill linking loop from echoing without affecting later synchronization steps.

That loop intentionally confirms the source skill directory rather than the two destinations it replaces. One source is the unit of work, both destination roots are fixed, and `__symlink` removes and recreates each destination on every run, so naming them would report routine churn rather than the artifact being distributed. The removals stay in the CI trace through `__suppress`.

That subshell is also why `__suppress` rejects `__domfiles_exec`. It would absorb that function’s `exec`, letting the caller resume and run the remainder of `domfiles-sync` a second time. The echo there is suppressed by omitting the opt-in `--print` flag instead.

The prefix form `DOMFILES_SUPPRESSED=true __symlink …` is intentionally unused. POSIX leaves it unspecified whether a variable assignment preceding a function call persists after that function returns, and macOS `/bin/sh` is Bash 3.2 in POSIX mode, where it does persist and suppresses the remainder of the script.

No standardized environment variable covers command-echo suppression. `NO_COLOR` and `DO_NOT_TRACK` address color and telemetry only, so this name follows the prefixed convention of `HOMEBREW_NO_*` rather than an unprefixed `SUPPRESSED`, which any unrelated exported value in the invoking shell could set.
