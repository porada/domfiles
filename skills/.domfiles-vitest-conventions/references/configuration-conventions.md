# Configuration Conventions

## Setup Files

Choose the setup structure by the file’s contents:

- **Imports only:** Move runtime imports into the config’s `setupFiles` as package names or paths, preserving their order. Vitest resolves these entries from the configured project root, so adjust relative paths to keep the same targets. Preserve type-only imports and any types supplied by runtime imports through [Type Availability](#type-availability), then delete the setup file.
- **Other code:** Move the project’s other `setupFiles` entries into the setup file as imports, preserving their order and resolved targets. Keep the setup file itself in `setupFiles`, and leave entries supplied by a shared preset in place.

Preserve the existing setup execution order and sequencing semantics when moving entries or imports.

### Type Availability

When a moved import supplies types, including matcher augmentations through `declare module`, keep those types available to the project’s typecheck:

- **Declaration files:** Add packages that expose declaration files to `compilerOptions.types` in the tsconfig. Preserve the effective existing entries, including inherited ones. When introducing a `types` list, also retain any type packages the project previously included automatically, because an explicit list restricts that inclusion.
- **TypeScript source only:** Add `import type {} from '<package>';` to the config file, and ensure that the config participates in the project’s typecheck. TypeScript resolves declaration files through `types` and reports `TS2688` for a source-only package there.

Don’t import a setup package for its runtime side effects in the config to preserve its types. That executes the package in the process that loads the config, not in the test runner.

## State Resets

Before keeping a mock, environment, or global state reset call in a hook, check the effective configuration, shared presets, and installed Vitest defaults. Remove hook reset calls that duplicate resets the effective configuration already performs, provided their timing serves no separate cleanup requirement.

When several test files repeat required reset calls that the configuration doesn’t perform, propose the matching configuration options:

- **`clearMocks`:** Clears mock call history while preserving implementations. Vitest 5 enables it by default.
- **`mockReset`:** Performs `vi.resetAllMocks()`, restoring mock implementations to their initial behavior and clearing call history and queued one-time implementations. A mock created with `vi.fn(impl)` returns to `impl`, while one created with `vi.fn()` returns to an empty function that returns `undefined`. Disabled by default.
- **`restoreMocks`:** Restores original spied-on properties. This doesn’t itself clear retained mock history or reset retained mock implementations.
- **`unstubEnvs`:** Restores environment values changed through `vi.stubEnv`.
- **`unstubGlobals`:** Restores globals changed through `vi.stubGlobal`.

These options run before each test attempt, before `beforeEach`, rather than at teardown. Don’t replace required [cleanup](../SKILL.md#cleanup) merely because an option is enabled. Check suite-owned state before proposing restoration or unstubbing, because those options can undo `beforeAll` setup before the first test. Keep resets between phases of a test when they isolate later observations.

Add configuration options only after the user authorizes the change under [Convention Scope](../SKILL.md#convention-scope).
