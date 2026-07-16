import { defineOxlintConfig } from '@standard-config/oxlint';
import { defineConfig } from 'vite-plus';

export default defineConfig({
	test: {
		projects: ['packages/*'],
	},
	lint: defineOxlintConfig(),
	staged: {
		'*': () => 'pnpm format',
		'*.fish': 'pnpm lint:fish',
		'*.sh': 'pnpm lint:sh',
		'*.ts': () => 'pnpm lint:ts:check',
		'bin/!(domlib|git-diff-highlight)': [
			/* prettier-ignore */
			'chmod +x',
			'pnpm lint:sh',
		],
		'bin/domlib': 'pnpm lint:sh',
	},
});
