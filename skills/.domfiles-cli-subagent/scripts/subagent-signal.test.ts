import type { ChildProcessWithoutNullStreams } from 'node:child_process';
import type { TestContext } from 'node:test';
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import {
	chmodSync,
	mkdirSync,
	mkdtempSync,
	readFileSync,
	renameSync,
	rmSync,
	symlinkSync,
	utimesSync,
	writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import process from 'node:process';
import { describe, test } from 'node:test';
import { setTimeout as sleep } from 'node:timers/promises';
import { OPTIONS } from './subagent-signal.ts';

// A lookup is counted only after the listing is read, so a test may replace the
// listing as soon as it sees the count change
const CLAUDE_STUB = [
	'#!/bin/sh',
	'[ "$*" = \'agents --json --all\' ] || exit 64',
	'directory="$(dirname -- "$0")"',
	'cat "$directory/listing.json"',
	'status=$?',
	'printf x >> "$directory/calls"',
	'exit "$status"',
	'',
].join('\n');
const NAME = 'subagent-signal';
const OTHER_UUID = 'ffffffff-ffff-ffff-ffff-ffffffffffff';
const SCRIPT = join(import.meta.dirname, 'subagent-signal.ts');
const USAGE_PREFIX = 'node <skill-directory>/scripts/subagent-signal.ts';
const UUID = '0123abcd-4567-89ab-cdef-0123456789ab';

type Entry = Readonly<Record<string, unknown>>;

type Output = { observedAt: unknown; sessions: Entry[] };

type Result = { status: number | null; stderr: string; stdout: string };

type Sessions = ReadonlyArray<readonly [string, number]>;

const field = (value: unknown, ...keys: Array<number | string>): unknown => {
	let current = value;

	for (const key of keys) {
		current =
			typeof current === 'object' && current !== null
				? (current as Record<number | string, unknown>)[key]
				: undefined;
	}

	return current;
};

const withWait = (params: ReadonlyArray<string>): string[] =>
	params.includes('--wait') ? [...params] : [...params, '--wait', '0'];

const collect = async (
	child: ChildProcessWithoutNullStreams
): Promise<Result> =>
	new Promise((resolve, reject) => {
		let stderr = '';
		let stdout = '';

		child.stderr.setEncoding('utf8').on('data', (chunk: string) => {
			stderr += chunk;
		});
		child.stdout.setEncoding('utf8').on('data', (chunk: string) => {
			stdout += chunk;
		});
		child.on('error', reject);
		child.on('close', (status) => {
			resolve({ status, stderr, stdout });
		});
	});

const createFixture = (context: TestContext) => {
	const root = mkdtempSync(join(tmpdir(), `${NAME}-test-`));
	const bin = join(root, 'bin');
	const home = join(root, 'home é');
	const project = join(home, '.claude/projects/-Users-dom-Projects-a b');
	const environment = {
		...process.env,
		HOME: home,
		PATH: `${bin}:/usr/bin:/bin`,
	};

	mkdirSync(bin, { recursive: true });
	mkdirSync(project, { recursive: true });
	writeFileSync(join(bin, 'claude'), CLAUDE_STUB, { mode: 0o755 });
	context.after(() => {
		rmSync(root, { force: true, recursive: true });
	});

	const transcript = (uuid: string): string => join(project, `${uuid}.jsonl`);
	const lines = (records: ReadonlyArray<unknown>): string =>
		records.map((record) => `${JSON.stringify(record)}\n`).join('');

	// Every run starts outside the repository, so no result depends on the
	// caller’s directory
	const start = (
		params: ReadonlyArray<string>
	): ChildProcessWithoutNullStreams =>
		spawn(process.execPath, [SCRIPT, ...params], {
			cwd: root,
			env: environment,
		});

	const run = async (params: ReadonlyArray<string>): Promise<Result> =>
		collect(start(params));

	const checkObservation = (
		params: ReadonlyArray<string>,
		result: Result
	): string => {
		assert.equal(result.status, 0, `${params.join(' ')}: ${result.stderr}`);
		assert.equal(result.stderr, '', params.join(' '));
		assert.ok(
			result.stdout.endsWith('\n') &&
				result.stdout.split('\n').length === 2,
			result.stdout
		);

		return result.stdout;
	};

	// Every observation defaults to `--wait 0` and prints exactly one JSON line
	const observeParams = async (
		params: ReadonlyArray<string>
	): Promise<string> => checkObservation(params, await run(withWait(params)));

	const parse = (stdout: string): Output => {
		const output = JSON.parse(stdout) as Output;

		assert.ok(Number.isSafeInteger(output.observedAt), stdout);

		return output;
	};

	const sessionParams = (sessions: Sessions): string[] =>
		sessions.flatMap(([uuid, since]) => [
			'--claude',
			`${String(since)}:${uuid}`,
		]);

	const session = (output: Output, index = 0): Entry => {
		const entry = output.sessions[index];

		assert.ok(entry !== undefined, JSON.stringify(output));

		return entry;
	};

	const observeRaw = async (
		sessions: Sessions,
		extra: ReadonlyArray<string> = []
	): Promise<string> => observeParams([...sessionParams(sessions), ...extra]);

	const observeAll = async (
		sessions: Sessions,
		extra: ReadonlyArray<string> = []
	): Promise<Output> => parse(await observeRaw(sessions, extra));

	const calls = (): number => {
		try {
			return readFileSync(join(bin, 'calls')).length;
		} catch {
			return 0;
		}
	};

	const rollout = (uuid: string, day: number, contents: string): string => {
		const date = String(day).padStart(2, '0');
		const directory = join(home, `.codex/sessions/2026/10/${date}`);
		const path = join(
			directory,
			`rollout-2026-10-${date}T18-00-00-${uuid}.jsonl`
		);

		mkdirSync(directory, { recursive: true });
		writeFileSync(path, contents);

		return path;
	};

	return {
		bin,
		calls,
		home,
		listing: (content: unknown): void => {
			writeFileSync(join(bin, 'listing.json'), JSON.stringify(content));
		},
		observe: async (
			since: number,
			extra: ReadonlyArray<string> = []
		): Promise<Entry> => session(await observeAll([[UUID, since]], extra)),
		observeAll,
		observeKey: async (
			key: string,
			since: number,
			extra: ReadonlyArray<string> = []
		): Promise<unknown> => {
			const entry = session(await observeAll([[UUID, since]], extra));

			return entry[key];
		},
		observeRaw,
		observeWith: async (params: ReadonlyArray<string>): Promise<Output> =>
			parse(await observeParams(params)),
		project,
		refuse: async (params: ReadonlyArray<string>): Promise<string> => {
			const { status, stderr, stdout } = await run(params);

			assert.equal(status, 2, `${params.join(' ')}: ${stderr}`);
			assert.equal(stdout, '', params.join(' '));
			assert.ok(
				stderr.startsWith(`${NAME}: `) &&
					stderr.endsWith('\n') &&
					stderr.split('\n').length === 2,
				`${params.join(' ')}: ${stderr}`
			);

			return stderr;
		},
		rollout,
		run,
		session,
		start,
		subagent: (uuid: string, name: string, contents: string): string => {
			const directory = join(
				transcript(uuid).slice(0, -'.jsonl'.length),
				'subagents'
			);
			const path = join(directory, name);

			mkdirSync(directory, { recursive: true });
			writeFileSync(path, contents);

			return path;
		},
		transcript,
		waitForCalls: async (count: number): Promise<void> => {
			const deadline = performance.now() + 10_000;

			while (calls() < count) {
				assert.ok(
					performance.now() < deadline,
					`Lookup ${String(count)} must occur`
				);
				await sleep(10);
			}
		},
		write: (uuid: string, records: ReadonlyArray<unknown>): void => {
			writeFileSync(transcript(uuid), lines(records));
		},
		writeRollout: (
			uuid: string,
			day: number,
			records: ReadonlyArray<unknown>
		): string => rollout(uuid, day, lines(records)),
	};
};

const helpSection = (help: string, heading: string): string[] => {
	const lines = help.split('\n');
	const start = lines.indexOf(heading) + 1;
	const end = lines.indexOf('', start);

	return lines.slice(start, end === -1 ? undefined : end);
};

const at = (second: number): string =>
	`2026-10-09T18:00:${String(second).padStart(2, '0')}.000Z`;

const setModified = (path: string, milliseconds: number): void => {
	const time = new Date(milliseconds);

	utimesSync(path, time, time);
};

const bookkeeping = () => ({ type: 'ai-title', aiTitle: 'Review' });

const prompt = (second: number) => ({
	type: 'user',
	origin: { kind: 'human' },
	message: { role: 'user', content: 'Review the change' },
	timestamp: at(second),
});

const reply = (
	id: string,
	text: string,
	stopReason: string,
	second: number
) => ({
	type: 'assistant',
	message: {
		id,
		model: 'claude-opus-5-5',
		stop_reason: stopReason,
		content: [{ type: 'text', text }],
	},
	timestamp: at(second),
});

const thinking = (id: string, second: number) => ({
	type: 'assistant',
	message: {
		id,
		model: 'claude-opus-5-5',
		stop_reason: 'end_turn',
		content: [{ type: 'thinking', thinking: 'Considering the change' }],
	},
	timestamp: at(second),
});

const synthetic = (text: string, error: string | null, second: number) => ({
	type: 'assistant',
	message: {
		id: `synthetic-${String(second)}`,
		model: '<synthetic>',
		stop_reason: 'stop_sequence',
		content: [{ type: 'text', text }],
	},
	error,
	isApiErrorMessage: error !== null,
	timestamp: at(second),
});

const toolCall = (id: string, input: unknown, second: number) => ({
	type: 'assistant',
	message: {
		id: `message-${id}`,
		model: 'claude-opus-5-5',
		stop_reason: 'tool_use',
		content: [{ type: 'tool_use', id, name: 'Bash', input }],
	},
	timestamp: at(second),
});

const toolResult = (id: string, isError: boolean, second: number) => ({
	type: 'user',
	message: {
		role: 'user',
		content: [
			{
				type: 'tool_result',
				tool_use_id: id,
				is_error: isError,
				content: 'output',
			},
		],
	},
	timestamp: at(second),
});

const turnEnd = (second: number): Record<string, unknown> => ({
	type: 'system',
	subtype: 'turn_duration',
	durationMs: 1000,
	timestamp: at(second),
});

const sidechain = (record: object) => ({ ...record, isSidechain: true });

const compaction = (second: number) => ({
	type: 'system',
	subtype: 'compact_boundary',
	timestamp: at(second),
});

const notification = (id: string, second: number) => ({
	type: 'queue-operation',
	operation: 'enqueue',
	content: `<task-notification>\n<task-id>task</task-id>\n<tool-use-id>${id}</tool-use-id>\n<status>completed</status>\n</task-notification>`,
	timestamp: at(second),
});

const row = (
	uuid: string,
	state: string,
	status: string | null,
	waitingFor: string | null
) => ({ id: uuid.slice(0, 8), sessionId: uuid, state, status, waitingFor });

const rows = (
	state: string,
	status: string | null,
	waitingFor: string | null
) => [
	row(OTHER_UUID, 'working', 'busy', null),
	row(UUID, state, status, waitingFor),
];

const completedTurn = (): Array<Record<string, unknown>> => [
	bookkeeping(),
	prompt(1),
	toolCall('call', { command: 'git diff' }, 2),
	toolResult('call', false, 3),
	thinking('final', 4),
	reply('final', 'No findings', 'end_turn', 5),
	turnEnd(6),
	bookkeeping(),
];

const sessionMeta = (id: string, root: string, second: number) => ({
	type: 'session_meta',
	payload: { id, session_id: root },
	timestamp: at(second),
});

const event = (payload: object, second: number): Record<string, unknown> => ({
	type: 'event_msg',
	payload,
	timestamp: at(second),
});

const turnStarted = (second: number) => event({ type: 'task_started' }, second);

const turnComplete = (
	message: string | null,
	error: string | null,
	second: number
) =>
	event(
		{
			type: 'task_complete',
			last_agent_message: message,
			error:
				error === null
					? null
					: { message: error, codex_error_info: 'other' },
		},
		second
	);

const turnAborted = (reason: string, second: number) =>
	event({ type: 'turn_aborted', reason }, second);

const reasoning = (second: number) => ({
	type: 'response_item',
	payload: { type: 'reasoning' },
	timestamp: at(second),
});

const codexTurn = (text: string, start: number) => [
	turnStarted(start),
	reasoning(start + 1),
	turnComplete(text, null, start + 2),
];

const background = { command: 'pnpm test', run_in_background: true };

void describe(NAME, { concurrency: true }, () => {
	void test('prints help alone', async (context) => {
		const { status, stderr, stdout } = await createFixture(context).run([
			'--help',
		]);

		assert.equal(status, 0);
		assert.ok(stdout.startsWith('Usage:\n'));
		assert.equal(stderr, '');
	});

	void test('accepts every documented invocation', async (context) => {
		const fixture = createFixture(context);
		const value = `0:${UUID}`;
		const { stdout: help } = await fixture.run(['--help']);
		const usage = helpSection(help, 'Usage:');
		const usageOptions = (usage[0] ?? '')
			.split(/\s+/)
			.map((token) => token.replaceAll(/^[[\]()]+|[[\]()]+$/g, ''))
			.filter((token) => token.startsWith('--'))
			.toSorted();
		const documented = helpSection(help, 'Options:').map(
			(line) => line.trim().split(/\s+/)[0] ?? ''
		);

		assert.equal(usage.length, 2);
		assert.ok(
			usage.every((line) => line.trim().startsWith(`${USAGE_PREFIX} `))
		);
		assert.deepEqual(documented, documented.toSorted());
		assert.deepEqual(documented, usageOptions);
		assert.deepEqual(documented, [...OPTIONS]);

		const outputs = await Promise.all(
			documented.map(async (option) =>
				fixture.observeWith(
					option === '--wait'
						? ['--claude', value, '--wait', '0']
						: [option, value, '--wait', '0']
				)
			)
		);

		for (const [index, output] of outputs.entries()) {
			assert.equal(
				fixture.session(output).state,
				'working',
				documented[index]
			);
		}
	});

	void test('rejects invalid invocations', async (context) => {
		const fixture = createFixture(context);
		const value = `0:${UUID}`;
		const upper = `0:${UUID.toUpperCase()}`;
		const required =
			'At least one `--claude` or `--codex` session is required';
		const helpAlone = '`--help` must be the only argument';
		const repeated = `Session \`${UUID}\` must be given only once`;
		const malformed = 'must be `<since>:<uuid>` with a full session UUID';
		const nonInteger = '`<since>` must be a non-negative integer';
		const cases: Array<[string[], string]> = [
			[[], required],
			[['-h'], 'Unknown argument `-h`'],
			[['--help', '--claude', value], helpAlone],
			[['--claude', value, '--help'], helpAlone],
			[['--wait', '0'], required],
			[['--claude'], '`--claude` requires a value'],
			[['--claude', value, '--claude', value], repeated],
			[['--claude', value, '--claude', upper], repeated],
			[['--claude', value, '--verbose'], 'Unknown argument `--verbose`'],
			[
				['--claude', value, '--wait', '0', '--wait', '0'],
				'`--wait` must be given only once',
			],
			[
				['--claude', value, '--wait', 'x'],
				'`--wait` must be a non-negative integer',
			],
			[['--claude', value, '--wait'], '`--wait` requires a value'],
			[
				['--claude', value, '--wait', '99999999999999999999'],
				'`--wait` must be at most 90',
			],
			[
				['--claude', `9007199254740992:${UUID}`],
				'`<since>` must be at most 9007199254740991',
			],
			[['--claude', value, '--codex', value], repeated],
			[['--codex', value, '--codex', upper], repeated],
			[['--codex', UUID], malformed],
			[['--codex'], '`--codex` requires a value'],
			...[
				UUID,
				`0:${UUID.slice(1)}`,
				`0:${UUID.replaceAll('-', '_')}`,
				`0:${UUID}.jsonl`,
			].map((invalid): [string[], string] => [
				['--claude', invalid],
				malformed,
			]),
			...[
				`:${UUID}`,
				` 1:${UUID}`,
				`+1:${UUID}`,
				`-1:${UUID}`,
				`1.5:${UUID}`,
				`1e3:${UUID}`,
			].map((invalid): [string[], string] => [
				['--claude', invalid],
				nonInteger,
			]),
		];
		const refusals = await Promise.all(
			cases.map(async ([params]) => fixture.refuse(params))
		);

		for (const [index, [params, diagnostic]] of cases.entries()) {
			const stderr = refusals[index] ?? '';

			assert.ok(
				stderr.includes(diagnostic),
				`${params.join(' ')}: ${stderr}`
			);
		}

		assert.equal(fixture.calls(), 0);
	});

	void test('reports a failed write to standard output', async (context) => {
		const child = createFixture(context).start([
			'--wait',
			'0',
			'--claude',
			`0:${UUID}`,
		]);

		// A closed reader makes every write fail, so the observation cannot be
		// delivered
		child.stdout.destroy();

		const { status, stderr } = await collect(child);

		assert.equal(status, 2, stderr);
		assert.ok(
			stderr.startsWith(
				`${NAME}: Failed to write to standard output:\n\n`
			),
			stderr
		);
	});

	void test('accepts the documented maximum `--wait` and rejects larger values', async (context) => {
		const fixture = createFixture(context);
		const { stdout: help } = await fixture.run(['--help']);
		const line =
			helpSection(help, 'Options:').find((option) =>
				option.trim().startsWith('--wait ')
			) ?? '';
		const maximum = Number(/from `0` to `(\d+)`/.exec(line)?.[1]);

		assert.ok(
			Number.isSafeInteger(maximum),
			'Help must document the wait bound'
		);
		assert.ok(help.includes(`Defaults to \`${String(maximum)}\``));

		fixture.write(UUID, completedTurn());
		fixture.listing(rows('done', 'idle', null));
		assert.equal(
			await fixture.observeKey('state', 0, ['--wait', String(maximum)]),
			'ready'
		);

		const stderr = await fixture.refuse([
			'--claude',
			`0:${UUID}`,
			'--wait',
			String(maximum + 1),
		]);

		assert.ok(
			stderr.includes(`\`--wait\` must be at most ${String(maximum)}`),
			stderr
		);
	});

	void test('reports unreadable and malformed transcripts as errors', async (context) => {
		const fixture = createFixture(context);
		const transcript = fixture.transcript(UUID);

		writeFileSync(
			transcript,
			`${JSON.stringify(prompt(1))}\n{"secret": \n`
		);

		const output = await fixture.observe(0);
		const message = String(output.message);

		assert.equal(output.state, 'error', JSON.stringify(output));
		assert.ok(!('nextSince' in output), JSON.stringify(output));
		assert.ok(message.includes('line 2'), message);
		assert.ok(!message.includes('secret'), message);

		writeFileSync(transcript, '[1]\n');
		assert.equal(await fixture.observeKey('state', 0), 'error');

		chmodSync(transcript, 0o000);

		try {
			assert.equal(await fixture.observeKey('state', 0), 'error');
		} finally {
			chmodSync(transcript, 0o644);
		}
	});

	void test('reports a boundary beyond the transcript as an error', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1), bookkeeping()]);

		const output = await fixture.observe(3);

		assert.equal(output.state, 'error');
		assert.ok(
			String(output.message).includes('2 complete lines'),
			JSON.stringify(output)
		);
	});

	void test('reports a missing transcript as `working`', async (context) => {
		const fixture = createFixture(context);

		const raw = await fixture.observeRaw([[UUID, 0]]);

		assert.equal(
			raw.split(',"sessions":')[1],
			`[{"session":"${UUID}","state":"working","nextSince":0,"progressAt":null}]}\n`
		);
		assert.equal(fixture.calls(), 0);

		rmSync(join(fixture.home, '.claude'), { recursive: true });
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('reports a missing follow-up transcript as an error', async (context) => {
		const fixture = createFixture(context);
		const message = 'Found no transcript, but `<since>` is above 0';
		const codex = async (since: number): Promise<Entry> =>
			fixture.session(
				await fixture.observeWith([
					'--codex',
					`${String(since)}:${OTHER_UUID}`,
				])
			);

		assert.deepEqual(await fixture.observe(4), {
			session: UUID,
			state: 'error',
			message,
		});
		assert.deepEqual(await codex(4), {
			session: OTHER_UUID,
			state: 'error',
			message,
		});

		rmSync(join(fixture.home, '.claude'), { recursive: true });
		assert.equal(await fixture.observeKey('message', 4), message);

		// The monitor cannot read a rollout that Codex compressed while it
		// stayed idle
		const rollout = fixture.rollout(OTHER_UUID, 9, '');

		renameSync(rollout, `${rollout}.zst`);
		assert.deepEqual(await codex(4), {
			session: OTHER_UUID,
			state: 'working',
			nextSince: 4,
			progressAt: null,
		});
		assert.equal(fixture.calls(), 0);
	});

	void test('reports inaccessible projects as errors', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1)]);
		chmodSync(fixture.project, 0o000);

		try {
			const output = await fixture.observe(0);

			assert.equal(output.state, 'error', JSON.stringify(output));
		} finally {
			chmodSync(fixture.project, 0o755);
		}
	});

	void test('ignores plain files beside projects', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1)]);
		writeFileSync(join(fixture.home, '.claude/projects/.DS_Store'), '');
		fixture.listing(rows('working', 'busy', null));
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('reports ambiguous transcripts as errors', async (context) => {
		const fixture = createFixture(context);
		const other = join(
			fixture.home,
			'.claude/projects/-Users-dom-Projects-other'
		);

		fixture.write(UUID, [prompt(1)]);
		mkdirSync(other, { recursive: true });
		writeFileSync(
			join(other, `${UUID}.jsonl`),
			`${JSON.stringify(prompt(1))}\n`
		);

		const output = await fixture.observe(0);

		assert.equal(output.state, 'error');
		assert.ok(
			String(output.message).includes('Found 2 transcripts'),
			JSON.stringify(output)
		);
	});

	void test('ignores an unterminated final record', async (context) => {
		const fixture = createFixture(context);
		const transcript = fixture.transcript(UUID);

		writeFileSync(
			transcript,
			`${JSON.stringify(prompt(1))}\n{"type":"assist`
		);
		setModified(transcript, 1000);
		fixture.listing(rows('working', 'busy', null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'working',
			nextSince: 1,
			progressAt: 1000,
		});
	});

	void test('reports `working` before the prompt without a lookup', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			reply('a', 'Done', 'end_turn', 2),
			turnEnd(3),
		]);
		assert.deepEqual(await fixture.observe(3), {
			session: UUID,
			state: 'working',
			nextSince: 3,
			progressAt: null,
		});
		assert.equal(fixture.calls(), 0);
	});

	void test('anchors the turn on the latest prompt', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			reply('a', 'Done', 'end_turn', 2),
			turnEnd(3),
			synthetic('No response requested.', null, 4),
			prompt(5),
		]);
		setModified(fixture.transcript(UUID), 5000);
		fixture.listing(rows('done', 'idle', null));
		assert.deepEqual(await fixture.observe(3), {
			session: UUID,
			state: 'working',
			nextSince: 5,
			progressAt: 5000,
		});
	});

	void test('anchors only on prompts with a human origin', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			compaction(4),
			{
				type: 'user',
				isCompactSummary: true,
				message: {
					role: 'user',
					content: 'Summary of the earlier conversation',
				},
				timestamp: at(5),
			},
			{
				type: 'user',
				message: {
					role: 'user',
					content: '<local-command-stdout></local-command-stdout>',
				},
				timestamp: at(6),
			},
			reply('b', 'Started the tests', 'end_turn', 7),
			turnEnd(8),
		]);
		fixture.listing(rows('done', 'idle', null));

		const output = await fixture.observe(0);

		assert.equal(output.state, 'working', JSON.stringify(output));
		assert.equal(output.nextSince, 8);
	});

	void test('reports progress from the latest transcript write', async (context) => {
		const fixture = createFixture(context);
		const transcript = fixture.transcript(UUID);

		fixture.write(UUID, [prompt(10), toolCall('a', { command: 'ls' }, 11)]);
		setModified(transcript, 20_000);
		fixture.listing(rows('working', 'busy', null));
		assert.equal(await fixture.observeKey('progressAt', 0), 20_000);

		const earlier = fixture.subagent(UUID, 'agent-earlier.jsonl', '{}\n');
		const current = fixture.subagent(UUID, 'agent-current.jsonl', '{}\n');
		const metadata = fixture.subagent(
			UUID,
			'agent-current.meta.json',
			'{}'
		);

		setModified(earlier, 5000);
		setModified(current, 30_000);
		setModified(metadata, 40_000);
		assert.equal(await fixture.observeKey('progressAt', 0), 30_000);

		// Subagent transcripts count by their writes alone, so their contents
		// are never parsed
		writeFileSync(current, '{\n');
		setModified(current, 35_000);
		assert.equal(await fixture.observeKey('progressAt', 0), 35_000);

		// A subagent transcript removed after the listing has no write to
		// report
		symlinkSync(
			join(fixture.home, 'missing.jsonl'),
			join(dirname(current), 'agent-removed.jsonl')
		);
		assert.equal(await fixture.observeKey('progressAt', 0), 35_000);
	});

	void test('reports the final reply when `ready`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());

		const before = readFileSync(fixture.transcript(UUID));

		fixture.listing(rows('done', null, null));

		const output = await fixture.observeRaw([[UUID, 0]]);

		assert.ok(output.startsWith('{"observedAt":'), output);
		assert.ok(
			output.endsWith(
				`,"sessions":[{"session":"${UUID}","state":"ready","nextSince":8,"replies":["No findings"],"compactedAt":null}]}\n`
			),
			output
		);
		assert.deepEqual(readFileSync(fixture.transcript(UUID)), before);
	});

	void test('reports observation time in Unix milliseconds', async (context) => {
		const fixture = createFixture(context);
		const before = Date.now();

		fixture.write(UUID, [prompt(1)]);

		const { observedAt } = await fixture.observeAll([[UUID, 1]]);

		assert.ok(
			typeof observedAt === 'number' &&
				observedAt >= before &&
				observedAt <= Date.now()
		);
	});

	void test('waits for a session that is not `busy`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		setModified(fixture.transcript(UUID), 5000);
		fixture.listing(rows('working', 'busy', null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'working',
			nextSince: 8,
			progressAt: 5000,
		});

		fixture.listing(rows('working', 'idle', null));
		assert.equal(await fixture.observeKey('state', 0), 'ready');
	});

	void test('reports the latest compaction', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			compaction(0),
			bookkeeping(),
			compaction(0),
			...completedTurn(),
		]);
		fixture.listing(rows('done', null, null));
		assert.equal(await fixture.observeKey('compactedAt', 0), 3);
	});

	void test('reports turns that end without a reply', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			synthetic(
				'Please run /login · API Error: 401',
				'authentication_failed',
				2
			),
			turnEnd(3),
		]);
		fixture.listing(rows('blocked', 'idle', null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'attention',
			nextSince: 3,
			reason: 'noReply',
			detail: {
				error: 'authentication_failed',
				model: '<synthetic>',
				stopReason: 'stop_sequence',
				text: 'Please run /login · API Error: 401',
				textOmitted: 0,
			},
		});

		fixture.write(UUID, [
			prompt(1),
			reply('a', 'é'.repeat(520), 'max_tokens', 2),
			turnEnd(3),
		]);
		fixture.listing(rows('done', null, null));

		const output = await fixture.observe(0);

		assert.equal(field(output, 'detail', 'stopReason'), 'max_tokens');
		assert.equal(field(output, 'detail', 'text'), 'é'.repeat(500));
		assert.equal(field(output, 'detail', 'textOmitted'), 20);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', { command: 'ls' }, 2),
			turnEnd(3),
		]);
		assert.equal(await fixture.observeKey('reason', 0), 'noReply');
	});

	void test('reports `blocked` sessions with the pending tool', async (context) => {
		const fixture = createFixture(context);
		const input = { command: 'y'.repeat(600) };
		const serialized = JSON.stringify(input);

		fixture.write(UUID, [prompt(1), toolCall('a', input, 2)]);
		fixture.listing(rows('working', 'waiting', 'permission prompt'));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'attention',
			nextSince: 2,
			reason: 'blocked',
			detail: {
				tool: {
					input: serialized.slice(0, 500),
					inputOmitted: serialized.length - 500,
					name: 'Bash',
				},
				waitingFor: 'permission prompt',
			},
		});

		// A `blocked` session state without an open prompt is not a pending
		// approval
		fixture.listing(rows('blocked', 'idle', null));
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('waits for background work despite a `blocked` session state', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			reply('b', 'Should I also run lint?', 'end_turn', 4),
			turnEnd(5),
		]);
		fixture.listing(rows('blocked', 'idle', null));
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('reports a finished reply despite a `blocked` session state with an `idle` status', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		fixture.listing(rows('blocked', 'idle', null));
		assert.deepEqual(await fixture.observeKey('replies', 0), [
			'No findings',
		]);
	});

	void test('reports a finished reply despite a `blocked` session state without a status', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		fixture.listing(rows('blocked', null, null));
		assert.deepEqual(await fixture.observeKey('replies', 0), [
			'No findings',
		]);
	});

	void test('reports `failed` turns', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1), toolCall('a', { command: 'ls' }, 2)]);
		fixture.listing(rows('failed', null, null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'attention',
			nextSince: 2,
			reason: 'failed',
		});
	});

	void test('reports `stopped` turns', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1), toolCall('a', { command: 'ls' }, 2)]);
		fixture.listing(rows('stopped', null, null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'attention',
			nextSince: 2,
			reason: 'stopped',
		});
	});

	void test('reports open prompts after a reply as `blocked`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		fixture.listing(rows('working', 'waiting', null));
		assert.deepEqual(await fixture.observe(0), {
			session: UUID,
			state: 'attention',
			nextSince: 8,
			reason: 'blocked',
			detail: { tool: null, waitingFor: null },
		});

		fixture.write(UUID, [prompt(1), toolCall('a', { command: 'ls' }, 2)]);
		fixture.listing(rows('working', 'busy', 'dialog open'));
		assert.equal(
			field(await fixture.observe(0), 'detail', 'waitingFor'),
			'dialog open'
		);
	});

	void test('waits for pending background agents', async (context) => {
		const fixture = createFixture(context);
		const records = completedTurn();

		records[6] = { ...turnEnd(6), pendingBackgroundAgentCount: 1 };
		fixture.listing(rows('done', null, null));
		fixture.write(UUID, records);
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('waits for pending workflows', async (context) => {
		const fixture = createFixture(context);
		const records = completedTurn();

		records[6] = { ...turnEnd(6), pendingWorkflowCount: 1 };
		fixture.listing(rows('done', null, null));
		fixture.write(UUID, records);
		assert.equal(await fixture.observeKey('state', 0), 'working');
	});

	void test('reports `ready` without pending background agents', async (context) => {
		const fixture = createFixture(context);
		const records = completedTurn();

		records[6] = { ...turnEnd(6), pendingBackgroundAgentCount: 0 };
		fixture.listing(rows('done', null, null));
		fixture.write(UUID, records);
		assert.equal(await fixture.observeKey('state', 0), 'ready');
	});

	void test('reports `ready` without pending workflows', async (context) => {
		const fixture = createFixture(context);
		const records = completedTurn();

		records[6] = { ...turnEnd(6), pendingWorkflowCount: 0 };
		fixture.listing(rows('done', null, null));
		fixture.write(UUID, records);
		assert.equal(await fixture.observeKey('state', 0), 'ready');
	});

	void test('reports failed replies despite pending background work', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			synthetic('API Error: 500', 'server_error', 4),
			turnEnd(5),
		]);
		fixture.listing(rows('done', null, null));
		assert.equal(await fixture.observeKey('reason', 0), 'noReply');

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			reply('b', 'Started the tests', 'end_turn', 4),
			turnEnd(5),
		]);
		fixture.listing(rows('stopped', null, null));
		assert.equal(await fixture.observeKey('reason', 0), 'stopped');
	});

	void test('ignores sidechain records when deciding the turn', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', { command: 'ls' }, 2),
			sidechain(reply('s', 'Sidechain finding', 'end_turn', 4)),
			sidechain(toolResult('s', false, 5)),
			sidechain(turnEnd(6)),
			sidechain(prompt(7)),
		]);
		fixture.listing(rows('done', 'idle', null));

		const output = await fixture.observe(0);

		assert.equal(output.state, 'working', JSON.stringify(output));
		assert.equal(output.nextSince, 6);
	});

	void test('waits for background commands to report', async (context) => {
		const fixture = createFixture(context);
		const records: unknown[] = [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			reply('b', 'Started the tests', 'end_turn', 4),
			turnEnd(5),
		];

		fixture.write(UUID, records);
		fixture.listing(rows('done', 'idle', null));
		assert.equal(await fixture.observeKey('state', 0), 'working');

		// Claude answers a notification recorded after its turn ended, even
		// while the row shows `idle`
		records.push(notification('a', 6));
		fixture.write(UUID, records);
		assert.equal(await fixture.observeKey('state', 0), 'working');

		records.push(reply('c', 'The tests passed', 'end_turn', 7), turnEnd(8));
		fixture.write(UUID, records);
		assert.deepEqual(await fixture.observeKey('replies', 0), [
			'Started the tests',
			'The tests passed',
		]);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', true, 3),
			reply('b', 'The tests could not start', 'end_turn', 4),
			turnEnd(5),
		]);
		assert.equal(await fixture.observeKey('state', 0), 'ready');
	});

	void test('collects the reply of every turn since the prompt', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			reply('a', 'First', 'end_turn', 2),
			turnEnd(3),
			notification('x', 4),
			reply('b', 'Second', 'end_turn', 5),
			turnEnd(6),
		]);
		fixture.listing(rows('done', null, null));
		assert.deepEqual(await fixture.observeKey('replies', 0), [
			'First',
			'Second',
		]);
	});

	void test('reports unexpected session lookups as errors', async (context) => {
		const lookup = async (
			prepare: (bin: string) => void,
			message: string
		): Promise<void> => {
			const current = createFixture(context);

			current.write(UUID, completedTurn());
			prepare(current.bin);

			const output = await current.observe(0);

			assert.equal(output.state, 'error', JSON.stringify(output));
			assert.ok(
				String(output.message).startsWith(message),
				JSON.stringify(output)
			);
		};

		const listing =
			(content: unknown) =>
			(bin: string): void => {
				writeFileSync(
					join(bin, 'listing.json'),
					JSON.stringify(content)
				);
			};

		await Promise.all([
			lookup(() => undefined, '`claude agents` failed with'),
			lookup(
				listing([row(OTHER_UUID, 'done', null, null)]),
				'Expected one `claude agents` row for the session, found 0'
			),
			lookup(
				listing([
					row(UUID, 'done', null, null),
					row(UUID, 'done', null, null),
				]),
				'Expected one `claude agents` row for the session, found 2'
			),
			lookup(
				listing(row(UUID, 'done', null, null)),
				'`claude agents` did not print a JSON array'
			),
			lookup((bin) => {
				writeFileSync(join(bin, 'listing.json'), 'not json');
			}, '`claude agents` printed invalid JSON'),
			lookup((bin) => {
				rmSync(join(bin, 'claude'));
			}, 'Failed to run `claude agents`:\n\n'),
		]);
	});

	void test('observes several sessions in argument order', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		fixture.write(OTHER_UUID, [prompt(1)]);
		setModified(fixture.transcript(OTHER_UUID), 7000);
		fixture.listing(rows('done', null, null));

		const { sessions } = await fixture.observeAll([
			[OTHER_UUID, 0],
			[UUID, 0],
		]);

		assert.deepEqual(sessions, [
			{
				session: OTHER_UUID,
				state: 'working',
				nextSince: 1,
				progressAt: 7000,
			},
			{
				session: UUID,
				state: 'ready',
				nextSince: 8,
				replies: ['No findings'],
				compactedAt: null,
			},
		]);
		assert.equal(fixture.calls(), 1);
	});

	void test('keeps reporting healthy sessions beside errors', async (context) => {
		const fixture = createFixture(context);
		const missing = UUID.replaceAll('0', '9');

		fixture.write(UUID, [prompt(1)]);
		fixture.write(OTHER_UUID, [prompt(1)]);
		fixture.listing([row(OTHER_UUID, 'working', 'busy', null)]);

		const started = performance.now();
		const { sessions } = await fixture.observeAll(
			[
				[UUID, 0],
				[OTHER_UUID, 0],
				[missing, 0],
			],
			['--wait', '60']
		);

		// An `error` ends the wait like any other state that is not `working`
		assert.ok(performance.now() - started < 10_000);
		assert.equal(sessions[0]?.state, 'error');
		assert.equal(sessions[1]?.state, 'working');
		assert.deepEqual(sessions[2], {
			session: missing,
			state: 'working',
			nextSince: 0,
			progressAt: null,
		});
	});

	void test('returns as soon as any session leaves `working`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1)]);
		fixture.write(OTHER_UUID, [prompt(1)]);
		fixture.listing([
			row(OTHER_UUID, 'working', 'busy', null),
			row(UUID, 'done', 'idle', null),
		]);

		const started = performance.now();
		const observation = fixture.observeWith([
			'--claude',
			`0:${OTHER_UUID}`,
			'--claude',
			`0:${UUID}`,
			'--wait',
			'60',
		]);

		await fixture.waitForCalls(1);
		fixture.write(UUID, completedTurn());

		const { sessions } = await observation;

		assert.ok(performance.now() - started < 20_000);
		assert.equal(sessions[0]?.state, 'working');
		assert.equal(sessions[1]?.state, 'ready');
	});

	void test('refreshes the lookup when a prompt is recorded', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, []);
		fixture.write(OTHER_UUID, [prompt(1)]);
		fixture.listing([
			row(OTHER_UUID, 'working', 'busy', null),
			row(UUID, 'stopped', null, null),
		]);

		const observation = fixture.observeWith([
			'--claude',
			`0:${UUID}`,
			'--claude',
			`0:${OTHER_UUID}`,
			'--wait',
			'60',
		]);

		await fixture.waitForCalls(1);

		// A resumed session’s row changes before its prompt is recorded
		fixture.listing([
			row(OTHER_UUID, 'done', 'idle', null),
			row(UUID, 'working', 'busy', null),
		]);
		fixture.write(UUID, [prompt(1)]);
		await fixture.waitForCalls(2);
		fixture.write(OTHER_UUID, completedTurn());

		const output = await observation;

		assert.equal(
			output.sessions[0]?.state,
			'working',
			JSON.stringify(output)
		);
		assert.equal(
			output.sessions[1]?.state,
			'ready',
			JSON.stringify(output)
		);
	});

	void test('stops waiting once the session is `ready`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, completedTurn());
		fixture.listing(rows('done', null, null));

		const started = performance.now();

		assert.equal(
			await fixture.observeKey('state', 0, ['--wait', '60']),
			'ready'
		);
		assert.ok(performance.now() - started < 10_000);
		assert.equal(fixture.calls(), 1);
	});

	void test('waits until the deadline while the session is `working`', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1)]);
		fixture.listing(rows('working', 'busy', null));

		const started = performance.now();

		assert.equal(
			await fixture.observeKey('state', 0, ['--wait', '1']),
			'working'
		);
		assert.ok(performance.now() - started >= 1000);
		assert.ok(performance.now() - started < 10_000);
		assert.equal(fixture.calls(), 1);
	});

	void test('waits for readiness by default', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [prompt(1)]);
		fixture.listing(rows('done', 'idle', null));

		const observation = fixture.run(['--claude', `0:${UUID}`]);

		await fixture.waitForCalls(1);
		fixture.write(UUID, completedTurn());

		const { status, stderr, stdout } = await observation;

		assert.equal(status, 0, stderr);
		assert.equal(stderr, '');
		assert.equal(
			fixture.session(JSON.parse(stdout) as Output).state,
			'ready'
		);
	});

	void test('keeps the lookup interval while background work runs', async (context) => {
		const fixture = createFixture(context);

		fixture.write(UUID, [
			prompt(1),
			toolCall('a', background, 2),
			toolResult('a', false, 3),
			reply('b', 'Started the tests', 'end_turn', 4),
			turnEnd(5),
		]);
		fixture.listing(rows('done', 'idle', null));
		assert.equal(
			await fixture.observeKey('state', 0, ['--wait', '2']),
			'working'
		);
		assert.equal(fixture.calls(), 1);
	});

	void test('reports a finished Codex turn', async (context) => {
		const fixture = createFixture(context);

		fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			{ type: 'compacted', timestamp: at(1) },
			...codexTurn('First', 2),
			...codexTurn('Second', 10),
		]);

		const [startOutput, laterOutput] = await Promise.all([
			fixture.observeWith(['--codex', `0:${UUID}`]),
			fixture.observeWith(['--codex', `5:${UUID}`]),
		]);
		const expectedSession = {
			session: UUID,
			state: 'ready',
			nextSince: 8,
			replies: ['Second'],
			compactedAt: 2,
		};

		assert.deepEqual(fixture.session(startOutput), expectedSession);
		assert.deepEqual(fixture.session(laterOutput), expectedSession);
		assert.equal(fixture.calls(), 0);
	});

	void test('reports Codex turns without a reply', async (context) => {
		const fixture = createFixture(context);
		const value = `0:${UUID}`;

		fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(1),
			turnComplete(null, 'é'.repeat(520), 2),
		]);
		assert.deepEqual(
			fixture.session(await fixture.observeWith(['--codex', value])),
			{
				session: UUID,
				state: 'attention',
				nextSince: 3,
				reason: 'noReply',
				detail: {
					abortReason: null,
					error: 'é'.repeat(500),
					errorOmitted: 20,
				},
			}
		);

		fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(1),
			reasoning(2),
			turnAborted('interrupted', 3),
		]);
		assert.deepEqual(
			fixture.session(await fixture.observeWith(['--codex', value]))
				.detail,
			{ abortReason: 'interrupted', error: null, errorOmitted: 0 }
		);
	});

	void test('reports Codex sessions as `working` before the turn starts', async (context) => {
		const fixture = createFixture(context);
		const value = `0:${UUID}`;

		assert.deepEqual(
			fixture.session(await fixture.observeWith(['--codex', value])),
			{ session: UUID, state: 'working', nextSince: 0, progressAt: null }
		);

		fixture.writeRollout(UUID, 9, [sessionMeta(UUID, UUID, 0)]);
		assert.deepEqual(
			fixture.session(await fixture.observeWith(['--codex', value])),
			{ session: UUID, state: 'working', nextSince: 1, progressAt: null }
		);
		assert.equal(fixture.calls(), 0);
	});

	void test('reports ambiguous Codex rollouts as errors', async (context) => {
		const fixture = createFixture(context);

		fixture.writeRollout(UUID, 9, [sessionMeta(UUID, UUID, 0)]);
		fixture.writeRollout(UUID, 10, [sessionMeta(UUID, UUID, 0)]);

		const output = fixture.session(
			await fixture.observeWith(['--codex', `0:${UUID}`])
		);

		assert.equal(output.state, 'error', JSON.stringify(output));
		assert.ok(
			String(output.message).includes('Found 2 rollouts'),
			JSON.stringify(output)
		);
	});

	void test('reports Codex progress from descendants started during the turn', async (context) => {
		const child = 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa';
		const earlier = 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb';
		const grandchild = 'cccccccc-cccc-cccc-cccc-cccccccccccc';
		const partial = 'dddddddd-dddd-dddd-dddd-dddddddddddd';
		const unrelated = 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee';
		const fixture = createFixture(context);
		const value = `0:${UUID}`;
		const progress = async (): Promise<unknown> =>
			fixture.session(await fixture.observeWith(['--codex', value]))
				.progressAt;
		const root = fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(10),
		]);

		setModified(root, 10_000);
		assert.equal(await progress(), 10_000);

		for (const [path, time] of [
			[
				fixture.writeRollout(child, 9, [sessionMeta(child, UUID, 11)]),
				20_000,
			],
			[
				fixture.writeRollout(grandchild, 10, [
					sessionMeta(grandchild, UUID, 12),
				]),
				30_000,
			],
			[
				fixture.writeRollout(earlier, 10, [
					sessionMeta(earlier, UUID, 5),
				]),
				90_000,
			],
			[
				fixture.writeRollout(unrelated, 10, [
					sessionMeta(unrelated, OTHER_UUID, 13),
				]),
				80_000,
			],
			[
				fixture.rollout(
					partial,
					10,
					JSON.stringify(sessionMeta(partial, UUID, 14))
				),
				70_000,
			],
		] as const) {
			setModified(path, time);
		}

		assert.deepEqual(
			fixture.session(await fixture.observeWith(['--codex', value])),
			{
				session: UUID,
				state: 'working',
				nextSince: 2,
				progressAt: 30_000,
			}
		);

		// Without the turn’s start time, every subagent rollout could predate
		// the turn
		const { timestamp: _, ...started } = turnStarted(10);

		fixture.writeRollout(UUID, 9, [sessionMeta(UUID, UUID, 0), started]);
		setModified(root, 10_000);
		assert.equal(await progress(), 10_000);
	});

	void test('scans the root and two newest date directories', async (context) => {
		const fixture = createFixture(context);
		const progress = async (): Promise<unknown> =>
			fixture.session(await fixture.observeWith(['--codex', `0:${UUID}`]))
				.progressAt;
		const root = fixture.writeRollout(UUID, 1, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(10),
		]);
		const [current, , recent] = (
			[
				['aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 1, 20_000],
				['cccccccc-cccc-cccc-cccc-cccccccccccc', 8, 90_000],
				['dddddddd-dddd-dddd-dddd-dddddddddddd', 9, 30_000],
				['bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 10, 40_000],
			] as const
		).map(([uuid, day, time]) => {
			const path = fixture.writeRollout(uuid, day, [
				sessionMeta(uuid, UUID, 11),
			]);

			setModified(path, time);

			return path;
		});

		assert.ok(current !== undefined && recent !== undefined);
		setModified(root, 10_000);

		// The descendant in the third-newest directory never counts, whichever
		// in-window write is latest
		assert.equal(await progress(), 40_000);
		setModified(current, 60_000);
		assert.equal(await progress(), 60_000);
		setModified(recent, 70_000);
		assert.equal(await progress(), 70_000);
	});

	void test('reports unreadable rollouts but ignores unidentified ones', async (context) => {
		const fixture = createFixture(context);
		const value = `0:${UUID}`;
		const root = fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(10),
		]);
		const unreadable = join(dirname(root), 'rollout-unreadable.jsonl');

		setModified(root, 10_000);
		writeFileSync(join(dirname(root), 'rollout-notes.jsonl'), 'not json\n');
		assert.equal(
			fixture.session(await fixture.observeWith(['--codex', value]))
				.progressAt,
			10_000
		);

		writeFileSync(
			unreadable,
			`${JSON.stringify(sessionMeta(OTHER_UUID, OTHER_UUID, 0))}\n`
		);
		chmodSync(unreadable, 0o000);

		try {
			const output = fixture.session(
				await fixture.observeWith(['--codex', value])
			);

			assert.equal(output.state, 'error', JSON.stringify(output));
		} finally {
			chmodSync(unreadable, 0o644);
		}
	});

	void test('applies a new turn start to known descendants', async (context) => {
		const child = 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa';
		const fixture = createFixture(context);
		const records = [sessionMeta(UUID, UUID, 0), turnStarted(10)];
		const root = fixture.writeRollout(UUID, 9, records);
		const childRollout = fixture.writeRollout(child, 9, [
			sessionMeta(child, UUID, 11),
		]);

		setModified(root, 10_000);
		setModified(childRollout, 20_000);
		fixture.write(OTHER_UUID, [prompt(1)]);
		fixture.listing([row(OTHER_UUID, 'working', 'busy', null)]);

		const observation = fixture.observeWith([
			'--claude',
			`0:${OTHER_UUID}`,
			'--codex',
			`0:${UUID}`,
			'--wait',
			'4',
		]);

		// The first check reads the rollout before its Claude lookup, so once
		// the lookup has run, that check caches the child as a descendant of
		// the turn started at second 10
		await fixture.waitForCalls(1);
		fixture.writeRollout(UUID, 9, [...records, turnStarted(30)]);
		setModified(root, 50_000);
		setModified(childRollout, Date.now() + 3_600_000);

		const output = await observation;

		assert.equal(
			output.sessions[1]?.progressAt,
			50_000,
			JSON.stringify(output)
		);
	});

	void test('reports a duplicate rollout that appears while waiting', async (context) => {
		const fixture = createFixture(context);

		fixture.writeRollout(UUID, 9, [
			sessionMeta(UUID, UUID, 0),
			turnStarted(1),
		]);

		const started = performance.now();
		const observation = fixture.observeWith([
			'--codex',
			`0:${UUID}`,
			'--wait',
			'30',
		]);

		await sleep(1000);
		fixture.writeRollout(UUID, 10, [sessionMeta(UUID, UUID, 0)]);

		const output = await observation;

		assert.ok(performance.now() - started < 20_000);
		assert.equal(
			output.sessions[0]?.state,
			'error',
			JSON.stringify(output)
		);
	});

	void test('observes Claude and Codex sessions together', async (context) => {
		const fixture = createFixture(context);
		const root = fixture.writeRollout(OTHER_UUID, 9, [
			sessionMeta(OTHER_UUID, OTHER_UUID, 0),
			turnStarted(1),
		]);

		setModified(root, 4000);
		fixture.write(UUID, completedTurn());
		fixture.listing(rows('done', null, null));

		const { sessions } = await fixture.observeWith([
			'--codex',
			`0:${OTHER_UUID}`,
			'--claude',
			`0:${UUID}`,
		]);

		assert.deepEqual(sessions, [
			{
				session: OTHER_UUID,
				state: 'working',
				nextSince: 2,
				progressAt: 4000,
			},
			{
				session: UUID,
				state: 'ready',
				nextSince: 8,
				replies: ['No findings'],
				compactedAt: null,
			},
		]);
		assert.equal(fixture.calls(), 1);
	});
});
