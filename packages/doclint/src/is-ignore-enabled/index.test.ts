import { expect, test } from 'vite-plus/test';
import isIgnoreEnabled from './index.ts';

test.each([
	{ params: ['--fix', '--no-ignore', '--', 'foo.md'] },
	{ params: ['--ignore', '--no-ignore=true', 'foo.md'] },
	{ params: ['--ignore=false', 'foo.md'] },
	{ params: ['--no-ignore', '--', 'foo.md'] },
])('disables ignoring for $params', ({ params }) => {
	expect(isIgnoreEnabled(params)).toBe(false);
});

test.each([
	{ params: [] },
	{ params: ['--', '--no-ignore'] },
	{ params: ['--ignore-pattern=--no-ignore', 'foo.md'] },
	{ params: ['--no-ignore', '--ignore', 'foo.md'] },
	{ params: ['--no-ignore', '--ignore=true', 'foo.md'] },
	{ params: ['--no-ignore=false', 'foo.md'] },
])('keeps ignoring enabled for $params', ({ params }) => {
	expect(isIgnoreEnabled(params)).toBe(true);
});

test.each([
	{ name: 'spaces', whitespace: ' ' },
	{ name: 'tabs', whitespace: '\t' },
])('trims $name around Boolean values', ({ whitespace }) => {
	const falseValue = `${whitespace}false${whitespace}`;
	const trueValue = `${whitespace}true${whitespace}`;

	expect(isIgnoreEnabled([`--ignore=${falseValue}`])).toBe(false);
	expect(isIgnoreEnabled([`--ignore=${trueValue}`])).toBe(true);
	expect(isIgnoreEnabled([`--no-ignore=${falseValue}`])).toBe(true);
	expect(isIgnoreEnabled([`--no-ignore=${trueValue}`])).toBe(false);
});
