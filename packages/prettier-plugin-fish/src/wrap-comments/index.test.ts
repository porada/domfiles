import { expect, test } from 'vite-plus/test';
import wrapComments from './index.ts';

const OPTIONS = { printWidth: 14, tabWidth: 2 };

test('formats comment blocks at their indentation', async () => {
	const input =
		'function foo\n    # bar baz qux\n    # quux\n    corge\nend\n';

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(
		'function foo\n    # bar baz\n    # qux quux\n    corge\nend\n'
	);
});

test('counts tabs as `tabWidth` columns', async () => {
	const options = { ...OPTIONS, tabWidth: 4 };

	await expect(wrapComments('\t# foo bar baz\n', options)).resolves.toBe(
		'\t# foo bar\n\t# baz\n'
	);
});

test('counts tabs as no columns when `tabWidth` is 0', async () => {
	const input = '\t# foo bar baz\n';

	await expect(
		wrapComments(input, { printWidth: 13, tabWidth: 0 })
	).resolves.toBe(input);
});

test('measures characters by their display width', async () => {
	await expect(
		wrapComments('# 文字 文字 文字\n', { ...OPTIONS, printWidth: 12 })
	).resolves.toBe('# 文字 文字\n# 文字\n');
});

test('finds comments after characters outside the Basic Multilingual Plane', async () => {
	await expect(wrapComments('echo 😀\n# foo   bar\n', OPTIONS)).resolves.toBe(
		'echo 😀\n# foo bar\n'
	);
});

test('leaves comments as written when Fish counts characters differently', async () => {
	const input = 'echo "\u{F600}" #\necho foo\n# bar   baz\n';

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(input);
});

test('formats comments below the shebang', async () => {
	await expect(
		wrapComments('#!/usr/bin/env fish\n# foo   bar\n', OPTIONS)
	).resolves.toBe('#!/usr/bin/env fish\n# foo bar\n');
});

test('keeps `#` lines in strings as written', async () => {
	const input = "set foo 'bar\n# baz   qux\n'\n";

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(input);
});

test('leaves trailing comments as written', async () => {
	const input = 'echo foo # bar   baz\n# qux\n';

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(input);
});

test('removes redundant `#` characters and empty comment lines', async () => {
	await expect(
		wrapComments('#\n## foo\n#\n#\n#bar\n#\nbaz\n', OPTIONS)
	).resolves.toBe('# foo\n#\n# bar\nbaz\n');
	await expect(wrapComments('#\n#\nfoo\n', OPTIONS)).resolves.toBe(
		'#\nfoo\n'
	);
});

test('separates comment blocks with different indentation', async () => {
	const input = '  # foo\n# bar\n';

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(input);
});

test('treats directives as prose', async () => {
	await expect(
		wrapComments('# foo\n# vim: set filetype=fish:\n', {
			...OPTIONS,
			printWidth: 80,
		})
	).resolves.toBe('# foo vim: set filetype=fish:\n');
});

test('keeps code spans on one line', async () => {
	await expect(
		wrapComments('# foo bar baz `qux quux`\n', {
			...OPTIONS,
			printWidth: 20,
		})
	).resolves.toBe('# foo bar baz\n# `qux quux`\n');
});

test('keeps code spans that continue on the next line together', async () => {
	await expect(
		wrapComments('# foo bar baz `qux\n# quux`\n', {
			...OPTIONS,
			printWidth: 20,
		})
	).resolves.toBe('# foo bar baz\n# `qux quux`\n');
});

test('keeps inline tags on one line', async () => {
	await expect(
		wrapComments('# foo bar baz {@link qux | quux}\n', {
			...OPTIONS,
			printWidth: 20,
		})
	).resolves.toBe('# foo bar baz\n# {@link qux | quux}\n');
});

test('keeps words that would read as markup with the word before them', async () => {
	const options = { ...OPTIONS, printWidth: 13 };

	await expect(wrapComments('# foo bar baz - qux\n', options)).resolves.toBe(
		'# foo bar baz -\n# qux\n'
	);
	await expect(wrapComments('# foo bar baz * qux\n', options)).resolves.toBe(
		'# foo bar baz *\n# qux\n'
	);
	await expect(wrapComments('# foo bar baz + qux\n', options)).resolves.toBe(
		'# foo bar baz +\n# qux\n'
	);
	await expect(wrapComments('# foo bar baz 1. qux\n', options)).resolves.toBe(
		'# foo bar baz 1.\n# qux\n'
	);
	await expect(wrapComments('# foo bar baz 1) qux\n', options)).resolves.toBe(
		'# foo bar baz 1)\n# qux\n'
	);
	await expect(
		wrapComments('# foo bar baz prettier-ignore qux\n', options)
	).resolves.toBe('# foo bar baz prettier-ignore\n# qux\n');
});

test('keeps words that would open a code fence with the word before them', async () => {
	const options = { ...OPTIONS, printWidth: 13 };

	await expect(wrapComments('# foo bar baz ```qux\n', options)).resolves.toBe(
		'# foo bar baz ```qux\n'
	);
	await expect(wrapComments('# foo bar baz ~~~qux\n', options)).resolves.toBe(
		'# foo bar baz ~~~qux\n'
	);
});

test('keeps wrapped lines from opening a code fence', async () => {
	const input = '# ```` foo bar baz qux ````\n';

	await expect(
		wrapComments(input, { ...OPTIONS, printWidth: 16 })
	).resolves.toBe(input);
});

test('keeps the word after a leading marker on its line', async () => {
	await expect(
		wrapComments('# prettier-ignore disables wrapping\n', {
			...OPTIONS,
			printWidth: 20,
		})
	).resolves.toBe('# prettier-ignore disables\n# wrapping\n');
});

test('keeps blocks that contain lists, code fences, or indented lines as written', async () => {
	const inputWithList = '# foo   bar\n# - baz\n';
	const inputWithOrderedList = '# foo   bar\n# 1. baz\n';
	const inputWithFence = '# foo   bar\n#\n# ```fish\n# baz\n# ```\n';
	const inputWithTildeFence = '# foo   bar\n#\n# ~~~fish\n# baz\n# ~~~\n';
	const inputWithIndentedLine = '# foo   bar\n#   baz\n';
	const inputWithTabIndentedLine = '# foo   bar\n#\tbaz\n';

	await expect(wrapComments(inputWithList, OPTIONS)).resolves.toBe(
		inputWithList
	);
	await expect(wrapComments(inputWithOrderedList, OPTIONS)).resolves.toBe(
		inputWithOrderedList
	);
	await expect(wrapComments(inputWithFence, OPTIONS)).resolves.toBe(
		inputWithFence
	);
	await expect(wrapComments(inputWithTildeFence, OPTIONS)).resolves.toBe(
		inputWithTildeFence
	);
	await expect(wrapComments(inputWithIndentedLine, OPTIONS)).resolves.toBe(
		inputWithIndentedLine
	);
	await expect(wrapComments(inputWithTabIndentedLine, OPTIONS)).resolves.toBe(
		inputWithTabIndentedLine
	);
});

test('keeps blocks that contain `prettier-ignore` as written', async () => {
	const input = '# prettier-ignore\n## foo   bar\n#\n#\n';
	const inputWithLaterIgnore = '# foo   bar\n# prettier-ignore\n## baz\n';
	const inputWithLooseIgnore = '#prettier-ignore\n#\n## foo   bar\n';
	const inputWithTrailingWhitespace = '# prettier-ignore \n# foo   bar\n';

	await expect(wrapComments(input, OPTIONS)).resolves.toBe(input);
	await expect(wrapComments(inputWithLaterIgnore, OPTIONS)).resolves.toBe(
		inputWithLaterIgnore
	);
	await expect(wrapComments(inputWithLooseIgnore, OPTIONS)).resolves.toBe(
		inputWithLooseIgnore
	);
	await expect(
		wrapComments(inputWithTrailingWhitespace, OPTIONS)
	).resolves.toBe(inputWithTrailingWhitespace);
});

test('keeps blocks that contain `#!` lines as written', async () => {
	const inputWithLaterLine = '# foo   bar\n#!baz\n# qux\n';
	const inputWithEmptyFirstLine = '#\n#!/bin/false\n';

	await expect(wrapComments(inputWithLaterLine, OPTIONS)).resolves.toBe(
		inputWithLaterLine
	);
	await expect(wrapComments(inputWithEmptyFirstLine, OPTIONS)).resolves.toBe(
		inputWithEmptyFirstLine
	);
});
