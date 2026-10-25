# Check Markdown

Use `doclint` for mechanical Markdown checks when the task permits linting. Run it after the project’s Prettier formatting step in change workflows. In read-only reviews, don’t reformat documents just to run the check.

## Run `doclint`

Run `doclint --no-ignore --max-warnings 0 -- <file-path> …` from the target repository root with explicit Markdown file paths within the authorized read scope. Use this form by default, not bare `doclint` or directory inputs. When using a project’s prescribed Markdown lint command, pass the same flags and file paths.

In editing workflows, use `doclint --fix --no-ignore --max-warnings 0 -- <file-path> …` when mechanical corrections to every selected document are authorized. Inspect the resulting diff and keep fixes within the task scope. Omit `--fix` for reviews and validation-only tasks.

Use `doclint --help` for the native argument and flag reference, which identifies the command as ESLint. Apply the consuming project’s requirements for pnpm-backed validation. Without `--fix`, linting doesn’t rewrite documents, but the `PATH` launcher may reconcile the domfiles dependencies.

## Select Files

The base configuration selects `.md` and `.markdown` files. `--no-ignore` bypasses ignore rules, including exclusions for task fixtures, without changing ignore files. Inputs outside the working directory can still fall outside the configuration’s scope. For piped Markdown, use `--no-ignore --max-warnings 0 --stdin --stdin-filename <file-path>`.

Link checks may read linked Markdown or HTML documents even when they’re ignored or outside the selected files. Include those targets in the authorized read scope.

The shipped configuration is loaded by default, without automatic lookup of caller ESLint configuration. For project-specific checks, use `--config <config-file>` to select a trusted ESLint configuration. Where `doclint` is a project dependency, use `defineConfig` from `doclint/config` to apply the defaults before your configuration.

## Understand Coverage

The default configuration supports GitHub Flavored Markdown and YAML frontmatter and checks:

- Duplicate, empty, and unused definitions, including footnotes, and invalid or undefined reference labels. Use uppercase GitHub alert labels. Lowercase and mixed-case forms render on GitHub but remain undefined-reference errors here.
- Duplicate sibling headings, heading level increments, and multiple top-level headings.
- Empty link and image destinations, reversed link and image syntax, and URLs that look like defined reference labels.
- Fragment-only links to headings or anchors in the same document.
- Language labels on fenced code blocks, including configured canonical spellings.
- Local link and image targets, and fragments in linked Markdown or HTML documents.
- Missing image alt text, except in `README.md` files.
- Missing spaces after heading markers, spaces inside emphasis markers, and table cells that exceed the header’s column count.
- Trailing heading punctuation such as periods and colons. Question marks are allowed.

Verify external URLs, path capitalization, symlinked paths, renderer-specific anchors, prose conventions, list spacing, and other documentation requirements manually. Line number fragments aren’t range-checked. A clean result establishes only the configured checks, not that every link is correct.

## Interpret Results

Default clean output is silent. Ignored-file warnings mean those inputs weren’t checked. `--max-warnings 0` makes warnings fail validation. Use `--format json` for per-file results, distinguishing ignored-file warnings from lint results.

| Exit Status | Meaning                                                    |
| ----------- | ---------------------------------------------------------- |
| `0`         | No lint errors within the warning limit, or help displayed |
| `1`         | Lint errors or an exceeded warning limit                   |
| `2`         | Invocation, configuration, or execution error              |

Empty or entirely ignored directory inputs can exit with status `2`. A missing input path also exits with status `2` and stops reporting unless `--no-error-on-unmatched-pattern` is explicitly requested.
