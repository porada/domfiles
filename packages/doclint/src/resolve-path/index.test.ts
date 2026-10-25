import { expect, test } from 'vite-plus/test';
import resolvePath from './index.ts';

test('resolves paths relative to `base`', () => {
	expect(resolvePath('../doclint.ts', import.meta.url)).toContain(
		'packages/doclint/src/doclint.ts'
	);
});
