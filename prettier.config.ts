import { defineConfig } from '@standard-config/prettier';

export default defineConfig({
	plugins: [
		'prettier-plugin-fish',
		'prettier-plugin-rust',
		'prettier-plugin-toml',
	],
	overrides: [
		{
			files: [
				/* prettier-ignore */
				'**/zed/settings.json',
			],
			options: {
				jsonSortOrder: [
					'$schema',
					'default',
					'command',
					'args',
					'model',
					'provider',
					'*',
					'agent',
					'case_sensitive',
				],
			},
		},
		{
			files: [
				/* prettier-ignore */
				'skills/**/assets/*.txt',
			],
			options: {
				parser: 'markdown',
			},
		},
		{
			files: [
				/* prettier-ignore */
				'skills/posix-shell-scripting/references/*.md',
			],
			options: {
				tabWidth: 2,
			},
		},
	],
});
