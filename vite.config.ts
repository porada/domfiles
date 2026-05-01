import { defineOxlintConfig } from '@standard-config/eslint/utilities';
import { defineConfig } from 'vite-plus';

function chmod() {
	return (files: ReadonlyArray<string>) =>
		files
			.filter((file) => file !== 'bin/domlib')
			.map((file) => `chmod +x '${file}'`);
}

export default defineConfig({
	test: {
		projects: ['packages/*'],
	},
	lint: defineOxlintConfig(),
	staged: {
		'*': 'pnpm format',
		'*.fish': 'pnpm lint:fish',
		'*.sh': 'pnpm lint:sh',
		'*.ts': () => 'pnpm lint:ts:check',
		'bin/!(git-diff-highlight)': [chmod(), 'pnpm lint:sh'],
	},
});
