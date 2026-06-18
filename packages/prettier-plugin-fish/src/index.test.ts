import type { Plugin } from 'prettier';
import { format } from 'prettier';
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

test('handles empty files', async () => {
	const TEST_FISH = '\n';

	const output = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toBe('');
});
