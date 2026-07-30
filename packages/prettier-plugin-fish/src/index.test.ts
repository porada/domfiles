import type { Plugin } from 'prettier';
import { format, formatWithCursor } from 'prettier';
import { expect, expectTypeOf, test } from 'vite-plus/test';
import * as pluginFish from './index.ts';

test('exposes correct public API', () => {
	expectTypeOf(pluginFish).toExtend<Plugin>();
});

const TEST_FISH = `
set name ""

for param in $argv
  set value (string replace --regex '^--name=' '' -- "$param")

  if test "$value" != "$param"
    set name "$value"
  end
end

echo "$name" > /dev/null 2>&1
`;

test('formats Fish files', async () => {
	const output = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toMatchSnapshot();
});

test('formats `.fish` files without an explicit parser', async () => {
	const output = await format(TEST_FISH, {
		filepath: 'test.fish',
		plugins: [pluginFish],
	});

	expect(output).toMatchSnapshot();
});

test('handles empty files', async () => {
	const TEST_FISH = '\n';

	const output = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toBe('');
});

test('preserves a cursor at the end of the source', async () => {
	const source = 'echo    test\n';
	const result = await formatWithCursor(source, {
		cursorOffset: source.length,
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(result).toMatchObject({
		cursorOffset: 10,
		formatted: 'echo test\n',
	});
});
