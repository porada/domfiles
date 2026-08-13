import type { Plugin } from 'prettier';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { format, formatWithCursor, getFileInfo } from 'prettier';
import { expect, expectTypeOf, test } from 'vite-plus/test';
import * as pluginRust from './index.ts';

test('exposes correct public API', () => {
	expectTypeOf(pluginRust).toExtend<Plugin>();
});

const TEST_RUST = `
async fn greet(name:&str)->String{
  format!("Hello, {name}!")
}

async fn main(){
  println!("{}",greet("Dom").await);
}
`;

test('formats Rust files', async () => {
	const options = {
		parser: 'rust' as const,
		plugins: [pluginRust],
	};

	const output = await format(TEST_RUST, options);

	expect(output).toMatchSnapshot();
	await expect(format(output, options)).resolves.toBe(output);
});

test('infers Rust from the hashbang', async () => {
	const directory = await mkdtemp(join(tmpdir(), 'prettier-plugin-rust-'));

	const scriptPath = join(directory, 'script');
	const plainTextPath = join(directory, 'plain-text');

	const options = { plugins: [pluginRust] };

	try {
		await Promise.all([
			writeFile(scriptPath, '#!/usr/bin/env rust-script\n'),
			writeFile(plainTextPath, 'not Rust\n'),
		]);

		await expect(getFileInfo(scriptPath, options)).resolves.toMatchObject({
			inferredParser: 'rust',
		});
		await expect(
			getFileInfo(plainTextPath, options)
		).resolves.not.toMatchObject({
			inferredParser: 'rust',
		});
	} finally {
		await rm(directory, {
			force: true,
			recursive: true,
		});
	}
});

test('preserves native formatter output with `tabWidth` and `useTabs`', async () => {
	const options = {
		parser: 'rust' as const,
		plugins: [pluginRust],
	};

	const expectedOutput = await format(TEST_RUST, options);
	const output = await format(TEST_RUST, {
		...options,
		tabWidth: 8,
		useTabs: true,
	});

	expect(output).toBe(expectedOutput);
});

test('respects `cursorOffset` at the end of the input', async () => {
	const input = 'fn main(){println!("foo");}\n';
	const expectedOutput = 'fn main() {\n    println!("foo");\n}\n';

	const result = await formatWithCursor(input, {
		cursorOffset: input.length,
		parser: 'rust',
		plugins: [pluginRust],
	});

	expect(result).toMatchObject({
		cursorOffset: expectedOutput.length,
		formatted: expectedOutput,
	});
});

test('respects `embeddedLanguageFormatting`', async () => {
	const input = `\`\`\`rust
fn foo(){println!("bar");}
\`\`\`
`;

	const outputs: string[] = [];

	for (const embeddedLanguageFormatting of ['auto', 'off'] as const) {
		outputs.push(
			await format(input, {
				embeddedLanguageFormatting,
				parser: 'markdown',
				plugins: [pluginRust],
			})
		);
	}

	expect(outputs).toMatchInlineSnapshot(`
		[
		  "\`\`\`rust
		fn foo() {
		    println!("bar");
		}
		\`\`\`
		",
		  "\`\`\`rust
		fn foo(){println!("bar");}
		\`\`\`
		",
		]
	`);
});

test('respects `filepath`', async () => {
	const expectedOutput = await format(TEST_RUST, {
		parser: 'rust',
		plugins: [pluginRust],
	});
	const output = await format(TEST_RUST, {
		filepath: 'test.rs',
		plugins: [pluginRust],
	});

	expect(output).toBe(expectedOutput);
});

test('handles empty files', async () => {
	const input = '\n';

	const output = await format(input, {
		parser: 'rust',
		plugins: [pluginRust],
	});

	expect(output).toBe('');
});

test('preserves comment-only files', async () => {
	const input = '// Comment\n';

	const output = await format(input, {
		parser: 'rust',
		plugins: [pluginRust],
	});

	expect(output).toBe(input);
});

test('reports formatting errors', async () => {
	const input = 'fn gen() {}\n';
	const options = {
		parser: 'rust' as const,
		plugins: [pluginRust],
	};

	const errorWithSource = (await format(input, {
		filepath: 'foo/bar.rs',
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
		`"[prettier-plugin-rust] Failed to format \`foo/bar.rs\`:"`
	);

	expect(errorWithoutSource).toBeInstanceOf(Error);
	expect(errorWithoutSource.cause).toBeInstanceOf(Error);
	expect(errorMessageWithoutSource).toMatchInlineSnapshot(
		`"[prettier-plugin-rust] Failed to format:"`
	);
	expect(errorWithoutSource.message).toContain(
		'\n\nThe command `rustfmt --edition 2024 --emit stdout` exited with a non-zero status (1)'
	);
	expect(stderr).toContain('<stdin>:1:4');
	expect(stderr).toContain('reserved keyword `gen`');
	expect(stderr).toContain('r#gen');
	expect(errorWithoutSource.message.endsWith(stderr)).toBe(true);
});
