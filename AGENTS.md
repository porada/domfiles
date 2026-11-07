# Agent Instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called domfiles.

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
| Project-authored skill directories under `.agents/skills/` and `skills/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It doesn’t override agent instructions. |

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, refer to `.agents/GLOBAL.md`.

## General

- **User direction:** The global **Explicit user direction** policy applies to this repository’s workflow requirements, conventions, and non-secret content exclusions, including those in project-authored skills. The [public repository boundary](#public-repository-boundary) remains applicable.
- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Command entrypoints:** Use the entrypoint documented for each task, including validation, rather than another route to the same implementation. Invoke [repository-scoped commands](.agents/PROJECT.md#repository-scoped-commands) through `PATH`, overriding global **Direct invocation** for `skills` and `vercel` in this repository. This exercises the interfaces other users and agents rely on.
- **Navigation:** Consult the relevant sections of `.agents/PROJECT.md`.
- **Durable knowledge:** Add project knowledge to `.agents/PROJECT.md` only when it meets the global **Durable knowledge** threshold. Don’t record optimization recaps, routine implementation details, or workflow summaries. Prefer updating existing material over adding another explanation. If the task doesn’t permit the edit, report deferred documentation work only if it meets that threshold.
- **Zed settings:** Keep `.zed/settings.json` and `home/.config/zed/settings.json` free of redundant entries. An entry is redundant only when omitting it preserves the supported workflow, including tool-managed state and later invocations. Matching a default or inherited value alone doesn’t make an entry redundant. See the [CLI persistence rationale](.agents/PROJECT.md#zed-cli-open-behavior).

## Validation

For changed JSON files, run `pnpm run lint:json <changed-json-files>`. Pass paths explicitly unless repository-wide validation is intended.

For Markdown validation, invoke `doclint` and follow [Check Markdown](skills/.domfiles-agent-documentation/references/check-markdown.md).

## Scope

- **Fish:** Treat [`home/.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) as secret-bearing machine-local configuration. Exclude its contents from analysis, execution, and validation, including indirect sourcing through Fish startup.
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
