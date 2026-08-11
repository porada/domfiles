# Node package manager permissions

Follow the parent [terminal permission policy](terminal-permissions.md) throughout this branch. Apply this branch whenever `corepack`, `npm`, `npx`, `pnpm`, `pnpx`, `pnx`, `yarn`, or a delegated Node package binary is in scope.

## Apply functional parity

- Treat ordinary npm, pnpm, and Yarn workflows as intentional allowances.
- Give semantically equivalent workflows equivalent permission treatment without inventing unsupported aliases, commands, options, selectors, or separator positions.
- Keep each top-level executable in its own command-owner group. Use the manager’s native grammar and manager-specific confirmation and denial overrides rather than combining npm, pnpm, and Yarn in one pattern.
- Preserve the applicable fixed prefixes and wrappers from the terminal policy. Keep repeated prefix and selector grammar byte-identical within each manager’s patterns.
- Keep arbitrary `dlx`, `npx`, `pnpx`, or `pnx` package selection outside positive allowances. Preserve the automatic denials for Corepack manager selectors and discovery-looking package-runner operands described in [Zed automatic terminal denials](../../../PROJECT.md#zed-automatic-terminal-denials). Let remaining download-oriented or open-ended runner forms resolve through applicable confirmation rules and the terminal default.
- Treat `.config/zed/settings.json` as the canonical delegated binary inventory. Do not copy that inventory into documentation.

## Maintain pnpm configuration selectors

- Treat one or more attached `--config.<key>=<value>` selectors in pnpm’s pre-command global-option region as an intentional pnpm-specific namespace allowance. Keep the `config.` prefix, nonempty key, attached `=`, and nonempty value structural boundaries. Do not generalize this exception to space-separated values, selector placement after a command or pass-through boundary, malformed prefixes, or arbitrary non-config options.
- Apply the selector grammar consistently to pnpm root discovery, verified PATH-only discovery, delegated binaries, and ordinary workflows. Keep repeated grammar byte-identical within the pnpm owner group where the surrounding syntax role is the same.
- Treat the selector as part of pnpm configuration, not evidence that either the selector or underlying command is safe. This syntax-bounded wildcard intentionally admits unclassified current and future keys and values until a higher-precedence rule covers them. Preserve confirmation and denial overrides for credential, destructive, executable-selection, force, lifecycle-script, network, self-removal, trust, and equivalent hazardous behavior.
- Do not infer npm or Yarn parity. Their configuration and option namespaces require independent evidence and explicit policy.
- Do not research, recommend, or add dedicated positive grammar for deprecated or experimental pnpm interfaces unless the user explicitly requests them. The namespace wildcard does not establish support, safety classification, or endorsement for any individual key or value.
- Revalidate pnpm selector parsing, route boundaries, and hazard overrides when the pinned pnpm major version changes. A broader selector-safety audit remains separately scoped from ordinary permission maintenance unless the user includes it.

## Maintain delegated binary allowances

- Keep each trusted delegated binary in one dedicated `always_allow` pattern per supported package manager. One pattern may contain every verified native invocation form for that one manager and binary.
- Do not embed delegated binary names in a broad package-manager workflow pattern, combine distinct binaries in one pattern, or combine different top-level package managers in one pattern.
- Keep the delegated binary inventories identical across npm, pnpm, and Yarn unless verified manager behavior or an explicit user decision establishes an intentional exception.
- Keep unknown binary names outside positive allowances so they resolve through the terminal default and applicable confirmation or denial patterns.
- Keep bounded binary-path discovery in separate manager-owned discovery patterns only for verified PATH-only execution, such as supported pnpm or Yarn `exec which` forms. Treat npm’s `exec which` form as package execution and keep it outside positive allowances.
- Preserve applicable binary-specific confirmation and denial boundaries across direct and package-manager-mediated forms.

## Preserve manager-native invocation forms

The fixed prefixes and `<selectors>` below are optional. `<selectors>` represents the manager’s verified filter, project, working-directory, or workspace selection grammar.

```text
npm <selectors> exec <binary> …
npm <selectors> exec -- <binary> …

pnpm <selectors> <binary> …
pnpm <selectors> -- <binary> …
pnpm <selectors> exec <binary> …
pnpm <selectors> exec -- <binary> …

yarn <selectors> <binary> …
yarn <selectors> exec <binary> …
yarn <selectors> exec -- <binary> …
```

- npm requires `exec`. Preserve its verified optional `--` before the binary. A `--` after the binary remains part of the trailing binary argument grammar.
- pnpm permits direct and `exec` forms. Preserve its verified optional `--` before the binary in either route.
- Yarn Classic permits direct and `exec` forms. Preserve its verified optional `--` after `exec`.
- Apply verified selectors consistently to every supported invocation route for that manager. In particular, Yarn’s working-directory selector applies before direct and `exec` forms, including:

```text
yarn --cwd <directory> <binary> …
yarn --cwd <directory> exec <binary> …
yarn --cwd <directory> exec -- <binary> …
```

- Yarn’s direct form follows `yarn run` resolution and may select a same-named package script before a local binary. This receives the same permission classification because ordinary package-script execution is intentionally allowed.
- Do not infer `yarn -- <binary>` or another unverified separator position from npm or pnpm behavior.
- When npm must forward option-like binary arguments, prefer the pre-binary separator form in generated commands so npm does not consume those arguments itself.

## Maintain ordinary workflow parity

- Compare workflows by their behavior rather than by command spelling. Equivalent authentication, dependency, inspection, maintenance, ownership, packaging, publication, script, and testing operations should receive equivalent treatment when each manager supports them.
- Preserve manager-specific aliases and commands without creating synthetic counterparts.
- Apply confirmation and denial based on the resulting behavior. Equivalent credential, destructive, executable-selection, force, lifecycle-script, self-removal, and trust forms should receive equivalent decisions even when their option names differ.
- Treat a manager-specific capability as an intentional divergence only when verified behavior establishes that no equivalent counterpart exists.

## Validate Node package manager patterns

In addition to the parent terminal validation:

1. Derive the delegated binary inventory independently from the npm, pnpm, and Yarn owner groups without printing complete patterns.
2. Require one dedicated pattern for every supported manager and binary pair.
3. Compare the derived inventories and classify every difference as intentional or a candidate defect.
4. Test every supported direct, `exec`, selector, and separator form for each changed binary pattern.
5. Test verified PATH-only `exec which` forms and require `npm exec which` to remain outside positive allowances.
6. Require representative Corepack manager selectors and discovery-looking package-runner operands from the linked rationale to resolve to `deny`.
7. Test remaining download-oriented runners, near-miss separator positions, option-leading executable slots, and unknown binary names.
8. Resolve applicable confirmation and denial overrides through the complete effective permission workflow.
9. Verify that repeated prefix and selector grammar remains byte-identical within each manager’s patterns.
10. For changed pnpm selector coverage, test attached nonempty key and value forms before root discovery, PATH-only discovery, delegated binaries, and ordinary workflows. Test space-separated values, post-command placement, malformed prefixes, hazardous selectors, and unknown delegated binaries as near misses or higher-precedence cases.
11. When the pinned pnpm major version changes, inspect the installed parser and command routing before retaining the namespace allowance, then revalidate applicable confirmation and denial overrides.
