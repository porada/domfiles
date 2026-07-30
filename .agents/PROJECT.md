# Project documentation

This document records durable architecture, tooling decisions, compatibility constraints, and maintenance procedures that are not obvious from the source alone. `AGENTS.md` remains authoritative for agent instructions; use this file for project rationale and operational context.

Organize future entries under broad second-level sections so this document can grow without becoming a flat list of unrelated notes.

## Compatibility

### Supported environment

`domfiles` actively targets multiple Apple Silicon–based Macs. Bootstrap and synchronization must work on a fresh installation of macOS 26 or newer with Command Line Tools and Homebrew already installed.

`fish` is the default interactive shell on every managed machine. Shell behavior and setup logic must not assume that Bash or Zsh is the user’s default shell.

## Security

### Zed agent permission model

Treat Zed’s agent sandbox, tool defaults, command allowances, and confirmation overrides as separate security boundaries.

The terminal tool intentionally uses an allow-by-default baseline because the configured agent cannot install additional tools for itself. Narrow confirmation overrides protect hazardous forms such as arbitrary package runners, destructive operations, code-execution hooks, and commands that create or mutate Docker state.

## Tooling

### Codex distribution

Keep `codex` installed through Homebrew rather than declaring `@openai/codex` as a project dependency. The Homebrew cask runs the native executable directly, provisions Fish completions, and remains excluded from dependency installation in CI because `codex` is a development Homebrew dependency.

The npm package adds a large platform-specific native package to every environment that installs the root pnpm dependencies. Lockfile ownership does not outweigh that installation and CI overhead for this machine-level command.

### Repository-scoped commands

Keep `plugins` and `skills` in the root `dependencies`. They provide user-facing commands used outside repository development workflows and are therefore runtime dependencies rather than `devDependencies`.

The corresponding scripts in `bin/` are the stable command interfaces. Their implementations are resolved from the domfiles pnpm workspace so `package.json` and `pnpm-lock.yaml` remain the source of truth for installed versions. Do not install parallel copies through global pnpm state.

The wrappers rely on pnpm’s default `verifyDepsBeforeRun: install` behavior to reconcile missing or outdated project dependencies before executing a command. Revalidate this assumption when changing the pinned pnpm major version or overriding `verifyDepsBeforeRun`.

Projects that require a project-specific command version are expected to declare and invoke that command locally rather than relying on the domfiles command.
