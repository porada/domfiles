# Agent instructions

## Overview

- This repository is the home of all my actively used dotfiles (also referred to as `domfiles`).
- This project is public and open source.

## Public repository boundary

- Treat every tracked file, proposed repository artifact, patch, and task relay as publicly disclosed.
- Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to those surfaces.
- Before recommending or implementing a tool that uses authentication or private configuration, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source or external credential store.
    - A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone is insufficient.
- When no established public-safe route exists, report the tool as infeasible or ask the user to select a secret-storage boundary. Never request, inspect, echo, or invent the secret value.

## Agent documentation

| Source | Authority and ownership |
| --- | --- |
| `.config/zed/AGENTS.md` | Defines global defaults. Applicable project agent instructions override it. |
| `AGENTS.md` | Defines project instructions, scope, documentation authority, and skill routing. Applicable project instructions override global defaults. |
| `.agents/skills/*/` | Each skill directory defines delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. `SKILL.md` is its entrypoint and may route to canonical references in the same directory. `domfiles-*` skills are repository-scoped. Other skills are [portable global skills](.agents/PROJECT.md#global-agent-skills). |
| `.agents/PROJECT.md` | Records durable facts, rationale, constraints, and maintenance decisions. It does not override agent instructions. |
| Source and configuration | Define exact current values and implemented behavior. |

## General

- Follow the [supported environment](.agents/PROJECT.md#supported-environment)—including its default-shell requirement—and consult `.agents/PROJECT.md` for non-obvious project rationale and maintenance decisions.
    - Document newly discovered durable project knowledge there when the task permits that documentation edit. Otherwise report the update as deferred follow-up work.
    - Keep entries alphabetized when their order is irrelevant.

## Scope

- When Fish configuration or runtime behavior is in scope and [`.config/fish/local.fish`](.agents/PROJECT.md#fish-local-configuration) exists, include it in applicable analysis, execution, and validation.
    - Do not report `.gitignore` including `local.fish`.
    - Do not suggest adding additional documentation for `local.fish`.
- Do not analyze the contents of `bin/git-diff-highlight` (it’s a symlink).
- During commit review, do not analyze or validate changes to permission patterns in `.config/zed/settings.json` unless the user explicitly includes that analysis. Review surrounding non-pattern changes normally. If evaluating the patterns is necessary to complete the review, stop before that analysis and ask for permission.
- Do not read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- Do not report empty config files.
- Report any cases that would tie this repository to a fixed filesystem location.
    - Do not report `$HOME/*` paths, system paths, or vendor paths.
    - Do not report symlinks created via `domfiles sync`.
    - Do not report `.config/fish/fish_variables`.
    - Do not report documentation.

## Skills

- Write project-authored skill scripts in Rust by default and name Rust source stems in `snake_case`, retaining the established `.test.rs` suffix for adjacent contract tests. Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes the alternative materially more correct, maintainable, or proportionate than Rust.
    - Record the exception and its durable reason in the owning skill before implementation. Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.
    - Keep established Cargo target and CLI names unchanged when only source filenames change.
- Load each applicable `domfiles-*` skill immediately before the task pass whose resolved scope intersects its declared file scope. Do not preload domain skills for later passes.
- Treat every `domfiles-*` skill maintained in this repository as a living document.
    - After executing a task using a skill, suggest a concrete edit only when the execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
