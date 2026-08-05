# Agent instructions

## Overview

- This repository is the home of all my actively used dotfiles (also referred to as `domfiles`).
- This project is public and open source.

## General

- Follow the [supported environment](.agents/PROJECT.md#supported-environment)—including its default-shell requirement—and consult `.agents/PROJECT.md` for non-obvious project rationale and maintenance decisions.
    - Document newly discovered durable project knowledge there.
    - Keep entries alphabetized when their order is irrelevant.

## Scope

- Always consider `.config/fish/local.fish` an active part of domfiles if it exists.
    - Always include `local.fish` in any analysis or execution.
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

- Load every applicable `domfiles-*` skill whose declared file scope intersects the resolved task scope, even when those files were not named explicitly by the user.
- Treat every `domfiles-*` skill maintained in this repository as a living document.
    - After executing a task using a skill, suggest a concrete edit only when the execution reveals missing guidance, ambiguity, an outdated assumption, or avoidable friction.

## Bootstrap and synchronization

- Evaluate the setup instructions in `README.md` against the supported bootstrap environment documented in `.agents/PROJECT.md`.
    - Report commands whose prerequisites are neither guaranteed by that environment nor provisioned earlier by the documented setup flow.
- Always assume this repository is updated via `domfiles sync`.
    - Do not report `domfiles sync` overwriting initial state.
