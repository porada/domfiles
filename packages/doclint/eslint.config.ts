import markdown from '@eslint/markdown';
import markdownLinks from 'eslint-plugin-markdown-links';
import markdownPreferences from 'eslint-plugin-markdown-preferences';
import { defineConfig } from 'eslint/config';
import includeRootIgnores from './src/include-root-ignores/index.ts';

export default defineConfig([
	includeRootIgnores(process.cwd()),
	{
		ignores: [
			// Functional ignores
			'**/*.{js,cjs,mjs}',
		],
	},
	{
		ignores: [
			// Common ignore patterns
			'.agent-*/**',
			'.agents/skills/pnpm-*/**',
			'.claude/**',
		],
	},
	{
		linterOptions: {
			reportUnusedDisableDirectives: 'error',
			reportUnusedInlineConfigs: 'error',
		},
	},
	{
		files: [
			/* prettier-ignore */
			'**/*.md',
			'**/*.markdown',
		],
		plugins: {
			markdown,
			'markdown-links': markdownLinks,
			'markdown-preferences': markdownPreferences,
		},
		language: 'markdown/gfm',
		languageOptions: {
			frontmatter: 'yaml',
		},
		rules: {
			'markdown-links/no-missing-path': 'error',
			'markdown-preferences/canonical-code-block-language': [
				'error',
				{
					languages: {
						bash: 'sh',
						javascript: 'js',
						jsx: 'js',
						markdown: 'md',
						plaintext: 'text',
						tsx: 'ts',
						txt: 'text',
						typescript: 'ts',
						yml: 'yaml',
						zsh: 'sh',
					},
				},
			],
			'markdown-preferences/no-heading-trailing-punctuation': 'error',
			'markdown/fenced-code-language': 'error',
			'markdown/heading-increment': 'error',
			'markdown/no-duplicate-definitions': 'error',
			'markdown/no-duplicate-headings': [
				'error',
				{ checkSiblingsOnly: true },
			],
			'markdown/no-empty-definitions': 'error',
			'markdown/no-empty-images': 'error',
			'markdown/no-empty-links': 'error',
			'markdown/no-invalid-label-refs': 'error',
			'markdown/no-missing-atx-heading-space': 'error',
			'markdown/no-missing-label-refs': [
				'error',
				{
					allowLabels: [
						'!CAUTION',
						'!IMPORTANT',
						'!NOTE',
						'!TIP',
						'!WARNING',
					],
				},
			],
			'markdown/no-missing-link-fragments': 'error',
			'markdown/no-multiple-h1': 'error',
			'markdown/no-reference-like-urls': 'error',
			'markdown/no-reversed-media-syntax': 'error',
			'markdown/no-space-in-emphasis': 'error',
			'markdown/no-unused-definitions': 'error',
			'markdown/require-alt-text': 'error',
			'markdown/table-column-count': 'error',
		},
	},
	{
		files: [
			/* prettier-ignore */
			'**/README.md',
		],
		rules: {
			'markdown/require-alt-text': 'off',
		},
	},
]);
