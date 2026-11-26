import { spawnSync } from 'node:child_process';
import {
	chmod,
	copyFile,
	mkdir,
	mkdtempDisposable,
	readFile,
	rm,
	writeFile,
} from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { stripVTControlCharacters } from 'node:util';
import { expect, test } from 'vite-plus/test';

const FULL_RUN_STAGES = [
	'migrate',
	'setup',
	'install',
	'macos',
	'update',
	'clean',
];
const GIT_ENV = {
	GIT_AUTHOR_EMAIL: 'foo@example.com',
	GIT_AUTHOR_NAME: 'foo',
	GIT_COMMITTER_EMAIL: 'foo@example.com',
	GIT_COMMITTER_NAME: 'foo',
	GIT_CONFIG_GLOBAL: '/dev/null',
	GIT_CONFIG_NOSYSTEM: '1',
};
const REPOSITORY = join(import.meta.dirname, '../../..');
const STAGES = ['clean', 'install', 'macos', 'migrate', 'setup', 'update'];

async function writeExecutable(path: string, content: string) {
	await writeFile(path, content);
	await chmod(path, 0o755);
}

async function createFixture(directory: string) {
	const bin = join(directory, 'domfiles/home/.local/bin');
	const share = join(directory, 'domfiles/home/.local/share');
	const stubs = join(directory, 'stubs');

	await mkdir(join(share, 'domfiles/sync'), { recursive: true });
	await mkdir(bin, { recursive: true });
	await mkdir(stubs);
	await mkdir(join(directory, 'home'));

	for (const command of ['domfiles', 'domfiles-sync']) {
		await copyFile(
			join(REPOSITORY, 'home/.local/bin', command),
			join(bin, command)
		);
		await chmod(join(bin, command), 0o755);
	}

	await copyFile(
		join(REPOSITORY, 'home/.local/share/domlib'),
		join(share, 'domlib')
	);
	await copyFile(
		join(REPOSITORY, 'home/.local/share/domfiles/sync/repository'),
		join(share, 'domfiles/sync/repository')
	);

	for (const stage of STAGES) {
		await writeFile(
			join(share, 'domfiles/sync', stage),
			[
				`sync_${stage}() (`,
				`  printf 'stage %s\\n' ${stage}`,
				'',
				`  if [ "\${FAIL_STAGE-}" = ${stage} ]; then`,
				'    false',
				`    printf 'continued after failure\\n'`,
				'  fi',
				')',
				'',
			].join('\n')
		);
	}

	await writeExecutable(
		join(bin, 'domfiles-dependencies'),
		"#!/bin/sh\nprintf 'dependencies\\n'\n"
	);
	await writeExecutable(
		join(stubs, 'brew'),
		'#!/bin/sh\nprintf \'brew %s\\n\' "$*" >> "$COMMAND_LOG"\n'
	);
	await writeExecutable(
		join(stubs, 'git'),
		'#!/bin/sh\nprintf \'git %s\\n\' "$*" >> "$COMMAND_LOG"\nexit 1\n'
	);
	await writeExecutable(
		join(stubs, 'scutil'),
		"#!/bin/sh\nprintf 'foo\\n'\n"
	);
	await writeFile(join(directory, 'commands.log'), '');
}

async function runCommand(
	directory: string,
	command: string,
	params: string[],
	env: Record<string, string> = {}
) {
	const result = spawnSync(
		join(directory, 'domfiles/home/.local/bin', command),
		params,
		{
			encoding: 'utf8',
			env: {
				COMMAND_LOG: join(directory, 'commands.log'),
				HOME: join(directory, 'home'),
				PATH: `${join(directory, 'stubs')}:${process.env.PATH ?? ''}`,
				...env,
			},
		}
	);

	const stdout = stripVTControlCharacters(result.stdout);

	return {
		commands: await readFile(join(directory, 'commands.log'), 'utf8'),
		stages: stdout.split('\n').filter((line) => line.startsWith('stage ')),
		status: result.status,
		stderr: stripVTControlCharacters(result.stderr),
		stdout,
	};
}

function runGit(cwd: string, params: string[]) {
	const result = spawnSync('git', params, {
		cwd,
		encoding: 'utf8',
		env: { ...GIT_ENV, PATH: process.env.PATH ?? '' },
	});

	if (result.status !== 0) {
		throw new Error(result.stderr);
	}

	return result.stdout;
}

test('defines a function for every accepted stage', () => {
	const result = spawnSync(
		'sh',
		[
			'-c',
			'directory="$1"; shift; for stage do . "$directory/$stage"; command -v "sync_$stage" || exit; done',
			'sh',
			join(REPOSITORY, 'home/.local/share/domfiles/sync'),
			...STAGES,
		],
		{ encoding: 'utf8' }
	);

	expect(result.status).toBe(0);
	expect(result.stdout).toBe(
		STAGES.map((stage) => `sync_${stage}\n`).join('')
	);
});

test.each([
	['domfiles', ['sync']],
	['domfiles-sync', []],
])('runs every stage in order through `%s`', async (command, params) => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, command, params);

	expect(result.status).toBe(0);
	expect(result.stages).toStrictEqual(
		FULL_RUN_STAGES.map((stage) => `stage ${stage}`)
	);
	expect(result.stdout).toContain('dependencies');
	expect(result.commands).toContain('git -C');
});

test('restarts a full sync after an update repairs a broken stage', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const checkout = join(directory.path, 'domfiles');
	const upstream = join(directory.path, 'upstream.git');
	const update = join(directory.path, 'update');
	const stagePath = 'home/.local/share/domfiles/sync/install';
	const stage = await readFile(join(checkout, stagePath), 'utf8');

	await rm(join(directory.path, 'stubs/git'));
	await writeFile(
		join(checkout, stagePath),
		'sync_install() (\n  if then\n)\n'
	);
	runGit(checkout, ['init', '--quiet', '--initial-branch=main']);
	runGit(checkout, ['add', '--all']);
	runGit(checkout, ['commit', '--quiet', '--message=foo']);
	runGit(directory.path, ['clone', '--quiet', '--bare', checkout, upstream]);
	runGit(checkout, ['remote', 'add', 'origin', upstream]);
	runGit(checkout, ['fetch', '--quiet', 'origin']);
	runGit(checkout, ['branch', '--quiet', '--set-upstream-to=origin/main']);
	runGit(directory.path, ['clone', '--quiet', upstream, update]);
	await writeFile(join(update, stagePath), stage);
	runGit(update, ['commit', '--quiet', '--all', '--message=bar']);
	runGit(update, ['push', '--quiet']);

	const result = await runCommand(
		directory.path,
		'domfiles',
		['sync'],
		GIT_ENV
	);

	expect(result.status).toBe(0);
	expect(result.stages).toStrictEqual(
		FULL_RUN_STAGES.map((stage) => `stage ${stage}`)
	);
	expect(
		result.stdout.match(/Restarting the sync after the update/g)
	).toHaveLength(1);
	expect(result.commands).toBe('brew --version\nbrew --version\n');
	expect(runGit(checkout, ['log', '-1', '--format=%s'])).toBe('bar\n');
});

test.each(STAGES)(
	'runs only the `%s` stage from the current checkout',
	async (stage) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const result = await runCommand(directory.path, 'domfiles', [
			'sync',
			stage,
		]);

		expect(result.status).toBe(0);
		expect(result.stages).toStrictEqual([`stage ${stage}`]);
		expect(result.stdout).not.toContain('dependencies');
		expect(result.commands).toBe('brew --version\n');
	}
);

test('runs a named stage through the checkout entrypoint', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, 'domfiles-sync', ['setup']);

	expect(result.status).toBe(0);
	expect(result.stages).toStrictEqual(['stage setup']);
	expect(result.commands).toBe('brew --version\n');
});

test.each([
	['domfiles', ['sync', '--help']],
	['domfiles-sync', ['--help']],
])(
	'prints help through `%s` without running any command',
	async (command, params) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const result = await runCommand(directory.path, command, params);

		expect(result.status).toBe(0);
		expect(result.stdout).toMatchInlineSnapshot(`
			"Usage: domfiles sync [--help | clean | install | macos | migrate | setup | update]

			\`domfiles sync\` checks for repository updates and runs all stages.
			\`domfiles sync <stage>\` runs one stage without updating the repository.
			"
		`);
		expect(result.stderr).toBe('');
		expect(result.commands).toBe('');
	}
);

test.each([
	[['foo']],
	[['']],
	[['-h']],
	[['repository']],
	[['install ']],
	[['install', 'foo']],
	[['--help', 'foo']],
])('rejects %j before running any command', async (params) => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, 'domfiles', [
		'sync',
		...params,
	]);

	expect(result.status).toBe(1);
	expect(result.stdout).toBe('');
	expect(result.stderr).toMatchInlineSnapshot(`
		"× Usage: domfiles sync [--help | clean | install | macos | migrate | setup | update]
		"
	`);
	expect(result.commands).toBe('');
});

test('skips the `macos` stage outside macOS', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);
	await copyFile(
		join(REPOSITORY, 'home/.local/share/domfiles/sync/macos'),
		join(directory.path, 'domfiles/home/.local/share/domfiles/sync/macos')
	);
	await writeExecutable(
		join(directory.path, 'stubs/uname'),
		'#!/bin/sh\nprintf \'uname %s\\n\' "$*" >> "$COMMAND_LOG"\nprintf \'Linux\\n\'\n'
	);

	for (const command of ['chflags', 'defaults', 'killall', 'pmset', 'sudo']) {
		await writeExecutable(
			join(directory.path, 'stubs', command),
			`#!/bin/sh\nprintf '${command} %s\\n' "$*" >> "$COMMAND_LOG"\n`
		);
	}

	const result = await runCommand(directory.path, 'domfiles', [
		'sync',
		'macos',
	]);

	expect(result.status).toBe(0);
	expect(result.stdout).toContain('Configuring macOS');
	expect(result.stdout).toContain(
		'`domfiles sync macos` is a macOS-only command'
	);
	expect(result.commands).not.toMatch(
		/^(chflags|defaults|killall|pmset|sudo) /m
	);
});

test('stops a stage at its first failing command', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(
		directory.path,
		'domfiles',
		['sync', 'install'],
		{
			FAIL_STAGE: 'install',
		}
	);

	expect(result.status).toBe(1);
	expect(result.stages).toStrictEqual(['stage install']);
	expect(result.stdout).not.toContain('continued after failure');
});

test('stops a full sync at its first failing stage', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, 'domfiles', ['sync'], {
		FAIL_STAGE: 'install',
	});

	expect(result.status).toBe(1);
	expect(result.stages).toStrictEqual([
		'stage migrate',
		'stage setup',
		'stage install',
	]);
	expect(result.stdout).not.toContain('continued after failure');
	expect(result.stdout).not.toContain('dependencies');
});
