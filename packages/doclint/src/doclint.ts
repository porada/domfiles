#!/usr/bin/env node

import eslint from 'eslint/package.json' with { type: 'json' };
import { spawnSync } from 'node:child_process';
import resolvePath from './resolve-path/index.ts';

const result = spawnSync(
	process.execPath,
	[
		resolvePath(
			eslint.bin.eslint,
			import.meta.resolve('eslint/package.json')
		),
		'--config',
		resolvePath('../eslint.config.ts', import.meta.url),
		'--flag',
		'unstable_native_nodejs_ts_config',
		...process.argv.slice(2),
	],
	{ stdio: 'inherit' }
);

if (result.error) {
	process.stderr.write(
		`doclint: Failed to start ESLint:\n\n${result.error.message}\n`
	);
	process.exitCode = 2;
} else if (result.signal) {
	process.kill(process.pid, result.signal);
} else {
	process.exitCode = result.status ?? 2;
}
