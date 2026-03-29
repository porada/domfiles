import type { AstPath, Plugin } from 'prettier';
import { exec } from 'tinyexec';

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

		/* v8 ignore next -- @preserve */
		locStart: () => 0,
		/* v8 ignore next -- @preserve */
		locEnd: (text: string) => text.length,

		parse: async (text: string): Promise<string> => {
			const command = exec('fish_indent', [], { throwOnError: true });
			command.process?.stdin?.end(text);

			const { stdout } = await command;
			return `${stdout.trim()}\n`;
		},
	},
};

export const printers: Plugin['printers'] = {
	'fish-text': {
		print: (path: AstPath<string>): string => path.node,
	},
};
