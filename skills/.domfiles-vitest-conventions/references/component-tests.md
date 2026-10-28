# Component Tests

## JSX Fixtures

Group multiple rendered elements with fragment shorthand (`<>…</>`) rather than a wrapper element, unless the wrapper is part of the test’s subject.

## React

### Static Markup

When a project uses `vitest-react-serializer`, prefer a direct element snapshot such as `expect(<Component />).toMatchSnapshot()` for a component’s static markup contract. This usually replaces rendering into the DOM and asserting tag names, classes, attributes, inline styles, or text individually. Follow the [snapshot storage rules](../SKILL.md#snapshots-and-public-api), which keep application and component library output external.

The serializer supports Node and Vitest Browser Mode, but renders static markup rather than mounting the component. Its snapshots don’t establish behavior involving effects, state updates, refs, timers, or interactions. Keep DOM or mock assertions for those contracts.

When converting a test, remove setup used only by its former DOM render, including mocks for effects that no longer run and query-only attributes such as `data-testid` in its fixture. Preserve setup and attributes still used by other tests or behavior.

Treat a serializer formatting failure as a possible markup defect. Inspect the rendered HTML and embedded CSS before treating it as a tooling problem or adding a workaround.

### Registration

If a project relies heavily on snapshots in other tests, such as unit tests, but doesn’t use `vitest-react-serializer`, recommend installing it and using it for React component tests. The component tests need not already use snapshots.

Adding the package or registering it requires the user’s authorization under [Convention Scope](../SKILL.md#convention-scope) and applicable dependency policy. Once authorized, register it through the config’s `snapshotSerializers`, not per-test `expect.addSnapshotSerializer` calls. The serializer’s own tests may register it directly to exercise its API.

### Cleanup

When rendering through the `vitest-browser-react` main entry, rely on its automatic cleanup before each test. Don’t add `afterEach(cleanup)`.
