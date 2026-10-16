import { Buffer } from 'node:buffer';
import { spawnSync } from 'node:child_process';
import {
	closeSync,
	openSync,
	readdirSync,
	readSync,
	statSync,
	writeSync,
} from 'node:fs';
import { basename, dirname, extname, join } from 'node:path';
import process from 'node:process';
import { setTimeout as sleep } from 'node:timers/promises';

const DETAIL_LIMIT = 500;
const LISTING_INTERVAL_MS = 10_000;
const MAX_WAIT_SECONDS = 90;
const NAME = 'subagent-signal';

// The value options the parser accepts other than `--help`, which the help must
// document exactly
export const OPTIONS = ['--claude', '--codex', '--wait'] as const;

const POLL_INTERVAL_MS = 2000;
const SYNTHETIC_MODEL = '<synthetic>';

type JsonObject = Readonly<Record<string, unknown>>;

type Listing = {
	state: string | undefined;
	status: string | undefined;
	waitingFor: string | undefined;
};

type Message = {
	error: string | undefined;
	model: string | undefined;
	stopReason: string | undefined;
	text: string;
};

type Provider = 'claude' | 'codex';

type Session = {
	provider: Provider;
	since: number;
	uuid: string;
};

type Options = {
	sessions: Session[];
	waitMs: number;
};

type Route = { kind: 'help' } | { kind: 'observe'; options: Options };

type Signal =
	| { compactedAt: number | undefined; kind: 'ready'; replies: string[] }
	| { detail: JsonObject | undefined; kind: 'attention'; reason: string }
	| { kind: 'working'; progressAt: number | undefined };

type Observation =
	| { kind: 'error'; message: string }
	| { kind: 'observed'; nextSince: number; signal: Signal };

type Transcript = {
	anchor: number | undefined;
	path: string;
	records: JsonObject[];
};

type Turn = {
	closed: boolean;
	finalMessage: Message | undefined;
	pendingWork: boolean;
	replies: string[];
	unresolved: unknown[];
};

type Entry = {
	session: Session;
	starts: Map<string, string | undefined>;
};

type Check = Entry & {
	anchor: number | undefined;
	transcript: ReportableError | Transcript | undefined;
	turn: Turn | undefined;
};

// An expected failure that the command reports rather than a defect in the
// command itself
class ReportableError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'ReportableError';
	}
}

const help = (): string =>
	[
		'Usage:',
		'  node <skill-directory>/scripts/subagent-signal.ts [--wait <seconds>] (--claude <since>:<uuid> | --codex <since>:<uuid>)...',
		'  node <skill-directory>/scripts/subagent-signal.ts --help',
		'',
		'Report whether the latest requested turn of each listed Claude or Codex session is working, ready, or needs attention. Return as soon as any session leaves `working`. `<skill-directory>` is the directory containing the `cli-subagent` skill’s `SKILL.md`',
		'',
		'Options:',
		'  --claude <since>:<uuid>  A Claude session to observe, given once per session. `<since>` is the number of complete transcript lines recorded before the latest dispatch, or `0` for a fresh launch. Keep it fixed for every check of that turn. `<uuid>` selects the `~/.claude/projects/*/<uuid>.jsonl` transcript and the session’s `claude agents --json --all` row',
		'  --codex <since>:<uuid>   A Codex session to observe, given once per session. `<since>` works as for `--claude`. `<uuid>` selects the `~/.codex/sessions/*/*/*/rollout-*-<uuid>.jsonl` rollout',
		`  --wait <seconds>         Wait up to this many seconds, from \`0\` to \`${String(MAX_WAIT_SECONDS)}\`, for any session to become \`ready\`, need \`attention\`, or report an \`error\`. Defaults to \`${String(MAX_WAIT_SECONDS)}\`. A slow \`claude agents\` lookup can extend the call beyond this wait`,
		'',
		'Output:',
		'  One JSON object on standard output with `observedAt`, the observation time in Unix milliseconds, and `sessions`, one entry per session in the given order',
		'  Each entry has `session` and `state`. Every state except `error` adds `nextSince`, the `<since>` value for the next dispatch',
		'  `working` adds `progressAt`, the latest write to the session’s transcripts in Unix milliseconds, or `null` before the turn start is recorded',
		'  `ready` adds `replies`, each turn’s final reply since the turn start, and `compactedAt`, the latest compaction line or `null`',
		'  `attention` adds `reason`, one of `blocked`, `failed`, `noReply`, or `stopped`, and `detail` for `blocked` and `noReply`. Codex sessions report only `noReply`, whose `detail` has `abortReason`, `error`, and `errorOmitted`',
		'  `error` adds `message`, the reason the session could not be observed, such as a missing transcript when `<since>` is above `0`',
		'',
		'Exit statuses:',
		'  0  The sessions were observed, including any `error` entries, or help was displayed',
		'  2  Invalid arguments or a failed write to standard output',
		'',
	].join('\n');

const isObject = (value: unknown): value is JsonObject =>
	typeof value === 'object' && value !== null && !Array.isArray(value);

// Missing keys and values that are not objects read as `undefined`, so a lookup
// never throws
const field = (value: unknown, ...keys: string[]): unknown => {
	let current = value;

	for (const key of keys) {
		current = isObject(current) ? current[key] : undefined;
	}

	return current;
};

const stringValue = (value: unknown): string | undefined =>
	typeof value === 'string' ? value : undefined;

const compare = (left: string, right: string): number => {
	if (left === right) {
		return 0;
	}

	return left < right ? -1 : 1;
};

const errorCode = (error: unknown): unknown =>
	error instanceof Error && 'code' in error ? error.code : undefined;

const errorMessage = (error: unknown): string =>
	error instanceof Error ? error.message : String(error);

// The upstream error follows its context unchanged, after one blank line
const readFailure = (path: string, error: unknown): ReportableError =>
	new ReportableError(
		`Failed to read \`${path}\`:\n\n${errorMessage(error)}`
	);

const capture = <T>(operation: () => T): ReportableError | T => {
	try {
		return operation();
	} catch (error) {
		if (error instanceof ReportableError) {
			return error;
		}

		throw error;
	}
};

// Digits beyond the safe integer range still parse, so each caller reports its
// own upper bound
const count = (value: string, name: string): number => {
	if (!/^[0-9]+$/.test(value)) {
		throw new ReportableError(`\`${name}\` must be a non-negative integer`);
	}

	return Number(value);
};

const isUuid = (value: string): boolean =>
	/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(
		value
	);

const parseSession = (
	value: string,
	provider: Provider,
	option: string
): Session => {
	const separator = value.indexOf(':');
	const uuid = value.slice(separator + 1);

	if (separator === -1 || !isUuid(uuid)) {
		throw new ReportableError(
			`\`${option}\` must be \`<since>:<uuid>\` with a full session UUID`
		);
	}

	const since = count(value.slice(0, separator), '<since>');

	if (since > Number.MAX_SAFE_INTEGER) {
		throw new ReportableError(
			`\`<since>\` must be at most ${String(Number.MAX_SAFE_INTEGER)}`
		);
	}

	return { provider, since, uuid: uuid.toLowerCase() };
};

const parseArguments = (arguments_: ReadonlyArray<string>): Route => {
	if (arguments_.length === 1 && arguments_[0] === '--help') {
		return { kind: 'help' };
	}

	const sessions: Session[] = [];
	let wait: number | undefined;
	const remaining = arguments_.values();

	for (const argument of remaining) {
		if (argument === '--help') {
			throw new ReportableError('`--help` must be the only argument');
		}

		const option = OPTIONS.find((candidate) => candidate === argument);

		if (option === undefined) {
			throw new ReportableError(`Unknown argument \`${argument}\``);
		}

		const next = remaining.next();

		if (next.done === true) {
			throw new ReportableError(`\`${option}\` requires a value`);
		}

		if (option === '--wait') {
			const seconds = count(next.value, '--wait');

			if (wait !== undefined) {
				throw new ReportableError('`--wait` must be given only once');
			}

			wait = seconds;
			continue;
		}

		const session = parseSession(
			next.value,
			option === '--claude' ? 'claude' : 'codex',
			option
		);

		if (sessions.some((existing) => existing.uuid === session.uuid)) {
			throw new ReportableError(
				`Session \`${session.uuid}\` must be given only once`
			);
		}

		sessions.push(session);
	}

	if (sessions.length === 0) {
		throw new ReportableError(
			'At least one `--claude` or `--codex` session is required'
		);
	}

	const seconds = wait ?? MAX_WAIT_SECONDS;

	if (seconds > MAX_WAIT_SECONDS) {
		throw new ReportableError(
			`\`--wait\` must be at most ${String(MAX_WAIT_SECONDS)}`
		);
	}

	return { kind: 'observe', options: { sessions, waitMs: seconds * 1000 } };
};

const homeDirectory = (): ReportableError | string => {
	const home = process.env.HOME;

	return home === undefined || home === ''
		? new ReportableError('`HOME` is not set')
		: home;
};

// Only a missing path reads as absent. Any other failure surfaces when the path
// is read
const isMissing = (path: string): boolean => {
	try {
		statSync(path);
		return false;
	} catch (error) {
		return errorCode(error) === 'ENOENT';
	}
};

const modifiedAt = (path: string): number => {
	const { mtimeNs } = statSync(path, { bigint: true });

	return Math.max(0, Number(mtimeNs / 1_000_000n));
};

const modified = (path: string): number => {
	try {
		return modifiedAt(path);
	} catch (error) {
		throw readFailure(path, error);
	}
};

// Returns `undefined` for a file removed after its directory was listed, or a
// dangling link
const modifiedIfPresent = (path: string): number | undefined => {
	try {
		return modifiedAt(path);
	} catch (error) {
		if (errorCode(error) === 'ENOENT') {
			return undefined;
		}

		throw readFailure(path, error);
	}
};

const childDirectories = (directory: string): string[] => {
	let entries;

	try {
		entries = readdirSync(directory, { withFileTypes: true });
	} catch (error) {
		if (errorCode(error) === 'ENOENT') {
			return [];
		}

		throw readFailure(directory, error);
	}

	return entries
		.filter((entry) => entry.isDirectory())
		.map((entry) => join(directory, entry.name))
		.toSorted(compare);
};

// Codex files each rollout under the `YYYY/MM/DD` directory of its local
// creation date
const dateDirectories = (sessions: string): string[] =>
	childDirectories(sessions).flatMap((year) =>
		childDirectories(year).flatMap((month) => childDirectories(month))
	);

// Sorted paths keep the result, and the first reported failure, the same for
// unchanged files
const sortedEntries = (directory: string): string[] => {
	try {
		return readdirSync(directory)
			.map((name) => join(directory, name))
			.toSorted(compare);
	} catch (error) {
		throw readFailure(directory, error);
	}
};

const isRollout = (path: string, suffix: string): boolean => {
	const name = basename(path);

	return name.startsWith('rollout-') && name.endsWith(suffix);
};

const rollouts = (sessions: string, suffix: string): string[] =>
	dateDirectories(sessions).flatMap((day) =>
		sortedEntries(day).filter((path) => isRollout(path, suffix))
	);

const findRollout = (sessions: string, uuid: string): string | undefined => {
	const matches = rollouts(sessions, `-${uuid}.jsonl`);

	if (matches.length > 1) {
		throw new ReportableError(
			`Found ${String(matches.length)} rollouts for the session`
		);
	}

	return matches[0];
};

const findTranscript = (projects: string, uuid: string): string | undefined => {
	if (isMissing(projects)) {
		return undefined;
	}

	const matches: string[] = [];

	for (const project of sortedEntries(projects)) {
		const candidate = join(project, `${uuid}.jsonl`);

		try {
			if (statSync(candidate).isFile()) {
				matches.push(candidate);
			}
		} catch (error) {
			// A plain file such as `.DS_Store` cannot hold a project’s
			// transcripts
			if (
				errorCode(error) !== 'ENOENT' &&
				errorCode(error) !== 'ENOTDIR'
			) {
				throw readFailure(candidate, error);
			}
		}
	}

	if (matches.length > 1) {
		throw new ReportableError(
			`Found ${String(matches.length)} transcripts for the session`
		);
	}

	return matches[0];
};

const parseRecord = (line: string): unknown => {
	try {
		return JSON.parse(line) as unknown;
	} catch {
		return undefined;
	}
};

// Yields each newline-terminated line without its newline. Reading in chunks
// keeps whole-file and string length limits from capping a transcript. An
// unterminated final line may still be in progress, so a later check reads it
function* completeLines(descriptor: number): Generator<Buffer, void> {
	const buffer = Buffer.allocUnsafe(1_048_576);
	let pending: Buffer[] = [];

	while (true) {
		const length = readSync(descriptor, buffer, 0, buffer.length, null);

		if (length === 0) {
			return;
		}

		let chunk = buffer.subarray(0, length);

		for (
			let end = chunk.indexOf('\n');
			end !== -1;
			end = chunk.indexOf('\n')
		) {
			yield Buffer.concat([...pending, chunk.subarray(0, end)]);
			pending = [];
			chunk = chunk.subarray(end + 1);
		}

		if (chunk.length > 0) {
			pending.push(Buffer.from(chunk));
		}
	}
}

const readRecords = (path: string): JsonObject[] => {
	let descriptor;

	try {
		descriptor = openSync(path, 'r');
	} catch (error) {
		throw readFailure(path, error);
	}

	try {
		const records: JsonObject[] = [];

		for (const line of completeLines(descriptor)) {
			const record = parseRecord(line.toString('utf8'));

			if (!isObject(record)) {
				throw new ReportableError(
					`Malformed JSON record at line ${String(records.length + 1)} of \`${path}\``
				);
			}

			records.push(record);
		}

		return records;
	} catch (error) {
		if (error instanceof ReportableError) {
			throw error;
		}

		throw readFailure(path, error);
	} finally {
		closeSync(descriptor);
	}
};

const subagentTranscripts = (transcript: string): string[] => {
	const directory = join(
		transcript.slice(0, transcript.length - extname(transcript).length),
		'subagents'
	);

	if (isMissing(directory)) {
		return [];
	}

	return sortedEntries(directory).filter(
		(path) => extname(path) === '.jsonl'
	);
};

// Any write by the session or its subagents counts as progress. Older subagent
// transcripts were last written before this turn’s prompt, so they never move
// the result
const latestWrite = (transcript: string): number => {
	let latest = modified(transcript);

	for (const path of subagentTranscripts(transcript)) {
		latest = Math.max(latest, modifiedIfPresent(path) ?? latest);
	}

	return latest;
};

const blocks = (record: unknown): unknown[] => {
	const content = field(record, 'message', 'content');

	return Array.isArray(content) ? content : [];
};

const isSidechain = (record: JsonObject): boolean =>
	record.isSidechain === true;

// Compaction summaries and command output are user records without a human
// origin
const isPrompt = (record: JsonObject): boolean =>
	record.type === 'user' &&
	record.isMeta !== true &&
	!isSidechain(record) &&
	field(record, 'origin', 'kind') === 'human';

const notifiedToolUses = (content: string): string[] =>
	content
		.split('<tool-use-id>')
		.slice(1)
		.flatMap((tail) => {
			const end = tail.indexOf('</tool-use-id>');

			return end === -1 ? [] : [tail.slice(0, end)];
		});

const isReply = (message: Message): boolean =>
	message.model !== SYNTHETIC_MODEL &&
	message.stopReason === 'end_turn' &&
	message.text.trim() !== '';

const analyzeTurn = (records: ReadonlyArray<JsonObject>): Turn => {
	const finals: Array<string | undefined> = [];
	let lastActivity: number | undefined;
	let lastDuration: { index: number; record: JsonObject } | undefined;
	let lastNotification: number | undefined;
	const messages = new Map<string, Message>();
	const notified = new Set<string>();
	const results = new Map<string, boolean>();
	let segmentMessage: string | undefined;
	const toolUses: unknown[] = [];

	for (const [index, record] of records.entries()) {
		if (isSidechain(record)) {
			continue;
		}

		switch (record.type) {
			case 'assistant': {
				const key =
					stringValue(field(record, 'message', 'id')) ??
					`record ${String(index)}`;
				const message = messages.get(key) ?? {
					error: undefined,
					model: undefined,
					stopReason: undefined,
					text: '',
				};

				messages.set(key, message);
				message.model =
					stringValue(field(record, 'message', 'model')) ??
					message.model;
				message.stopReason =
					stringValue(field(record, 'message', 'stop_reason')) ??
					message.stopReason;
				message.error = stringValue(record.error) ?? message.error;

				for (const block of blocks(record)) {
					const type = field(block, 'type');
					const text = stringValue(field(block, 'text'));

					if (type === 'text' && text !== undefined) {
						message.text +=
							message.text === '' ? text : `\n\n${text}`;
					} else if (type === 'tool_use') {
						toolUses.push(block);
					}
				}

				lastActivity = index;
				segmentMessage = key;
				break;
			}

			case 'queue-operation': {
				const content = stringValue(record.content);

				if (content !== undefined) {
					const ids = notifiedToolUses(content);

					if (record.operation === 'enqueue' && ids.length > 0) {
						lastNotification = index;
					}

					for (const id of ids) {
						notified.add(id);
					}
				}

				break;
			}

			case 'system':
				if (record.subtype === 'turn_duration') {
					finals.push(segmentMessage);
					segmentMessage = undefined;
					lastDuration = { index, record };
				}

				break;

			case 'user':
				for (const block of blocks(record)) {
					const id = stringValue(field(block, 'tool_use_id'));

					if (
						field(block, 'type') === 'tool_result' &&
						id !== undefined
					) {
						results.set(id, field(block, 'is_error') === true);
						lastActivity = index;
					}
				}

				break;

			default:
				break;
		}
	}

	const closed =
		lastDuration !== undefined &&
		(lastActivity === undefined || lastDuration.index > lastActivity);
	const unresolved = toolUses.filter((block) => {
		const id = stringValue(field(block, 'id'));

		return id === undefined || !results.has(id);
	});
	const backgroundPending = toolUses.some((block) => {
		const id = stringValue(field(block, 'id'));

		return (
			field(block, 'input', 'run_in_background') === true &&
			id !== undefined &&
			results.get(id) === false &&
			!notified.has(id)
		);
	});
	const durationPending =
		lastDuration !== undefined &&
		['pendingBackgroundAgentCount', 'pendingWorkflowCount'].some((name) => {
			const pending = lastDuration.record[name];

			return Number.isInteger(pending) && Number(pending) > 0;
		});
	// Claude answers a notification that arrives after its turn ended with a
	// new turn
	const notificationPending =
		lastNotification !== undefined &&
		(lastDuration === undefined || lastDuration.index < lastNotification);
	const finalKey = finals.at(-1);

	return {
		closed,
		finalMessage:
			finalKey === undefined ? undefined : messages.get(finalKey),
		pendingWork:
			backgroundPending || durationPending || notificationPending,
		replies: finals.flatMap((key) => {
			const message = key === undefined ? undefined : messages.get(key);

			return message !== undefined && isReply(message)
				? [message.text]
				: [];
		}),
		unresolved,
	};
};

const readListing = (): unknown[] => {
	const result = spawnSync('claude', ['agents', '--json', '--all'], {
		encoding: 'utf8',
		maxBuffer: 2 ** 30,
		stdio: ['ignore', 'pipe', 'pipe'],
	});

	if (result.error !== undefined) {
		throw new ReportableError(
			`Failed to run \`claude agents\`:\n\n${result.error.message}`
		);
	}

	if (result.status !== 0) {
		const outcome =
			result.status === null
				? `signal: ${String(result.signal)}`
				: `exit status: ${String(result.status)}`;

		throw new ReportableError(`\`claude agents\` failed with ${outcome}`);
	}

	let rows: unknown;

	try {
		rows = JSON.parse(result.stdout) as unknown;
	} catch {
		throw new ReportableError('`claude agents` printed invalid JSON');
	}

	if (!Array.isArray(rows)) {
		throw new ReportableError('`claude agents` did not print a JSON array');
	}

	return rows;
};

const listingFor = (rows: ReadonlyArray<unknown>, uuid: string): Listing => {
	const matches = rows.filter((row) => field(row, 'sessionId') === uuid);
	const [row] = matches;

	if (matches.length !== 1) {
		throw new ReportableError(
			`Expected one \`claude agents\` row for the session, found ${String(matches.length)}`
		);
	}

	const value = (name: string): string | undefined => {
		const text = stringValue(field(row, name));

		return text === '' ? undefined : text;
	};

	return {
		state: value('state'),
		status: value('status'),
		waitingFor: value('waitingFor'),
	};
};

// Counts Unicode code points, so a limit never splits a character
const truncate = (text: string): [string, number] => {
	let kept = '';
	let total = 0;

	for (const character of text) {
		if (total < DETAIL_LIMIT) {
			kept += character;
		}

		total += 1;
	}

	return [kept, Math.max(0, total - DETAIL_LIMIT)];
};

const blocked = (tool: unknown, listing: Listing): Signal => {
	let detailTool: JsonObject | undefined;

	if (tool !== undefined) {
		const [input, inputOmitted] = truncate(
			JSON.stringify(field(tool, 'input') ?? null)
		);

		detailTool = { input, inputOmitted, name: field(tool, 'name') ?? null };
	}

	return {
		detail: {
			tool: detailTool ?? null,
			waitingFor: listing.waitingFor ?? null,
		},
		kind: 'attention',
		reason: 'blocked',
	};
};

const noReply = (message: Message | undefined): Signal => {
	const [text, textOmitted] = truncate(message?.text ?? '');

	return {
		detail: {
			error: message?.error ?? null,
			model: message?.model ?? null,
			stopReason: message?.stopReason ?? null,
			text,
			textOmitted,
		},
		kind: 'attention',
		reason: 'noReply',
	};
};

// Returns `undefined` while the turn is still working
const decide = (
	turn: Turn,
	listing: Listing,
	compactedAt: number | undefined
): Signal | undefined => {
	// Only an open prompt counts as blocked. A `blocked` session state alone
	// can mean that a reply asks a question, which the coordinator reads in the
	// reply itself
	const prompting =
		listing.status === 'waiting' || listing.waitingFor !== undefined;
	const tool = turn.unresolved.at(-1);

	if (turn.closed) {
		const message = turn.finalMessage;

		if (message === undefined || tool !== undefined || !isReply(message)) {
			return noReply(message);
		}

		// Pending background work keeps a replied turn unfinished
		if (!turn.pendingWork) {
			if (prompting) {
				return blocked(undefined, listing);
			}

			return listing.status === 'busy'
				? undefined
				: { compactedAt, kind: 'ready', replies: turn.replies };
		}
	}

	if (prompting) {
		return blocked(tool, listing);
	}

	return listing.state === 'failed' || listing.state === 'stopped'
		? { detail: undefined, kind: 'attention', reason: listing.state }
		: undefined;
};

// Returns the one-based line of the latest matching record
const latestLine = (
	records: ReadonlyArray<JsonObject>,
	matches: (record: JsonObject) => boolean
): number | undefined => {
	const index = records.findLastIndex((record) => matches(record));

	return index === -1 ? undefined : index + 1;
};

const compactedAt = (records: ReadonlyArray<JsonObject>): number | undefined =>
	latestLine(
		records,
		(record) =>
			record.type === 'system' && record.subtype === 'compact_boundary'
	);

const isTurnStart = (record: JsonObject): boolean =>
	record.type === 'event_msg' &&
	field(record, 'payload', 'type') === 'task_started';

const isTurnEnd = (record: JsonObject): boolean => {
	const type = field(record, 'payload', 'type');

	return (
		record.type === 'event_msg' &&
		(type === 'task_complete' || type === 'turn_aborted')
	);
};

const codexReply = (record: JsonObject): string | undefined => {
	const text = stringValue(field(record, 'payload', 'last_agent_message'));

	return field(record, 'payload', 'type') === 'task_complete' &&
		text !== undefined &&
		text.trim() !== ''
		? text
		: undefined;
};

// Returns the first record once its line is complete, so a file still being
// created or removed is skipped until a later check. A complete line that is
// not JSON identifies no session
const firstRecord = (path: string): { record: unknown } | undefined => {
	let descriptor;

	try {
		descriptor = openSync(path, 'r');
	} catch (error) {
		if (errorCode(error) === 'ENOENT') {
			return undefined;
		}

		throw readFailure(path, error);
	}

	try {
		const { done, value } = completeLines(descriptor).next();

		return done === true
			? undefined
			: { record: parseRecord(value.toString('utf8')) };
	} catch (error) {
		throw readFailure(path, error);
	} finally {
		closeSync(descriptor);
	}
};

// Descendants at any depth carry the root thread’s `session_id`, and only those
// started during this turn count. A turn shorter than a day spans at most the
// two newest date directories besides the root’s own. Each file’s start time is
// cached, so a newer turn start still filters it
const latestCodexWrite = (
	root: string,
	uuid: string,
	start: string,
	sessions: string,
	starts: Map<string, string | undefined>
): number => {
	let latest = modified(root);
	const directories = new Set([
		...dateDirectories(sessions).slice(-2),
		dirname(root),
	]);

	for (const directory of [...directories].toSorted(compare)) {
		for (const path of sortedEntries(directory)) {
			if (path === root || !isRollout(path, '.jsonl')) {
				continue;
			}

			if (!starts.has(path)) {
				const first = firstRecord(path);

				if (first === undefined) {
					continue;
				}

				const { record } = first;

				starts.set(
					path,
					field(record, 'type') === 'session_meta' &&
						field(record, 'payload', 'session_id') === uuid &&
						field(record, 'payload', 'id') !== uuid
						? stringValue(field(record, 'timestamp'))
						: undefined
				);
			}

			const started = starts.get(path);

			if (started !== undefined && started >= start) {
				latest = Math.max(latest, modifiedIfPresent(path) ?? latest);
			}
		}
	}

	return latest;
};

const observeCodex = (
	transcript: Transcript,
	anchor: number,
	uuid: string,
	sessions: string,
	starts: Map<string, string | undefined>
): Signal => {
	const { path, records } = transcript;
	const after = records.slice(anchor + 1);
	const end = after.findLast((record) => isTurnEnd(record));

	if (end !== undefined) {
		if (codexReply(end) !== undefined) {
			return {
				compactedAt: latestLine(
					records,
					(record) => record.type === 'compacted'
				),
				kind: 'ready',
				replies: after.flatMap((record) => {
					const reply = codexReply(record);

					return reply === undefined ? [] : [reply];
				}),
			};
		}

		const message = stringValue(field(end, 'payload', 'error', 'message'));
		const [error, errorOmitted] =
			message === undefined ? [undefined, 0] : truncate(message);

		return {
			detail: {
				abortReason:
					stringValue(field(end, 'payload', 'reason')) ?? null,
				error: error ?? null,
				errorOmitted,
			},
			kind: 'attention',
			reason: 'noReply',
		};
	}

	const start = stringValue(field(records[anchor], 'timestamp'));

	return {
		kind: 'working',
		// Without the turn’s start time, no subagent rollout can be attributed
		// to this turn
		progressAt:
			start === undefined
				? modified(path)
				: latestCodexWrite(path, uuid, start, sessions, starts),
	};
};

// Every check finds the transcript again, so a later duplicate or a removed
// file is reported
const readTranscript = (
	session: Session,
	home: ReportableError | string,
	sessions: string
): Transcript | undefined => {
	if (home instanceof ReportableError) {
		throw home;
	}

	const path =
		session.provider === 'claude'
			? findTranscript(join(home, '.claude/projects'), session.uuid)
			: findRollout(sessions, session.uuid);

	if (path === undefined) {
		// Only a fresh launch may precede its transcript, because a follow-up
		// appends to it. Codex compresses a rollout that stays idle for a week,
		// and a resumption restores the plain file before appending, so a
		// compressed-only rollout is still pending
		if (
			session.since === 0 ||
			(session.provider === 'codex' &&
				rollouts(sessions, `-${session.uuid}.jsonl.zst`).length > 0)
		) {
			return undefined;
		}

		throw new ReportableError(
			'Found no transcript, but `<since>` is above 0'
		);
	}

	const records = readRecords(path);

	if (session.since > records.length) {
		throw new ReportableError(
			`\`<since>\` exceeds the transcript’s ${String(records.length)} complete lines`
		);
	}

	const isAnchor = session.provider === 'claude' ? isPrompt : isTurnStart;
	const anchor = records.findLastIndex(
		(record, index) => index >= session.since && isAnchor(record)
	);

	return { anchor: anchor === -1 ? undefined : anchor, path, records };
};

const check = (
	{ session, starts }: Entry,
	home: ReportableError | string,
	codexSessions: string
): Check => {
	const transcript = capture(() =>
		readTranscript(session, home, codexSessions)
	);
	const loaded =
		transcript instanceof ReportableError ? undefined : transcript;
	const anchor = session.provider === 'claude' ? loaded?.anchor : undefined;

	return {
		anchor,
		session,
		starts,
		transcript,
		turn:
			loaded === undefined || anchor === undefined
				? undefined
				: analyzeTurn(loaded.records.slice(anchor + 1)),
	};
};

// One lookup serves every session. A closed turn without pending work needs
// fresh session state to become ready, and a newly recorded prompt needs state
// from after that prompt
const needsLookup = (
	checks: ReadonlyArray<Check>,
	lookedUp: ReadonlyArray<number | undefined>,
	checkedAt: number | undefined
): boolean =>
	checks.some((current) => current.turn !== undefined) &&
	(checks.some((current, index) => current.anchor !== lookedUp[index]) ||
		checkedAt === undefined ||
		performance.now() - checkedAt >= LISTING_INTERVAL_MS ||
		checks.some(
			(current) =>
				current.turn !== undefined &&
				current.turn.closed &&
				!current.turn.pendingWork
		));

const observeTurn = (
	transcript: Transcript,
	turn: Turn,
	rows: ReportableError | unknown[],
	uuid: string
): Signal => {
	if (rows instanceof ReportableError) {
		throw rows;
	}

	return (
		decide(
			turn,
			listingFor(rows, uuid),
			compactedAt(transcript.records)
		) ?? {
			kind: 'working',
			progressAt: latestWrite(transcript.path),
		}
	);
};

const observeCheck = (
	{ session, starts, transcript, turn }: Check,
	rows: ReportableError | unknown[],
	codexSessions: string
): Observation => {
	if (transcript instanceof ReportableError) {
		return { kind: 'error', message: transcript.message };
	}

	if (transcript === undefined) {
		return {
			kind: 'observed',
			nextSince: session.since,
			signal: { kind: 'working', progressAt: undefined },
		};
	}

	const signal = capture((): Signal => {
		if (session.provider === 'claude' && turn !== undefined) {
			return observeTurn(transcript, turn, rows, session.uuid);
		}

		if (session.provider === 'codex' && transcript.anchor !== undefined) {
			return observeCodex(
				transcript,
				transcript.anchor,
				session.uuid,
				codexSessions,
				starts
			);
		}

		return { kind: 'working', progressAt: undefined };
	});

	return signal instanceof ReportableError
		? { kind: 'error', message: signal.message }
		: { kind: 'observed', nextSince: transcript.records.length, signal };
};

const isWorking = (observation: Observation): boolean =>
	observation.kind === 'observed' && observation.signal.kind === 'working';

const render = (uuid: string, observation: Observation): JsonObject => {
	if (observation.kind === 'error') {
		return { session: uuid, state: 'error', message: observation.message };
	}

	const { nextSince, signal } = observation;

	switch (signal.kind) {
		case 'attention':
			return {
				session: uuid,
				state: 'attention',
				nextSince,
				reason: signal.reason,
				...(signal.detail === undefined
					? {}
					: { detail: signal.detail }),
			};

		case 'ready':
			return {
				session: uuid,
				state: 'ready',
				nextSince,
				replies: signal.replies,
				compactedAt: signal.compactedAt ?? null,
			};

		case 'working':
			return {
				session: uuid,
				state: 'working',
				nextSince,
				progressAt: signal.progressAt ?? null,
			};
	}
};

const observe = async ({ sessions, waitMs }: Options): Promise<string> => {
	const deadline = performance.now() + waitMs;
	const home = homeDirectory();
	const codexSessions =
		home instanceof ReportableError ? '' : join(home, '.codex/sessions');
	const entries: Entry[] = sessions.map((session) => ({
		session,
		starts: new Map<string, string | undefined>(),
	}));
	let checkedAt: number | undefined;
	let lookedUp: Array<number | undefined> = sessions.map(() => undefined);
	// The first poll with a turn always looks the sessions up, so no turn sees
	// this empty value
	let rows: ReportableError | unknown[] = [];

	while (true) {
		const checks = entries.map((entry) =>
			check(entry, home, codexSessions)
		);

		if (needsLookup(checks, lookedUp, checkedAt)) {
			rows = capture(readListing);
			checkedAt = performance.now();
			lookedUp = checks.map((current) => current.anchor);
		}

		const currentRows = rows;
		const observations = checks.map((current) => ({
			observation: observeCheck(current, currentRows, codexSessions),
			uuid: current.session.uuid,
		}));
		const now = performance.now();

		if (
			!observations.every(({ observation }) => isWorking(observation)) ||
			now >= deadline
		) {
			return `${JSON.stringify({
				observedAt: Date.now(),
				sessions: observations.map(({ observation, uuid }) =>
					render(uuid, observation)
				),
			})}\n`;
		}

		await sleep(Math.min(POLL_INTERVAL_MS, deadline - now));
	}
};

// A caller may hand over a non-blocking descriptor, so a full pipe is retried
// rather than failed
const writeAll = (descriptor: number, text: string): void => {
	const buffer = Buffer.from(text);
	let offset = 0;

	while (offset < buffer.length) {
		try {
			offset += writeSync(descriptor, buffer, offset);
		} catch (error) {
			if (errorCode(error) !== 'EAGAIN') {
				throw error;
			}

			Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 1);
		}
	}
};

// The failure status still reports the error when standard error rejects the
// diagnostic
const fail = (diagnostic: string): number => {
	try {
		writeAll(2, `${NAME}: ${diagnostic}\n`);
	} catch {
		// Nothing else can report the failure
	}

	return 2;
};

const main = async (arguments_: ReadonlyArray<string>): Promise<number> => {
	let output;

	try {
		const route = parseArguments(arguments_);

		output = route.kind === 'help' ? help() : await observe(route.options);
	} catch (error) {
		if (error instanceof ReportableError) {
			return fail(error.message);
		}

		throw error;
	}

	try {
		writeAll(1, output);
	} catch (error) {
		return fail(
			`Failed to write to standard output:\n\n${errorMessage(error)}`
		);
	}

	return 0;
};

if (import.meta.main) {
	process.exitCode = await main(process.argv.slice(2));
}
