import type { Plugin } from 'prettier';
import { format, formatWithCursor } from 'prettier';
import { expect, expectTypeOf, test } from 'vite-plus/test';
import * as pluginTOML from './index.ts';

test('exposes correct public API', () => {
	expectTypeOf(pluginTOML).toExtend<Plugin>();
});

const TEST_TOML = `
[package]
name="prettier-plugin-toml"
version="0.0.0"

[dependencies]
tinyexec={version="1.0.0"}
`;

test('formats TOML files', async () => {
	const options = {
		parser: 'toml' as const,
		plugins: [pluginTOML],
	};

	const output = await format(TEST_TOML, options);

	expect(output).toMatchSnapshot();
	await expect(format(output, options)).resolves.toBe(output);
});

test('preserves native formatter output with `tabWidth` and `useTabs`', async () => {
	const options = {
		parser: 'toml' as const,
		plugins: [pluginTOML],
	};

	const expectedOutput = await format(TEST_TOML, options);
	const output = await format(TEST_TOML, {
		...options,
		tabWidth: 8,
		useTabs: true,
	});

	expect(output).toBe(expectedOutput);
});

test('respects `cursorOffset` at the end of the input', async () => {
	const input = 'foo="bar"\n';
	const expectedOutput = 'foo = "bar"\n';

	const result = await formatWithCursor(input, {
		cursorOffset: input.length,
		parser: 'toml',
		plugins: [pluginTOML],
	});

	expect(result).toMatchObject({
		cursorOffset: expectedOutput.length,
		formatted: expectedOutput,
	});
});

test('respects `embeddedLanguageFormatting`', async () => {
	const input = `\`\`\`toml
foo={bar="baz"}
\`\`\`
`;

	const outputs: string[] = [];

	for (const embeddedLanguageFormatting of ['auto', 'off'] as const) {
		outputs.push(
			await format(input, {
				embeddedLanguageFormatting,
				parser: 'markdown',
				plugins: [pluginTOML],
			})
		);
	}

	expect(outputs).toMatchInlineSnapshot(`
		[
		  "\`\`\`toml
		foo = { bar = "baz" }
		\`\`\`
		",
		  "\`\`\`toml
		foo={bar="baz"}
		\`\`\`
		",
		]
	`);
});

test('respects `filepath`', async () => {
	const expectedOutput = await format(TEST_TOML, {
		parser: 'toml',
		plugins: [pluginTOML],
	});
	const output = await format(TEST_TOML, {
		filepath: 'test.toml',
		plugins: [pluginTOML],
	});

	expect(output).toBe(expectedOutput);
});

test('handles empty files', async () => {
	const input = '\n';

	const output = await format(input, {
		parser: 'toml',
		plugins: [pluginTOML],
	});

	expect(output).toBe('');
});

test('preserves comment-only files', async () => {
	const input = '# Comment\n';

	const output = await format(input, {
		parser: 'toml',
		plugins: [pluginTOML],
	});

	expect(output).toBe(input);
});

test('reports formatting errors', async () => {
	const input = 'foo =\n';
	const options = {
		parser: 'toml' as const,
		plugins: [pluginTOML],
	};

	const errorWithSource = (await format(input, {
		filepath: 'foo/bar.toml',
		...options,
	}).catch((error: unknown) => error)) as Error;
	const errorWithoutSource = (await format(input, options).catch(
		(error: unknown) => error
	)) as Error;

	const [errorMessageWithSource] = errorWithSource.message.split('\n');
	const [errorMessageWithoutSource] = errorWithoutSource.message.split('\n');
	const { stderr } = (
		errorWithoutSource.cause as { output: { stderr: string } }
	).output;

	expect(errorWithSource).toBeInstanceOf(Error);
	expect(errorWithSource.cause).toBeInstanceOf(Error);
	expect(errorMessageWithSource).toMatchInlineSnapshot(
		`"[prettier-plugin-toml] Failed to format \`foo/bar.toml\`:"`
	);

	expect(errorWithoutSource).toBeInstanceOf(Error);
	expect(errorWithoutSource.cause).toBeInstanceOf(Error);
	expect(errorMessageWithoutSource).toMatchInlineSnapshot(
		`"[prettier-plugin-toml] Failed to format:"`
	);
	expect(errorWithoutSource.message).toContain(
		'\n\nThe command `taplo fmt -` exited with a non-zero status (1)'
	);
	expect(stderr).toContain('-:1:6');
	expect(stderr).toContain('expected value');
	expect(errorWithoutSource.message.endsWith(stderr)).toBe(true);
});
