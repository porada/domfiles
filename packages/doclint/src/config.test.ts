import { expect, test, vi } from 'vite-plus/test';
import * as exports from './config.ts';

vi.mock(import('./is-ignore-enabled/index.ts'), () => ({
	default: () => false,
}));

test('exposes correct public API', () => {
	expect({ ...exports }).toMatchInlineSnapshot(`
		{
		  "defineConfig": [Function],
		  "includeIgnoreFile": [Function],
		  "resolvePath": [Function],
		}
	`);
});

test('defines the default config', () => {
	const config = exports
		.defineConfig()
		.map(({ plugins: _plugins, ...entry }) => entry);

	expect(config).toMatchSnapshot();
});
