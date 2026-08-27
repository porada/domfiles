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
				'skills/domfiles-posix-shell-scripting/references/*.md',
			],
			options: {
				tabWidth: 2,
			},
		},
	],
});
