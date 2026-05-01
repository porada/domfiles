import { defineConfig } from '@standard-config/prettier';

export default defineConfig({
	plugins: ['@domfiles/prettier-plugin-fish'],
	overrides: [
		{
			files: [
				/* prettier-ignore */
				'.zed/keymap.json',
				'**/zed/keymap.json',
			],
			options: {
				jsonSortOrder: ['context', 'bindings'],
			},
		},
		{
			files: [
				/* prettier-ignore */
				'.zed/settings.json',
				'**/zed/settings.json',
			],
			options: {
				jsonSortOrder: ['default', '*', 'agent'],
			},
		},
	],
});
