# Agent instructions

## Overview

- This repository is the home of all my actively used dotfiles (also referred to as `domfiles`).
- This project is public and open source.

## Agent documentation

| Source | Authority and ownership |
| --- | --- |
| `.config/zed/AGENTS.md` | Defines global defaults. Applicable project agent instructions override it. |
| `AGENTS.md` | Defines repository-wide instructions, scope, documentation authority, and skill routing. |
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
- Do not read or analyze `.config/npm/user.npmrc` (it contains secrets).

## Reporting

- Do not report empty config files.
- Report any cases that would tie this repository to a fixed filesystem location.
    - Do not report `$HOME/*` paths, system paths, or vendor paths.
    - Do not report symlinks created via `domfiles sync`.
    - Do not report `.config/fish/fish_variables`.
    - Do not report documentation.

## Skills

- Load each applicable `domfiles-*` skill immediately before the task pass whose resolved scope intersects its declared file scope. Do not preload domain skills for later passes.
- Treat every `domfiles-*` skill maintained in this repository as a living document.
    - After executing a task using a skill, suggest a concrete edit only when the execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.
