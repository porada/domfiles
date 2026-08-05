import type { AstPath, Plugin } from 'prettier';
import { exec } from 'tinyexec';

type FishAst = {
	formattedText: string;
	sourceLength: number;
};

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
		locEnd: (node: FishAst) => node.sourceLength,

		parse: async (text: string): Promise<FishAst> => {
			const { stdout } = await exec('fish_indent', [], {
				nodePath: false,
				stdin: text,
				throwOnError: true,
			});

			return {
				formattedText: stdout,
				sourceLength: text.length,
			};
		},
	},
};

export const printers: Plugin['printers'] = {
	'fish-text': {
		print: (path: AstPath<FishAst>): string => path.node.formattedText,
	},
};
