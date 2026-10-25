const IGNORE_OPTION = /^--(no-)?ignore(?:=(.*))?$/v;

/**
 * Checks whether ignore patterns are enabled, based on the last `--ignore`
 * or `--no-ignore` option before `--`
 */
export default function isIgnoreEnabled(params: string[]): boolean {
	let enabled = true;

	for (const param of params) {
		if (param === '--') {
			break;
		}

		const match = IGNORE_OPTION.exec(param);

		if (match) {
			const [, negation, value = 'true'] = match;

			enabled = (value.trim() === 'true') !== (negation !== undefined);
		}
	}

	return enabled;
}
