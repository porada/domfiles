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

test('defines a valid config', () => {
	const config = exports.defineConfig();

	for (const { plugins: _plugins, ...entry } of config) {
		expect(entry).toMatchSnapshot();
	}
});
