# Agent instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called `domfiles`.
- **Disclosure:** The repository is public and open source.

## Public repository boundary

- **Public surfaces:** Treat every tracked file, proposed repository artifact, patch, and task relay as publicly disclosed.
- **Prohibited content:** Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to those surfaces.
- **Authentication review:** Before recommending or implementing an authenticated or privately configured tool, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- **Feasibility:** Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source or external credential store.
    - **Ignored-file boundary:** A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone is insufficient.
- **No safe route:** When no established public-safe route exists, report the tool as infeasible or ask the user to select a secret-storage boundary. Never request, inspect, echo, or invent the secret value.

## Agent documentation

| Source | Authority and ownership |
| --- | --- |
| `.config/zed/AGENTS.md` | Defines global defaults. Applicable project agent instructions override it. |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| `.agents/skills/*/` and `skills/*/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. `SKILL.md` is the entrypoint and may route to same-directory references. Distribution follows the [skill classification](#skills). |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

## General

- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement. Consult `.agents/PROJECT.md` for non-obvious rationale and maintenance decisions.
- **Durable knowledge:** Document newly discovered durable project knowledge in `.agents/PROJECT.md` when the task permits that documentation edit. Otherwise report the update as deferred follow-up work.
- **Ordering:** Keep entries alphabetized when their order is irrelevant. In source, treat a contiguous run of top-level constant declarations as one such list only when their initializers and behavior do not depend on declaration order, and check the complete qualifying run rather than the changed lines alone.

## Scope

- **Fish:** When Fish configuration or runtime behavior is in scope and [`.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation.
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

Classify every project-authored skill by its canonical source and supported installation surface. `metadata.internal: true` marks a skill as unsupported for public installation. It does not make tracked source private.

| Category | Canonical source | `metadata.internal` | Supported installation |
| --- | --- | --- | --- |
| Internal | `.agents/skills/domfiles-<skill-name>` | `true` | Project-local to `domfiles`. |
| Global | `skills/<skill-name>` | `true` | Globally exposed through the system established by `domfiles sync`. |
| Public | `skills/<skill-name>` | Omitted | Globally exposed through `domfiles sync` and independently installable through `skills` without `domfiles`. |

Skills in the global category may rely on the domfiles-managed global instructions and complete globally exposed skill set. Skills in the public category must provide their advertised behavior when installed independently.

- **Category changes:** Update the canonical location, metadata, documentation links, and synchronization behavior together. Bring a skill’s scripts into conformance with the [portable skill script contract](skills/agent-documentation/references/portable-skill-scripts.md) before promoting it from internal to global, and remove or relocate them before promoting it into the public category.
- **Installation-safe links:** Apply the [distributed-skill link contract](skills/agent-documentation/SKILL.md#keep-distributed-skill-links-installation-safe) to every global or public skill.
- **Script ownership:** Internal and global skills may own scripts. A global skill’s scripts run from this repository through the `domfiles sync` symlink and take every separate project they inspect or change as an explicitly selected target, following the [portable skill script contract](skills/agent-documentation/references/portable-skill-scripts.md). Public skills remain documentation-only because an independently installed copy has no host repository to execute through.
- **Implementation default:** Write project-authored skill scripts in Rust with `snake_case` source stems, retaining the established `.test.rs` suffix for adjacent contract tests.
- **Language exception:** Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.
    - Record the exception and its durable reason in the owning skill before implementation.
    - Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.
- **Cargo names:** Keep established Cargo target and CLI names unchanged when only source filenames change.
- **Routing:** Load each applicable `domfiles-*` skill immediately before the task pass whose resolved scope intersects its declared file scope. Do not preload domain skills for later passes.
- **Maintenance:** Treat every maintained `domfiles-*` skill as a living document. After using one, suggest a concrete edit only when execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
