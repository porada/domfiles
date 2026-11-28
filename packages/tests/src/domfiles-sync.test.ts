import { spawn, spawnSync } from 'node:child_process';
import {
	appendFile,
	chmod,
	copyFile,
	cp,
	lstat,
	mkdir,
	mkdtempDisposable,
	readdir,
	readFile,
	readlink,
	realpath,
	rm,
	symlink,
	writeFile,
} from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, join } from 'node:path';
import { stripVTControlCharacters } from 'node:util';
import { expect, onTestFinished, test, vi } from 'vite-plus/test';

// Logs each `brew` command like the fixture’s stub and pauses the first one
// until `BREW_RELEASE` exists when `BREW_GATE` names a log
const BREW_GATE_STUB = `#!/bin/sh
printf 'brew %s\\n' "$*" >> "$COMMAND_LOG"

if [ -n "\${BREW_GATE-}" ] && [ ! -e "$BREW_GATE.done" ]; then
  : > "$BREW_GATE.done"
  printf 'gate\\n' >> "$BREW_GATE"

  until [ -e "$BREW_RELEASE" ]; do
    command -p sleep 0.05
  done
fi
`;

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
const CHANGED_CHECKOUT_RESTART =
	'Restarting the sync with the updated checkout';
const FAILABLE_PS_STUB = `#!/bin/sh
if [ -e "$PS_FAILURE" ]; then
  printf 'failed\\n' >> "$PS_LOG"
  exit 1
fi

PATH="\${PATH#*:}" exec ps "$@"
`;
// Answers lookups of the checkout’s commit like `FIXTURE_GIT_STUB`, but fails
// lookup number `FAILING_LOOKUP`, counting them in `LOOKUP_LOG`
const FAILING_LOOKUP_GIT_STUB = `#!/bin/sh
printf 'git %s\\n' "$*" >> "$COMMAND_LOG"

case "$*" in
  *' rev-parse HEAD')
    printf 'lookup\\n' >> "$LOOKUP_LOG"

    if [ "$(($(wc -l < "$LOOKUP_LOG")))" -eq "$FAILING_LOOKUP" ]; then
      exit 128
    fi

    printf '%s\\n' 0123456789abcdef0123456789abcdef01234567
    exit 0
    ;;
esac

exit 1
`;
const FAILING_STUB = '#!/bin/sh\nexit 1\n';
// Logs each Git command and fails it, except the lookup of the checkout’s
// commit that every sync makes, which it answers as a checkout would
const FIXTURE_GIT_STUB = `#!/bin/sh
printf 'git %s\\n' "$*" >> "$COMMAND_LOG"

case "$*" in
  *' rev-parse HEAD')
    printf '%s\\n' 0123456789abcdef0123456789abcdef01234567
    exit 0
    ;;
esac

exit 1
`;
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
const HANGING_TRANSPORT = `#!/bin/sh
printf '%s\\n' "$$" "$PPID" >> "$TRANSPORT_PIDS"
exec tail -f /dev/null
`;
// Reaches the first one-minute limit once the transport starts and no later one
const LIMIT_SLEEP_STUB = `#!/bin/sh
if [ "$1" != 60 ]; then
  exit
fi

if [ -e "$LIMIT_REACHED" ]; then
  exec tail -f /dev/null
fi

until [ -s "$TRANSPORT_PIDS" ]; do
  command -p sleep 0.1
done

: > "$LIMIT_REACHED"
`;
const LOCK_TEST_TIMEOUT = 20_000;
const LOCK_WAITING_MESSAGE = '▴ Waiting for another sync to finish';
const PACKAGE_TEST_TIMEOUT = 20_000;
const REPOSITORY = join(import.meta.dirname, '../../..');
const REPOSITORY_TEST_TIMEOUT = 10_000;
const STAGE_GATE_STUB = `#!/bin/sh
trap 'printf "signaled %s\\n" "$$" >> "$STAGE_LOG"' HUP INT QUIT TERM
printf 'gate %s\\n' "$$" >> "$STAGE_LOG"

until [ -e "$STAGE_GATE" ]; do
  command -p sleep 0.05
done
`;
const STAGES = ['clean', 'install', 'macos', 'migrate', 'setup', 'update'];
const TERM_IGNORING_TRANSPORT = `#!/bin/sh
trap '' TERM
printf '%s\\n' "$$" "$PPID" >> "$TRANSPORT_PIDS"
exec tail -f /dev/null
`;
// Pauses the first fetch from the upstream until `FETCH_RELEASE` exists and
// logs `gate` to `FETCH_GATE` while it waits
const UPLOAD_PACK_GATE_STUB = `#!/bin/sh
if [ ! -e "$FETCH_GATE.done" ]; then
  : > "$FETCH_GATE.done"
  printf 'gate\\n' >> "$FETCH_GATE"

  until [ -e "$FETCH_RELEASE" ]; do
    command -p sleep 0.05
  done
fi

exec git-upload-pack "$@"
`;

async function writeExecutable(path: string, content: string) {
	await writeFile(path, content);
	await chmod(path, 0o755);
}

function createStageStub(stage: string) {
	return `sync_${stage}() (
  printf 'stage %s\\n' ${stage}

  if [ -n "\${STAGE_LOG-}" ]; then
    trap 'printf "signaled %s\\n" "$$" >> "$STAGE_LOG"' HUP INT QUIT TERM
    printf 'start %s %s\\n' ${stage} "$$" >> "$STAGE_LOG"
    printf 'holder %s\\n' "\${DOMFILES_SYNC_LOCK_HOLDER-}" >> "$STAGE_LOG"

    if [ -n "\${STAGE_LINGER-}" ]; then
      (
        exec < /dev/null > /dev/null 2>&1

        until [ -e "$STAGE_LINGER" ]; do
          command -p sleep 0.05
        done
      ) &
    fi

    if [ -n "\${STAGE_OWNER-}" ]; then
      printf 'owner %s\\n' "$(ps -o stat= -p "$STAGE_OWNER" | tr -d ' ')" >> "$STAGE_LOG"
    fi

    if [ -n "\${STAGE_NESTED-}" ]; then
      nested_status=0
      "$DOMFILES_BIN_DIR/domfiles" sync "$STAGE_NESTED" < /dev/null > "$NESTED_OUTPUT" 2>&1 || nested_status="$?"
      printf 'nested %s\\n' "$nested_status" >> "$STAGE_LOG"
    fi

    if [ -n "\${STAGE_GATE-}" ]; then
      stage-gate
    fi

    printf 'end %s %s\\n' ${stage} "$$" >> "$STAGE_LOG"
  fi

  if [ "\${FAIL_STAGE-}" = ${stage} ]; then
    false
    printf 'continued after failure\\n'
  fi
)
`;
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

	for (const module of ['lock', 'repository']) {
		await copyFile(
			join(REPOSITORY, 'home/.local/share/domfiles/sync', module),
			join(share, 'domfiles/sync', module)
		);
	}

	for (const stage of [...STAGES, 'core']) {
		await writeFile(
			join(share, 'domfiles/sync', stage),
			createStageStub(stage)
		);
	}

	await writeExecutable(join(stubs, 'stage-gate'), STAGE_GATE_STUB);

	await writeExecutable(
		join(bin, 'domfiles-dependencies'),
		"#!/bin/sh\nprintf 'dependencies\\n'\n"
	);
	await writeExecutable(
		join(stubs, 'brew'),
		'#!/bin/sh\nprintf \'brew %s\\n\' "$*" >> "$COMMAND_LOG"\n'
	);
	await writeExecutable(join(stubs, 'git'), FIXTURE_GIT_STUB);
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
	return startCommand(directory, command, params, env).done;
}

function startCommand(
	directory: string,
	command: string,
	params: string[],
	env: Record<string, string> = {},
	checkout = 'domfiles'
) {
	const child = spawn(
		join(directory, checkout, 'home/.local/bin', command),
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
	const done = new Promise<number | null>((resolve, reject) => {
		child.on('close', resolve);
		child.on('error', reject);
	}).then(async (status) => {
		const output = stripVTControlCharacters(stdout);

		return {
			commands: await readFile(join(directory, 'commands.log'), 'utf8'),
			stages: output
				.split('\n')
				.filter((line) => line.startsWith('stage ')),
			status,
			stderr: stripVTControlCharacters(stderr),
			stdout: output,
		};
	});

	// Treat a rejection as handled until the test awaits it
	done.catch(() => undefined);

	return {
		done,
		pid,
		stderr: () => stripVTControlCharacters(stderr),
		stdout: () => stripVTControlCharacters(stdout),
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

function isRunning(pid: number) {
	try {
		process.kill(pid, 0);
	} catch (error) {
		if ((error as NodeJS.ErrnoException).code === 'ESRCH') {
			return false;
		}

		throw error;
	}

	return true;
}

async function readPids(path: string) {
	const content = await readFile(path, 'utf8');

	return content.split('\n').filter(Boolean).map(Number);
}

async function createLockFixture(directory: string) {
	await createFixture(directory);

	const gate = join(directory, 'gate');
	const log = join(directory, 'stages.log');

	await writeFile(log, '');

	return { env: { STAGE_GATE: gate, STAGE_LOG: log }, gate, log };
}

// Makes the `install` stage depend on a new `domlib` helper so mixing revisions
// fails. Pauses the first upstream fetch until `release` runs
async function createChangedCodeFixture(directory: string) {
	const { env, gate, log } = await createLockFixture(directory);
	const { checkout, update } = await createUpstreamFixture(
		directory,
		'master'
	);
	const fetchGate = join(directory, 'fetch.log');
	const fetchRelease = join(directory, 'fetch.release');
	const stagePath = 'home/.local/share/domfiles/sync/install';
	const stage = await readFile(join(update, stagePath), 'utf8');

	await appendFile(
		join(update, 'home/.local/share/domlib'),
		"\n__fixture_helper() {\n  printf 'helper\\n'\n}\n"
	);
	await writeFile(
		join(update, stagePath),
		stage.replace(
			'sync_install() (\n',
			'sync_install() (\n  __fixture_helper\n'
		)
	);
	runGit(update, ['commit', '--quiet', '--all', '--message=bar']);
	runGit(update, ['push', '--quiet']);
	await writeFile(fetchGate, '');
	await writeExecutable(
		join(directory, 'stubs/upload-pack'),
		UPLOAD_PACK_GATE_STUB
	);
	runGit(checkout, [
		'config',
		'remote.origin.uploadpack',
		join(directory, 'stubs/upload-pack'),
	]);

	return {
		env: {
			...GIT_ENV,
			FETCH_GATE: fetchGate,
			FETCH_RELEASE: fetchRelease,
			STAGE_LOG: env.STAGE_LOG,
		},
		fetchGate,
		gate,
		log,
		release: async () => {
			await writeFile(fetchRelease, '');
		},
	};
}

async function readLog(path: string) {
	const content = await readFile(path, 'utf8');

	return content.split('\n').filter(Boolean);
}

function stageEvents(lines: string[]) {
	return lines.filter((line) => /^(?:end|start) /.test(line));
}

async function waitForLine(path: string, line: string) {
	await vi.waitFor(
		async () => {
			await expect(readLog(path)).resolves.toContain(line);
		},
		{ interval: 50, timeout: LOCK_TEST_TIMEOUT }
	);
}

async function waitForGate(path: string, pid: number | undefined) {
	let gatePid = 0;

	await vi.waitFor(
		async () => {
			const lines = await readLog(path);
			const start = lines.indexOf(`start core ${pid}`);
			const gate = lines
				.slice(start + 1)
				.find((line) => line.startsWith('gate '));

			expect(start).toBeGreaterThanOrEqual(0);
			expect(gate).toBeDefined();
			gatePid = Number(gate?.split(' ')[1]);
		},
		{ interval: 50, timeout: LOCK_TEST_TIMEOUT }
	);

	return gatePid;
}

async function createSystemPath(directory: string, excluded: string[]) {
	const path = join(directory, 'system-bin');

	await mkdir(path);

	for (const source of ['/usr/bin', '/bin', '/usr/sbin', '/sbin']) {
		const entries = await readdir(source).catch(() => []);

		for (const entry of entries) {
			if (!excluded.includes(entry)) {
				await symlink(join(source, entry), join(path, entry)).catch(
					(error: unknown) => {
						if (
							(error as NodeJS.ErrnoException).code !== 'EEXIST'
						) {
							throw error;
						}
					}
				);
			}
		}
	}

	return path;
}

function listChildProcesses(pid: number) {
	const result = spawnSync('ps', ['-A', '-o', 'pid=,ppid='], {
		encoding: 'utf8',
	});

	return result.stdout
		.split('\n')
		.map((line) => line.trim().split(/\s+/).map(Number))
		.filter(([, parent]) => parent === pid)
		.map(([child]) => child);
}

function listFixtureProcesses(directory: string) {
	const result = spawnSync('ps', ['-A', '-o', 'pid=,command='], {
		encoding: 'utf8',
	});

	return result.stdout
		.split('\n')
		.filter((line) => line.includes(basename(directory)));
}

function isProcessLive(pid: number) {
	const result = spawnSync('ps', ['-o', 'stat=', '-p', String(pid)], {
		encoding: 'utf8',
	});
	const state = result.stdout.trim();

	return state !== '' && !state.startsWith('Z');
}

async function readSyncHolder(directory: string) {
	const content = await readFile(
		join(directory, 'home/.local/state/domfiles/run'),
		'utf8'
	);

	return Number(content.split('\n', 1)[0]?.split(' ')[1]);
}

async function waitForOutput(run: { stdout: () => string }, text: string) {
	await vi.waitFor(
		() => {
			expect(run.stdout()).toContain(text);
		},
		{ interval: 50, timeout: LOCK_TEST_TIMEOUT }
	);
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

test.each([
	{ behavior: 'exits on `TERM`', script: HANGING_TRANSPORT },
	{ behavior: 'ignores `TERM`', script: TERM_IGNORING_TRANSPORT },
])(
	'stops a repository fetch whose transport $behavior at its time limit and continues the sync',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async ({ script }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout } = await createUpstreamFixture(
			directory.path,
			'master'
		);
		const transport = join(directory.path, 'transport');
		const transportPids = join(directory.path, 'transport.pids');

		await writeFile(transportPids, '');
		await writeExecutable(transport, script);
		await writeExecutable(
			join(directory.path, 'stubs/sleep'),
			LIMIT_SLEEP_STUB
		);
		runGit(checkout, ['config', 'protocol.ext.allow', 'always']);
		runGit(checkout, ['remote', 'set-url', 'origin', `ext::${transport}`]);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...GIT_ENV,
				LIMIT_REACHED: join(directory.path, 'limit-reached'),
				TRANSPORT_PIDS: transportPids,
			}
		);
		const pids = await readPids(transportPids);

		expect(result.status).toBe(0);
		expect(result.stderr).toBe(
			`$ git -C ${await realpath(checkout)} fetch --quiet\n× Failed to fetch the latest changes within 1 minute. Continuing with the local checkout\n`
		);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(pids).toHaveLength(2);
		await vi.waitFor(
			() => {
				expect(pids.filter(isRunning)).toStrictEqual([]);
			},
			{ timeout: REPOSITORY_TEST_TIMEOUT }
		);
	}
);

test(
	'reports a fetch that Git ends with status 124 as an ordinary failure',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout } = await createUpstreamFixture(
			directory.path,
			'master'
		);

		await writeExecutable(
			join(directory.path, 'stubs/git'),
			`#!/bin/sh
if [ "$3" = fetch ]; then
  exit 124
fi

PATH="\${PATH#*:}" exec git "$@"
`
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(result.stderr).toBe(
			`$ git -C ${await realpath(checkout)} fetch --quiet\n× Failed to fetch the latest changes. Continuing with the local checkout\n`
		);
		expect(result.stages).toStrictEqual(['stage core']);
	}
);

test(
	'stops the SSH access check at its time limit and leaves `origin` unchanged',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout } = await createUpstreamFixture(
			directory.path,
			'master'
		);
		const origin = 'https://github.com/foo/bar';
		const ssh = join(directory.path, 'home/.ssh');
		const transport = join(directory.path, 'transport');
		const transportPids = join(directory.path, 'transport.pids');

		await mkdir(ssh);
		await writeFile(join(ssh, 'id_ed25519'), 'foo\n');
		await writeFile(join(ssh, 'id_ed25519.pub'), 'foo\n');
		await writeFile(transportPids, '');
		await writeExecutable(transport, HANGING_TRANSPORT);
		await writeExecutable(
			join(directory.path, 'stubs/sleep'),
			LIMIT_SLEEP_STUB
		);
		runGit(checkout, ['config', 'protocol.https.allow', 'never']);
		runGit(checkout, ['remote', 'set-url', 'origin', origin]);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...GIT_ENV,
				GIT_SSH_COMMAND: transport,
				GIT_SSH_VARIANT: 'simple',
				LIMIT_REACHED: join(directory.path, 'limit-reached'),
				TRANSPORT_PIDS: transportPids,
			}
		);
		const pids = await readPids(transportPids);
		const checkoutPath = await realpath(checkout);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '$ ')).toStrictEqual([
			`$ git -C ${checkoutPath} ls-remote git@github.com:foo/bar HEAD`,
			`$ git -C ${checkoutPath} fetch --quiet`,
		]);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			`× Failed to verify SSH access to \`git@github.com:foo/bar\` within 1 minute. Leaving \`origin\` as \`${origin}\``,
			'× Failed to fetch the latest changes. Continuing with the local checkout',
		]);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(runGit(checkout, ['remote', 'get-url', 'origin'])).toBe(
			`${origin}\n`
		);
		expect(pids).toHaveLength(2);
		await vi.waitFor(
			() => {
				expect(pids.filter(isRunning)).toStrictEqual([]);
			},
			{ timeout: REPOSITORY_TEST_TIMEOUT }
		);
	}
);

test(
	'stops the fetch time limit once the fetch finishes',
	{ timeout: REPOSITORY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const { checkout, upstream } = await createUpstreamFixture(
			directory.path,
			'master'
		);
		const timerPids = join(directory.path, 'timer.pids');
		const transport = join(directory.path, 'transport');

		await writeFile(timerPids, '');
		await writeExecutable(
			join(directory.path, 'stubs/sleep'),
			'#!/bin/sh\nprintf \'%s\\n\' "$$" >> "$TIMER_PIDS"\nexec tail -f /dev/null\n'
		);
		await writeExecutable(
			transport,
			'#!/bin/sh\nuntil [ -s "$TIMER_PIDS" ]; do\n  command -p sleep 0.1\ndone\n\nexec git upload-pack "$1"\n'
		);
		runGit(checkout, ['config', 'protocol.ext.allow', 'always']);
		runGit(checkout, [
			'remote',
			'set-url',
			'origin',
			`ext::${transport} ${upstream}`,
		]);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{ ...GIT_ENV, TIMER_PIDS: timerPids }
		);
		const pids = await readPids(timerPids);

		expect(result.status).toBe(0);
		expect(result.stdout).toContain('✓ Up to date');
		expect(result.stages).toStrictEqual(['stage core']);
		expect(pids).toHaveLength(1);
		await vi.waitFor(
			() => {
				expect(pids.filter(isRunning)).toStrictEqual([]);
			},
			{ timeout: REPOSITORY_TEST_TIMEOUT }
		);
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
		const checkout = await realpath(join(directory.path, 'domfiles'));

		expect(result.status).toBe(0);
		expect(result.stages).toStrictEqual([`stage ${stage}`]);
		expect(result.stdout).not.toContain('dependencies');
		expect(result.commands).toBe(
			`git -C ${checkout}/home/.local/bin/../../.. rev-parse HEAD\nbrew --version\ngit -C ${checkout} rev-parse HEAD\n`
		);
	}
);

test('runs a named stage through the checkout entrypoint', async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	await createFixture(directory.path);

	const result = await runCommand(directory.path, 'domfiles-sync', ['setup']);
	const checkout = await realpath(join(directory.path, 'domfiles'));

	expect(result.status).toBe(0);
	expect(result.stages).toStrictEqual(['stage setup']);
	expect(result.commands).toBe(
		`git -C ${checkout}/home/.local/bin/../../.. rev-parse HEAD\nbrew --version\ngit -C ${checkout} rev-parse HEAD\n`
	);
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
	'keeps links that already point at their source and replaces other links',
	{ timeout: PACKAGE_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { checkout, env } = await createPackageFixture(directory.path);
		const current = join(directory.path, 'home/.config');
		const newline = join(directory.path, 'home/.curlrc');
		const stale = join(directory.path, 'home/.vimrc');

		await symlink(join(checkout, 'home/.config'), current);
		await symlink(`${join(checkout, 'home/.curlrc')}\n`, newline);
		await symlink(join(directory.path, 'foo'), stale);

		const before = await lstat(current, { bigint: true });
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
		const after = await lstat(current, { bigint: true });

		expect(result.status).toBe(0);
		expect({ ctimeNs: after.ctimeNs, ino: after.ino }).toStrictEqual({
			ctimeNs: before.ctimeNs,
			ino: before.ino,
		});
		await expect(readlink(newline)).resolves.toBe(
			join(checkout, 'home/.curlrc')
		);
		await expect(readlink(stale)).resolves.toBe(
			join(checkout, 'home/.vimrc')
		);
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

test.each([
	{
		failStage: '',
		message: 'An identical sync succeeded. Skipping this run',
		outcome: 'success',
		status: 0,
		stream: 'stdout',
	},
	{
		failStage: 'core',
		message: 'An identical sync failed. Exiting with status 1',
		outcome: 'failure',
		status: 1,
		stream: 'stderr',
	},
] as const)(
	'passes the owner’s $outcome to an identical waiter without running it',
	{ timeout: LOCK_TEST_TIMEOUT },
	async ({ failStage, message, status, stream }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				FAIL_STAGE: failStage,
			}
		);

		await waitForLine(log, `start core ${owner.pid}`);

		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;

		expect(ownerResult.status).toBe(status);
		expect(waiterResult.status).toBe(status);
		expect(waiterResult[stream]).toContain(message);
		expect(waiterResult.stages).toStrictEqual([]);
		expect(stageEvents(await readLog(log))).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
		]);
	}
);

test.each([
	{
		checkout: 'domfiles',
		params: ['sync', 'setup'],
		stage: 'setup',
		title: 'a different request',
	},
	{
		checkout: 'domfiles-copy',
		params: ['sync', '--core'],
		stage: 'core',
		title: 'the same request from another checkout',
	},
])(
	'queues $title and runs it after the owner finishes',
	{ timeout: LOCK_TEST_TIMEOUT },
	async ({ checkout, params, stage }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);

		await cp(
			join(directory.path, 'domfiles'),
			join(directory.path, 'domfiles-copy'),
			{ recursive: true }
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForLine(log, `start core ${owner.pid}`);

		const waiter = startCommand(
			directory.path,
			'domfiles',
			params,
			env,
			checkout
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;

		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stages).toStrictEqual([`stage ${stage}`]);
		expect(stageEvents(await readLog(log))).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
			`start ${stage} ${waiter.pid}`,
			`end ${stage} ${waiter.pid}`,
		]);
	}
);

test.each(['SIGKILL', 'SIGTERM'] as const)(
	'lets a waiter take over from an owner killed with %s',
	{ timeout: LOCK_TEST_TIMEOUT },
	async (signal) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);
		const gatePid = await waitForGate(log, owner.pid);
		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		process.kill(owner.pid!, signal);
		process.kill(gatePid, 'SIGKILL');
		await waitForLine(log, `start core ${waiter.pid}`);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;
		const laterResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		expect(ownerResult.status).toBeNull();
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).toContain(
			'The previous sync left no result. Starting this run'
		);
		expect(laterResult.status).toBe(0);
		expect(laterResult.stages).toStrictEqual(['stage core']);
	}
);

test(
	'runs queued requests one at a time after the owner dies',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);
		const gatePid = await waitForGate(log, owner.pid);
		const waiters = ['install', 'migrate', 'setup'].map((stage) =>
			startCommand(directory.path, 'domfiles', ['sync', stage], env)
		);

		await Promise.all(
			waiters.map(async (waiter) =>
				waitForOutput(waiter, LOCK_WAITING_MESSAGE)
			)
		);
		process.kill(owner.pid!, 'SIGKILL');
		process.kill(gatePid, 'SIGKILL');
		await writeFile(gate, '');

		const results = await Promise.all(
			waiters.map(async (waiter) => waiter.done)
		);
		const events = stageEvents(await readLog(log));
		const starts = events
			.slice(1)
			.filter((line) => line.startsWith('start '));

		expect(results.map((result) => result.status)).toStrictEqual([0, 0, 0]);
		expect(events[0]).toBe(`start core ${owner.pid}`);
		expect(events.slice(1)).toStrictEqual(
			starts.flatMap((line) => [line, line.replace(/^start/, 'end')])
		);
		expect(
			starts
				.map((line) => line.replace(/^start (\S+) .*$/, '$1'))
				.toSorted((first, second) => first.localeCompare(second))
		).toStrictEqual(['install', 'migrate', 'setup']);
	}
);

test(
	'keeps the lock across the restart after a repository update',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const { update } = await createUpstreamFixture(
			directory.path,
			'master'
		);

		runGit(update, ['commit', '--quiet', '--allow-empty', '--message=bar']);
		runGit(update, ['push', '--quiet']);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...GIT_ENV,
				...env,
			}
		);

		await waitForLine(log, `start core ${owner.pid}`);

		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			{
				...GIT_ENV,
				...env,
			}
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;
		const lines = await readLog(log);

		expect(ownerResult.status).toBe(0);
		expect(ownerResult.stdout).toContain(
			'Restarting the sync after the update'
		);
		expect(ownerResult.stdout).not.toContain(LOCK_WAITING_MESSAGE);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).not.toContain(CHANGED_CHECKOUT_RESTART);
		expect(lines).not.toContainEqual(expect.stringMatching(/^holder \S/));
		expect(stageEvents(lines)).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
			`start setup ${waiter.pid}`,
			`end setup ${waiter.pid}`,
		]);
	}
);

test.each([
	{ command: 'sync', params: ['sync'], stages: FULL_RUN_STAGES },
	{
		command: 'sync install',
		params: ['sync', 'install'],
		stages: ['install'],
	},
])(
	'runs `domfiles $command` with the updated code after waiting for the lock',
	{ timeout: LOCK_TEST_TIMEOUT },
	async ({ params, stages }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, fetchGate, release } = await createChangedCodeFixture(
			directory.path
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForLine(fetchGate, 'gate');

		const waiter = startCommand(directory.path, 'domfiles', params, env);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await release();

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;

		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).toContain(CHANGED_CHECKOUT_RESTART);
		expect(lines(waiterResult.stdout, 'helper')).toStrictEqual(['helper']);
		expect(waiterResult.stages).toStrictEqual(
			stages.map((stage) => `stage ${stage}`)
		);
	}
);

test(
	'keeps the lock across the restart after the checkout changed during the wait',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, fetchGate, gate, log, release } =
			await createChangedCodeFixture(directory.path);
		const gatedEnv = { ...env, STAGE_GATE: gate };

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForLine(fetchGate, 'gate');

		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			gatedEnv
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);

		const holders = listChildProcesses(waiter.pid ?? 0);

		await release();
		await waitForLine(log, `start install ${waiter.pid}`);

		const holder = await readSyncHolder(directory.path);

		const third = startCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			gatedEnv
		);

		await waitForOutput(third, LOCK_WAITING_MESSAGE);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;
		const thirdResult = await third.done;
		const events = await readLog(log);

		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).toContain(CHANGED_CHECKOUT_RESTART);
		expect(holders).toContain(holder);
		expect(thirdResult.status).toBe(0);
		expect(events).not.toContainEqual(expect.stringMatching(/^holder \S/));
		expect(stageEvents(events)).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
			`start install ${waiter.pid}`,
			`end install ${waiter.pid}`,
			`start setup ${third.pid}`,
			`end setup ${third.pid}`,
		]);
	}
);

test(
	'restarts when an update finishes between loading the modules and waiting for the lock',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, fetchGate, gate, log, release } =
			await createChangedCodeFixture(directory.path);
		const brewGate = join(directory.path, 'brew.log');
		const brewRelease = join(directory.path, 'brew.release');

		await writeFile(brewGate, '');
		await writeExecutable(
			join(directory.path, 'stubs/brew'),
			BREW_GATE_STUB
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				...env,
				STAGE_GATE: gate,
			}
		);

		await waitForLine(fetchGate, 'gate');

		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			{ ...env, BREW_GATE: brewGate, BREW_RELEASE: brewRelease }
		);

		await waitForLine(brewGate, 'gate');
		await release();
		await waitForLine(log, `start core ${owner.pid}`);
		await writeFile(brewRelease, '');
		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;

		expect(ownerResult.status).toBe(0);
		expect(ownerResult.stdout).toContain(
			'Restarting the sync after the update'
		);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).toContain(CHANGED_CHECKOUT_RESTART);
		expect(lines(waiterResult.stdout, 'helper')).toStrictEqual(['helper']);
		expect(waiterResult.stages).toStrictEqual(['stage install']);
	}
);

test.each([
	{ lookup: 1, when: 'before it waits for the lock' },
	{ lookup: 2, when: 'once it holds the lock' },
])(
	'stops when it cannot read the checkout’s commit $when',
	{ timeout: LOCK_TEST_TIMEOUT },
	async ({ lookup }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const log = join(directory.path, 'lookups.log');

		await writeFile(log, '');
		await writeExecutable(
			join(directory.path, 'stubs/git'),
			FAILING_LOOKUP_GIT_STUB
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			{ FAILING_LOOKUP: String(lookup), LOOKUP_LOG: log }
		);

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to read `HEAD` in the domfiles checkout',
		]);
		expect(result.stages).toStrictEqual([]);
		await expect(readLog(log)).resolves.toHaveLength(lookup);
	}
);

test(
	'answers `--help` and rejects usage errors while the lock is held',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForLine(log, `start core ${owner.pid}`);

		const helpResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--help'],
			env
		);
		const usageResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'foo'],
			env
		);

		await writeFile(gate, '');

		const ownerResult = await owner.done;

		expect(helpResult.status).toBe(0);
		expect(helpResult.stdout).toContain('Usage: domfiles sync');
		expect(usageResult.status).toBe(1);
		expect(usageResult.stderr).toContain('Usage: domfiles sync');
		expect(ownerResult.status).toBe(0);
	}
);

test.each(['SIGHUP', 'SIGINT', 'SIGQUIT', 'SIGTERM'] as const)(
	'keeps the lock until a sync that survives %s to its group exits',
	{ timeout: LOCK_TEST_TIMEOUT },
	async (signal) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const sync = join(
			directory.path,
			'domfiles/home/.local/bin/domfiles-sync'
		);
		const source = await readFile(sync, 'utf8');

		await writeFile(
			sync,
			source.replace(
				/^.*\n/,
				(shebang) =>
					`${shebang}\ntrap 'printf "sync signaled %s\\n" "$$" >> "$STAGE_LOG"' HUP INT QUIT TERM\n`
			)
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);
		const gatePid = await waitForGate(log, owner.pid);
		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			{
				...env,
				STAGE_OWNER: String(owner.pid),
			}
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);

		const holderPid = await readSyncHolder(directory.path);

		process.kill(-owner.pid!, signal);
		await waitForLine(log, `signaled ${gatePid}`);

		const holderLive = isProcessLive(holderPid);

		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;
		const lines = await readLog(log);

		expect(holderLive).toBe(true);
		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(stageEvents(lines)).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
			`start setup ${waiter.pid}`,
			`end setup ${waiter.pid}`,
		]);
		expect(lines).toContainEqual(
			expect.stringMatching(/^owner (?:Z\S*)?$/)
		);
	}
);

test(
	'runs an identical request whose owner died, even after an earlier identical run finished',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);

		await writeFile(gate, '');

		const firstResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await rm(gate);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);
		const gatePid = await waitForGate(log, owner.pid);
		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		process.kill(owner.pid!, 'SIGKILL');
		process.kill(gatePid, 'SIGKILL');
		await waitForLine(log, `start core ${waiter.pid}`);
		await writeFile(gate, '');
		await owner.done;

		const waiterResult = await waiter.done;

		expect(firstResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stdout).toContain(
			'The previous sync left no result. Starting this run'
		);
		expect(waiterResult.stages).toStrictEqual(['stage core']);
	}
);

test(
	'stops a sync started inside another sync at once',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { log } = await createLockFixture(directory.path);
		const nestedOutput = join(directory.path, 'nested.log');
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				NESTED_OUTPUT: nestedOutput,
				STAGE_LOG: log,
				STAGE_NESTED: 'setup',
			}
		);
		const nested = stripVTControlCharacters(
			await readFile(nestedOutput, 'utf8')
		);

		expect(result.status).toBe(0);
		await expect(readLog(log)).resolves.toContain('nested 1');
		expect(nested).toContain(
			'× `domfiles sync` cannot run inside another sync'
		);
		expect(nested).not.toContain(LOCK_WAITING_MESSAGE);
	}
);

test(
	'keeps the lock while `ps` fails during a run',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const psFailure = join(directory.path, 'ps-failure');
		const psLog = join(directory.path, 'ps.log');
		const psEnv = { ...env, PS_FAILURE: psFailure, PS_LOG: psLog };

		await writeFile(psLog, '');
		await writeExecutable(
			join(directory.path, 'stubs/ps'),
			FAILABLE_PS_STUB
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			psEnv
		);

		await waitForLine(log, `start core ${owner.pid}`);

		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			psEnv
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);

		const holderPid = await readSyncHolder(directory.path);

		await writeFile(psFailure, '');
		await vi.waitFor(
			async () => {
				const calls = await readLog(psLog);

				expect(calls.length).toBeGreaterThanOrEqual(2);
			},
			{ interval: 50, timeout: LOCK_TEST_TIMEOUT }
		);

		const holderLive = isProcessLive(holderPid);

		await writeFile(gate, '');

		const ownerResult = await owner.done;
		const waiterResult = await waiter.done;

		expect(holderLive).toBe(true);
		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		await vi.waitFor(
			() => {
				expect(listFixtureProcesses(directory.path)).toStrictEqual([]);
			},
			{ interval: 50, timeout: LOCK_TEST_TIMEOUT }
		);
		expect(stageEvents(await readLog(log))).toStrictEqual([
			`start core ${owner.pid}`,
			`end core ${owner.pid}`,
			`start setup ${waiter.pid}`,
			`end setup ${waiter.pid}`,
		]);
	}
);

test(
	'stops a sync that would wait while `ps` fails',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const psFailure = join(directory.path, 'ps-failure');
		const psEnv = {
			...env,
			PS_FAILURE: psFailure,
			PS_LOG: join(directory.path, 'ps.log'),
		};

		await writeExecutable(
			join(directory.path, 'stubs/ps'),
			FAILABLE_PS_STUB
		);

		const owner = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			psEnv
		);

		await waitForLine(log, `start core ${owner.pid}`);
		await writeFile(psFailure, '');

		const waiterResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'setup'],
			psEnv
		);

		await writeFile(gate, '');

		const ownerResult = await owner.done;

		expect(waiterResult.status).toBe(1);
		expect(waiterResult.stderr).toContain(
			'× Failed to check whether this sync is nested. `ps` could not list processes'
		);
		expect(waiterResult.stages).toStrictEqual([]);
		expect(ownerResult.status).toBe(0);
	}
);

test(
	'runs the restarted sync while `ps` fails',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { env, gate, log } = await createLockFixture(directory.path);
		const { update } = await createUpstreamFixture(
			directory.path,
			'master'
		);

		runGit(update, ['commit', '--quiet', '--allow-empty', '--message=bar']);
		runGit(update, ['push', '--quiet']);
		await writeExecutable(join(directory.path, 'stubs/ps'), FAILING_STUB);
		await writeFile(gate, '');

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{ ...GIT_ENV, ...env }
		);

		expect(result.status).toBe(0);
		expect(result.stdout).toContain('Restarting the sync after the update');
		expect(result.stdout).not.toContain(LOCK_WAITING_MESSAGE);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(stageEvents(await readLog(log))).toHaveLength(2);
	}
);

test(
	'runs a waiter without reporting a missing result when the previous sync had finished',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { log } = await createLockFixture(directory.path);
		const linger = join(directory.path, 'linger');
		const psFailure = join(directory.path, 'ps-failure');

		await writeFile(psFailure, '');
		await writeExecutable(
			join(directory.path, 'stubs/ps'),
			FAILABLE_PS_STUB
		);

		const ownerResult = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{
				PS_FAILURE: psFailure,
				PS_LOG: join(directory.path, 'ps.log'),
				STAGE_LINGER: linger,
				STAGE_LOG: log,
			}
		);
		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{ STAGE_LOG: log }
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);
		await writeFile(linger, '');

		const waiterResult = await waiter.done;

		expect(ownerResult.status).toBe(0);
		expect(waiterResult.status).toBe(0);
		expect(waiterResult.stages).toStrictEqual(['stage core']);
		expect(waiterResult.stdout).not.toContain(
			'The previous sync left no result'
		);
	}
);

test(
	'restricts the sync state directory to the user and keeps the mode of its parent',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const parent = join(directory.path, 'home/.local/state');

		await mkdir(parent, { recursive: true });
		await chmod(parent, 0o755);

		const result = await runCommand(directory.path, 'domfiles', [
			'sync',
			'--core',
		]);
		const parentStats = await lstat(parent);
		const stateStats = await lstat(join(parent, 'domfiles'));

		expect(result.status).toBe(0);
		expect(stateStats.isDirectory()).toBe(true);
		expect(stateStats.mode % 0o1000).toBe(0o700);
		expect(parentStats.mode % 0o1000).toBe(0o755);
	}
);

test.each([
	{
		error: '× Failed to prepare the sync state directory `~/.local/state/domfiles`',
		path: 'home/.local/state',
	},
	{
		error: '× The sync state path `~/.local/state/domfiles` must be a directory',
		path: 'home/.local/state/domfiles',
	},
])(
	'stops without changing a file at `$path`',
	{ timeout: LOCK_TEST_TIMEOUT },
	async ({ error, path }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const file = join(directory.path, path);

		await mkdir(join(file, '..'), { recursive: true });
		await writeFile(file, 'foo\n');
		await chmod(file, 0o644);

		const result = await runCommand(directory.path, 'domfiles', [
			'sync',
			'--core',
		]);
		const stats = await lstat(file);

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([error]);
		expect(result.stages).toStrictEqual([]);
		expect(stats.isFile()).toBe(true);
		expect(stats.mode % 0o1000).toBe(0o644);
		await expect(readFile(file, 'utf8')).resolves.toBe('foo\n');
	}
);

test(
	'stops without following a symlink at the sync state directory',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const link = join(directory.path, 'home/.local/state/domfiles');
		const target = join(directory.path, 'foo');

		await mkdir(target, { mode: 0o755 });
		await mkdir(join(link, '..'), { recursive: true });
		await symlink(target, link);

		const result = await runCommand(directory.path, 'domfiles', [
			'sync',
			'--core',
		]);

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Cannot use symlink `~/.local/state/domfiles` as the sync state directory',
		]);
		expect(result.stages).toStrictEqual([]);
		await expect(readlink(link)).resolves.toBe(target);
		await expect(readdir(target)).resolves.toStrictEqual([]);
		const stats = await lstat(target);

		expect(stats.mode % 0o1000).toBe(0o755);
	}
);

test(
	'stops before any stage without `lockf` or `flock`',
	{ timeout: LOCK_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		await createFixture(directory.path);

		const systemPath = await createSystemPath(directory.path, [
			'flock',
			'lockf',
		]);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			{ PATH: `${join(directory.path, 'stubs')}:${systemPath}` }
		);

		expect(result.status).toBe(1);
		expect(result.stderr).toContain(
			'× The sync lock requires `lockf` or `flock`'
		);
		expect(result.stages).toStrictEqual([]);
	}
);
