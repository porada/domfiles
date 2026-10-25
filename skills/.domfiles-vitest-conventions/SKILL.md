---
name: vitest-conventions
description: |-
    Use when writing, refactoring, reviewing, or auditing JavaScript or TypeScript package tests with any test runner. This includes fixtures, assertions, snapshots, mocks, and cleanup.

    Don’t use merely to run existing tests or for tasks limited to configuring the test runner.

metadata:
    internal: true
---

# Vitest Conventions

## Convention Scope

Some guidance is written with Vitest in mind. When working with another test runner, use supported equivalent APIs and techniques that preserve the intended test behavior.

Preserve the project’s runner, test API import source, meaningful test distinctions, and explicitly chosen conventions. Project policies, including intentional import forms and intentionally shared global or mock state, take precedence over these defaults. Don’t change tooling or configuration to satisfy these conventions.

An explicit change takes precedence when the request also uses review or audit language. For changes, apply the conventions within the authorized test scope. For standalone reviews and audits, assess them without editing. Use `human-facing-writing` for test title wording. Leave mechanical matcher and test API choices to the project’s lint rules.

For Vitest API and execution details, use the `vitest` skill when available. Otherwise, consult the project’s test setup and documentation matching its installed Vitest version.

Test package-owned behavior and integration choices rather than unchanged upstream features or individual vendored rules. An integration check may exercise upstream behavior to establish that the package works with that mode. Select cases from the package’s contracts rather than a fixed upstream option checklist. Explicit minimum-test policies take precedence, including policies satisfied by small type tests.

## Test Structure

- **File layout:** Put runtime module tests in `src/<module>/index.test.ts` and entrypoint tests in `src/index.test.ts`, adapting the extension for JSX where needed. Colocate separate type contracts as `index.test-d.ts` beside the tested module, including `src/types/index.test-d.ts` for the `types` module. Put integration tests spanning several packages in a private workspace test package. Import the tested runtime module from `./index.ts`, or `./index.tsx` for JSX modules.
- **File-wide setup:** Declare file-wide constants and helpers at the top of the file, after imports and before tests. A self-contained public API test may come first.
- **Local helpers:** Keep helpers and fake factories in their consuming test file, even when another file duplicates them. Reserve shared mocks directories or packages for independently reusable mocks, such as modules that install a `vi.mock` on import. Create fresh functions or objects when fixtures need distinct identities, even if their behavior is identical.
- **Grouping:** Default to flat tests. Multi-parser plugins may use the grouping exception in [Prettier Plugins](references/prettier-plugins.md). Flatten other multi-dimensional matrices into `test.each` rows instead of nested `describe.each` groups or nested loops.
- **Phases:** Separate meaningful setup, action, and assertion phases with blank lines without mechanical phase comments or forcing tiny tests into three sections. State transition tests may alternate actions and assertions.

## Parameterized Cases

Use `test.each` when a case needs setup, several assertions, or separate row-level diagnosis. Otherwise, prefer consecutive assertions for a few one-line examples of one behavior, and a loop for a larger uniform set whose iterations each contain one assertion. Choose scalar arrays, tuples, or object rows for the varying inputs rather than imposing one schema on every matrix.

When the runner has no parameterization API and lint rejects generated tests, run rows inside one test with independent fixtures and row-identifying assertion messages.

Phrase titles so inserted values read naturally instead of appending a generic “with X %s”. Rows may share a title when they don’t fit a natural distinguishing one. Give independent cases their own tests or rows rather than repeatedly assigning to one `let`. Reassignment remains appropriate for genuine state transitions.

Preserve order where it carries meaning, including CLI parameter sequences, plugin precedence, and ignore negations.

## Test Titles

Title public API tests “exposes correct public API” and type surface tests “exposes valid types”.

For option tests, use “respects `<upstream-option>`” for inherited behavior and “supports `<package-option>`” for package-owned behavior.

For every semantically shared test, keep titles and inputs identical across related packages. Related packages implement the same feature family or maintain copies of a shared module, including in separate repositories. Preserve meaningful differences in formats and supported capabilities.

## Fixtures and Names

Keep fixtures small and close to their tests. Name shared sample documents by format, such as `TEST_JSON` or `TEST_MARKDOWN`. Extract a shared constant when tests reuse an input, not merely because a literal can be extracted.

- **Incidental values:** Use `foo`, `bar`, `baz`, `qux`, `quux`, `corge`, `grault`, `garply`, `waldo`, `fred`, `plugh`, `xyzzy`, and `thud`, in that order. This includes incidental filenames such as `foo.md` and `foo bar.md`.
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
- **Separate observations:** Capture distinct observed values and assert them separately. When rows require different matchers or observations, split them into separate tests or tables by expected behavior. Selecting a row’s assertion subject or expected value is fine when every row uses the same matcher, negation, and `.resolves` or `.rejects` modifier. Don’t reduce observations to a row-dependent boolean.
- **Invocation behavior:** Use `vi.fn` when asserting call counts or arguments. Use a plain counter only when it drives the fake’s behavior, such as a recursion guard, and name it `…CallCount`. Clear setup calls before measuring later invocations.
- **Callback assertions:** Assertions inside a callback can observe temporary state. Make the callback a `vi.fn` and assert its call count so a callback that never runs can’t produce a false pass.
- **Async stubs:** Keep `vi.fn().mockResolvedValue(undefined)` when plain function alternatives conflict with the project’s async lint rules.

### Error Contracts

Choose the assertion by the behavior being tested:

- **Propagation:** Create a sentinel `error` and assert `.rejects.toBe(error)`. For synchronous capture, assign the caught value to `result`, and compare `result` with the sentinel `error`. With `node:assert`, pass an identity validation function such as `(caughtError) => caughtError === error`. Passing an error instance compares properties, not identity.
- **Messages:** Use `toThrowErrorMatchingInlineSnapshot` when the message is the contract. Defer a throwing synchronous operation in a callback.
- **Fields or recovery:** Capture errors manually when inspecting fields, `cause`, or subsequent recovery. Verify the instance type before reading fields.

## Transformation and Parity Checks

Apply these checks where the package owns a transformation, such as a formatter, compiler, or code transform, or an integration contract with another implementation:

- **Reference output:** When matching another implementation or a default mode is the contract, produce the expected result by running the same input through that reference with comparable settings, varying only the behavior under test. Keep any dependency the input requires in the reference run.
- **Idempotence:** Require a second pass in integration tests of a transformation that promises idempotence. Direct module tests may include deliberately chosen idempotence checks. In either case, transform changed output again with the same options and expect it unchanged. Put that assertion directly after the main assertion, without a blank line.
- **Idempotence exemptions:** Skip the second pass when the output equals the input, and in tests of the public API, errors, cursor positions, input-relative range offsets, other source positions, or parity between builds or runtimes. Omit idempotence checks from lifecycle tests, which verify hook invocation, delegation, composition, or state restoration rather than transformation stability. A second invocation intended to verify reuse or state restoration remains a separate behavioral check.
- **Runtime and build parity:** Compare output from an alternate build or runtime, such as a standalone bundle, with the primary implementation’s output by equality rather than keeping a second snapshot.

## Snapshots and Public API

Choose snapshot storage by what the snapshot represents, not its length:

- **External:** Outputs from shared `TEST_<format>` documents and complete generated artifacts, such as configurations, compiled stylesheets, and rendered components.
- **Inline:** Outputs from a test’s own input and diagnostics, however long.

Keep a parameterized test’s snapshot external when its rows produce different outputs at the same snapshot call, because they share one inline snapshot location.

A snapshot can be sufficient for its contract. Don’t require additional effect assertions beside it. When one input is transformed under several option values, snapshot a single object named `outputs`, keyed by those values.

For public API tests, use an inline snapshot of `{ ...exports }` by default. Use `toStrictEqual` with `expect.any(…)` only when the snapshot would expose more than the public contract needs. Prettier plugins use the specialized API checks in their dedicated guidance.

## Type Assertions

Inline type assertions in runtime test files are valid when the project’s typecheck includes them, even without Vitest’s typecheck mode.

Use exact `toEqualTypeOf` checks for public type contracts, including chained property checks. Kind-only checks remain suitable for internal declarations and explicit minimum-test policies. Check a declared return type once with `expectTypeOf(fn).returns`, not repeatedly after assigning different runtime results.

Use `/* @ts-expect-error */` when rejection by the type system is itself the contract. Use `as never` for a forbidden value that tests only runtime handling. Don’t duplicate a type assertion between the runtime public API test and a `.test-d.ts` file. Keep the API test’s copy.

When an API allows several runtime forms, validate the form a test needs before narrowing it. A local guard that throws `TypeError` can establish that form. Use deliberate casts for intentionally partial fixture objects, not to conceal an unexpected API shape.

## Cleanup

Register restoration immediately after each mutation with `onTestFinished`, using a block body. An existing `try`/`finally` established immediately after the mutation remains valid.

Prefer `using` or `await using` for self-disposing resources, such as spies and disposable temporary directories, where the project’s runtime and types support disposal. Don’t use `using` inside hooks. Configure a disposable spy on the next line so the binding is used:

```ts
using spy = vi.spyOn(object, 'method');
spy.mockReturnValue(value);
```

Pair `vi.doMock` with `vi.doUnmock` unless every test in the file explicitly establishes that module’s mock state. `vi.resetModules()` doesn’t clear the mock registry. Reset the module cache only where import state matters, then dynamically import to obtain independent module state.

## Prettier Plugins

For Prettier plugin tests, also follow [Prettier Plugins](references/prettier-plugins.md) for the ecosystem-specific API checks, parser grouping exception, and application of the general transformation checks.
