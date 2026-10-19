# Agent Instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called domfiles.
- **Disclosure:** The repository is public and open source.

## Public Repository Boundary

- **Public surfaces:** Treat every tracked file, proposed repository artifact, patch, and relay as publicly disclosed.
- **Authentication review:** Before recommending or implementing an authenticated or privately configured tool, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- **Feasibility:** Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source or external credential store.
    - **Ignored-file boundary:** A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone is insufficient.
- **No safe route:** When no established public-safe route exists, report the tool as infeasible or ask the user to select a secret storage boundary.

## Agent Documentation

| Source | Authority and Ownership |
| --- | --- |
| `.agents/GLOBAL.md` | Defines global user defaults. |
| `AGENTS.md` | Defines project instructions, scope, and documentation authority. Applicable project instructions override global defaults. |
| Project-authored skill directories under `.agents/skills/` and `skills/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Distribution follows the [skill classification](#skills). |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, refer to `.agents/GLOBAL.md`.

### Editorial Refresh

Only agents whose identity is Zed Agent may perform this editorial refresh workflow or maintain its list. Running another agent inside Zed doesn’t qualify. Other agents may make authorized task edits without performing a refresh. To rename or delete a listed document, they must coordinate with a Zed Agent to complete the list maintenance in the same change, or defer the operation.

Within `agent-documentation`’s change workflow, use `human-facing-writing` for whole-document prose refreshes of paths in the [editorial refresh list](.agents/assets/editorial-refresh.txt). Refreshing `AGENTS.md` or `.agents/GLOBAL.md` requires an explicit user opt-in for that document in the current task. For every other listed document, any authorized edit requires the refresh unless the user explicitly opts out for that task. Complete any required refresh before handoff. Requested edits can proceed when the whole-document refresh is skipped.

These rules authorize the applicable refresh and corresponding list maintenance within the task, subject to explicit user limits and existing approval, mutation, and security requirements. Read-only work doesn’t trigger a refresh.

After making the task edits, perform a separate editorial pass over the complete final document, including sections untouched by the task. Apply `human-facing-writing` to all prose eligible for refresh, not just the changed lines. Reading the file for context, checking the task diff, or reviewing the focused changes does not complete this pass.

Preserve the document’s meaning, requirements, permissions, conditions, exceptions, and workflow behavior. Preserve introductory paragraphs, typography rules, headings, structure, links, examples, commands, literal tokens, and metadata other than description wording. Skill descriptions may change wording without changing applicability. Leave `.agents/GLOBAL.md`’s final `Writing` section unchanged, and leave already sound prose alone.

After completing the whole-document editorial pass, applicable validation of the final document, and preserved-meaning review of the complete refresh, remove the path. A refresh requiring no wording changes counts only when the full pass establishes that every eligible passage is already sound. Leave the path listed if any of these steps is skipped, deferred, unfinished, or unverified.

Keep repository-relative paths in ascending bytewise order, one per line. Don’t add candidates or refresh untouched files. Carry a pending entry to its new path after an authorized rename. Remove the entry when the task intentionally deletes its document.

When no entries remain, delete the entire list file and this **Editorial Refresh** subsection in the same change. Remove `.agents/assets/` only if it is then empty. Don’t leave an empty list or a completion notice.

## General

- **User direction:** The global **Explicit user direction** policy applies to this repository’s workflow requirements, conventions, and non-secret content exclusions, including those in project-authored skills. The [public repository boundary](#public-repository-boundary) remains applicable.
- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Navigation:** Read only the section of `.agents/PROJECT.md` that applies, reaching it through an existing link or by locating its heading first, rather than reading the document.
- **Documentation alignment:** When changing behavior, interfaces, or defaults, search tracked documentation for affected identifiers and descriptions. Update statements the change invalidates, following the applicable documentation workflow and mutation requirements. Keep the search focused on the changed contract.
- **Durable knowledge:** Add project knowledge to `.agents/PROJECT.md` only when its absence would likely cause a wrong future decision or substantial repeated investigation. Capture non-obvious constraints and rationale not already clear from canonical configuration, instructions, or source. Do not record optimization recaps, routine implementation details, or workflow summaries. Prefer updating existing material over appending another explanation. When the task does not permit the edit, report deferred documentation work only if it meets this threshold.
- **Global redundancy baseline:** Use Zed’s native Agent instructions and tool contracts as the sole client baseline for redundancy decisions in `.agents/GLOBAL.md` under the [composition rule](skills/.domfiles-agent-documentation/references/compose-documentation.md). Other clients may lack rules that Zed supplies.
- **Zed settings:** Keep `.zed/settings.json` and `home/.config/zed/settings.json` free of redundant entries. An entry is redundant only when omitting it preserves the supported workflow, including tool-managed state and later invocations. Matching a default or inherited value is not sufficient. See the [CLI persistence rationale](.agents/PROJECT.md#zed-cli-open-behavior).

## Validation

Prefix every agent-selected pnpm-backed validation command with `env PNPM_CONFIG_FROZEN_LOCKFILE=true PNPM_CONFIG_PM_ON_FAIL=error PNPM_CONFIG_VERIFY_DEPS_BEFORE_RUN=error`, including checks selected through domain skills. Keep validation separate from package manager and dependency reconciliation. When a guard stops a check, diagnose the prerequisite failure and apply the global **Dependencies** policy. Complete authorized acquisition or reconciliation separately, including obtaining or selecting the project-prescribed package manager version, then rerun the check with all guards intact. Report a limitation only when recovery cannot proceed within applicable boundaries. See [pnpm Execution and Lockfile Behavior](.agents/PROJECT.md#pnpm-execution-and-lockfile-behavior) for the overrides’ effects.

For changed JSON files, run `env PNPM_CONFIG_FROZEN_LOCKFILE=true PNPM_CONFIG_PM_ON_FAIL=error PNPM_CONFIG_VERIFY_DEPS_BEFORE_RUN=error pnpm run lint:json <changed-json-files>`. Pass paths explicitly unless repository-wide validation is intended.

## Scope

- **Fish:** When Fish configuration or runtime behavior is in scope and [`home/.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation unless the [publication audit mode](.agents/skills/audit/SKILL.md#resolve-audit-scope) excludes it.
    - Do not report that `.gitignore` lists `local.fish`.
    - Do not suggest further documentation for `local.fish`.
- **Symlink:** Exclude the contents of `home/.local/bin/git-diff-highlight` by default because its target is outside the repository. Include its target only when the user explicitly requests that analysis and access boundaries permit it.
- **Secret-bearing local files:** Do not read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- **Empty configuration:** Do not report empty configuration files.
- **Fixed locations:** Report cases that would tie this repository to a fixed filesystem location, except:
    - `$HOME/*` paths, system paths, or vendor paths.
    - Symlinks created through `domfiles sync`.
    - `home/.config/fish/fish_variables`.
    - Documentation.

## Skills

Classify every project-authored skill by canonical source and supported installation. `metadata.internal: true` means public installation is unsupported, not that tracked source is private. Categories below widen installation reach, with the global overlay variant beside its category.

| Category | Canonical Source | `name` | `metadata.internal` | Supported Installation |
| --- | --- | --- | --- | --- |
| Internal | `.agents/skills/<skill-name>` | `<skill-name>` | `true` | Project-local to domfiles. |
| Global | `skills/.domfiles-<skill-name>` | `<skill-name>` | `true` | Globally exposed as `<skill-name>` through the system established by `domfiles sync`. |
| Global overlay | `skills/.dom-<base-name>` | `dom-<base-name>` | `true` | Same installation surface as the global category. |
| Public | `skills/<skill-name>` | `<skill-name>` | Omitted | Globally exposed through `domfiles sync` and independently installable through `skills` without `domfiles`. |

Follow the [domfiles skill policy](skills/.domfiles-skill-development/references/domfiles-skill-policy.md) for project-authored skill work, including scripts and their test integration.
