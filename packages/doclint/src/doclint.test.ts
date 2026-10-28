import eslint from 'eslint/package.json' with { type: 'json' };
import { spawnSync } from 'node:child_process';
import { beforeEach, expect, onTestFinished, test, vi } from 'vite-plus/test';
import resolvePath from './resolve-path/index.ts';

vi.mock(import('node:child_process'));

const SPAWN_RESULT = {
	output: [null, null, null],
	pid: 1,
	signal: null,
	status: 0,
	stderr: Buffer.alloc(0),
	stdout: Buffer.alloc(0),
};

beforeEach(() => {
	const { argv, exitCode } = process;
	process.argv = ['node', resolvePath('doclint.ts', import.meta.url)];

	onTestFinished(() => {
		process.argv = argv;
	});

	onTestFinished(() => {
		process.exitCode = exitCode;
	});

	vi.resetModules();
	vi.mocked(spawnSync).mockReturnValue(SPAWN_RESULT);
});

test.each([
	{ params: [] },
	{ params: ['--config', 'doclint.config.ts', '--', 'foo bar.md'] },
	{ params: ['--no-ignore', '--', 'foo.md'] },
	{ params: ['foo.md', '--format', 'json'] },
])('forwards arguments to bundled ESLint', async ({ params }) => {
	process.argv.push(...params);

	await import('./doclint.ts');

	expect(spawnSync).toHaveBeenCalledExactlyOnceWith(
		process.execPath,
		[
			resolvePath(
				eslint.bin.eslint,
				import.meta.resolve('eslint/package.json')
			),
			'--config',
			resolvePath('../eslint.config.ts', import.meta.url),
			'--flag',
			'unstable_native_nodejs_ts_config',
			...params,
		],
		{ stdio: 'inherit' }
	);
});

test.each([0, 1, 2, 3, null])('propagates child status', async (status) => {
	vi.mocked(spawnSync).mockReturnValue({
		...SPAWN_RESULT,
		status,
	});

	await import('./doclint.ts');

	expect(process.exitCode).toBe(status ?? 2);
});

test('reports a launcher failure', async () => {
	using stderr = vi.spyOn(process.stderr, 'write').mockReturnValue(true);

	vi.mocked(spawnSync).mockReturnValue({
		...SPAWN_RESULT,
		error: new Error('Spawn failed'),
		status: null,
	});

	await import('./doclint.ts');

	expect(process.exitCode).toBe(2);
	expect(stderr).toHaveBeenCalledExactlyOnceWith(
		'doclint: Failed to start ESLint:\n\nSpawn failed\n'
	);
});

test('propagates the child termination signal', async () => {
	using kill = vi.spyOn(process, 'kill').mockReturnValue(true);

	vi.mocked(spawnSync).mockReturnValue({
		...SPAWN_RESULT,
		signal: 'SIGTERM',
		status: null,
	});

	await import('./doclint.ts');

	expect(kill).toHaveBeenCalledExactlyOnceWith(process.pid, 'SIGTERM');
});
