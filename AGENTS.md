# Agent Instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called `domfiles`.
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
| `CLAUDE.md` | Bridges Claude to the canonical project instructions in `AGENTS.md`. It defines no independent policy. |
| Project-authored skill directories under `.agents/skills/` and `skills/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. Distribution follows the [skill classification](#skills). |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, refer to `.agents/GLOBAL.md`.

## General

- **User direction:** The global **Explicit user direction** policy applies to this repository’s workflow requirements, conventions, and non-secret content exclusions, including those in project-authored skills. The [public repository boundary](#public-repository-boundary) remains applicable.
- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Navigation:** Read only the section of `.agents/PROJECT.md` that applies, reaching it through an existing link or by locating its heading first, rather than reading the document.
- **Documentation alignment:** When changing behavior, interfaces, or defaults, search tracked documentation for affected identifiers and descriptions. Update statements the change invalidates, following the applicable documentation workflow and mutation requirements. Keep the search focused on the changed contract.
- **Durable knowledge:** Add project knowledge to `.agents/PROJECT.md` only when its absence would likely cause a wrong future decision or substantial repeated investigation. Capture non-obvious constraints and rationale not already clear from canonical configuration, instructions, or source. Do not record optimization recaps, routine implementation details, or workflow summaries. Prefer updating existing material over appending another explanation. When the task does not permit the edit, report deferred documentation work only if it meets this threshold.

## Validation

Prefix every agent-selected pnpm-backed validation command with `env PNPM_CONFIG_FROZEN_LOCKFILE=true PNPM_CONFIG_PM_ON_FAIL=error PNPM_CONFIG_VERIFY_DEPS_BEFORE_RUN=error`, including checks selected through domain skills. Keep validation separate from dependency reconciliation. If a check needs reconciliation, apply the global **Dependencies** policy, perform any authorized acquisition separately, then rerun the guarded check. Report a limitation when acquisition is outside the task’s authorization or cannot proceed within applicable boundaries. See the [repository command rationale](.agents/PROJECT.md#repository-scoped-commands) for the overrides’ effects.

For changed JSON or TOML files, run `env PNPM_CONFIG_FROZEN_LOCKFILE=true PNPM_CONFIG_PM_ON_FAIL=error PNPM_CONFIG_VERIFY_DEPS_BEFORE_RUN=error pnpm run lint:<format> <changed-format-files>`. The JSON wrapper requires exactly one parsed JSON value, and the TOML wrapper runs `tombi lint --offline`. Pass paths explicitly unless repository-wide validation is intended.

## Scope

- **Fish:** When Fish configuration or runtime behavior is in scope and [`home/.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation unless the [publication audit mode](.agents/skills/domfiles-repository-audit/SKILL.md#resolve-audit-scope) excludes it.
    - Do not report `.gitignore` including `local.fish`.
    - Do not suggest adding additional documentation for `local.fish`.
- **Symlink:** Exclude the contents of `home/.local/bin/git-diff-highlight` by default because it’s a symlink. Include its target only when the user explicitly requests that analysis and access boundaries permit it.
- **Secret-bearing local files:** Do not read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- **Empty configuration:** Do not report empty config files.
- **Fixed locations:** Report cases that would tie this repository to a fixed filesystem location, except:
    - `$HOME/*` paths, system paths, or vendor paths.
    - Symlinks created through `domfiles sync`.
    - `home/.config/fish/fish_variables`.
    - Documentation.

## Skills

Classify every project-authored skill by canonical source and supported installation. `metadata.internal: true` means public installation is unsupported, not that tracked source is private. Categories below widen installation reach, with the global overlay variant beside its category.

| Category | Canonical Source | `name` | `metadata.internal` | Supported Installation |
| --- | --- | --- | --- | --- |
| Internal | `.agents/skills/domfiles-<skill-name>` | `domfiles-<skill-name>` | `true` | Project-local to `domfiles`. |
| Global | `skills/.domfiles-<skill-name>` | `<skill-name>` | `true` | Globally exposed as `<skill-name>` through the system established by `domfiles sync`. |
| Global overlay | `skills/.dom-<base-name>` | `dom-<base-name>` | `true` | Same installation surface as the global category. |
| Public | `skills/<skill-name>` | `<skill-name>` | Omitted | Globally exposed through `domfiles sync` and independently installable through `skills` without `domfiles`. |

Global overlays inherit global requirements. Their `<base-name>` must exactly match the base skill’s frontmatter `name`, without shortening, rewording, or dropping qualifiers.

Global skills may rely on domfiles-managed global instructions and the complete globally exposed skill set. Public skills must deliver their advertised behavior when independently installed.

When changing a public skill contract or its standalone fallback, consult the [fallback family map](.agents/PROJECT.md#public-skill-fallback-families) to identify related guidance.

- **Public peers:** Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.
- **Script ownership:** Internal and global skills may own scripts. Public skills remain documentation-only.
- **Implementation default:** Write project-authored skill scripts in Rust with `snake_case` source stems, retaining the established `.test.rs` suffix for adjacent contract tests.
- **Language exception:** Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.
    - Record the exception and its durable reason in the owning skill before implementation.
    - Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.
- **Cargo names:** Keep established Cargo target and CLI names unchanged when only source filenames change.
