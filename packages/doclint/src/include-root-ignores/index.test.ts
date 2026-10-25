import type { Linter } from 'eslint';
import { ESLint } from 'eslint';
import {
	mkdirSync,
	mkdtempSync,
	rmSync,
	symlinkSync,
	writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative } from 'node:path';
import { expect, onTestFinished, test, vi } from 'vite-plus/test';
import includeRootIgnores from './index.ts';

const TEST_MARKDOWN = '# Foo\n';

function createFixture(files: Record<string, string>): string {
	const root = mkdtempSync(join(tmpdir(), 'doclint-'));

	onTestFinished(() => {
		rmSync(root, { force: true, recursive: true });
	});

	for (const [path, content] of Object.entries(files)) {
		mkdirSync(dirname(join(root, path)), { recursive: true });
		writeFileSync(join(root, path), content);
	}

	return root;
}

function setParams(params: string[]): void {
	const { argv } = process;

	process.argv = [...argv.slice(0, 2), ...params];

	onTestFinished(() => {
		process.argv = argv;
	});
}

async function loadConfig(
	cwd: string,
	params: string[] = []
): Promise<Linter.Config[]> {
	setParams(params);
	using spy = vi.spyOn(process, 'cwd');
	spy.mockReturnValue(cwd);
	vi.resetModules();

	const { default: config } = await import('../../eslint.config.ts');

	return config;
}

async function lint(cwd: string): Promise<string[]> {
	const eslint = new ESLint({
		cwd,
		overrideConfig: await loadConfig(cwd),
		overrideConfigFile: true,
	});
	const results = await eslint.lintFiles(['.']);

	return results.map(({ filePath }) => relative(cwd, filePath)).toSorted();
}

test('excludes paths ignored by the root `.gitignore`', async () => {
	const cwd = createFixture({
		'.gitignore': 'build/\n*.draft.md\n!keep.draft.md\n',
		'build/foo.md': TEST_MARKDOWN,
		'docs/foo.draft.md': TEST_MARKDOWN,
		'foo.md': TEST_MARKDOWN,
		'keep.draft.md': TEST_MARKDOWN,
	});

	await expect(lint(cwd)).resolves.toStrictEqual(['foo.md', 'keep.draft.md']);
});

test('lints every Markdown file without a root `.gitignore`', async () => {
	const cwd = createFixture({
		'docs/foo.md': TEST_MARKDOWN,
		'foo.md': TEST_MARKDOWN,
	});

	await expect(lint(cwd)).resolves.toStrictEqual(['docs/foo.md', 'foo.md']);
});

test('disregards ancestor and nested `.gitignore` files', async () => {
	const parent = createFixture({
		'.gitignore': '*.md\n',
		'project/docs/.gitignore': '*.md\n',
		'project/docs/foo.md': TEST_MARKDOWN,
		'project/foo.md': TEST_MARKDOWN,
	});
	const cwd = join(parent, 'project');

	await expect(lint(cwd)).resolves.toStrictEqual(['docs/foo.md', 'foo.md']);
});

test('keeps default ignores despite `.gitignore` negations', async () => {
	const cwd = createFixture({
		'.claude/foo.md': TEST_MARKDOWN,
		'.gitignore': '!*.js\n!*.md\n',
		'foo.js': 'export {};\n',
		'foo.md': TEST_MARKDOWN,
	});

	await expect(lint(cwd)).resolves.toStrictEqual(['foo.md']);
});

test.each([
	{
		name: 'a directory',
		setup: (path: string) => mkdirSync(path),
	},
	{
		name: 'a broken symlink',
		setup: (path: string) => symlinkSync('missing', path),
	},
])('fails when the root `.gitignore` is $name', async ({ setup }) => {
	const cwd = createFixture({});
	const path = join(cwd, '.gitignore');

	setup(path);

	await expect(loadConfig(cwd)).rejects.toThrow(
		`Failed to read \`${path}\`:`
	);
});

test('leaves root ignores out of the config under `--no-ignore`', async () => {
	const cwd = createFixture({});
	const params = ['--no-ignore', '--', 'foo.md'];

	mkdirSync(join(cwd, '.gitignore'));

	await expect(loadConfig(cwd, params)).resolves.not.toContainEqual(
		expect.objectContaining({ basePath: cwd })
	);
});

test('inspects the root `.gitignore` while ignoring is enabled', () => {
	const cwd = createFixture({ 'foo.md': TEST_MARKDOWN });
	const file = join(cwd, 'foo.md');

	setParams([]);

	expect(() => includeRootIgnores(file)).toThrow('ENOTDIR');
});

test.each(['--ignore= false ', '--no-ignore', '--no-ignore= \ttrue\t '])(
	'skips the root `.gitignore` without touching the filesystem',
	(param) => {
		// Any lookup of `.gitignore` below a regular file fails with `ENOTDIR`
		const cwd = createFixture({ 'foo.md': TEST_MARKDOWN });
		const file = join(cwd, 'foo.md');

		setParams([param, '--', 'foo.md']);

		expect(includeRootIgnores(file)).toStrictEqual([]);
	}
);
