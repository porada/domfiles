import { defineOxlintConfig } from '@standard-config/oxlint';
import { defineConfig } from 'vite-plus';

export default defineConfig({
	test: {
		projects: [
			/* prettier-ignore */
			'packages/**/vite.config.ts',
		],
	},
	lint: defineOxlintConfig({
		overrides: [
			{
				files: [
					/* prettier-ignore */
					'skills/**/*.test.ts',
				],
				rules: {
					'vitest/no-import-node-test': 'off',
				},
			},
		],
	}),
	staged: {
		'*': [
			() => 'pnpm install --frozen-lockfile --ignore-scripts',
			'pnpm prettier --ignore-unknown --write',
		],
		'*.fish': 'pnpm lint:fish',
		'*.json': 'pnpm lint:json',
		'*.md': 'pnpm lint:md:check --no-warn-ignored',
		'*.sh': 'pnpm lint:sh',
		'*.ts': [
			/* prettier-ignore */
			() => 'pnpm lint:ts:check',
			() => 'pnpm test',
		],
		'.github/workflows/*.yaml': 'actionlint',
		'home/.local/bin/!(git-diff-highlight)': [
			/* prettier-ignore */
			'chmod +x',
			'pnpm lint:sh',
		],
		'home/.local/share/*': 'pnpm lint:sh',
	},
});
