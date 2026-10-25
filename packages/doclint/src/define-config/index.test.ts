import type { Linter } from 'eslint';
import { expect, test, vi } from 'vite-plus/test';
import config from '../../eslint.config.ts';
import defineConfig from './index.ts';

vi.mock(import('../is-ignore-enabled/index.ts'), () => ({
	default: () => false,
}));

test('defines a config that extends the defaults', () => {
	const defaultConfig = defineConfig();

	const extensionConfig: Linter.Config = {
		files: ['**/templates/*.md'],
		rules: {
			'markdown/no-empty-links': 'off',
		},
	};

	const extendedConfig = defineConfig(extensionConfig);

	expect(defaultConfig).toStrictEqual(config);
	expect(extendedConfig).toStrictEqual([...defaultConfig, extensionConfig]);
});
