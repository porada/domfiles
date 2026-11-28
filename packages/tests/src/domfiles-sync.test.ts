import { spawn, spawnSync } from 'node:child_process';
import {
	appendFile,
	chmod,
	copyFile,
	lstat,
	mkdir,
	mkdtempDisposable,
	readFile,
	readlink,
	realpath,
	rm,
	symlink,
	writeFile,
} from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { stripVTControlCharacters } from 'node:util';
import { expect, onTestFinished, test } from 'vite-plus/test';

const BREW_STUB = `#!/bin/sh
printf 'brew %s\\n' "$*" >> "$COMMAND_LOG"

if [ -n "\${BREW_FAIL-}" ] && [ "$1 $2 $3" = "install --quiet $BREW_FAIL" ]; then
  exit 1
fi

has() {
  for name in $1; do
    if [ "$name" = "$2" ]; then
      return 0
    fi
  done

  return 1
}

case "$*" in
  --version) printf 'Homebrew 0.0.0\\n' ;;
  --prefix) printf '%s\\n' "$FAKE_PREFIX" ;;
  '--cellar '* | '--prefix '*) printf '%s/%s\\n' "$FAKE_PREFIX" "$2" ;;
  'info '*) printf '%s\\n' '{"formulae":[{"installed":[{"installed_on_request":true}],"keg_only":false,"linked_keg":"1.0"}]}' ;;
  'list --cask') printf '%s\\n' $BREW_CASKS ;;
  'list --cask --versions '*) has "$BREW_CASKS" "$4" ;;
  'list --formula') printf '%s\\n' $BREW_FORMULAS ;;
  'list --formula --versions '*) has "$BREW_FORMULAS" "$4" ;;
esac
`;
const CASKS = ['claude-code@latest', 'codex', 'google-chrome', 'zed'];
const FORMULAS = [
	'actionlint',
	'cmark-gfm',
	'coreutils',
	'fd',
	'fish',
	'fisher',
	'gh',
	'git',
	'jq',
	'just',
	'mole',
	'node',
	'pandoc',
	'pnpm',
	'ripgrep',
	'rust',
	'shellcheck',
	'tailscale',
	'vim',
	'yarn',
	'yq',
	'zizmor',
];
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
const PACKAGE_TEST_TIMEOUT = 20_000;
const REPOSITORY = join(import.meta.dirname, '../../..');
const REPOSITORY_TEST_TIMEOUT = 10_000;
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

	for (const stage of [...STAGES, 'core']) {
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

function killProcessGroup(pid: number) {
	try {
		process.kill(-pid, 'SIGKILL');
	} catch (error) {
		const { code } = error as NodeJS.ErrnoException;

		// macOS reports EPERM for a group whose members have all exited
		if (code !== 'EPERM' && code !== 'ESRCH') {
			throw error;
		}
	}
}

async function runCommand(
	directory: string,
	command: string,
	params: string[],
	env: Record<string, string> = {}
) {
	const child = spawn(
		join(directory, 'domfiles/home/.local/bin', command),
		params,
		{
			cwd: directory,
			detached: true,
			env: {
				COMMAND_LOG: join(directory, 'commands.log'),
				HOME: join(directory, 'home'),
				PATH: `${join(directory, 'stubs')}:${process.env.PATH ?? ''}`,
				...env,
			},
			stdio: ['ignore', 'pipe', 'pipe'],
		}
	);
	const { pid } = child;

	// Register cleanup before the first await so a timed-out test still runs it
	onTestFinished(() => {
		if (pid !== undefined) {
			killProcessGroup(pid);
		}
	});

	let stderr = '';
	let stdout = '';

	child.stderr.setEncoding('utf8').on('data', (chunk: string) => {
		stderr += chunk;
	});
	child.stdout.setEncoding('utf8').on('data', (chunk: string) => {
		stdout += chunk;
	});

	// Resolve only once the command has exited and closed its output
	const status = await new Promise<number | null>((resolve, reject) => {
		child.on('close', resolve);
		child.on('error', reject);
	});

	stdout = stripVTControlCharacters(stdout);

	return {
		commands: await readFile(join(directory, 'commands.log'), 'utf8'),
		stages: stdout.split('\n').filter((line) => line.startsWith('stage ')),
		status,
		stderr: stripVTControlCharacters(stderr),
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

async function createUpstreamFixture(directory: string, branch: string) {
	const checkout = join(directory, 'domfiles');
	const upstream = join(directory, 'upstream.git');
	const update = join(directory, 'update');

	await rm(join(directory, 'stubs/git'));
	runGit(checkout, ['init', '--quiet', `--initial-branch=${branch}`]);
	runGit(checkout, ['add', '--all']);
	runGit(checkout, ['commit', '--quiet', '--message=foo']);
	runGit(directory, ['clone', '--quiet', '--bare', checkout, upstream]);
	runGit(checkout, ['remote', 'add', 'origin', upstream]);
	runGit(checkout, ['fetch', '--quiet', 'origin']);
	runGit(checkout, [
		'branch',
		'--quiet',
		`--set-upstream-to=origin/${branch}`,
	]);
	runGit(directory, ['clone', '--quiet', upstream, update]);

	return { checkout, upstream, update };
}

async function createPackageFixture(directory: string) {
	await createFixture(directory);

	const checkout = join(directory, 'domfiles');
	const modules = join(checkout, 'home/.local/share/domfiles/sync');
	const stubs = join(directory, 'stubs');
	const diffHighlight = join(
		directory,
		'prefix/git/share/git-core/contrib/diff-highlight'
	);

	for (const module of ['core', 'install', 'migrate', 'setup']) {
		await copyFile(
			join(REPOSITORY, 'home/.local/share/domfiles/sync', module),
			join(modules, module)
		);
	}

	await appendFile(
		join(modules, 'install'),
		'\nprovision_jq() (\n  install -d "$HOME/jq-hook"\n  printf \'hook jq %s\\n\' "$1" >> "$COMMAND_LOG"\n)\n'
	);
	await mkdir(diffHighlight, { recursive: true });
	await writeFile(join(diffHighlight, 'diff-highlight'), '');

	for (const source of ['.agents', 'home/.claude', 'skills']) {
		await mkdir(join(checkout, source), { recursive: true });
	}

	await rm(join(stubs, 'git'));
	await writeExecutable(join(stubs, 'brew'), BREW_STUB);

	for (const command of ['chsh', 'defaults', 'fish', 'pnpm', 'sudo', 'vim']) {
		await writeExecutable(
			join(stubs, command),
			`#!/bin/sh\nprintf '${command} %s\\n' "$*" >> "$COMMAND_LOG"\n`
		);
	}

	for (const command of ['launchctl', 'xcodebuild']) {
		await writeExecutable(join(stubs, command), '#!/bin/sh\nexit 1\n');
	}

	for (const { hook, hookDirectory } of [
		{ hook: 'provision_codex', hookDirectory: directory },
		{ hook: 'provision_fd', hookDirectory: stubs },
	]) {
		await writeExecutable(
			join(hookDirectory, hook),
			`#!/bin/sh\nprintf '${hook} %s\\n' "$*" >> "$COMMAND_LOG"\n`
		);
	}

	await writeExecutable(
		join(stubs, 'curl'),
		[
			'#!/bin/sh',
			'printf \'curl %s\\n\' "$*" >> "$COMMAND_LOG"',
			'',
			'while [ $# -gt 0 ]; do',
			'  if [ "$1" = --output ]; then',
			'    mkdir -p "$(dirname "$2")"',
			'    printf \'plug\\n\' > "$2"',
			'  fi',
			'',
			'  shift',
			'done',
			'',
		].join('\n')
	);
	await writeExecutable(
		join(stubs, 'getent'),
		"#!/bin/sh\nprintf 'foo:x:1:1:foo:/home/foo:/bin/zsh\\n'\n"
	);
	await writeExecutable(
		join(stubs, 'id'),
		"#!/bin/sh\ncase \"$1\" in -P) printf 'foo:*:1:1::0:0:foo:/home/foo:/bin/zsh\\n' ;; *) printf 'foo\\n' ;; esac\n"
	);
	await writeExecutable(
		join(stubs, 'uname'),
		"#!/bin/sh\ncase \"$1\" in -s) printf 'Darwin\\n' ;; *) printf 'foo\\n' ;; esac\n"
	);
	runGit(checkout, ['init', '--quiet', '--initial-branch=foo']);
	runGit(checkout, ['add', '--all']);
	runGit(checkout, ['commit', '--quiet', '--message=foo']);

	return {
		checkout: await realpath(checkout),
		env: {
			...GIT_ENV,
			FAKE_PREFIX: join(directory, 'prefix'),
			PATH: `${stubs}::${process.env.PATH ?? ''}`,
		},
	};
}

function lines(text: string, prefix: string) {
	return text.split('\n').filter((line) => line.startsWith(prefix));
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

test('rejects a command that fails to launch', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);
	await writeExecutable(
		join(directory.path, 'domfiles/home/.local/bin/foo'),
		'#!/nonexistent/sh\n'
	);

	let result: unknown;

	try {
		await runCommand(directory.path, 'foo', []);
	} catch (error) {
		result = error;
	}

	if (!(result instanceof Error)) {
		throw new TypeError('Expected a launch error');
	}

	expect((result as NodeJS.ErrnoException).code).toBe('ENOENT');
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

test('runs only core setup through `domfiles sync --core`', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, 'domfiles', [
		'sync',
		'--core',
	]);

	expect(result.status).toBe(0);
	expect(result.stages).toStrictEqual(['stage core']);
	expect(result.stdout).not.toContain('dependencies');
	expect(result.commands).toContain('git -C');
});

test.each([
	{ command: 'domfiles sync', params: ['sync'], stages: FULL_RUN_STAGES },
	{
		command: 'domfiles sync --core',
		params: ['sync', '--core'],
		stages: ['core'],
	},
])(
	'restarts `$command` after an update repairs a broken stage',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async ({ params, stages }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const checkout = join(directory.path, 'domfiles');
		const stagePath = 'home/.local/share/domfiles/sync/install';
		const stage = await readFile(join(checkout, stagePath), 'utf8');

		await writeFile(
			join(checkout, stagePath),
			'sync_install() (\n  if then\n)\n'
		);

		const { update } = await createUpstreamFixture(
			directory.path,
			'master'
		);

		await writeFile(join(update, stagePath), stage);
		runGit(update, ['commit', '--quiet', '--all', '--message=bar']);
		runGit(update, ['push', '--quiet']);

		const result = await runCommand(
			directory.path,
			'domfiles',
			params,
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(result.stages).toStrictEqual(
			stages.map((stage) => `stage ${stage}`)
		);
		expect(
			result.stdout.match(/Restarting the sync after the update/g)
		).toHaveLength(1);
		expect(result.stderr).toContain(`$ domfiles ${params.join(' ')}\n`);
		expect(result.commands).toBe('brew --version\nbrew --version\n');
		expect(runGit(checkout, ['log', '-1', '--format=%s'])).toBe('bar\n');
	}
);

test(
	'updates the repository on `master` when a tag shares its name',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout, update } = await createUpstreamFixture(
			directory.path,
			'master'
		);

		runGit(checkout, ['tag', 'master']);
		runGit(update, ['commit', '--quiet', '--allow-empty', '--message=bar']);
		runGit(update, ['push', '--quiet']);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(result.stdout).not.toContain('Skipping repository update');
		expect(result.stages).toStrictEqual(['stage core']);
		expect(runGit(checkout, ['log', '-1', '--format=%s'])).toBe('bar\n');
	}
);

test.each([
	{
		branch: 'main',
		detach: false,
		message: 'Skipping repository update on branch `main`',
		state: 'on a branch other than `master`',
	},
	{
		branch: 'master',
		detach: true,
		message:
			'Skipping repository update because `HEAD` does not reference a branch',
		state: 'with a detached `HEAD`',
	},
])(
	'skips the repository update $state and continues the sync',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async ({ branch, detach, message }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout, update } = await createUpstreamFixture(
			directory.path,
			branch
		);
		const remoteBranch = `refs/remotes/origin/${branch}`;
		const remoteRevision = runGit(checkout, ['rev-parse', remoteBranch]);

		if (detach) {
			runGit(checkout, ['checkout', '--quiet', '--detach']);
		}

		runGit(update, ['commit', '--quiet', '--allow-empty', '--message=bar']);
		runGit(update, ['push', '--quiet']);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(result.stdout).toContain(message);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(runGit(checkout, ['rev-parse', remoteBranch])).toBe(
			remoteRevision
		);
		expect(runGit(checkout, ['log', '-1', '--format=%s'])).toBe('foo\n');
	}
);

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
			"Usage: domfiles sync [--core | --help | clean | install | macos | migrate | setup | update]

			\`domfiles sync\` runs all stages.

			\`domfiles sync --core\` refreshes links and installs missing packages.

			Both commands check for repository updates only when the checkout is on \`master\`.

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
	[['--core', 'install']],
	[['install', '--core']],
	[['--core', '--help']],
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
		"× Usage: domfiles sync [--core | --help | clean | install | macos | migrate | setup | update]
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

test.each([
	{ param: 'install', stage: 'install' },
	{ param: '--core', stage: 'core' },
])(
	'stops `domfiles sync $param` at its first failing command',
	async ({ param, stage }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', param],
			{ FAIL_STAGE: stage }
		);

		expect(result.status).toBe(1);
		expect(result.stages).toStrictEqual([`stage ${stage}`]);
		expect(result.stdout).not.toContain('continued after failure');
	}
);

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

test(
	'refreshes links without Homebrew work in a core sync when every package is installed',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { checkout, env } = await createPackageFixture(directory.path);
		const skills = join(directory.path, 'home/.agents/skills');

		await mkdir(skills, { recursive: true });
		await symlink(join(checkout, 'skills/prose'), join(skills, 'prose'));

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				BREW_CASKS: CASKS.join(' '),
				BREW_FORMULAS: FORMULAS.join(' '),
			}
		);

		expect(result.status).toBe(0);
		expect(result.commands).toBe(
			[
				'brew --version',
				'brew list --formula',
				'brew list --cask',
				`pnpm --dir ${checkout} --loglevel=error install --frozen-lockfile`,
				'',
			].join('\n')
		);
		expect(result.stdout).toContain('All packages installed');
		expect(result.stdout).not.toContain('Linking Homebrew-installed tools');
		expect(result.stdout).not.toContain('Setting up Tailscale');
		expect(result.stderr).not.toMatch(/No such file|cannot|failed/i);
		await expect(lstat(join(skills, 'prose'))).rejects.toMatchObject({
			code: 'ENOENT',
		});
		await expect(
			readlink(join(skills, 'human-facing-writing'))
		).resolves.toBe(join(checkout, 'skills/human-facing-writing'));
	}
);

test(
	'installs missing packages and runs only their core hooks',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env } = await createPackageFixture(directory.path);
		const missing = new Set([
			'fd',
			'git',
			'jq',
			'pnpm',
			'tailscale',
			'vim',
		]);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				BREW_CASKS: CASKS.filter((cask) => cask !== 'codex').join(' '),
				BREW_FORMULAS: [
					...FORMULAS.filter((formula) => !missing.has(formula)),
					'codex',
				].join(' '),
			}
		);
		const commands = result.commands.split('\n');
		const hookDirectory = await lstat(join(directory.path, 'home/jq-hook'));

		expect(result.status).toBe(0);
		expect(lines(result.commands, 'brew install ')).toStrictEqual([
			'brew install --quiet git',
			'brew install --quiet pnpm',
			'brew install --quiet codex --cask',
			'brew install --quiet fd',
			'brew install --quiet jq',
			'brew install --quiet vim',
			'brew install --quiet tailscale',
		]);
		expect(lines(result.commands, 'brew update')).toStrictEqual([
			'brew update --quiet',
		]);
		expect(commands.indexOf('brew update --quiet')).toBeLessThan(
			commands.indexOf('brew install --quiet git')
		);
		expect(lines(result.commands, 'hook ')).toStrictEqual(['hook jq core']);
		expect(hookDirectory.isDirectory()).toBe(true);
		expect(result.stdout).toContain('Linking Homebrew-installed tools');
		expect(result.stdout).toContain('Setting up Tailscale');
		expect(result.stdout).not.toContain('Configuring `pnpm`');
		expect(result.stdout).not.toContain('default shell');
		expect(result.commands).not.toMatch(
			/^(chsh|curl|fish|pnpm config|provision_codex|provision_fd) /m
		);
		expect(result.commands).toContain('install --frozen-lockfile');
	}
);

test(
	'runs hooks for installed packages and finishes the install stage with system setup',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env } = await createPackageFixture(directory.path);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			{
				...env,
				BREW_CASKS: CASKS.join(' '),
				BREW_FORMULAS: FORMULAS.join(' '),
			}
		);
		const output = lines(result.stdout, '· ');

		expect(result.status).toBe(0);
		expect(lines(result.commands, 'brew install ')).toStrictEqual([]);
		expect(lines(result.commands, 'hook ')).toStrictEqual(['hook jq full']);
		expect(output).toStrictEqual(
			expect.arrayContaining([
				'· Linking Homebrew-installed tools',
				'· Configuring `pnpm`',
				'· Installing `vim-plug`',
				'· Setting up Tailscale',
			])
		);
		expect(output.at(-1)).toBe('· Installing `pnpm` dependencies');
		expect(result.stdout.trimEnd().split('\n').at(-1)).toBe(
			'▴ Setting `fish` as the default shell requires an interactive terminal'
		);
		expect(result.commands).not.toMatch(/^provision_(codex|fd) /m);
	}
);

test(
	'skips the frozen `pnpm` install for a dirty checkout during a core sync',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { checkout, env } = await createPackageFixture(directory.path);

		await appendFile(
			join(checkout, 'home/.local/bin/domfiles-dependencies'),
			'\n'
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				BREW_CASKS: CASKS.join(' '),
				BREW_FORMULAS: FORMULAS.join(' '),
			}
		);

		expect(result.status).toBe(0);
		expect(result.stdout).toContain('Skipping `pnpm install`');
		expect(result.commands).not.toContain('install --frozen-lockfile');
	}
);

test(
	'stops a core sync before hooks when a package fails to install',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env } = await createPackageFixture(directory.path);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				BREW_CASKS: CASKS.join(' '),
				BREW_FAIL: 'git',
				BREW_FORMULAS: FORMULAS.filter(
					(formula) => formula !== 'git'
				).join(' '),
			}
		);

		expect(result.status).toBe(1);
		expect(lines(result.commands, 'brew install ')).toStrictEqual([
			'brew install --quiet git',
		]);
		expect(result.stdout).not.toContain('Linking Homebrew-installed tools');
		expect(result.commands).not.toContain('install --frozen-lockfile');
	}
);
