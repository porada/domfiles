import type { AstPath, ParserOptions, Plugin } from 'prettier';
import { exec, NonZeroExitError } from 'tinyexec';
import separateShebang from './separate-shebang/index.ts';
import wrapComments from './wrap-comments/index.ts';

type FishAst = {
	formattedText: string;
	sourceLength: number;
};

export const languages: Plugin['languages'] = [
	{
		extensions: ['.fish'],
		interpreters: ['fish'],
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

		parse: async (
			text: string,
			options: ParserOptions
		): Promise<FishAst> => {
			try {
				const { stdout } = await exec('fish_indent', [], {
					nodePath: false,
					stdin: text,
					throwOnError: true,
				});

				// Separating the shebang first puts a comment right below the
				// `#!` lines in a block of its own. Otherwise, `wrapComments`
				// would leave that comment as written and only reflow it in a
				// second pass
				return {
					formattedText: await wrapComments(
						separateShebang(stdout),
						options
					),
					sourceLength: text.length,
				};
			} catch (error: unknown) {
				reportFormattingError(options.filepath, error);
			}
		},
	},
};

export const printers: Plugin['printers'] = {
	'fish-text': {
		print: (path: AstPath<FishAst>): string => path.node.formattedText,
	},
};

function reportFormattingError(
	filepath: string | undefined,
	error: unknown
): never {
	let message = '[prettier-plugin-fish] Failed to format';

	if (filepath) {
		message += ` \`${filepath}\``;
	}

	let details = error instanceof Error && error.message ? error.message : '';

	if (
		error instanceof NonZeroExitError &&
		error.output?.stderr &&
		!details.includes(error.output.stderr)
	) {
		details += `${details ? '\n\n' : ''}${error.output.stderr}`;
	}

	if (details) {
		message += `:\n\n${details}`;
	}

	throw new Error(message, { cause: error });
}
