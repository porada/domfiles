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
	rename,
	rm,
	symlink,
	writeFile,
} from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, join } from 'node:path';
import { stripVTControlCharacters } from 'node:util';
import { expect, onTestFinished, test, vi } from 'vite-plus/test';

const ANY_CAUSE =
	/Creating a directory for|Replacing the directory|Sync cannot add|Sync cannot update paths|Sync prepared the update using|The checkout changed while sync prepared|The update would overwrite|`master` moved before sync could update it/;
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
const CASE_RENAMES = [
	{ added: 'Grault', from: 'grault', spelled: 'Grault', to: 'Grault' },
	{ added: 'Garply/waldo', from: 'garply', spelled: 'Garply', to: 'Garply' },
];
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
// Whether the fixture filesystem treats uppercase and lowercase names as equal
const FOLDS_CASE = await (async () => {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);

	await writeFile(join(directory.path, 'foo'), '');

	try {
		await lstat(join(directory.path, 'FOO'));

		return true;
	} catch {
		return false;
	}
})();
// Runs `grep`, but finds `garply` among the switch paths, so that recovery
// accepts a symlink in place of that directory and leaves it to the rollback
const FOREIGN_PARENT_GREP_STUB = `#!/bin/sh
PATH="\${PATH#*:}"

case "$*" in
  *' -- garply '*/switch.names)
    exit 0
    ;;
esac

exec grep "$@"
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
// Pauses the first Git command matching `GIT_GATE`. `GIT_FAILURES` lists
// command patterns to fail in order with status 128. Both match commands
// without global options, with a `temp-index` prefix for a temporary index.
// Records commands and failures in separate logs
const GIT_GATE_STUB = `#!/bin/sh
PATH="\${PATH#*:}"

print_command() {
  if [ -n "\${GIT_INDEX_FILE-}" ]; then
    printf 'temp-index '
  fi

  while :; do
    case "$1" in
      -c | -C)
        shift 2
        ;;
      *)
        break
        ;;
    esac
  done

  printf '%s\\n' "$*"
}

if [ -n "\${GIT_GATE-}\${GIT_FAILURES-}\${GIT_COMMAND_LOG-}" ]; then
  command="$(print_command "$@")"
fi

if [ -n "\${GIT_GATE-}" ] && [ ! -e "$GIT_GATE_LOG.done" ]; then
  case "$command" in
    $GIT_GATE)
      : > "$GIT_GATE_LOG.done"
      printf 'gate %s %s\\n' "$$" "$command" >> "$GIT_GATE_LOG"

      until [ -e "$GIT_GATE_RELEASE" ]; do
        command -p sleep 0.05
      done
      ;;
  esac
fi

if [ -n "\${GIT_FAILURES-}" ]; then
  failed="$(wc -l < "$FAILURE_LOG")"
  pattern="$(printf '%s\\n' "$GIT_FAILURES" | sed -n "$((failed + 1))p")"

  if [ -n "$pattern" ]; then
    case "$command" in
      $pattern)
        printf '%s\\n' "$command" >> "$FAILURE_LOG"
        exit 128
        ;;
    esac
  fi
fi

if [ -n "\${GIT_COMMAND_LOG-}" ]; then
  printf '%s\\n' "$command" >> "$GIT_COMMAND_LOG"
fi

exec git "$@"
`;
const HANGING_TRANSPORT = `#!/bin/sh
printf '%s\\n' "$$" "$PPID" >> "$TRANSPORT_PIDS"
exec tail -f /dev/null
`;
const INTERRUPTIONS = [
	{ signal: 'SIGTERM', status: 143, target: 'process' },
	{ signal: 'SIGTERM', status: 143, target: 'group' },
	{ signal: 'SIGKILL', status: null, target: 'process' },
	{ signal: 'SIGKILL', status: null, target: 'group' },
] as const;
// Skip permission tests when running as root
const IS_ROOT = process.getuid?.() === 0;
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
const MOVED_MASTER_TREE_MESSAGE =
	'The tree at the new tip of `master` differs from both the original tree and the saved index';
const PACKAGE_TEST_TIMEOUT = 20_000;
const RECOVERY_CAUSE =
	/Recovery expected a directory at|The checkout is no longer on|The index differs from both versions|contains a change the interrupted update cannot account for|contains skipped files|has a Git-quoted name that sync cannot restore|may still have an operation in progress|points to a commit outside this update/;
const RECOVERY_TEST_TIMEOUT = 40_000;
const RECOVERY_WAITING_MESSAGE =
	'Waiting for the previous repository update to release the checkout lock';
const REPOSITORY = join(import.meta.dirname, '../../..');
const REPOSITORY_TEST_TIMEOUT = 10_000;
// Stalls the first smudge that sync runs through a temporary index, while Git
// holds that index’s lock
const SMUDGE_GATE_STUB = `#!/bin/sh
if [ -n "\${SMUDGE_LOG-}" ] && [ -n "\${GIT_INDEX_FILE-}" ] && [ ! -e "$SMUDGE_LOG.done" ]; then
  : > "$SMUDGE_LOG.done"
  printf 'smudge %s\\n' "$$" >> "$SMUDGE_LOG"

  until [ -e "$SMUDGE_RELEASE" ]; do
    command -p sleep 0.05
  done
fi

exec cat
`;
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
// Moves files like `mv`, then removes read permission from the first
// `switch.untracked` it writes and logs the change to `FAILURE_LOG`
const UNREADABLE_LISTING_STUB = `#!/bin/sh
PATH="\${PATH#*:}"
mv "$@" || exit

for param do
  target="$param"
done

case "$target" in
  */switch.untracked)
    if [ ! -s "$FAILURE_LOG" ]; then
      chmod 000 "$target"
      printf 'mv %s\\n' "$*" >> "$FAILURE_LOG"
    fi
    ;;
esac
`;
const UPDATE_STEPS = [
	{ gate: 'temp-index stash create', step: 'saving the uncommitted changes' },
	{ gate: 'worktree add *', step: 'adding its worktree' },
	{ gate: 'rebase *', step: 'rebasing' },
	{ gate: 'stash apply *', step: 'reapplying the changes' },
	{ gate: 'temp-index read-tree -m -u *', step: 'writing the working tree' },
	{ gate: 'read-tree [0-9a-f]*', step: 'writing the index' },
	{ gate: 'update-ref -m * refs/heads/master *', step: 'moving `master`' },
];
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

	for (const module of ['lock', 'recovery', 'repository']) {
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

function runGit(cwd: string, params: string[], input?: string) {
	const result = spawnSync('git', params, {
		cwd,
		encoding: 'utf8',
		env: { ...GIT_ENV, PATH: process.env.PATH ?? '' },
		input,
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

async function waitForOutput(
	run: { stdout: () => string },
	text: string,
	timeout = LOCK_TEST_TIMEOUT
) {
	await vi.waitFor(
		() => {
			expect(run.stdout()).toContain(text);
		},
		{ interval: 50, timeout }
	);
}

function numberedLines(first: string, last: string) {
	const lines = Array.from({ length: 18 }, (_, index) => `${index + 2}`);

	return `${[first, ...lines, last].join('\n')}\n`;
}

// Builds a checkout on `master` one commit ahead of its upstream and one behind
// it, with staged, unstaged, and untracked changes and a stash entry
async function createRecoveryFixture(directory: string) {
	await createFixture(directory);

	const checkout = join(directory, 'domfiles');

	await writeFile(join(checkout, 'foo'), numberedLines('1', '20'));
	await writeFile(join(checkout, 'bar'), 'bar\n');
	await writeFile(join(checkout, 'baz'), 'baz\n');
	await writeFile(join(checkout, 'grault'), 'grault\n');
	await mkdir(join(checkout, 'garply'));
	await writeFile(join(checkout, 'garply/waldo'), 'waldo\n');

	const { update } = await createUpstreamFixture(directory, 'master');

	await writeExecutable(join(directory, 'stubs/git'), GIT_GATE_STUB);
	await writeFile(join(update, 'foo'), numberedLines('1', 'twenty'));
	await writeFile(join(update, 'qux'), 'qux\n');
	runGit(update, ['add', '--all']);
	runGit(update, ['commit', '--quiet', '--message=bar']);
	runGit(update, ['push', '--quiet']);
	await writeFile(join(checkout, 'quux'), 'quux\n');
	runGit(checkout, ['add', 'quux']);
	runGit(checkout, ['commit', '--quiet', '--message=baz']);
	await writeFile(join(checkout, 'bar'), 'corge\n');
	runGit(checkout, ['stash', 'push', '--quiet', '--message=foo']);
	await writeFile(join(checkout, 'foo'), numberedLines('one', '20'));
	runGit(checkout, ['add', 'foo']);
	await writeFile(join(checkout, 'bar'), 'grault\n');
	await writeFile(join(checkout, 'baz'), 'garply\n');
	runGit(checkout, ['add', 'baz']);
	await writeFile(join(checkout, 'baz'), 'waldo\n');
	await writeFile(join(checkout, 'fred'), 'fred\n');

	return checkout;
}

function readCheckoutState(checkout: string) {
	return {
		branch: runGit(checkout, ['rev-parse', '--symbolic-full-name', 'HEAD']),
		commits: runGit(checkout, ['log', '--format=%H %s']),
		index: runGit(checkout, ['ls-files', '--stage']),
		stashes: runGit(checkout, ['stash', 'list', '--format=%H']),
		status: runGit(checkout, [
			'status',
			'--porcelain',
			'--untracked-files=all',
		]),
		worktree: runGit(checkout, ['diff']),
	};
}

async function readUpdatedState(checkout: string) {
	return {
		branch: runGit(checkout, ['symbolic-ref', 'HEAD']),
		commits: runGit(checkout, ['log', '--format=%s']),
		files: {
			bar: await readFile(join(checkout, 'bar'), 'utf8'),
			baz: await readFile(join(checkout, 'baz'), 'utf8'),
			foo: await readFile(join(checkout, 'foo'), 'utf8'),
			stagedBaz: runGit(checkout, ['show', ':baz']),
			stagedFoo: runGit(checkout, ['show', ':foo']),
		},
		status: runGit(checkout, [
			'status',
			'--porcelain',
			'--untracked-files=all',
		]),
	};
}

async function readUpdateId(checkout: string) {
	const result = spawnSync(
		'sh',
		[
			'-c',
			'printf "%s" "$1" | cksum | awk "{ print \\$1 \\"-\\" \\$2 }"',
			'sh',
			await realpath(checkout),
		],
		{ encoding: 'utf8' }
	);

	return result.stdout.trim();
}

async function readUpdateLeftovers(directory: string, checkout: string) {
	const entries = await readdir(
		join(directory, 'home/.local/state/domfiles')
	);
	const worktrees = runGit(checkout, ['worktree', 'list', '--porcelain']);

	return {
		files: entries.filter(
			(entry) =>
				entry.startsWith('update-') &&
				!entry.endsWith('.lock') &&
				!entry.endsWith('.tmp')
		),
		refs: runGit(checkout, ['for-each-ref', 'refs/domfiles-sync/']),
		worktrees: lines(worktrees, 'worktree ').length,
	};
}

async function readUpdatePhase(directory: string, checkout: string) {
	const marker = await readFile(
		join(
			directory,
			'home/.local/state/domfiles',
			`update-${await readUpdateId(checkout)}`
		),
		'utf8'
	);

	return marker.split('\n')[2];
}

// Use the Git trace to observe when sync reaches the index lock check
async function waitForIndexLockLookup(readCommands: () => Promise<string>) {
	await vi.waitFor(
		async () => {
			await expect(readCommands()).resolves.toMatch(
				/^rev-parse --path-format=absolute --git-path index\.lock$/m
			);
		},
		{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
	);
}

async function waitForSyncOwner(directory: string, pid: number | undefined) {
	await vi.waitFor(
		async () => {
			const content = await readFile(
				join(directory, 'home/.local/state/domfiles/run'),
				'utf8'
			);

			expect(Number(content.split(' ', 1)[0])).toBe(pid);
		},
		{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
	);
}

async function startGatedSync(
	directory: string,
	params: string[],
	gate: string,
	name = 'gate',
	env: Record<string, string> = {}
) {
	const log = join(directory, `${name}.log`);
	const release = join(directory, `${name}.release`);

	await writeFile(log, '');

	const run = startCommand(directory, 'domfiles', params, {
		...GIT_ENV,
		...env,
		GIT_GATE: gate,
		GIT_GATE_LOG: log,
		GIT_GATE_RELEASE: release,
	});

	await vi.waitFor(
		async () => {
			const content = await readLog(log);

			expect(content).toHaveLength(1);
		},
		{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
	);

	return {
		release: async () => {
			await writeFile(release, '');
		},
		run,
	};
}

// Stops a sync at Git command `gate` and sends `signal` to the sync process
// alone or to its process group
async function interruptSync(
	directory: string,
	params: string[],
	gate: string,
	{
		signal,
		target,
	}: {
		signal: NodeJS.Signals;
		status?: number | null;
		target: 'group' | 'process';
	},
	name = 'gate'
) {
	const gated = await startGatedSync(directory, params, gate, name);
	const pid = gated.run.pid ?? 0;

	process.kill(target === 'group' ? -pid : pid, signal);

	return gated;
}

// Runs a named sync stage after an interruption. If `KILL` stopped only the
// sync process, starts the stage before releasing its surviving Git command
async function recoverInterruptedSync(
	directory: string,
	gated: Awaited<ReturnType<typeof startGatedSync>>,
	{
		signal,
		target,
	}: {
		signal: NodeJS.Signals;
		status?: number | null;
		target: 'group' | 'process';
	}
) {
	if (signal === 'SIGKILL' && target === 'process') {
		const recovery = startCommand(
			directory,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		await waitForSyncOwner(directory, recovery.pid);
		await gated.release();

		return {
			interrupted: await gated.run.done,
			recovered: await recovery.done,
		};
	}

	if (target === 'process') {
		await gated.release();
	}

	const interrupted = await gated.run.done;
	const recovered = await runCommand(
		directory,
		'domfiles',
		['sync', 'install'],
		GIT_ENV
	);

	return { interrupted, recovered };
}

// Runs the stubbed command from the rest of `PATH`. Fails the first call with
// an argument matching `pattern` with status 2 and logs it to `FAILURE_LOG`
function createArgumentFailureStub(pattern: string) {
	return `#!/bin/sh
PATH="\${PATH#*:}"

if [ ! -s "$FAILURE_LOG" ]; then
  for param do
    case "$param" in
      ${pattern})
        printf '%s\\n' "\${0##*/} $*" >> "$FAILURE_LOG"
        exit 2
        ;;
    esac
  done
fi

exec "\${0##*/}" "$@"
`;
}

type Failure = { command: string; stub: string } | { git: string[] };

// Installs a command stub or configures the Git failures for this case. Returns
// the environment and readers for the command and failure logs
async function injectFailure(directory: string, failure?: Failure) {
	const commandLog = join(directory, 'git.log');
	const failureLog = join(directory, 'failures.log');
	const patterns = failure && 'git' in failure ? failure.git : [];

	await writeFile(commandLog, '');
	await writeFile(failureLog, '');

	if (failure && 'stub' in failure) {
		await writeExecutable(
			join(directory, 'stubs', failure.command),
			failure.stub
		);
	}

	return {
		env: {
			...GIT_ENV,
			FAILURE_LOG: failureLog,
			GIT_COMMAND_LOG: commandLog,
			GIT_FAILURES: patterns.join('\n'),
		},
		failures: failure && 'stub' in failure ? 1 : patterns.length,
		readCommands: async () => readFile(commandLog, 'utf8'),
		readFailures: async () => readLog(failureLog),
	};
}

// Sets the mode of each path in `modes` under `checkout` and returns a function
// that restores the original modes
async function restrictModes(
	checkout: string,
	modes: Array<{ mode: number; path: string }> = []
) {
	const originals: Array<{ mode: number; path: string }> = [];

	for (const { mode, path } of modes) {
		const stats = await lstat(join(checkout, path));

		originals.unshift({ mode: stats.mode % 0o10000, path });
		await chmod(join(checkout, path), mode);
	}

	return async () => {
		for (const { mode, path } of originals) {
			await chmod(join(checkout, path), mode);
		}
	};
}

// Stops a core sync with `KILL` at Git command `gate`, by default once the
// switch has written the working tree, and applies `local`. Returns the
// checkout and its state at that point
async function interruptSwitch(
	directory: string,
	{
		change,
		gate = 'read-tree [0-9a-f]*',
		local,
	}: {
		gate?: string;
		change?: (update: string) => Promise<void>;
		local?: (checkout: string) => Promise<void>;
	}
) {
	const checkout = await createRecoveryFixture(directory);

	if (change) {
		await pushUpstreamChange(directory, change);
	}

	const update = await interruptSync(directory, ['sync', '--core'], gate, {
		signal: 'SIGKILL',
		status: null,
		target: 'group',
	});

	await update.run.done;
	await local?.(checkout);

	return { checkout, interrupted: readCheckoutState(checkout) };
}

// Links `plugh` to `thud` and truncates `thud/xyzzy` locally. Upstream replaces
// the symlink with a directory where `plugh/<added>` contains the original
// bytes of `thud/xyzzy`
async function createSymlinkedParentFixture(directory: string, added: string) {
	const checkout = await createRecoveryFixture(directory);

	const addParent = async (root: string) => {
		await mkdir(join(root, 'thud'));
		await writeFile(join(root, 'thud/xyzzy'), 'xyzzy\nthud\n');
		await symlink('thud', join(root, 'plugh'));
	};

	await pushUpstreamChange(directory, addParent);
	await pushUpstreamChange(directory, async (update) => {
		await rm(join(update, 'plugh'));
		await mkdir(join(update, 'plugh'));
		await writeFile(join(update, 'plugh', added), 'xyzzy\nthud\n');
	});
	await addParent(checkout);
	runGit(checkout, ['add', '--', 'plugh', 'thud']);
	runGit(checkout, [
		'commit',
		'--quiet',
		'--only',
		'--message=qux',
		'--',
		'plugh',
		'thud',
	]);
	await writeFile(join(checkout, 'thud/xyzzy'), 'xyzzy\n');

	return checkout;
}

type FailureCase = {
	failure?: Failure;
	gate?: string;
	modes?: Array<{ mode: number; path: string }>;
	reason: string;
	change?: (update: string) => Promise<void>;
	local?: (checkout: string) => Promise<void>;
};

// Runs a core sync with the failure of `row`, setting its modes once the
// changes are saved, and checks that sync skipped the switch, named no cause,
// and left the checkout unchanged
async function skipsSwitchWithoutCause(row: FailureCase) {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	const checkout = await createRecoveryFixture(directory.path);

	if (row.change) {
		await pushUpstreamChange(directory.path, row.change);
	}

	await row.local?.(checkout);

	const before = readCheckoutState(checkout);
	const injected = await injectFailure(directory.path, row.failure);
	const gated = await startGatedSync(
		directory.path,
		['sync', '--core'],
		'rebase *',
		'gate',
		injected.env
	);
	const restore = await restrictModes(checkout, row.modes);

	await gated.release();

	const result = await gated.run.done.finally(restore);

	expect(result.status).toBe(0);
	expect(lines(result.stderr, '× ')).toStrictEqual([
		'× Failed to update the repository',
	]);
	expect(result.stdout).not.toMatch(ANY_CAUSE);
	expect(result.stages).toStrictEqual(['stage core']);
	expect(readCheckoutState(checkout)).toStrictEqual(before);
	await expect(injected.readFailures()).resolves.toHaveLength(
		injected.failures
	);
	await expect(injected.readCommands()).resolves.not.toMatch(
		/^temp-index read-tree -m -u /m
	);
	await expect(
		readUpdateLeftovers(directory.path, checkout)
	).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
}

// Recovers an interrupted switch with the failure and modes of `row` and checks
// that recovery stopped with `row.message`, named no cause, and left the
// checkout and the saved state unchanged
async function stopsRecoveryWithoutCause(
	row: FailureCase & { message: string }
) {
	await using directory = await mkdtempDisposable(
		join(tmpdir(), 'domfiles-sync-')
	);
	const { checkout, interrupted } = await interruptSwitch(
		directory.path,
		row
	);
	const injected = await injectFailure(directory.path, row.failure);
	const restore = await restrictModes(checkout, row.modes);

	const result = await runCommand(
		directory.path,
		'domfiles',
		['sync', 'install'],
		injected.env
	).finally(restore);

	expect(result.status).toBe(1);
	expect(lines(result.stderr, '× ')).toStrictEqual([
		'× Recovery stopped before changing the checkout. Nothing was discarded',
	]);
	expect(result.stdout).toContain(row.message);
	expect(result.stdout).not.toMatch(RECOVERY_CAUSE);
	expect(result.stages).toStrictEqual([]);
	expect(readCheckoutState(checkout)).toStrictEqual(interrupted);
	await expect(injected.readFailures()).resolves.toHaveLength(
		injected.failures
	);
	await expect(
		readUpdateLeftovers(directory.path, checkout)
	).resolves.toMatchObject({
		files: expect.arrayContaining([
			expect.stringMatching(/^update-\d+-\d+$/),
		]),
		worktrees: 2,
	});
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

			\`domfiles sync <stage>\` runs one stage without checking for new updates.
			Sync handles an interrupted repository update before running any stage.
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

test(
	'updates the repository and restores uncommitted changes without using the stash list',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		runGit(checkout, ['update-ref', 'ORIG_HEAD', 'HEAD^']);

		const before = readCheckoutState(checkout);
		const original = runGit(checkout, ['rev-parse', 'HEAD']);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '▴ ')).toStrictEqual([
			'▴ The repository contains uncommitted changes',
		]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		expect(runGit(checkout, ['rev-parse', 'ORIG_HEAD'])).toBe(original);
		expect(result.stages).toStrictEqual(['stage core']);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			before.stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'leaves the checkout unchanged when local commits conflict with the upstream',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		runGit(checkout, ['stash', 'push', '--quiet', '--message=bar']);
		await writeFile(join(checkout, 'foo'), numberedLines('1', 'plugh'));
		runGit(checkout, ['commit', '--quiet', '--all', '--message=qux']);
		runGit(checkout, ['stash', 'pop', '--index', '--quiet']);

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Local changes kept',
		]);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'leaves the checkout unchanged when uncommitted changes conflict with the upstream',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await writeFile(join(checkout, 'foo'), numberedLines('one', 'plugh'));

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to reapply the saved changes in the private worktree',
		]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout unchanged',
		]);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each(
	UPDATE_STEPS.flatMap((step) =>
		INTERRUPTIONS.map((interruption) => ({ ...step, ...interruption }))
	)
)(
	'restores the checkout after `$signal` to the sync $target while $step',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ gate, ...interruption }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const gated = await interruptSync(
			directory.path,
			['sync', '--core'],
			gate,
			interruption
		);
		const { interrupted, recovered } = await recoverInterruptedSync(
			directory.path,
			gated,
			interruption
		);

		expect(interrupted.status).toBe(interruption.status);
		expect(interrupted.stages).toStrictEqual([]);
		expect(recovered.status).toBe(0);
		expect(recovered.stages).toStrictEqual(['stage install']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each(INTERRUPTIONS)(
	'keeps a finished update after `$signal` to the sync $target while it cleans up',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (interruption) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const gated = await interruptSync(
			directory.path,
			['sync', '--core'],
			'update-ref -d *',
			interruption
		);
		const { interrupted, recovered } = await recoverInterruptedSync(
			directory.path,
			gated,
			interruption
		);

		expect(interrupted.status).toBe(interruption.status);
		expect(recovered.status).toBe(0);
		expect(recovered.stages).toStrictEqual(['stage install']);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			before.stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'waits for a Git command that outlives a killed sync before restoring the checkout',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const gated = await interruptSync(
			directory.path,
			['sync', '--core'],
			'temp-index read-tree -m -u *',
			{ signal: 'SIGKILL', status: null, target: 'process' }
		);
		const recovery = startCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		await waitForOutput(
			recovery,
			RECOVERY_WAITING_MESSAGE,
			RECOVERY_TEST_TIMEOUT
		);
		await gated.release();

		const result = await recovery.done;

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(result.stages).toStrictEqual(['stage install']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(gated.run.done).resolves.toMatchObject({ status: null });
	}
);

test.each([
	{ signal: 'SIGKILL', status: null, target: 'group' },
	{ signal: 'SIGKILL', status: null, target: 'process' },
] as const)(
	'restores the checkout after `$signal` to the sync $target interrupts the restoration',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (interruption) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'update-ref -m * refs/heads/master *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const restoration = await interruptSync(
			directory.path,
			['sync', 'install'],
			'temp-index checkout-index *',
			interruption,
			'restoration'
		);
		const { interrupted, recovered } = await recoverInterruptedSync(
			directory.path,
			restoration,
			interruption
		);

		expect(interrupted.status).toBe(interruption.status);
		expect(recovered.status).toBe(0);
		expect(recovered.stages).toStrictEqual(['stage install']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'finishes the cleanup after a restoration without checking later edits',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'update-ref -m * refs/heads/master *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const cleanup = await interruptSync(
			directory.path,
			['sync', 'install'],
			'update-ref -d refs/domfiles-sync/*/saved',
			{ signal: 'SIGKILL', status: null, target: 'group' },
			'cleanup'
		);

		await cleanup.run.done;
		await writeFile(join(checkout, 'foo'), 'plugh\n');

		const edited = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(result.stages).toStrictEqual(['stage install']);
		expect(readCheckoutState(checkout)).toStrictEqual(edited);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'restores files that the interrupted switch left cut short',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;
		await writeFile(join(checkout, 'foo'), 'one\n2\n3\n4');
		await writeFile(join(checkout, 'qux'), '');

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);

test(
	'stops every sync without changes after the checkout changed during an interruption',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;
		await writeFile(join(checkout, 'foo'), 'plugh\n');

		const interrupted = readCheckoutState(checkout);
		const first = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);
		const second = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(first.status).toBe(1);
		expect(lines(first.stderr, '× ')).toStrictEqual([
			'× Recovery stopped before changing the checkout. Nothing was discarded',
		]);
		expect(first.stdout).toContain(
			'`foo` contains a change the interrupted update cannot account for'
		);
		expect(first.stages).toStrictEqual([]);
		expect(second.status).toBe(1);
		expect(second.stderr).toBe(first.stderr);
		expect(second.stages).toStrictEqual([]);
		expect(readCheckoutState(checkout)).toStrictEqual(interrupted);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toMatchObject({
			files: expect.arrayContaining([
				expect.stringMatching(/^update-\d+-\d+$/),
			]),
			worktrees: 2,
		});
	}
);

test(
	'waits for a Git lock file and stops without removing it',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);
		const lock = join(checkout, '.git/index.lock');

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'update-ref -m * refs/heads/master *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;
		await writeFile(lock, '');

		const stopped = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		await expect(lstat(lock)).resolves.toMatchObject({ size: 0 });
		await rm(lock);

		const restored = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(stopped.status).toBe(1);
		expect(lines(stopped.stderr, '× ')).toStrictEqual([
			`× Git lock file \`${await realpath(checkout)}/.git/index.lock\` still exists after 5 seconds. Remove it only after confirming that no Git process is using it`,
			'× Recovery could not finish restoring the checkout. The saved state is intact',
		]);
		expect(stopped.stages).toStrictEqual([]);
		expect(restored.status).toBe(0);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);

test(
	'switches the working tree after the existing index lock is released',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const foo = await readFile(join(checkout, 'foo'), 'utf8');
		const lock = join(checkout, '.git/index.lock');
		const injected = await injectFailure(directory.path);
		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *',
			'gate',
			injected.env
		);

		await writeFile(lock, '');
		await gated.release();
		await waitForIndexLockLookup(injected.readCommands);

		const phase = await readUpdatePhase(directory.path, checkout);
		const fooDuringWait = await readFile(join(checkout, 'foo'), 'utf8');
		const commandsDuringWait = await injected.readCommands();

		await rm(lock);

		const result = await gated.run.done;

		expect(phase).toBe('prepared');
		expect(fooDuringWait).toBe(foo);
		expect(commandsDuringWait).not.toMatch(/^temp-index read-tree -m -u /m);
		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
	}
);

test(
	'skips the switch when the lock holder changes the index',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const commits = runGit(checkout, ['log', '--format=%H %s']);
		const lock = join(checkout, '.git/index.lock');
		const injected = await injectFailure(directory.path);
		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *',
			'gate',
			injected.env
		);

		// Hold the lock as Git does, with a new index that stages `fred`
		await copyFile(join(checkout, '.git/index'), lock);

		const staged = spawnSync('git', ['add', 'fred'], {
			cwd: checkout,
			env: {
				...GIT_ENV,
				GIT_INDEX_FILE: lock,
				PATH: process.env.PATH ?? '',
			},
		});

		await gated.release();
		await waitForIndexLockLookup(injected.readCommands);

		const phase = await readUpdatePhase(directory.path, checkout);

		await rename(lock, join(checkout, '.git/index'));

		const result = await gated.run.done;

		expect(staged.status).toBe(0);
		expect(phase).toBe('prepared');
		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× The checkout changed while sync prepared the update. Sync skipped the switch',
		]);
		await expect(injected.readCommands()).resolves.not.toMatch(
			/^temp-index read-tree -m -u /m
		);
		expect(runGit(checkout, ['log', '--format=%H %s'])).toBe(commits);
		expect(runGit(checkout, ['status', '--porcelain'])).toBe(
			' M bar\nMM baz\nM  foo\nA  fred\n'
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'keeps the checkout and the Git lock when the wait times out before the switch',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);
		const lock = join(checkout, '.git/index.lock');
		const injected = await injectFailure(directory.path);
		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *',
			'gate',
			injected.env
		);

		await writeFile(lock, '');
		await gated.release();

		const result = await gated.run.done;
		const lockAfter = await lstat(lock);

		await rm(lock);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			`× Git lock file \`${await realpath(checkout)}/.git/index.lock\` still exists after 5 seconds. Remove it only after confirming that no Git process is using it`,
			'× Failed to update the repository',
		]);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(lockAfter.size).toBe(0);
		await expect(injected.readCommands()).resolves.not.toMatch(
			/^temp-index read-tree -m -u /m
		);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'waits for a new lock before writing the index during the switch',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const lock = join(checkout, '.git/index.lock');
		const injected = await injectFailure(directory.path);
		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'temp-index read-tree -m -u *',
			'gate',
			injected.env
		);

		await writeFile(lock, '');
		await gated.release();
		await vi.waitFor(
			async () => {
				const commands = await injected.readCommands();

				expect(
					commands.match(
						/^rev-parse --path-format=absolute --git-path index\.lock$/gm
					)
				).toHaveLength(2);
			},
			{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
		);
		await rm(lock);

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nbar\nfoo\n'
		);
	}
);

test(
	'keeps a commit made on `master` during the rebase and updates after the restart',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *'
		);

		runGit(checkout, [
			'commit',
			'--quiet',
			'--allow-empty',
			'--only',
			'--message=qux',
		]);
		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× The checkout changed while sync prepared the update. Sync skipped the switch',
		]);
		expect(
			result.stdout.match(/Restarting the sync after the update/g)
		).toHaveLength(2);
		expect(result.stages).toStrictEqual(['stage core']);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'qux\nbaz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'leaves the checkout unchanged when a file the update changes is edited during the rebase',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *'
		);

		await writeFile(join(checkout, 'foo'), 'plugh\n');

		const before = readCheckoutState(checkout);

		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(
			'Sync prepared the update using an earlier version of `foo`. Rerun your sync command to use your current changes'
		);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'keeps an edit made during the rebase to a file the update leaves alone',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'rebase *'
		);

		await writeFile(join(checkout, 'bar'), 'plugh\n');
		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(result.stages).toStrictEqual(['stage core']);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'plugh\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
	}
);

test(
	'updates the repository after the marker of an interrupted update was removed',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'stash apply *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const state = join(directory.path, 'home/.local/state/domfiles');
		const entries = await readdir(state);
		const marker = entries.find(
			(entry) => entry.startsWith('update-') && !entry.includes('.')
		);

		await rm(join(state, marker ?? 'missing'));

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nbar\nfoo\n'
		);
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each(['.gitignore', '.git/info/exclude'])(
	'leaves the checkout unchanged and names a file ignored through `%s` where the update adds one',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (ignoreFile) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await appendFile(join(checkout, ignoreFile), 'qux\n');
		await writeFile(join(checkout, 'qux'), 'plugh\n');

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(
			'The update would overwrite a local file or symlink at `qux`. Even if Git ignores it, move it outside the checkout and rerun your sync command'
		);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(readFile(join(checkout, 'qux'), 'utf8')).resolves.toBe(
			'plugh\n'
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'leaves the checkout unchanged and names a path the update adds with a Git-quoted name',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const update = join(directory.path, 'update');

		await writeFile(join(update, 'corge"'), 'corge\n');
		runGit(update, ['add', '--all']);
		runGit(update, ['commit', '--quiet', '--message=qux']);
		runGit(update, ['push', '--quiet']);

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(
			'Sync cannot update paths that Git lists in quotes, including "corge\\"". Back up any ignored files you need because Git can overwrite them. Update this checkout directly with Git'
		);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);

test(
	'restores the checkout after `KILL` stops Git while it holds the switch index lock',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const log = join(directory.path, 'smudge.log');
		const smudge = join(directory.path, 'smudge');

		await writeExecutable(smudge, SMUDGE_GATE_STUB);
		await writeFile(log, '');
		await writeFile(
			join(checkout, '.git/info/attributes'),
			'foo filter=gate\n'
		);
		runGit(checkout, ['config', 'filter.gate.smudge', smudge]);

		const before = readCheckoutState(checkout);
		const env = {
			...GIT_ENV,
			SMUDGE_LOG: log,
			SMUDGE_RELEASE: join(directory.path, 'smudge.release'),
		};
		const run = startCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			env
		);

		await vi.waitFor(
			async () => {
				await expect(readLog(log)).resolves.toHaveLength(1);
			},
			{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
		);
		process.kill(-(run.pid ?? 0), 'SIGKILL');
		await run.done;

		const lock = join(
			directory.path,
			`home/.local/state/domfiles/update-${await readUpdateId(checkout)}.tmp/index.w0.lock`
		);

		await expect(lstat(lock)).resolves.toBeDefined();

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			env
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each(['index.check', 'index.w0', 'index.w1'])(
	'restores the checkout past a stale `%s.lock` left by an interrupted switch',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (index) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', target: 'group' }
		);

		await update.run.done;
		await writeFile(
			join(
				directory.path,
				`home/.local/state/domfiles/update-${await readUpdateId(checkout)}.tmp/${index}.lock`
			),
			''
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each(['original', 'saved', 'updated'])(
	'updates the repository past a stale lock of the private `%s` ref',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (name) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const refs = join(
			checkout,
			`.git/refs/domfiles-sync/${await readUpdateId(checkout)}`
		);

		await mkdir(refs, { recursive: true });
		await writeFile(join(refs, `${name}.lock`), '');

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nbar\nfoo\n'
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
		await expect(lstat(join(refs, `${name}.lock`))).rejects.toMatchObject({
			code: 'ENOENT',
		});
	}
);

test.each(['original', 'saved', 'updated'])(
	'removes the private `%s` ref past a stale lock while recovery cleans up',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (name) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', target: 'group' }
		);

		await update.run.done;
		await writeFile(
			join(
				checkout,
				`.git/refs/domfiles-sync/${await readUpdateId(checkout)}/${name}.lock`
			),
			''
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each([
	{ signal: 'SIGHUP', status: 129 },
	{ signal: 'SIGINT', status: 130 },
] as const)(
	'restores the checkout and exits with $status after `$signal` to the sync group during the switch',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ signal, status }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);

		const gated = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal, target: 'group' }
		);
		const result = await gated.run.done;

		expect(result.status).toBe(status);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× A signal interrupted the repository update',
		]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(result.stages).toStrictEqual([]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'keeps a commit that moves `master` while the switch writes the index',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*'
		);
		const commit = runGit(checkout, [
			'commit-tree',
			'HEAD^{tree}',
			'-p',
			'HEAD',
			'-m',
			'qux',
		]).trim();

		runGit(checkout, ['update-ref', 'refs/heads/master', commit]);
		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× `master` moved before sync could update it',
		]);
		expect(
			result.stdout.match(/Restarting the sync after the update/g)
		).toHaveLength(2);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'qux\nbaz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'stops without changing the checkout when it cannot read the tree of a moved `master`',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const injected = await injectFailure(directory.path, {
			git: ['rev-parse --quiet --verify refs/heads/master^{tree}'],
		});

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			'gate',
			injected.env
		);
		const commit = runGit(checkout, [
			'commit-tree',
			'HEAD^{tree}',
			'-p',
			'HEAD',
			'-m',
			'qux',
		]).trim();

		runGit(checkout, ['update-ref', 'refs/heads/master', commit]);
		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× `master` moved before sync could update it',
			'× Recovery stopped before changing the checkout. Nothing was discarded',
		]);
		expect(result.stdout).toContain(
			'Sync could not compare the trees of `master` and the original commit'
		);
		await expect(injected.readFailures()).resolves.toHaveLength(1);
		expect(runGit(checkout, ['rev-parse', 'HEAD']).trim()).toBe(commit);
	}
);

test(
	'keeps a commit of the staged changes that moves `master` while the switch writes the index',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const staged = runGit(checkout, ['write-tree']).trim();
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*'
		);
		const commit = runGit(checkout, [
			'commit-tree',
			staged,
			'-p',
			'HEAD',
			'-m',
			'qux',
		]).trim();

		runGit(checkout, ['update-ref', 'refs/heads/master', commit]);
		await gated.release();

		const result = await gated.run.done;

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× `master` moved before sync could update it',
		]);
		expect(
			result.stdout.match(/Restarting the sync after the update/g)
		).toHaveLength(2);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'qux\nbaz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\n M baz\n?? fred\n',
		});
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each([
	{
		listTree: (checkout: string) => {
			const blob = runGit(
				checkout,
				['hash-object', '-w', '--stdin'],
				'thud\n'
			).trim();

			return `${runGit(checkout, ['ls-tree', 'HEAD'])}100644 blob ${blob}\tthud\n`;
		},
		tip: 'a commit that adds a file',
	},
	{
		listTree: (checkout: string) => {
			const [info = ''] = runGit(checkout, [
				'ls-files',
				'--stage',
				'foo',
			]).split('\t');
			const [, blob = ''] = info.split(' ');

			return runGit(checkout, ['ls-tree', 'HEAD']).replace(
				/^(100644 blob )[0-9a-f]+(\tfoo)$/m,
				`$1${blob}$2`
			);
		},
		tip: 'a commit of part of the staged changes',
	},
])(
	'stops without changing the checkout when `master` moves to $tip while the switch writes the index',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ listTree }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const tree = runGit(checkout, ['mktree'], listTree(checkout)).trim();

		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*'
		);
		const commit = runGit(checkout, [
			'commit-tree',
			tree,
			'-p',
			'HEAD',
			'-m',
			'qux',
		]).trim();

		runGit(checkout, ['update-ref', 'refs/heads/master', commit]);
		await gated.release();

		const stopped = await gated.run.done;
		const state = readCheckoutState(checkout);
		const recovered = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(stopped.status).toBe(1);
		expect(lines(stopped.stderr, '× ')).toStrictEqual([
			'× `master` moved before sync could update it',
			'× Recovery stopped before changing the checkout. Nothing was discarded',
		]);
		expect(stopped.stdout).toContain(MOVED_MASTER_TREE_MESSAGE);
		expect(stopped.stages).toStrictEqual([]);
		expect(runGit(checkout, ['rev-parse', 'HEAD']).trim()).toBe(commit);
		expect(recovered.status).toBe(1);
		expect(lines(recovered.stderr, '× ')).toStrictEqual([
			'× Recovery stopped before changing the checkout. Nothing was discarded',
		]);
		expect(recovered.stdout).toContain(
			'`master` points to a commit outside this update'
		);
		expect(recovered.stages).toStrictEqual([]);
		expect(readCheckoutState(checkout)).toStrictEqual(state);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toMatchObject({
			files: expect.arrayContaining([
				expect.stringMatching(/^update-\d+-\d+$/),
			]),
			refs: expect.stringContaining('refs/domfiles-sync/'),
			worktrees: 2,
		});
	}
);

async function pushUpstreamChange(
	directory: string,
	change: (update: string) => Promise<void>
) {
	const update = join(directory, 'update');

	await change(update);
	runGit(update, ['add', '--all']);
	runGit(update, ['commit', '--quiet', '--message=qux']);
	runGit(update, ['push', '--quiet']);
}

async function readEntry(path: string) {
	const stats = await lstat(path);

	return stats.isSymbolicLink()
		? { link: await readlink(path) }
		: { content: await readFile(path, 'utf8') };
}

test.each([
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		local: async () => {},
		path: 'plugh/xyzzy',
		shape: 'adds a file in a new directory',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'garply/thud'), 'xyzzy\n');
		},
		local: async () => {},
		path: 'garply/thud',
		shape: 'adds a file in an existing directory',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'grault'));
			await mkdir(join(update, 'grault'));
			await writeFile(join(update, 'grault/xyzzy'), 'xyzzy\n');
		},
		local: async () => {},
		path: 'grault/xyzzy',
		shape: 'replaces a tracked file with a directory',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		local: async () => {},
		path: 'garply',
		shape: 'replaces a directory of tracked files with a file',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'plugh'), 'xyzzy\n');
		},
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'plugh/xyzzy'), { recursive: true });
		},
		path: 'plugh',
		shape: 'replaces an empty local directory with a file',
	},
])(
	'preserves local changes when the update $shape',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ change, local, path }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		await pushUpstreamChange(directory.path, change);
		await local(checkout);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		await expect(readFile(join(checkout, path), 'utf8')).resolves.toBe(
			'xyzzy\n'
		);
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nqux\nbar\nfoo\n'
		);
		expect(runGit(checkout, ['status', '--porcelain'])).toBe(
			' M bar\nMM baz\nM  foo\n?? fred\n'
		);
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each([
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		entry: 'plugh',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'plugh'), 'plugh\n');
		},
		message:
			'Creating a directory for `plugh/xyzzy` would overwrite the local file or symlink at `plugh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'an ignored file where the update needs a parent directory',
	},
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		entry: 'plugh',
		local: async (checkout: string, outside: string) => {
			await symlink(outside, join(checkout, 'plugh'));
		},
		message:
			'Creating a directory for `plugh/xyzzy` would overwrite the local file or symlink at `plugh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'an ignored symlink to a directory where the update needs a parent directory',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		entry: 'garply/plugh',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'garply/plugh'), 'plugh\n');
		},
		message:
			'Replacing the directory `garply` with a file would delete the local file or symlink at `garply/plugh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'an ignored file inside a directory the update replaces',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'plugh'), 'xyzzy\n');
		},
		entry: 'plugh',
		local: async (checkout: string, outside: string) => {
			await symlink(outside, join(checkout, 'plugh'));
		},
		message:
			'The update would overwrite a local file or symlink at `plugh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'an ignored symlink at a path the update adds',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'pl*gh'), 'xyzzy\n');
		},
		entry: 'pl*gh',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'pl*gh'), 'plugh\n');
		},
		message:
			'The update would overwrite a local file or symlink at `pl*gh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'the literal glob character in an untracked filename',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		entry: 'garply/plugh/xyzzy',
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'garply/plugh'));
			runGit(join(checkout, 'garply/plugh'), ['init', '--quiet']);
			await writeFile(join(checkout, 'garply/plugh/xyzzy'), 'xyzzy\n');
		},
		message:
			'Replacing the directory `garply` with a file would delete the untracked directory `garply/plugh/` and its contents. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'a nested repository as a single untracked directory',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		entry: 'garply/plugh\t',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'garply/plugh\t'), 'plugh\n');
		},
		message:
			'Replacing the directory `garply` with a file would delete the local file or symlink at `garply/plugh\t`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'the trailing tab in a filename inside a directory the update replaces',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		entry: 'garply/\tplugh',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'garply/\tplugh'), 'plugh\n');
		},
		message:
			'Replacing the directory `garply` with a file would delete the local file or symlink at `garply/\tplugh`. Even if Git ignores it, move it outside the checkout and rerun your sync command',
		shape: 'the leading tab in a filename inside a directory the update replaces',
	},
])(
	'reports $shape and skips the switch',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ change, entry, local, message }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const outside = join(directory.path, 'outside');

		await mkdir(outside);
		await pushUpstreamChange(directory.path, change);
		await local(checkout, outside);
		await appendFile(
			join(checkout, '.git/info/exclude'),
			'/plugh\n/garply/plugh\n'
		);

		const before = readCheckoutState(checkout);
		const entryBefore = await readEntry(join(checkout, entry));
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(message);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(readEntry(join(checkout, entry))).resolves.toStrictEqual(
			entryBefore
		);
		await expect(readdir(outside)).resolves.toStrictEqual([]);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.runIf(FOLDS_CASE).each(CASE_RENAMES)(
	'refuses to rename `$from` to `$to` on a case-insensitive filesystem',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ added, from, spelled, to }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await pushUpstreamChange(directory.path, async (update) => {
			await Promise.resolve();
			runGit(update, ['mv', from, 'thud']);
			runGit(update, ['mv', 'thud', to]);
		});

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(
			`Sync cannot add \`${added}\` because \`${spelled}\` has a different spelling on disk. Back up any ignored files you need because Git can overwrite them. Update this checkout directly with Git`
		);
		expect(result.stdout).not.toContain('local file');
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.skipIf(FOLDS_CASE).each(CASE_RENAMES)(
	'renames `$from` to `$to` on a case-sensitive filesystem',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ added, from, to }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await pushUpstreamChange(directory.path, async (update) => {
			await Promise.resolve();
			runGit(update, ['mv', from, 'thud']);
			runGit(update, ['mv', 'thud', to]);
		});

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([]);
		expect(runGit(checkout, ['ls-files', '--', from, to])).toBe(
			`${added}\n`
		);
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nqux\nbar\nfoo\n'
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'keeps the completed update if `KILL` interrupts the write to `ORIG_HEAD`',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		runGit(checkout, ['update-ref', 'ORIG_HEAD', 'HEAD^']);

		const stale = runGit(checkout, ['rev-parse', 'ORIG_HEAD']);
		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'update-ref ORIG_HEAD *',
			{ signal: 'SIGKILL', target: 'group' }
		);

		await update.run.done;

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Completed update kept',
		]);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: 'garply\n',
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		});
		expect(runGit(checkout, ['rev-parse', 'ORIG_HEAD'])).toBe(stale);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

// Leaves the index spelling `Garply` while the disk keeps `garply`, as Git does
// when it cannot rename a directory that holds an ignored file. The upstream
// renamed the directory and adds `Garply/plugh` to it
async function createRespelledFixture(directory: string) {
	const checkout = await createRecoveryFixture(directory);

	await pushUpstreamChange(directory, async (update) => {
		runGit(update, ['mv', 'garply', 'thud']);
		runGit(update, ['mv', 'thud', 'Garply']);
		await writeFile(join(update, 'Garply/plugh'), 'plugh\n');
	});

	respellTrackedEntry(checkout, 'garply', 'Garply');

	return checkout;
}

// Commits a rename of top-level entry `from` to `to` and renames its index
// entries to match, while the disk keeps the spelling `from`
function respellTrackedEntry(checkout: string, from: string, to: string) {
	const listing = runGit(checkout, ['ls-tree', 'HEAD']);
	const tree = runGit(
		checkout,
		['mktree'],
		listing.replace(`\t${from}\n`, `\t${to}\n`)
	).trim();
	const commit = runGit(checkout, [
		'commit-tree',
		tree,
		'-p',
		'HEAD',
		'-m',
		'thud',
	]).trim();
	const entries = runGit(checkout, ['ls-files', '--stage', '--', from])
		.split('\n')
		.filter(Boolean);

	runGit(checkout, ['update-ref', 'refs/heads/master', commit]);

	for (const entry of entries) {
		const [info = '', path = ''] = entry.split('\t');
		const [mode = '', blob = ''] = info.split(' ');

		runGit(checkout, ['rm', '--cached', '--quiet', '--', path]);
		runGit(checkout, [
			'update-index',
			'--add',
			'--cacheinfo',
			`${mode},${blob},${to}${path.slice(from.length)}`,
		]);
	}
}

async function readGarplySpellings(checkout: string) {
	const entries = await readdir(checkout);

	return entries.filter((entry) => entry.toLowerCase() === 'garply');
}

test.runIf(FOLDS_CASE)(
	'keeps the parent directory’s spelling on disk when adding a file',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRespelledFixture(directory.path);
		const stashes = runGit(checkout, ['stash', 'list', '--format=%H']);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		expect(runGit(checkout, ['ls-files', '--', 'Garply'])).toBe(
			'Garply/plugh\nGarply/waldo\n'
		);
		await expect(readGarplySpellings(checkout)).resolves.toStrictEqual([
			'garply',
		]);
		await expect(
			readFile(join(checkout, 'garply/plugh'), 'utf8')
		).resolves.toBe('plugh\n');
		expect(runGit(checkout, ['log', '--format=%s'])).toBe(
			'baz\nqux\nbar\nfoo\n'
		);
		expect(runGit(checkout, ['status', '--porcelain'])).toBe(
			' M bar\nMM baz\nM  foo\n?? fred\n'
		);
		expect(runGit(checkout, ['stash', 'list', '--format=%H'])).toBe(
			stashes
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.runIf(FOLDS_CASE)(
	'removes the added file and keeps its parent directory’s spelling on rollback after `KILL`',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRespelledFixture(directory.path);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', target: 'group' }
		);

		await update.run.done;

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(readGarplySpellings(checkout)).resolves.toStrictEqual([
			'garply',
		]);
		await expect(
			lstat(join(checkout, 'garply/plugh'))
		).rejects.toMatchObject({ code: 'ENOENT' });
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.runIf(FOLDS_CASE).each([
	{
		added: 'garply/waldo',
		prepare: async (directory: string) => {
			const checkout = await createRecoveryFixture(directory);

			await pushUpstreamChange(directory, async (update) => {
				await Promise.resolve();
				runGit(update, ['mv', 'garply', 'thud']);
				runGit(update, ['mv', 'thud', 'Garply']);
			});
			await pushUpstreamChange(directory, async (update) => {
				await Promise.resolve();
				runGit(update, ['mv', 'Garply', 'thud']);
				runGit(update, ['mv', 'thud', 'garply']);
			});
			respellTrackedEntry(checkout, 'garply', 'Garply');

			return checkout;
		},
		tracked: 'garply/waldo',
		where: 'at the added path',
	},
	{
		added: 'grault/xyzzy',
		prepare: async (directory: string) => {
			const checkout = await createRecoveryFixture(directory);

			await pushUpstreamChange(directory, async (update) => {
				await Promise.resolve();
				runGit(update, ['mv', 'grault', 'thud']);
				runGit(update, ['mv', 'thud', 'Grault']);
			});
			await pushUpstreamChange(directory, async (update) => {
				await rm(join(update, 'Grault'));
				await mkdir(join(update, 'grault'));
				await writeFile(join(update, 'grault/xyzzy'), 'xyzzy\n');
			});
			respellTrackedEntry(checkout, 'grault', 'Grault');

			return checkout;
		},
		tracked: 'grault',
		where: 'where the update needs a parent directory',
	},
])(
	'reports a possible spelling difference for a tracked file $where',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async ({ added, prepare, tracked }) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await prepare(directory.path);
		const before = readCheckoutState(checkout);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).toContain(
			`Sync cannot add \`${added}\` because \`${tracked}\` is in the way. Git did not list it as untracked and may track it under another spelling. Back up any ignored files you need because Git can overwrite them. Update this checkout directly with Git`
		);
		expect(result.stdout).not.toContain('local file');
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'skips the update without naming an obstruction if Git cannot list untracked paths',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await writeFile(join(checkout, 'qux'), 'plugh\n');
		await writeExecutable(
			join(directory.path, 'stubs/git'),
			`#!/bin/sh
PATH="\${PATH#*:}"

for param do
  if [ "$param" = --others ]; then
    exit 128
  fi
done

exec git "$@"
`
		);

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).not.toMatch(ANY_CAUSE);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(readFile(join(checkout, 'qux'), 'utf8')).resolves.toBe(
			'plugh\n'
		);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.runIf(FOLDS_CASE)(
	'omits the cause when a line of an untracked filename matches a tracked path',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await pushUpstreamChange(directory.path, async (update) => {
			await Promise.resolve();
			runGit(update, ['mv', 'grault', 'thud']);
			runGit(update, ['mv', 'thud', 'Grault']);
		});
		await pushUpstreamChange(directory.path, async (update) => {
			await rm(join(update, 'Grault'));
			await mkdir(join(update, 'grault'));
			await writeFile(join(update, 'grault/xyzzy'), 'xyzzy\n');
		});
		respellTrackedEntry(checkout, 'grault', 'Grault');
		await writeFile(join(checkout, 'plugh\ngrault'), 'plugh\n');

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).not.toMatch(ANY_CAUSE);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);

test(
	'omits the cause when Git lists an untracked filename containing a newline',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await writeFile(join(checkout, 'qux'), 'plugh\n');
		await writeFile(join(checkout, 'plugh\nqux'), 'plugh\n');

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).not.toMatch(ANY_CAUSE);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(readFile(join(checkout, 'qux'), 'utf8')).resolves.toBe(
			'plugh\n'
		);
	}
);

test(
	'omits the cause when a nested repository in a replaced directory contains a filename with a newline',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const nested = join(checkout, 'garply/plugh');

		await pushUpstreamChange(directory.path, async (update) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		});
		await mkdir(nested);
		runGit(nested, ['init', '--quiet']);
		await writeFile(join(nested, 'xyzzy\nthud'), 'xyzzy\n');

		const before = readCheckoutState(checkout);
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(result.stdout).not.toMatch(ANY_CAUSE);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readFile(join(nested, 'xyzzy\nthud'), 'utf8')
		).resolves.toBe('xyzzy\n');
	}
);

test.each<FailureCase>([
	{
		failure: { git: ['temp-index write-tree'] },
		reason: 'Git cannot write a tree from the index copy',
	},
	{
		failure: { git: ['temp-index update-index -q --refresh'] },
		reason: 'Git cannot refresh the comparison index',
	},
	{
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/dirty.w0'),
		},
		reason: 'sync cannot look up a path in the recorded differences',
	},
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/switch.removed'),
		},
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'plugh'), 'plugh\n');
		},
		reason: 'sync cannot look up a parent in the list of removals',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/switch.names'),
		},
		reason: 'sync cannot look up a directory entry in the switch list',
	},
])(
	'skips the switch without reporting a checkout change when $reason',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	skipsSwitchWithoutCause
);

test.skipIf(IS_ROOT).each<FailureCase>([
	{
		change: async (update: string) => {
			await writeFile(join(update, 'plugh'), 'xyzzy\n');
		},
		failure: { command: 'mv', stub: UNREADABLE_LISTING_STUB },
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'plugh'), 'plugh\n');
		},
		reason: 'sync cannot read its list of untracked paths',
	},
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'plugh'));
			await writeFile(join(checkout, 'plugh/xyzzy'), 'plugh\n');
		},
		modes: [{ mode: 0o311, path: 'plugh' }],
		reason: 'an added path’s parent is searchable but unreadable',
	},
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'plugh'));
			await writeFile(join(checkout, 'plugh/xyzzy'), 'plugh\n');
			await writeFile(join(checkout, 'plugh/*'), 'plugh\n');
		},
		modes: [{ mode: 0o311, path: 'plugh' }],
		reason: 'an unreadable directory contains a file literally named `*`',
	},
	{
		change: async (update: string) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		},
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'plugh'));
		},
		modes: [{ mode: 0o600, path: 'plugh' }],
		reason: 'an added path’s parent lacks search permission',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'garply/waldo'), 'xyzzy\n');
		},
		modes: [{ mode: 0o600, path: 'garply' }],
		reason: 'a changed file’s parent lacks search permission',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'grault'), 'xyzzy\n');
		},
		modes: [{ mode: 0o000, path: 'grault' }],
		reason: 'a changed file lacks read permission',
	},
])(
	'skips the switch without reporting a checkout change when $reason',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	skipsSwitchWithoutCause
);

test.each([
	{
		failure: 'diff --quiet',
		stagedBaz: 'garply\n',
		status: ' M bar\nMM baz\nM  foo\n?? fred\n',
		what: 'the working tree with the index',
	},
	{
		failure: 'diff --cached --quiet',
		prepare: (checkout: string) => {
			runGit(checkout, ['add', '--update']);
		},
		stagedBaz: 'waldo\n',
		status: 'M  bar\nM  baz\nM  foo\n?? fred\n',
		what: 'the index with `HEAD`',
	},
])(
	'saves local changes without a warning when Git cannot compare $what',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (row) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		row.prepare?.(checkout);

		const injected = await injectFailure(directory.path, {
			git: [row.failure],
		});

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			injected.env
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '▴ ')).toStrictEqual([]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Repository updated',
			'✓ Changes restored',
			'✓ Up to date',
		]);
		await expect(injected.readFailures()).resolves.toHaveLength(1);
		await expect(readUpdatedState(checkout)).resolves.toStrictEqual({
			branch: 'refs/heads/master\n',
			commits: 'baz\nbar\nfoo\n',
			files: {
				bar: 'grault\n',
				baz: 'waldo\n',
				foo: numberedLines('one', 'twenty'),
				stagedBaz: row.stagedBaz,
				stagedFoo: numberedLines('one', 'twenty'),
			},
			status: row.status,
		});
	}
);

test(
	'rolls back without claiming that `master` moved when Git cannot verify its tip',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);
		const injected = await injectFailure(directory.path, {
			git: [
				'update-ref -m * refs/heads/master *',
				'rev-parse --quiet --verify refs/heads/master',
			],
		});

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', '--core'],
			injected.env
		);

		expect(result.status).toBe(0);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Failed to update the repository',
		]);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Changes restored',
		]);
		expect(result.stages).toStrictEqual(['stage core']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(injected.readFailures()).resolves.toHaveLength(2);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test.each<FailureCase & { message: string }>([
	{
		failure: { git: ['symbolic-ref --quiet HEAD'] },
		message: 'Sync could not identify the current branch',
		reason: 'Git cannot identify the branch named by `HEAD`',
	},
	{
		failure: { git: ['rev-parse --quiet --verify refs/heads/master'] },
		message: 'Sync could not resolve `master` to a commit',
		reason: 'Git cannot resolve the tip of `master`',
	},
	{
		failure: {
			git: [
				'rev-parse --path-format=absolute --git-path CHERRY_PICK_HEAD *',
			],
		},
		message:
			'Sync could not determine whether a Git operation is in progress',
		reason: 'Git cannot locate its operation markers',
	},
	{
		failure: { git: ['ls-files -v *'] },
		message: 'Sync could not check for files marked assume-unchanged',
		reason: 'Git cannot check the index flags for skipped files',
	},
	{
		failure: { git: ['temp-index write-tree'] },
		message: 'Sync could not verify the checkout’s index',
		reason: 'Git cannot write a tree from the index copy',
	},
	{
		failure: { git: ['diff-tree *'] },
		message:
			'Sync could not compare the working tree with the versions saved by the update',
		reason: 'Git cannot compare the recorded working trees',
	},
	{
		failure: { git: ['cat-file --filters *', 'cat-file --filters *'] },
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'foo'), 'one\n2\n3\n4');
		},
		message: 'Sync could not check `foo`',
		reason: 'Git cannot filter either saved version of a truncated file',
	},
	{
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/dirty.w[01]'),
		},
		message: 'Sync could not check `foo`',
		reason: 'sync cannot look up a path in the recorded differences',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/switch.names'),
		},
		gate: 'temp-index read-tree -m -u *',
		message: 'Sync could not check `garply`',
		reason: 'sync cannot look up a directory entry in the switch list',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'garply/thud'), 'xyzzy\n');
		},
		failure: {
			command: 'grep',
			stub: createArgumentFailureStub('*/switch.names'),
		},
		local: async (checkout: string) => {
			await rename(join(checkout, 'garply'), join(checkout, '../garply'));
			await symlink('../garply', join(checkout, 'garply'));
		},
		message: 'Sync could not check `garply/thud`',
		reason: 'sync cannot check whether the switch changed a parent symlink',
	},
])(
	'stops before rollback and identifies the incomplete check when $reason',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	stopsRecoveryWithoutCause
);

test.skipIf(IS_ROOT).each<FailureCase & { message: string }>([
	{
		change: async (update: string) => {
			await writeFile(join(update, 'garply/thud'), 'xyzzy\n');
		},
		message: 'Sync could not check `garply/thud`',
		modes: [{ mode: 0o600, path: 'garply' }],
		reason: 'a switch path’s parent lacks search permission',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		gate: 'temp-index read-tree -m -u *',
		local: async (checkout: string) => {
			await mkdir(join(checkout, 'garply/plugh'));
			await writeFile(join(checkout, 'garply/plugh/xyzzy'), 'xyzzy\n');
		},
		message: 'Sync could not check `garply`',
		modes: [{ mode: 0o300, path: 'garply/plugh' }],
		reason: 'a subdirectory of a switch path cannot be listed',
	},
	{
		message: 'Sync could not check `foo`',
		modes: [{ mode: 0o000, path: 'foo' }],
		reason: 'a file at a switch path is unreadable',
	},
])(
	'stops before rollback and identifies the incomplete check when $reason',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	stopsRecoveryWithoutCause
);

test.each([
	{
		local: async (checkout: string) => {
			await Promise.resolve();
			runGit(checkout, ['update-ref', '--no-deref', 'HEAD', 'HEAD']);
		},
		message: 'The checkout is no longer on `master`',
		reason: '`HEAD` is detached',
	},
	{
		local: async (checkout: string) => {
			await Promise.resolve();
			runGit(checkout, ['update-ref', 'refs/heads/master', 'HEAD^']);
		},
		message: '`master` points to a commit outside this update',
		reason: 'the tip of `master` matches neither recorded commit',
	},
	{
		local: async (checkout: string) => {
			await writeFile(
				join(checkout, '.git/MERGE_HEAD'),
				runGit(checkout, ['rev-parse', 'HEAD'])
			);
		},
		message: 'Git may still have an operation in progress in this checkout',
		reason: 'Git’s merge marker is present',
	},
	{
		local: async (checkout: string) => {
			await Promise.resolve();
			runGit(checkout, ['update-index', '--assume-unchanged', 'bar']);
		},
		message:
			'The repository contains skipped files. Unskip them before restoring the original state',
		reason: 'a tracked file is marked assume-unchanged',
	},
	{
		local: async (checkout: string) => {
			await Promise.resolve();
			runGit(checkout, ['add', 'fred']);
		},
		message: 'The index differs from both versions recorded by the update',
		reason: 'the index matches neither recorded tree',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		gate: 'temp-index read-tree -m -u *',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'garply/plugh'), 'plugh\n');
		},
		message:
			'`garply` contains a change the interrupted update cannot account for',
		reason: 'a directory the switch would replace contains an unexpected file',
	},
	{
		change: async (update: string) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		},
		gate: 'temp-index read-tree -m -u *',
		local: async (checkout: string) => {
			await writeFile(join(checkout, 'garply/waldo\nfoo'), 'plugh\n');
		},
		message:
			'`garply` contains a change the interrupted update cannot account for',
		reason: 'a directory the switch would replace contains a filename with a newline',
	},
	{
		change: async (update: string) => {
			await writeFile(join(update, 'garply/thud'), 'xyzzy\n');
		},
		local: async (checkout: string) => {
			await rename(join(checkout, 'garply'), join(checkout, '../garply'));
			await symlink('../garply', join(checkout, 'garply'));
		},
		message:
			'Recovery expected a directory at `garply` and found a file or symlink there',
		reason: 'an unexpected symlink occupies a parent directory',
	},
])(
	'stops before rollback and reports why when $reason',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (row) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const { checkout, interrupted } = await interruptSwitch(
			directory.path,
			row
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Recovery stopped before changing the checkout. Nothing was discarded',
		]);
		expect(result.stdout).toContain(row.message);
		expect(result.stages).toStrictEqual([]);
		expect(readCheckoutState(checkout)).toStrictEqual(interrupted);
	}
);

test.each(['xyzzy', 'waldo'])(
	'keeps the symlink target’s unstaged edit when an interrupted update would add `plugh/%s`',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async (added) => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createSymlinkedParentFixture(
			directory.path,
			added
		);
		const before = readCheckoutState(checkout);

		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'temp-index read-tree -m -u *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readFile(join(checkout, 'thud/xyzzy'), 'utf8')
		).resolves.toBe('xyzzy\n');
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toStrictEqual({ files: [], refs: '', worktrees: 1 });
	}
);

test(
	'restores the original directory and its file after the switch replaced them',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await pushUpstreamChange(directory.path, async (update) => {
			await rm(join(update, 'garply'), { recursive: true });
			await writeFile(join(update, 'garply'), 'xyzzy\n');
		});

		const before = readCheckoutState(checkout);
		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'read-tree [0-9a-f]*',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const interrupted = await lstat(join(checkout, 'garply'));
		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(interrupted.isFile()).toBe(true);
		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
		await expect(
			readFile(join(checkout, 'garply/waldo'), 'utf8')
		).resolves.toBe('waldo\n');
	}
);

test(
	'recovers when the switch stopped before creating an added file’s parent directory',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);

		await pushUpstreamChange(directory.path, async (update) => {
			await mkdir(join(update, 'plugh'));
			await writeFile(join(update, 'plugh/xyzzy'), 'xyzzy\n');
		});

		const before = readCheckoutState(checkout);
		const update = await interruptSync(
			directory.path,
			['sync', '--core'],
			'temp-index read-tree -m -u *',
			{ signal: 'SIGKILL', status: null, target: 'group' }
		);

		await update.run.done;

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);

test(
	'stops an active rollback before restoring a file through a parent symlink',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const moved = join(directory.path, 'garply');
		const { checkout } = await interruptSwitch(directory.path, {
			change: async (update) => {
				await rm(join(update, 'garply/waldo'));
				await mkdir(join(update, 'garply/waldo'));
				await writeFile(join(update, 'garply/waldo/xyzzy'), 'xyzzy\n');
			},
			local: async (root) => {
				await rename(join(root, 'garply'), moved);
				await symlink('../garply', join(root, 'garply'));
				await rm(join(moved, 'waldo/xyzzy'));
				await mkdir(join(moved, 'waldo/plugh'));
			},
		});

		await writeExecutable(
			join(directory.path, 'stubs/grep'),
			FOREIGN_PARENT_GREP_STUB
		);

		const result = await runCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);
		const parent = await lstat(join(checkout, 'garply'));

		expect(result.status).toBe(1);
		expect(lines(result.stderr, '× ')).toStrictEqual([
			'× Recovery could not finish restoring the checkout. The saved state is intact',
		]);
		expect(parent.isSymbolicLink()).toBe(true);
		await expect(readdir(join(moved, 'waldo'))).resolves.toStrictEqual([
			'plugh',
		]);
		await expect(
			readUpdateLeftovers(directory.path, checkout)
		).resolves.toMatchObject({
			files: expect.arrayContaining([
				expect.stringMatching(/^update-\d+-\d+$/),
			]),
			worktrees: 2,
		});
	}
);

test(
	'restores the interrupted switch before deciding whether to restart',
	{ timeout: RECOVERY_TEST_TIMEOUT },
	async () => {
		await using directory = await mkdtempDisposable(
			join(tmpdir(), 'domfiles-sync-')
		);
		const checkout = await createRecoveryFixture(directory.path);
		const before = readCheckoutState(checkout);
		const original = runGit(checkout, ['rev-parse', 'refs/heads/master']);
		const gated = await startGatedSync(
			directory.path,
			['sync', '--core'],
			'update-ref -m * refs/heads/master *'
		);
		const owner = gated.run.pid ?? 0;
		const waiter = startCommand(
			directory.path,
			'domfiles',
			['sync', 'install'],
			GIT_ENV
		);

		await waitForOutput(waiter, LOCK_WAITING_MESSAGE);

		// Let Git move `master` while the owner can’t write the `switched`
		// marker, then end the owner
		process.kill(owner, 'SIGSTOP');
		await gated.release();
		await vi.waitFor(
			() => {
				expect(
					runGit(checkout, ['rev-parse', 'refs/heads/master'])
				).not.toBe(original);
			},
			{ interval: 50, timeout: RECOVERY_TEST_TIMEOUT }
		);
		process.kill(owner, 'SIGKILL');

		const result = await waiter.done;

		expect(result.status).toBe(0);
		expect(lines(result.stdout, '✓ ')).toStrictEqual([
			'✓ Checkout restored to its state before the update',
		]);
		expect(result.stdout).not.toContain(CHANGED_CHECKOUT_RESTART);
		expect(result.stages).toStrictEqual(['stage install']);
		expect(readCheckoutState(checkout)).toStrictEqual(before);
	}
);
