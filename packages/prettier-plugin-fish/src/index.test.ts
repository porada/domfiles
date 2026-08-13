import type { Plugin } from 'prettier';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { format, formatWithCursor, getFileInfo } from 'prettier';
import { expect, expectTypeOf, test, vi } from 'vite-plus/test';
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
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	const output = await format(TEST_FISH, options);

	expect(output).toMatchSnapshot();
	await expect(format(output, options)).resolves.toBe(output);
});

test('infers Fish from the hashbang', async () => {
	const directory = await mkdtemp(join(tmpdir(), 'prettier-plugin-fish-'));

	const scriptPath = join(directory, 'script');
	const plainTextPath = join(directory, 'plain-text');

	const options = { plugins: [pluginFish] };

	try {
		await Promise.all([
			writeFile(scriptPath, '#!/usr/bin/env fish\n'),
			writeFile(plainTextPath, 'not Fish\n'),
		]);

		await expect(getFileInfo(scriptPath, options)).resolves.toMatchObject({
			inferredParser: 'fish',
		});
		await expect(
			getFileInfo(plainTextPath, options)
		).resolves.not.toMatchObject({
			inferredParser: 'fish',
		});
	} finally {
		await rm(directory, {
			force: true,
			recursive: true,
		});
	}
});

test('preserves escaped trailing whitespace from `fish_indent`', async () => {
	const input = "printf '<%s>\\n' test\\ \n";

	const output = await format(input, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toBe(input);
});

test('preserves native formatter output with `tabWidth` and `useTabs`', async () => {
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	const expectedOutput = await format(TEST_FISH, options);
	const output = await format(TEST_FISH, {
		...options,
		tabWidth: 8,
		useTabs: true,
	});

	expect(output).toBe(expectedOutput);
});

test('respects `cursorOffset` at the end of the input', async () => {
	const input = 'echo    foo\n';
	const expectedOutput = 'echo foo\n';

	const result = await formatWithCursor(input, {
		cursorOffset: input.length,
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(result).toMatchObject({
		cursorOffset: expectedOutput.length,
		formatted: expectedOutput,
	});
});

test('respects `embeddedLanguageFormatting`', async () => {
	const input = `\`\`\`fish
echo    foo
\`\`\`
`;

	const outputs: string[] = [];

	for (const embeddedLanguageFormatting of ['auto', 'off'] as const) {
		outputs.push(
			await format(input, {
				embeddedLanguageFormatting,
				parser: 'markdown',
				plugins: [pluginFish],
			})
		);
	}

	expect(outputs).toMatchInlineSnapshot(`
		[
		  "\`\`\`fish
		echo foo
		\`\`\`
		",
		  "\`\`\`fish
		echo    foo
		\`\`\`
		",
		]
	`);
});

test('respects `filepath`', async () => {
	const expectedOutput = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});
	const output = await format(TEST_FISH, {
		filepath: 'test.fish',
		plugins: [pluginFish],
	});

	expect(output).toBe(expectedOutput);
});

test('handles empty files', async () => {
	const input = '\n';

	const output = await format(input, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toBe('');
});

test('preserves comment-only files', async () => {
	const input = '# Comment\n';

	const output = await format(input, {
		parser: 'fish',
		plugins: [pluginFish],
	});

	expect(output).toBe(input);
});

test('reports formatting errors', async () => {
	const input = 'echo foo\n';
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	vi.stubEnv('PATH', '');

	try {
		const errorWithSource = (await format(input, {
			filepath: 'foo/bar.fish',
			...options,
		}).catch((error: unknown) => error)) as Error;
		const errorWithoutSource = (await format(input, options).catch(
			(error: unknown) => error
		)) as Error;

		const [errorMessageWithSource] = errorWithSource.message.split('\n');
		const [errorMessageWithoutSource] =
			errorWithoutSource.message.split('\n');

		expect(errorWithSource).toBeInstanceOf(Error);
		expect(errorWithSource.cause).toBeInstanceOf(Error);
		expect(errorMessageWithSource).toMatchInlineSnapshot(
			`"[prettier-plugin-fish] Failed to format \`foo/bar.fish\`:"`
		);

		expect(errorWithoutSource).toBeInstanceOf(Error);
		expect(errorWithoutSource.cause).toBeInstanceOf(Error);
		expect(errorMessageWithoutSource).toMatchInlineSnapshot(
			`"[prettier-plugin-fish] Failed to format:"`
		);
		expect(errorWithoutSource.message).toContain('\n\n');
	} finally {
		vi.unstubAllEnvs();
	}
});
