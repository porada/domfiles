import type { AstPath, ParserOptions, Plugin } from 'prettier';
import { exec, NonZeroExitError } from 'tinyexec';

type RustAst = {
	formattedText: string;
	sourceLength: number;
};

export const languages: Plugin['languages'] = [
	{
		extensions: ['.rs'],
		interpreters: ['rust-script'],
		name: 'Rust',
		parsers: ['rust'],
	},
];

export const parsers: Plugin['parsers'] = {
	rust: {
		astFormat: 'rust-text',

		/* v8 ignore next -- @preserve */
		locStart: () => 0,
		/* v8 ignore next -- @preserve */
		locEnd: (node: RustAst) => node.sourceLength,

		parse: async (
			text: string,
			options: ParserOptions
		): Promise<RustAst> => {
			try {
				const { stdout } = await exec(
					'rustfmt',
					['--edition', '2024', '--emit', 'stdout'],
					{
						nodePath: false,
						stdin: text,
						throwOnError: true,
					}
				);

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
	'rust-text': {
		print: (path: AstPath<RustAst>): string => path.node.formattedText,
	},
};

function reportFormattingError(
	filepath: string | undefined,
	error: unknown
): never {
	let message = '[prettier-plugin-rust] Failed to format';

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
