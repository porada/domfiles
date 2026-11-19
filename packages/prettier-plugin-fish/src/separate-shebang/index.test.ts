import { expect, test } from 'vite-plus/test';
import separateShebang from './index.ts';

test('adds an empty line after a shebang', () => {
	expect(separateShebang('#!/usr/bin/env fish\necho foo\n')).toBe(
		'#!/usr/bin/env fish\n\necho foo\n'
	);
});

test('keeps an existing empty line after a shebang', () => {
	const input = '#!/usr/bin/env fish\n\necho foo\n';

	expect(separateShebang(input)).toBe(input);
});

test('leaves a file that only holds a shebang unchanged', () => {
	const input = '#!/usr/bin/env fish\n';

	expect(separateShebang(input)).toBe(input);
});

test('keeps `#!` lines below a shebang together', () => {
	expect(
		separateShebang(
			'#! /usr/bin/env nix-shell\n#! nix-shell -i fish -p jq\njq --version\n'
		)
	).toBe(
		'#! /usr/bin/env nix-shell\n#! nix-shell -i fish -p jq\n\njq --version\n'
	);
});

test('leaves a file that only holds `#!` lines unchanged', () => {
	const input = '#! /usr/bin/env nix-shell\n#! nix-shell -i fish -p jq\n';

	expect(separateShebang(input)).toBe(input);
});

test('leaves text without a shebang unchanged', () => {
	const input = '# #!/usr/bin/env fish\necho foo\n';

	expect(separateShebang(input)).toBe(input);
});
