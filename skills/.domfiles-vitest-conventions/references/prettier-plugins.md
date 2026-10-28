# Prettier Plugins

## Public API

Use `expectTypeOf(plugin).toExtend<Plugin>()` for the plugin object. Check each top-level runtime export in the plugin’s public contract with `toHaveProperty`, including `defaultOptions`, `languages`, `options`, `parsers`, and `printers` when part of that contract. Also check each expected parser and printer name within its map. Don’t replace these checks with a complete strict comparison of the plugin object.

## Formatting Contracts

Apply the general [transformation and parity checks](../SKILL.md#transformation-and-parity-checks) to Prettier’s interfaces:

- **Baselines:** For native Prettier compatibility, use the output from formatting without the plugin as `expectedOutput`. If the parser requires the plugin, retain it in the baseline and compare its normal invocation with the option variation.
- **Idempotence:** Prettier formatting is an idempotent contract. Use `await expect(format(output, options)).resolves.toBe(output);` for the second formatting pass. The idempotence exemptions for source positions include tests of `cursorOffset` and input-relative `rangeEnd` and `rangeStart` offsets.
- **Standalone parity:** Compare standalone output with the Node `format` output.

## Parser Matrices and Fixtures

When tests across parsers need `test.each` under [Parameterized Cases](../SKILL.md#parameterized-cases), use `test.each(<format>_PARSER_NAMES)` with titles ending “with the `%s` parser”.

For parser lifecycle fixtures, narrow parser entries to direct parser objects when that’s the form the test needs. Use `as unknown as ParserOptions` for intentionally partial parser options.
