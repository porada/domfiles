# Node package manager permissions

This branch specializes the parent [terminal permission policy](terminal-permissions.md) for Node package managers and delegated binaries.

## Apply manager boundaries

- Apply the parent command-owner policy separately to each top-level executable. Use the manager’s native grammar and manager-specific confirmation and denial overrides rather than combining npm, pnpm, and Yarn in one pattern.
- Treat optional Corepack mediation as a wrapper around the selected npm, pnpm, or Yarn manager. Keep Corepack’s own selector operations Corepack-owned.
- Apply the parent prefix and wrapper policy. Keep repeated selector grammar byte-identical within each manager’s patterns.
- Treat pnpm 12’s tool-name routes by resulting behavior. `pnpm add` requests for `bun`, `npm`, `pnpm`, or `yarn` select a package manager, while requests for `deno` or `node` select a runtime. Apply executable-selection confirmation to project and global forms, and to every specifier a tool name carries, including a bare name, a plain version pin such as `yarn@1.22.22`, and an explicit package locator such as `yarn@npm:yarn@1.22.22`. A request for any other package name remains an ordinary package request.
- Keep arbitrary `dlx`, `npx`, `pnpx`, or `pnx` requests outside positive allowances, including requests that provision package managers or runtimes. Preserve the automatic denials for Corepack manager selectors and discovery-looking package-runner operands described in [Zed automatic terminal denials](../../../PROJECT.md#zed-automatic-terminal-denials). Let remaining download-oriented or open-ended runner forms resolve through applicable confirmation rules and the terminal default.
- Classify pnpm’s `shim` command family by effect. Allow exact `pnpm shim --help` and the read-only `list` and `ls` subcommands. Require confirmation for `add`, `remove`, `rm`, and `uninstall`, and leave unknown subcommands confirmable.
- Keep pnpm version forms confirmable because pnpm 12 can reconcile the [package-manager environment lock](../../../PROJECT.md#repository-scoped-commands) before printing its version.
- Treat `.config/zed/settings.json` as the canonical delegated binary inventory.

## Maintain pnpm configuration selectors

- Treat one optional attached `--config.<config-key>=<config-value>` selector as an intentional pnpm-specific namespace allowance at either edge of the verified pre-command selector sequence: immediately after `pnpm`, or after one or more verified selectors and immediately before the command. Keep the `config.` prefix, nonempty key, attached `=`, and nonempty value structural boundaries.
- For a verified selector with a separate value, require a non-option value. Keep an option-looking value eligible only through that selector’s attached `=` form so a malformed selector cannot consume a following configuration selector as its value.
- Leave repeated configuration selectors, a configuration selector followed by another selector, space-separated configuration values, configuration selectors after a command or pass-through boundary, malformed prefixes, and arbitrary non-config options confirmable.
- Apply the selector grammar consistently to pnpm root discovery, verified PATH-only discovery, delegated binaries, and ordinary workflows. Keep the grammar byte-identical within the pnpm owner group where the surrounding syntax role is the same.
- Treat the selector as part of pnpm configuration, not evidence that either the selector or underlying command is safe. This syntax-bounded wildcard intentionally admits unclassified current and future keys and values until a higher-precedence rule covers them. Preserve confirmation and denial overrides for credential, destructive, executable-selection, force, lifecycle-script, network, self-removal, trust, and equivalent hazardous behavior.
- Classify each recognized pnpm setting by behavior even when the namespace wildcard admits it. Require confirmation for `audit.ignorePrune`, `auditIgnorePrune`, `globalShims`, and `remoteSideEffectsCache`, applying the same classification to each accepted naming-case spelling. Deny a command-line selector for `remoteSideEffectsCache.privateKey`. Keep pnpm 12’s `verifyDepsBeforeRun=error` form outside that confirmation family, because it only refuses a stale install rather than weakening a check, mutating the project, or waiting on input.
- Do not infer npm or Yarn parity. Their configuration and option namespaces require independent evidence and explicit policy.
- Do not research, recommend, or add dedicated positive grammar for deprecated or experimental pnpm interfaces unless the user explicitly requests them. Keep known experimental or unknown interfaces behind confirmation. The namespace wildcard does not establish support, safety classification, or endorsement for any individual key or value.
- During every applicable pnpm permission audit or pnpm-version compatibility review, inventory the installed pnpm configuration namespace and inspect the installed parser and command routing. Reassess newly introduced hazardous selectors, route boundaries, and higher-precedence confirmation and denial coverage.

## Maintain delegated binary allowances

- Within each manager owner group, keep each trusted delegated binary in one dedicated `always_allow` pattern containing every verified native invocation form for that manager and binary.
- Do not embed delegated binary names in a broad package-manager workflow pattern or combine distinct binaries in one pattern.
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

## Maintain functional parity

- Treat ordinary npm, pnpm, and Yarn workflows as intentional allowances. Compare workflows by behavior rather than spelling, and give equivalent supported operations equivalent permission treatment. Preserve manager-specific aliases and commands without inventing unsupported counterparts, options, selectors, or separator positions.
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
7. Include applicable manager-specific confirmation and denial forms, download-oriented runners, near-miss separator positions, option-leading executable slots, and unknown binary names in the shared [matcher suite](permission-evaluator.md#compile-and-match-permission-patterns).
8. For changed pnpm selector coverage, test attached nonempty key and value forms immediately after `pnpm` for every supported role and after one or more verified selectors for PATH-only discovery, delegated binaries, and ordinary workflows. Test a configuration selector followed by another selector, repeated selectors, space-separated values, post-command placement, missing or option-looking selector values, malformed prefixes, hazardous selectors, and unknown delegated binaries as near misses or higher-precedence cases.
