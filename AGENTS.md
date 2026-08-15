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
| `.agents/skills/*/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. `SKILL.md` is the entrypoint and may route to same-directory references. `domfiles-*` skills are repository-scoped. Others are [portable global skills](.agents/PROJECT.md#global-agent-skills). |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

## General

- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement. Consult `.agents/PROJECT.md` for non-obvious rationale and maintenance decisions.
- **Durable knowledge:** Document newly discovered durable project knowledge in `.agents/PROJECT.md` when the task permits that documentation edit. Otherwise report the update as deferred follow-up work.
- **Ordering:** Keep entries alphabetized when their order is irrelevant.

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

- **Implementation default:** Write project-authored skill scripts in Rust with `snake_case` source stems, retaining the established `.test.rs` suffix for adjacent contract tests.
- **Language exception:** Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.
    - Record the exception and its durable reason in the owning skill before implementation.
    - Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.
- **Cargo names:** Keep established Cargo target and CLI names unchanged when only source filenames change.
- **Routing:** Load each applicable `domfiles-*` skill immediately before the task pass whose resolved scope intersects its declared file scope. Do not preload domain skills for later passes.
- **Maintenance:** Treat every maintained `domfiles-*` skill as a living document. After using one, suggest a concrete edit only when execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
