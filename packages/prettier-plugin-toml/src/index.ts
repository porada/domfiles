import type { AstPath, ParserOptions, Plugin } from 'prettier';
import { exec, NonZeroExitError } from 'tinyexec';

type TOMLAst = {
	formattedText: string;
	sourceLength: number;
};

export const languages: Plugin['languages'] = [
	{
		extensions: ['.toml'],
		name: 'TOML',
		parsers: ['toml'],
	},
];

export const parsers: Plugin['parsers'] = {
	toml: {
		astFormat: 'toml-text',

		/* v8 ignore next -- @preserve */
		locStart: () => 0,
		/* v8 ignore next -- @preserve */
		locEnd: (node: TOMLAst) => node.sourceLength,

		parse: async (
			text: string,
			options: ParserOptions
		): Promise<TOMLAst> => {
			try {
				const { stdout } = await exec('taplo', ['fmt', '-'], {
					nodePath: false,
					stdin: text,
					throwOnError: true,
				});

				return {
					formattedText: stdout,
					sourceLength: text.length,
				};
			} catch (error: unknown) {
				reportFormattingError(options.filepath, error);
			}
		},
	},
};

export const printers: Plugin['printers'] = {
	'toml-text': {
		print: (path: AstPath<TOMLAst>): string => path.node.formattedText,
	},
};

function reportFormattingError(
	filepath: string | undefined,
	error: unknown
): never {
	let message = '[prettier-plugin-toml] Failed to format';

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
