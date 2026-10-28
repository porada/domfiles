# Agent Instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called domfiles.
- **Disclosure:** The repository is public and open source.

## Public Repository Boundary

- **Public surfaces:** Treat every tracked file, proposed repository artifact, patch, and relay as publicly disclosed.
- **Authentication review:** Before recommending or implementing a tool that uses authentication or private configuration, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- **Feasibility:** Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source or external credential store.
    - **Ignored-file boundary:** A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone isn’t enough.
- **No safe route:** If no established public-safe route exists, report the tool as infeasible or ask the user to choose a secret storage boundary.

## Agent Documentation

| Source | Authority and Ownership |
| --- | --- |
| `.agents/GLOBAL.md` | Defines global user defaults. |
| `AGENTS.md` | Defines project instructions, scope, and documentation authority. Applicable project instructions override global defaults. |
| Project-authored skill directories under `.agents/skills/` and `skills/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Distribution follows the [skill classification](skills/.domfiles-skill-development/references/domfiles-skill-policy.md#classify-skills). |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It doesn’t override agent instructions. |

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, refer to `.agents/GLOBAL.md`.

## General

- **User direction:** The global **Explicit user direction** policy applies to this repository’s workflow requirements, conventions, and non-secret content exclusions, including those in project-authored skills. The [public repository boundary](#public-repository-boundary) remains applicable.
- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Command entrypoints:** Use the entrypoint documented for each task, including validation, rather than another route to the same implementation. Invoke [repository-scoped commands](.agents/PROJECT.md#repository-scoped-commands) through `PATH`, overriding global **Direct invocation** for `skills` and `vercel` in this repository. This exercises the interfaces other users and agents rely on.
- **Navigation:** Read only the applicable section of `.agents/PROJECT.md`. Follow an existing link or locate its heading first, rather than reading the whole document.
- **Documentation alignment:** When you change behavior, interfaces, or defaults, search tracked documentation for affected identifiers and descriptions. Update statements the change makes inaccurate, following the applicable documentation workflow and mutation requirements. Keep the search focused on the changed contract.
- **Durable knowledge:** Add project knowledge to `.agents/PROJECT.md` only when it meets the global **Durable knowledge** threshold. Don’t record optimization recaps, routine implementation details, or workflow summaries. Prefer updating existing material over adding another explanation. If the task doesn’t permit the edit, report deferred documentation work only if it meets that threshold.
- **Zed settings:** Keep `.zed/settings.json` and `home/.config/zed/settings.json` free of redundant entries. An entry is redundant only when omitting it preserves the supported workflow, including tool-managed state and later invocations. Matching a default or inherited value alone doesn’t make an entry redundant. See the [CLI persistence rationale](.agents/PROJECT.md#zed-cli-open-behavior).

## Validation

Prefix every agent-selected pnpm-backed validation command with `env PNPM_CONFIG_FROZEN_LOCKFILE=true PNPM_CONFIG_PM_ON_FAIL=error PNPM_CONFIG_VERIFY_DEPS_BEFORE_RUN=error`, including checks selected through domain skills. Keep validation separate from package manager and dependency reconciliation.

If a guard stops a check, diagnose the prerequisite failure and apply the global **Dependencies** policy. Complete authorized acquisition or reconciliation separately, including obtaining or selecting the package manager version prescribed by the project. Then rerun the check with all guards intact. Report a limitation only when recovery can’t proceed within applicable boundaries. See [pnpm Execution and Lockfile Behavior](.agents/PROJECT.md#pnpm-execution-and-lockfile-behavior) for the overrides’ effects.

For changed JSON files, run `pnpm run lint:json <changed-json-files>` with that prefix. Pass paths explicitly unless repository-wide validation is intended.

For Markdown validation, invoke `doclint` and follow [Check Markdown](skills/.domfiles-agent-documentation/references/check-markdown.md).

## Scope

- **Fish:** When Fish configuration or runtime behavior is in scope and [`home/.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation unless the [publication audit mode](.agents/skills/audit/SKILL.md#resolve-audit-scope) excludes it.
    - Don’t report that `.gitignore` lists `local.fish`.
    - Don’t suggest further documentation for `local.fish`.
    - Don’t report its [documented sourcing behavior](.agents/PROJECT.md#fish-local-configuration) as hidden diagnostics.
- **Symlink:** Exclude the contents of `home/.local/bin/git-diff-highlight` by default because its target is outside the repository. Include its target only when the user explicitly requests that analysis and access boundaries permit it.
- **Secret-bearing local files:** Don’t read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- **Empty configuration:** Don’t report empty configuration files.
- **Fixed locations:** Report cases that would tie this repository to a fixed filesystem location, except:
    - `$HOME/*` paths, system paths, or vendor paths.
    - Symlinks created through `domfiles sync`.
    - `home/.config/fish/fish_variables`.
    - Documentation.

## Skills

Follow the [domfiles skill policy](skills/.domfiles-skill-development/references/domfiles-skill-policy.md) for project-authored skill work, including classification, supported installation, scripts, and test integration.
