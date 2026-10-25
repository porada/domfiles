import type { Linter } from 'eslint';
import { includeIgnoreFile } from 'eslint/config';
import { lstatSync } from 'node:fs';
import { join } from 'node:path';
import isIgnoreEnabled from '../is-ignore-enabled/index.ts';

/**
 * Provides global ignores from the directory’s own `.gitignore` when the
 * file exists and ignore patterns are enabled
 */
export default function includeRootIgnores(directory: string): Linter.Config[] {
	if (!isIgnoreEnabled(process.argv.slice(2))) {
		return [];
	}

	const path = join(directory, '.gitignore');

	try {
		return lstatSync(path, { throwIfNoEntry: false })
			? [includeIgnoreFile(path, { gitignoreResolution: true })]
			: [];
	} catch (error) {
		const { message } = error as Error;

		throw new Error(`Failed to read \`${path}\`:\n\n${message}`, {
			cause: error,
		});
	}
}
