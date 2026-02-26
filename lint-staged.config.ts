import type { Configuration } from 'lint-staged';

function chmod() {
	return (files: ReadonlyArray<string>) =>
		files
			.filter((file) => file !== 'bin/domlib')
			.map((file) => `chmod +x '${file}'`);
}

const config: Configuration = {
	'*': 'pnpm format',
	'*.fish': 'pnpm lint:fish',
	'*.sh': 'pnpm lint:sh',
	'*.ts': () => 'pnpm lint:ts:check && pnpm lint:ts',
	'bin/!(git-diff-highlight)': [chmod(), 'pnpm lint:sh'],
};

export default config;
