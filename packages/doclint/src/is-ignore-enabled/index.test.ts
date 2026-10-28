import { expect, test } from 'vite-plus/test';
import isIgnoreEnabled from './index.ts';

test('disables ignoring', () => {
	expect(isIgnoreEnabled(['--fix', '--no-ignore', '--', 'foo.md'])).toBe(
		false
	);
	expect(isIgnoreEnabled(['--ignore', '--no-ignore=true', 'foo.md'])).toBe(
		false
	);
	expect(isIgnoreEnabled(['--ignore=false', 'foo.md'])).toBe(false);
	expect(isIgnoreEnabled(['--no-ignore', '--', 'foo.md'])).toBe(false);
});

test('keeps ignoring enabled', () => {
	expect(isIgnoreEnabled([])).toBe(true);
	expect(isIgnoreEnabled(['--', '--no-ignore'])).toBe(true);
	expect(isIgnoreEnabled(['--ignore-pattern=--no-ignore', 'foo.md'])).toBe(
		true
	);
	expect(isIgnoreEnabled(['--no-ignore', '--ignore', 'foo.md'])).toBe(true);
	expect(isIgnoreEnabled(['--no-ignore', '--ignore=true', 'foo.md'])).toBe(
		true
	);
	expect(isIgnoreEnabled(['--no-ignore=false', 'foo.md'])).toBe(true);
});

test('trims whitespace around Boolean values', () => {
	expect(isIgnoreEnabled(['--ignore= false '])).toBe(false);
	expect(isIgnoreEnabled(['--ignore= true '])).toBe(true);
	expect(isIgnoreEnabled(['--no-ignore= false '])).toBe(true);
	expect(isIgnoreEnabled(['--no-ignore= true '])).toBe(false);
	expect(isIgnoreEnabled(['--ignore=\tfalse\t'])).toBe(false);
	expect(isIgnoreEnabled(['--ignore=\ttrue\t'])).toBe(true);
	expect(isIgnoreEnabled(['--no-ignore=\tfalse\t'])).toBe(true);
	expect(isIgnoreEnabled(['--no-ignore=\ttrue\t'])).toBe(false);
});
