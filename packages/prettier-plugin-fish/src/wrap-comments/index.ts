import type { ParserOptions } from 'prettier';
import { util } from 'prettier';
import { exec } from 'tinyexec';

type BlockOptions = WrapOptions & {
	indentation: string;
};

type WrapOptions = Pick<ParserOptions, 'printWidth' | 'tabWidth'>;

// An opening backtick fence can’t hold a backtick after its marker
const FENCE_PATTERN = /^(?:`{3,}(?!.*`)|~{3,})/;
const IGNORE_MARKER_PATTERN = /^#+[ \t]*prettier-ignore[ \t]*$/;
const LIST_MARKER_PATTERN = /^([*+-]|\d+[).])(?:[ \t]+|$)/;

/**
 * Wraps comments on their own lines using the print width as a target.
 */
export default async function wrapComments(
	text: string,
	options: WrapOptions
): Promise<string> {
	const lines = text.split('\n');

	if (
		!lines.some(
			(line, index) => /^[ \t]*#/.test(line) && !isShebang(line, index)
		)
	) {
		return text;
	}

	const commentColumns = await findCommentColumns(text);
	const isFullLineComment = (index: number) =>
		!isShebang(lines[index]!, index) &&
		commentColumns.get(index) === getIndentation(lines[index]!).length;
	const outputLines: string[] = [];
	let index = 0;

	while (index < lines.length) {
		const line = lines[index]!;

		if (!isFullLineComment(index)) {
			outputLines.push(line);
			index += 1;
			continue;
		}

		const indentation = getIndentation(line);
		const block: string[] = [];

		while (
			index < lines.length &&
			isFullLineComment(index) &&
			getIndentation(lines[index]!) === indentation
		) {
			block.push(lines[index]!.slice(indentation.length));
			index += 1;
		}

		for (const commentLine of formatCommentBlock(block, {
			...options,
			indentation,
		})) {
			outputLines.push(indentation + commentLine);
		}
	}

	return outputLines.join('\n');
}

/**
 * Maps each line index to the UTF-16 column of its first comment. Fish’s
 * tokenizer skips `#` inside quoted strings. Returns an empty map if the token
 * offsets disagree with the source length in code points.
 */
async function findCommentColumns(text: string): Promise<Map<number, number>> {
	const { stdout } = await exec('fish_indent', ['--pygments'], {
		nodePath: false,
		stdin: text,
		throwOnError: true,
	});

	const commentOffsets = new Set<number>();
	let tokensEnd = 0;

	// Fish reports each token as `start,end,role`
	for (const token of stdout.split('\n')) {
		const [start, end, role] = token.split(',');

		if (role === 'comment') {
			commentOffsets.add(Number(start));
		}

		tokensEnd = Math.max(tokensEnd, Number(end ?? 0));
	}

	const commentColumns = new Map<number, number>();
	let line = 0;
	let column = 0;
	let offset = 0;

	for (const character of text) {
		if (commentOffsets.has(offset) && !commentColumns.has(line)) {
			commentColumns.set(line, column);
		}

		if (character === '\n') {
			line += 1;
			column = 0;
		} else {
			column += character.length;
		}

		offset += 1;
	}

	// Fish counts some reserved code points as several characters. A mismatch
	// leaves us unable to tell which comment offsets have shifted.
	if (offset !== tokensEnd) {
		return new Map();
	}

	return commentColumns;
}

/**
 * Formats the lines of one comment block, given without their indentation.
 * Preserves the block if a line starts with `#!` or if `prettier-ignore` is the
 * only text after a line’s `#` characters, allowing spaces or tabs before and
 * after the marker. Also preserves the block if the rest of a line after its
 * `#` characters and one optional space starts with a code fence, a list
 * marker, or a space or tab followed by more text.
 */
function formatCommentBlock(
	lines: ReadonlyArray<string>,
	options: BlockOptions
): string[] {
	const paragraphs: string[][] = [];
	let paragraph: string[] | undefined;

	for (const line of lines) {
		const rest = line.replace(/^#+/, '');
		const indentation = getIndentation(rest);
		const text = rest.slice(indentation.length);

		// An empty comment line ends the paragraph above it
		if (text === '') {
			paragraph = undefined;
			continue;
		}

		// Preserve blocks with a `#!` line in case they hold license text. Also
		// preserve blocks marked with `prettier-ignore` or containing markup
		if (
			line.startsWith('#!') ||
			IGNORE_MARKER_PATTERN.test(line) ||
			indentation.length > 1 ||
			indentation === '\t' ||
			FENCE_PATTERN.test(text) ||
			LIST_MARKER_PATTERN.test(text)
		) {
			return [...lines];
		}

		if (paragraph) {
			paragraph.push(text);
		} else {
			paragraph = [text];
			paragraphs.push(paragraph);
		}
	}

	// Removing every line can leave a stray blank line that a later formatting
	// pass collapses
	if (paragraphs.length === 0) {
		return ['#'];
	}

	const formattedLines: string[] = [];

	for (const [paragraphIndex, texts] of paragraphs.entries()) {
		if (paragraphIndex > 0) {
			formattedLines.push('#');
		}

		// Join lines first so a code span can extend across a line break
		for (const line of fillWords(splitWords(texts.join(' ')), options)) {
			formattedLines.push(line);
		}
	}

	return formattedLines;
}

/**
 * Fills lines greedily. May exceed the print width to avoid starting a line
 * with a fence marker, list marker, or `prettier-ignore`, or ending a line that
 * would read as a fence opener or a standalone `prettier-ignore` marker.
 */
function fillWords(
	words: ReadonlyArray<string>,
	{ indentation, printWidth, tabWidth }: BlockOptions
): string[] {
	const fits = (line: string) =>
		getLineWidth(indentation + line, tabWidth) <= printWidth;
	const lines: string[] = [];
	let line = `# ${words[0]!}`;

	for (const word of words.values().drop(1)) {
		const extendedLine = `${line} ${word}`;

		// Check both sides of the break. Moving a closing backtick run to the
		// next line can make the current line read as a fence opener.
		if (
			fits(extendedLine) ||
			IGNORE_MARKER_PATTERN.test(line) ||
			FENCE_PATTERN.test(line.slice(2)) ||
			FENCE_PATTERN.test(word) ||
			LIST_MARKER_PATTERN.test(word) ||
			word === 'prettier-ignore'
		) {
			line = extendedLine;
		} else {
			lines.push(line);
			line = `# ${word}`;
		}
	}

	lines.push(line);
	return lines;
}

/**
 * Returns the spaces and tabs at the start of a line.
 */
function getIndentation(line: string): string {
	return /^[ \t]*/.exec(line)![0];
}

/**
 * Measures a line in columns. Tabs advance to the next tab stop unless
 * `tabWidth` is zero, in which case they add no columns.
 */
function getLineWidth(line: string, tabWidth: number): number {
	let width = 0;

	for (const [index, segment] of line.split('\t').entries()) {
		if (index > 0 && tabWidth > 0) {
			width += tabWidth - (width % tabWidth);
		}

		width += util.getStringWidth(segment);
	}

	return width;
}

/**
 * Checks whether a line is the shebang that starts a script.
 */
function isShebang(line: string, index: number): boolean {
	return index === 0 && line.startsWith('#!');
}

/**
 * Splits text at spaces and tabs, except between a backtick and the next
 * backtick or between `{@` and the next `}`.
 */
function splitWords(text: string): string[] {
	const words: string[] = [];
	let word = '';

	for (let index = 0; index < text.length; index += 1) {
		const character = text[index]!;

		if (
			character === '`' ||
			(character === '{' && text[index + 1] === '@')
		) {
			const end = text.indexOf(character === '`' ? '`' : '}', index + 1);

			if (end !== -1) {
				word += text.slice(index, end + 1);
				index = end;
				continue;
			}
		}

		if (character === ' ' || character === '\t') {
			if (word !== '') {
				words.push(word);
				word = '';
			}

			continue;
		}

		word += character;
	}

	if (word !== '') {
		words.push(word);
	}

	return words;
}
