import type { Plugin } from 'prettier';
import { mkdtempDisposable, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { format, formatWithCursor, getFileInfo } from 'prettier';
import { expect, expectTypeOf, test, vi } from 'vite-plus/test';
import * as pluginFish from './index.ts';

test('exposes correct public API', () => {
	expectTypeOf(pluginFish).toExtend<Plugin>();

	expect(pluginFish).toHaveProperty('languages');

	expect(pluginFish).toHaveProperty('parsers');
	expect(pluginFish.parsers).toHaveProperty('fish');

	expect(pluginFish).toHaveProperty('printers');
	expect(pluginFish.printers).toHaveProperty('fish-text');
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
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'prettier-plugin-fish-')
	);

	const scriptPath = join(directory.path, 'foo');
	const plainTextPath = join(directory.path, 'bar');

	const options = { plugins: [pluginFish] };

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
});

test('preserves escaped trailing whitespace from `fish_indent`', async () => {
	const input = "printf '<%s>\\n' foo\\ \n";

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
		tabWidth: 8,
		useTabs: true,
	};

	const expectedOutput = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});
	const output = await format(TEST_FISH, options);

	expect(output).toBe(expectedOutput);
	await expect(format(output, options)).resolves.toBe(output);
});

test('leaves input unchanged for partial ranges', async () => {
	const input = 'echo    foo\necho    bar\necho    baz\n';
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	await expect(format(input, { ...options, rangeEnd: 12 })).resolves.toBe(
		input
	);
	await expect(
		format(input, { ...options, rangeEnd: 24, rangeStart: 12 })
	).resolves.toBe(input);
	await expect(format(input, { ...options, rangeStart: 12 })).resolves.toBe(
		input
	);
});

test('formats without a pragma when `requirePragma` is enabled', async () => {
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
		requirePragma: true,
	};

	const output = await format('echo    foo\n', options);

	expect(output).toBe('echo foo\n');
	await expect(format(output, options)).resolves.toBe(output);
});

test.each(['@noformat', '@noprettier'])(
	'formats despite `%s` when `checkIgnorePragma` is enabled',
	async (pragma) => {
		const options = {
			checkIgnorePragma: true,
			parser: 'fish' as const,
			plugins: [pluginFish],
		};

		const output = await format(`# ${pragma}\necho    foo\n`, options);

		expect(output).toBe(`# ${pragma}\necho foo\n`);
		await expect(format(output, options)).resolves.toBe(output);
	}
);

test('formats without adding a pragma when `insertPragma` is enabled', async () => {
	const options = {
		insertPragma: true,
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	const output = await format('echo    foo\n', options);

	expect(output).toBe('echo foo\n');
	await expect(format(output, options)).resolves.toBe(output);
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
	const options = {
		embeddedLanguageFormatting: 'auto' as const,
		parser: 'markdown' as const,
		plugins: [pluginFish],
	};

	const outputs = {
		auto: await format(input, options),
		off: await format(input, {
			...options,
			embeddedLanguageFormatting: 'off',
		}),
	};

	expect(outputs).toMatchInlineSnapshot(`
		{
		  "auto": "\`\`\`fish
		echo foo
		\`\`\`
		",
		  "off": "\`\`\`fish
		echo    foo
		\`\`\`
		",
		}
	`);
	await expect(format(outputs.auto, options)).resolves.toBe(outputs.auto);
});

test('respects `filepath`', async () => {
	const options = {
		filepath: 'foo.fish',
		plugins: [pluginFish],
	};

	const expectedOutput = await format(TEST_FISH, {
		parser: 'fish',
		plugins: [pluginFish],
	});
	const output = await format(TEST_FISH, options);

	expect(output).toBe(expectedOutput);
	await expect(format(output, options)).resolves.toBe(output);
});

test('formats newline-only files as empty', async () => {
	const input = '\n';
	const options = {
		parser: 'fish' as const,
		plugins: [pluginFish],
	};

	const output = await format(input, options);

	expect(output).toBe('');
	await expect(format(output, options)).resolves.toBe(output);
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

		expect(errorWithSource).toBeInstanceOf(Error);
		expect(errorWithSource.cause).toBeInstanceOf(Error);
		expect(errorWithSource.message.split('\n')[0]).toMatchInlineSnapshot(
			`"[prettier-plugin-fish] Failed to format \`foo/bar.fish\`:"`
		);

		expect(errorWithoutSource).toBeInstanceOf(Error);
		expect(errorWithoutSource.cause).toBeInstanceOf(Error);
		expect(errorWithoutSource.message.split('\n')[0]).toMatchInlineSnapshot(
			`"[prettier-plugin-fish] Failed to format:"`
		);
		expect(errorWithoutSource.message).toContain('\n\n');
	} finally {
		vi.unstubAllEnvs();
	}
});
