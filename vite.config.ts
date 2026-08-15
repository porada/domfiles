import { defineOxlintConfig } from '@standard-config/oxlint';
import { configDefaults, defineConfig } from 'vite-plus';

export default defineConfig({
	test: {
		exclude: [
			/* prettier-ignore */
			...configDefaults.exclude,
			'.agent-*/**',
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
		'*.json': 'pnpm lint:json',
		'*.rs': [
			/* prettier-ignore */
			() => 'pnpm lint:rs:check',
			() => 'pnpm test:rs',
		],
		'*.sh': 'pnpm lint:sh',
		'*.toml': 'pnpm lint:toml',
		'*.ts': [
			/* prettier-ignore */
			() => 'pnpm lint:ts:check',
			() => 'pnpm test:ts',
		],
		'*.yaml': 'pnpm lint:yaml',
		'.github/workflows/*.yaml': 'actionlint',
		'bin/!(domlib|git-diff-highlight)': [
			/* prettier-ignore */
			'chmod +x',
			'pnpm lint:sh',
		],
		'bin/domlib': 'pnpm lint:sh',
	},
});
