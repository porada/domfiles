import { defineConfig } from '@standard-config/prettier';

export default defineConfig({
	plugins: [
		'prettier-plugin-fish',
		'prettier-plugin-rust',
		'prettier-plugin-toml',
	],
});
