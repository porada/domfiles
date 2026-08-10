import { defineOxlintConfig } from '@standard-config/oxlint';
import { configDefaults, defineConfig } from 'vite-plus';

export default defineConfig({
	test: {
		exclude: [
			/* prettier-ignore */
			...configDefaults.exclude,
			'.agent-*/**',
			'.agents/**',
		],
		projects: [
			/* prettier-ignore */
			'packages/**/vite.config.ts',
		],
	},
	lint: defineOxlintConfig(),
	staged: {
		'*': 'pnpm prettier --ignore-unknown --write',
		'*.fish': 'pnpm lint:fish',
		'*.rs': () => 'pnpm lint:rs:check',
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
