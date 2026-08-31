# Agent instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called `domfiles`.
- **Disclosure:** The repository is public and open source.

## Public repository boundary

- **Public surfaces:** Treat every tracked file, proposed repository artifact, patch, and relay as publicly disclosed.
- **Authentication review:** Before recommending or implementing an authenticated or privately configured tool, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- **Feasibility:** Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source or external credential store.
    - **Ignored-file boundary:** A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone is insufficient.
- **No safe route:** When no established public-safe route exists, report the tool as infeasible or ask the user to select a secret-storage boundary.

## Agent documentation

| Source | Authority and ownership |
| --- | --- |
| `.config/zed/AGENTS.md` | Defines global defaults. Applicable project agent instructions override it. |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| `CLAUDE.md` | Bridges Claude to the canonical project instructions in `AGENTS.md`. It defines no independent policy. |
| `.agents/skills/*/` and `skills/*/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. `SKILL.md` is the entrypoint and may route to references within its own skill directory and to sibling skills. Distribution follows the [skill classification](#skills). |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

## General

- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Navigation:** Read only the section of `.agents/PROJECT.md` that applies, reaching it through an existing link or by locating its heading first, rather than reading the document.
- **Durable knowledge:** Document newly discovered durable project knowledge in `.agents/PROJECT.md` when the task permits that documentation edit. Otherwise report the update as deferred follow-up work.
- **Ordering:** Keep entries alphabetized when their order is irrelevant, including lookup tables and configuration arrays. Treat labeled instruction bullets as order-dependent, along with rows ordered to carry meaning. Order `.agents/PROJECT.md` second-level sections topically, appending a new section when no topical position is evident, and alphabetize the third-level sections within each. In source, treat a contiguous run of top-level constant declarations as one such list only when their initializers and behavior do not depend on declaration order, and check the complete qualifying run rather than the changed lines alone.

## Scope

- **Fish:** When Fish configuration or runtime behavior is in scope and [`.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation unless the [publication-audit mode](.agents/skills/domfiles-repository-audit/SKILL.md#resolve-the-scope) excludes it.
    - Do not report `.gitignore` including `local.fish`.
    - Do not suggest adding additional documentation for `local.fish`.
- **Symlink:** Do not analyze the contents of `bin/git-diff-highlight` (it’s a symlink).
- **Secret-bearing local files:** Do not read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- **Empty configuration:** Do not report empty config files.
- **Fixed locations:** Report cases that would tie this repository to a fixed filesystem location, except:
    - `$HOME/*` paths, system paths, or vendor paths.
    - Symlinks created through `domfiles sync`.
    - `.config/fish/fish_variables`.
    - Documentation.

## Skills

Classify every project-authored skill by its canonical source and supported installation surface. `metadata.internal: true` marks a skill as unsupported for public installation. It does not make tracked source private. The categories below are ordered by widening installation surface.

| Category | Canonical source | `name` | `metadata.internal` | Supported installation |
| --- | --- | --- | --- | --- |
| Internal | `.agents/skills/domfiles-<skill-name>` | `domfiles-<skill-name>` | `true` | Project-local to `domfiles`. |
| Global | `skills/domfiles-<skill-name>` | `<skill-name>` | `true` | Globally exposed as `<skill-name>` through the system established by `domfiles sync`. |
| Public | `skills/<skill-name>` | `<skill-name>` | Omitted | Globally exposed through `domfiles sync` and independently installable through `skills` without `domfiles`. |

The root `skills/*` tree stores global and public canonical sources and is not a project-local discovery surface. Client-specific project discovery remains backed by `.agents/skills/*`. `domfiles sync` removes the canonical `domfiles-` prefix when linking a global skill, so its frontmatter `name` and final symlink basename remain `<skill-name>`.

Skills in the global category may rely on the domfiles-managed global instructions and complete globally exposed skill set. Skills in the public category must provide their advertised behavior when installed independently.

- **Category maintenance:** Follow [skill category maintenance](skills/domfiles-agent-documentation/references/skill-category-maintenance.md) when authoring, reviewing, auditing, or maintaining project-authored skill documentation, changing a skill’s category, or maintaining its supported installation reach.
- **Installation-safe links:** Apply the [distributed-skill link contract](skills/domfiles-agent-documentation/SKILL.md#keep-distributed-skill-links-installation-safe) to every global or public skill.
- **Public skill writing:** For every edit to a public skill, resolve the agent-documentation contract, then apply `human-facing-writing` to every changed human-facing surface through the [public skill writing composition contract](skills/domfiles-agent-documentation/references/public-skill-portability.md#compose-public-skill-writing). This is source-authoring composition and does not create an installed runtime dependency.
- **Public peers:** Only public skills may declare GitHub-hosted fallbacks, and only to public peers in `porada/domfiles`.
- **Script ownership:** Internal and global skills may own scripts. A global skill’s scripts run from this repository through the `domfiles sync` symlink and take every separate project they inspect or change as an explicitly selected target, following the [portable skill script contract](skills/domfiles-agent-documentation/references/portable-skill-scripts.md). Public skills remain documentation-only because an independently installed copy has no host repository to execute through.
- **Implementation default:** Write project-authored skill scripts in Rust with `snake_case` source stems, retaining the established `.test.rs` suffix for adjacent contract tests.
- **Language exception:** Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.
    - Record the exception and its durable reason in the owning skill before implementation.
    - Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.
- **Cargo names:** Keep established Cargo target and CLI names unchanged when only source filenames change.
- **Routing:** Load each applicable project-local `domfiles-*` skill immediately before the task pass whose resolved scope intersects its declared scope. Do not preload domain skills for later passes.
- **Maintenance:** Treat every maintained project-local `domfiles-*` skill as a living document. After using one, suggest a concrete edit only when execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
