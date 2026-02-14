import type { AstPath, Plugin } from 'prettier';
import spawn from 'nano-spawn';

export const languages: Plugin['languages'] = [
	{
		extensions: ['.fish'],
		name: 'Fish',
		parsers: ['fish'],
	},
];

export const parsers: Plugin['parsers'] = {
	fish: {
		astFormat: 'fish-text',

		locStart: () => 0,
		locEnd: (text: string) => text.length,

		parse: async (text: string): Promise<string> => {
			const { stdout } = await spawn('fish_indent', {
				stdin: {
					string: text,
				},
			});

			return `${stdout.trim()}\n`;
		},
	},
};

export const printers: Plugin['printers'] = {
	'fish-text': {
		print: (path: AstPath<string>): string => path.node,
	},
};
