import { defineConfig as defineESLintConfig } from 'eslint/config';
import config from '../../eslint.config.ts';

/**
 * Defines a config that extends the defaults
 */
export default function defineConfig(
	...configs: Parameters<typeof defineESLintConfig>
): ReturnType<typeof defineESLintConfig> {
	return defineESLintConfig(config, ...configs);
}
