---
name: vitest-conventions
description: |-
    Use when writing, refactoring, reviewing, or auditing JavaScript or TypeScript tests that use Vitest, including through Vite+ and `vite-plus/test`. This includes fixtures, assertions, snapshots, mocks, and cleanup.

    Also use for Vitest configuration work involving setup files, `setupFiles`, their TypeScript type inclusion, automatic state resets, snapshot serializers, or lint overrides for test subjects and fixtures.

    Don’t use merely to run existing tests or for configuration outside those conventions.

metadata:
    internal: true
---

# Vitest Conventions

## Convention Scope

Preserve the project’s runner, test API import source, meaningful test distinctions, and explicitly chosen conventions. Project policies, including intentional import forms and intentionally shared global or mock state, take precedence over these defaults. Don’t change tooling or configuration without the user’s explicit authorization for that change. For authorized configuration changes, follow [Configuration Conventions](#configuration-conventions).

An explicit change takes precedence when the request also uses review or audit language. For changes, apply the conventions within the authorized scope. Standalone audits remain read-only. Standalone reviews remain read-only apart from corrections covered by the global **Test defect corrections** policy. For defects found while writing, refactoring, or reviewing tests, follow [Small Source Defects](references/small-source-defects.md). Use `human-facing-writing` for test title wording. Leave mechanical matcher and test API choices to the project’s lint rules.

When a lint rule conflicts with a test’s subject or fixture, such as a thenable fixture or a deprecated API under test, write the natural code and add a narrow next-line suppression in the project’s syntax. Don’t use indirection or hand-built framework internals to evade the rule. When the same conflict recurs across tests, propose an override for test files in the shared lint configuration rather than repeating suppressions. Add it only with the user’s explicit authorization. The **Async stubs** guidance under [Assertions](#assertions) takes precedence over this suppression and override policy.

For Vitest API and execution details, use the `vitest` skill when available. Otherwise, consult the project’s test setup and documentation matching its installed Vitest version.

Test package-owned behavior and integration choices rather than unchanged upstream features or individual vendored rules. An integration check may exercise upstream behavior to establish that the package works with that mode. Select cases from the package’s contracts rather than a fixed upstream option checklist. Choose cases so removing the behavior each covers would change its result. When different inputs independently enable the same effect, cover each with the other absent. When several conditions must all hold, make each rejected case fail only one condition. Add cases only where existing ones can’t detect removed behavior, without requiring every combination or mutation testing. Explicit minimum-test policies take precedence, including policies satisfied by small type tests.

## Configuration Conventions

For setup files, `setupFiles` entries, and their TypeScript type inclusion, follow [Setup Files](references/configuration-conventions.md#setup-files). For mock, environment, or global state reset calls in hooks, follow [State Resets](references/configuration-conventions.md#state-resets). React serializer registration follows [Registration](references/component-tests.md#registration). The lint-override conventions remain in [Convention Scope](#convention-scope).

## Test Structure

For JSX fixtures or React component tests, follow [Component Tests](references/component-tests.md).

- **File layout:** Put runtime module tests in `src/<module>/index.test.ts` and entrypoint tests in `src/index.test.ts`, adapting the extension for JSX where needed. Colocate separate type contracts as `index.test-d.ts` beside the tested module, including `src/types/index.test-d.ts` for the `types` module. Put integration tests spanning several packages in a private workspace test package. Import the tested runtime module from `./index.ts`, or `./index.tsx` for JSX modules.
- **File-wide setup:** Declare file-wide constants and helpers at the top of the file, after imports and before tests. A self-contained public API test may come first.
- **Local helpers:** Keep helpers and fake factories in their consuming test file, even when another file duplicates them. Reserve shared mocks directories or packages for independently reusable mocks, such as modules that install a `vi.mock` on import. Create fresh functions or objects when fixtures need distinct identities, even if their behavior is identical.
- **Grouping:** Default to flat tests. When several tests run against each of a few interchangeable implementations, such as a plugin’s parsers or components that share a contract, group them per implementation with `describe.each`. Implementation-specific tests may be registered conditionally inside the group to keep a consistent test order. When a matrix needs `test.each` under [Parameterized Cases](#parameterized-cases), flatten it into rows instead of nested `describe.each` groups.
- **Phases:** Separate meaningful setup, action, and assertion phases with blank lines without mechanical phase comments or forcing tiny tests into three sections. State transition tests may alternate actions and assertions.

## Parameterized Cases

Apply the `test.each` criteria below only to cases with the same observations, following **Separate observations** under [Assertions](#assertions). For snapshots of one input’s outputs under several option values, follow the `outputs` snapshot rule in [Snapshots and Public API](#snapshots-and-public-api) instead.

Keep a single case as an explicit test. Use `test.each` for two or more cases that would otherwise duplicate a flow that isn’t straightforward, such as multi-step setup, a comparison with a reference run, or a state transition. Also use it for two or more cases that each bind a result and make several assertions about it, and for two or more rows that each produce their own external snapshot, even with only one snapshot assertion per row. Choose scalar arrays, tuples, or object rows for the varying inputs rather than imposing one schema on every matrix.

Otherwise, keep straightforward cases as explicit calls and assertions regardless of their number, including when each case makes several one-line assertions with their own calls. Don’t rewrite listed test cases as loops or introduce abstractions merely to reduce repetition. This rule doesn’t govern loops over runtime collections.

Phrase titles so inserted values read naturally instead of appending a generic “with X %s”. Rows may share a title when they don’t fit a natural distinguishing one. When explicit cases share a test, use separate bindings rather than repeatedly assigning to one `let`. Reassignment remains appropriate for genuine state transitions.

Preserve order where it carries meaning, including CLI parameter sequences, plugin precedence, and ignore negations.

## Test Titles

Title public API tests “exposes correct public API” and type surface tests “exposes valid types”.

For option tests, use “respects `<upstream-option>`” for inherited behavior and “supports `<package-option>`” for package-owned behavior.

For every semantically shared test, keep titles and inputs identical across related packages. Related packages implement the same feature family or maintain copies of a shared module, including in separate repositories. Preserve meaningful differences in formats and supported capabilities.

## Fixtures and Names

Keep fixtures small and close to their tests. Name shared sample documents by format, such as `TEST_JSON` or `TEST_MARKDOWN`. Extract a shared constant when tests reuse an input, not merely because a literal can be extracted.

- **Incidental values:** Use `foo`, `bar`, `baz`, `qux`, `quux`, `corge`, `grault`, `garply`, `waldo`, `fred`, `plugh`, `xyzzy`, and `thud`, in that order. This includes incidental filenames such as `foo.md` and `foo bar.md`.
- **Enumeration scope:** Restart the applicable incidental sequence in each test or parameterized table. Advance past the first value only when the test needs several distinct incidental values. Shared fixtures have their own sequence, and tests referencing them preserve their identifiers. Choose unused sequence values when local values must differ from shared fixture values.
- **Sorting:** Use Greek names only when sort order is under test. Use `foo-2` and `foo-10` when numeric and lexical ordering differ.
- **Meaningful values:** Keep realistic type-constrained values, subjects of tests, values in configuration fields (including plugin aliases), domain data, and descriptive keys that distinguish behavior. Don’t replace meaningful globs, markup, numbers, or component fixtures merely to use incidental vocabulary.
- **Markers:** Reserve `Before` and `After` for positions the test checks. Prefer `|` for cursor positions. For comment placement, use `Comment` for a single comment and `Comment 1`, `Comment 2`, and so on for two or more.
- **Variables:** Prefer clear single word names such as `input`, `options`, `output`, `promise`, and `result`. Keep descriptive pairs when needed, such as `errorWithSource` and `errorWithoutSource`. Use `result` and `expectedResult` for structured observations. When a single word name already holds an object, name its identifier with a suffix, such as `parserName` beside `parser`.
- **Transformation rows:** Name row keys `input` and `output`. Destructure `{ input, output: expectedOutput }` when the test also binds the actual result to `output`.
- **CLI values:** Use `param` and `params`. Preserve `process.argv` and a captured `argv` when referring to Node’s process state.

## Assertions

Keep short pure calls inline in `expect()` when one assertion makes their purpose clear. Name substantial operations, reused results, or values needed for several assertions. Move lengthy fixture construction out of `expect()` rather than introducing a wrapper solely to shorten the line.

- **Observation scope:** Assert complete structures when the complete result is the contract. Use partial assertions only for deliberately partial observations, such as a path whose machine-dependent prefix isn’t part of the contract.
- **Identity:** Use `toBe` when reference identity matters, including for a value inside an otherwise partial observation.
- **Separate observations:** Capture distinct observed values and assert them separately. Keep cases with different observations explicit. This includes differences in matcher or negation, or in whether and how cases use snapshot assertions or reference runs. They may share one test when they share setup. Selecting a row’s assertion subject or expected value is fine when every row uses the same matcher, negation, and `.resolves` or `.rejects` modifier. Changing only row data for the same external snapshot assertion or reference comparison doesn’t change the observations. Don’t reduce observations to a row-dependent boolean.
- **Invocation behavior:** Use `vi.fn` when asserting call counts or arguments. Use a plain counter only when it drives the fake’s behavior, such as a recursion guard, and name it `…CallCount`. Clear setup calls before measuring later invocations.
- **Callback assertions:** Assertions inside a callback can observe temporary state. Make the callback a `vi.fn` and assert its call count so a callback that never runs can’t produce a false pass.
- **Async stubs:** Keep idiomatic forms such as `vi.fn().mockResolvedValue(undefined)`. When an async function or stub has nothing else to await, use `await Promise.resolve();` before its result rather than suppressing `typescript/require-await`.

### Error Contracts

Choose the assertion by the behavior being tested:

- **Propagation:** Create a sentinel `error` and assert `.rejects.toBe(error)`. For synchronous capture, assign the caught value to `result`, and compare `result` with the sentinel `error`.
- **Messages:** Use `toThrowErrorMatchingInlineSnapshot` when the message is the contract. Defer a throwing synchronous operation in a callback.
- **Fields or recovery:** Capture errors manually when inspecting fields, `cause`, or subsequent recovery. Verify the instance type before reading fields.

For manual captures, name the `catch` binding `error`, using `caughtError` only when the test already uses `error`, such as for a sentinel.

## Transformation and Parity Checks

Apply these checks where the package owns a transformation, such as a formatter, compiler, or code transform, or an integration contract with another implementation:

- **Reference output:** When matching another implementation or a default mode is the contract, produce the expected result by running the same input through that reference with comparable settings, varying only the behavior under test. Keep any dependency the input requires in the reference run.
- **Idempotence:** Require a second pass in integration tests of a transformation that promises idempotence. Direct module tests may include deliberately chosen idempotence checks. In either case, transform changed output again with the same options and expect it unchanged. Put that assertion directly after the main assertion, without a blank line.
- **Idempotence exemptions:** Skip the second pass when the output equals the input, and in tests of the public API, errors, cursor positions, input-relative range offsets, other source positions, or parity between builds or runtimes. Omit idempotence checks from lifecycle tests, which verify hook invocation, delegation, composition, or state restoration rather than transformation stability. A second invocation intended to verify reuse or state restoration remains a separate behavioral check.
- **Runtime and build parity:** Compare output from an alternate build or runtime, such as a standalone bundle, with the primary implementation’s output by equality rather than keeping a second snapshot.

## Snapshots and Public API

Choose snapshot storage by what the snapshot represents, not its length:

- **External:** Outputs from shared `TEST_<format>` documents and complete generated artifacts, such as configurations, compiled stylesheets, and rendered components in application or component library tests.
- **Inline:** Outputs from a test’s own input and diagnostics, however long. When testing a serializer or formatter itself, keep its output inline beside that input.

Keep a parameterized test’s snapshot external when its rows produce different outputs at the same snapshot call, because they share one inline snapshot location.

Don’t pass snapshot names or hints to `toMatchSnapshot()`. Each snapshot takes its key from its test or `test.each` row title.

A snapshot can be sufficient for its contract. Don’t require additional effect assertions beside it. When snapshotting one input’s outputs under several option values, use a single object named `outputs`, keyed by those values.

For public API tests, use an inline snapshot of `{ ...exports }` by default. Use `toStrictEqual` with `expect.any(…)` only when the snapshot would expose more than the public contract needs. Prettier plugins use the specialized API checks in their dedicated guidance.

## Type Assertions

Inline type assertions in runtime test files are valid when the project’s typecheck includes them, even without Vitest’s typecheck mode.

Use exact `toEqualTypeOf` checks for public type contracts, including chained property checks. Kind-only checks remain suitable for internal declarations and explicit minimum-test policies. Check a declared return type once with `expectTypeOf(fn).returns`, not repeatedly after assigning different runtime results.

Use `/* @ts-expect-error */` when rejection by the type system is itself the contract. Use `as never` for a forbidden value that tests only runtime handling. Don’t duplicate a type assertion between the runtime public API test and a `.test-d.ts` file. Keep the API test’s copy.

When an API allows several runtime forms, validate the form a test needs before narrowing it. A local guard that throws `TypeError` can establish that form. Use deliberate casts for intentionally partial fixture objects, not to conceal an unexpected API shape.

## Cleanup

In tests and `beforeEach`, register restoration immediately after each mutation with `onTestFinished`, using a block body. Register each restoration separately rather than waiting until a hook’s final mutation. `beforeEach` runs in the test’s context. For concurrent tests, use `onTestFinished` from that context rather than the global helper. An existing `try`/`finally` established immediately after the mutation remains valid.

Pair mutations in `beforeAll` with restoration in `afterAll`. `onTestFinished` isn’t available for suite-level setup. For any reset call in a hook, follow [State Resets](references/configuration-conventions.md#state-resets).

Prefer `using` or `await using` for self-disposing resources, such as spies and disposable temporary directories, where the project’s runtime and types support disposal. Don’t use `using` inside hooks. Configure a disposable spy on the next line so the binding is used:

```ts
using spy = vi.spyOn(object, 'method');
spy.mockReturnValue(value);
```

Pair `vi.doMock` with `vi.doUnmock` unless every test in the file explicitly establishes that module’s mock state. `vi.resetModules()` doesn’t clear the mock registry. Reset the module cache only where import state matters, then dynamically import to obtain independent module state.

## Prettier Plugins

For Prettier plugin tests, also follow [Prettier Plugins](references/prettier-plugins.md) for the ecosystem-specific API checks, parser matrices, and application of the general transformation checks.
