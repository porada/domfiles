# Skill-Owned Scripts

A skill-owned script is a small program that gathers recurring evidence, produces declared artifacts, or does both for one project-authored skill. Use one only when a named recurring consumer needs a workflow that is more deterministic, efficient, or repeatable than an existing repository workflow or a bounded direct tool sequence. Do not create one for a one-time transition, to encode review preferences, or as a general repository utility.

When the owning skill supports use across projects or project policy designates the script as a portable command, also apply [Support Portable Commands](#support-portable-commands) before resolving its interface. Portability does not change the owning skill’s installation category.

## Choose Contract Pass

For a script contract review or audit, inspect implementation and adjacent tests only as bounded evidence for a specific observable contract, then stop once the claim is established. Don’t assess algorithms, internal structure, language idioms, performance, dead code, duplication, or general test quality unless the user explicitly includes implementation. Evaluation criteria such as security, maintainability, or project values apply within the resolved scope and don’t expand it. Keep standalone reviews and audits read-only, following an applicable model-invocable project audit workflow when one exists. Otherwise, begin an audit with Git-tracked paths, add only explicitly named untracked material when local policy permits it, inspect the resolved scope, report findings, and stop without formatting or mutation.

When the resolved scope explicitly includes implementation, follow applicable project, domain, and language implementation and validation workflows for internal concerns. Keep the script contract pass focused on contract consequences, and update agent documentation only when the contract, routing, or documented invocation changes. Documentation work rejoins the shared `agent-documentation` lifecycle.

## Design the Smallest Sufficient Contract

The optimal contract is the least complex one that completely serves its named consumers within the declared operating model. For a new script or material expansion, define the observable contract before choosing dependencies or internal architecture. Then make one bounded design pass. In that pass, first try to remove the script or each contract element, then test what remains against boundary values, concurrency inside the declared model, malformed input, output failures, and partial operations. Derive each requirement from a basis allowed by the global **Proportionality** rule, authoritative behavior, a named recurring consumer, or a script-specific standing policy. Never derive requirements from invented consumers, threats, or use cases. Ask the user only when two materially different designs remain viable, and document only the accepted contract and non-obvious rationale.

A material expansion adds a dependency, durable artifact, input schema, mutation-authorizing decision, observable failure or status behavior, operation mode, or side effect. A fix reuses the accepted contract without reopening design when it only restores conformance and adds none of those elements. Reopen design only when new evidence invalidates an accepted assumption. A hypothetical case outside the declared operating model doesn’t expand the contract.

## Define Observable Interface

Treat these elements as the script’s observable interface:

- Accepted arguments, defaults, documented environment inputs, required arguments, and target selection rules.
- Authorization requirements, credential sources, external commands, network access, and read and write effects.
- Compatibility guarantees, input manifests, machine-readable schemas, and produced artifacts.
- Executable and operation names.
- Exit status meanings and standard error and standard output behavior.

Document the complete interface in the owning skill at the decision that invokes the script. Keep command help consistent with that documentation, and cover the contract with adjacent tests.

Every documented interface element is compatibility-sensitive. When a change removes, renames, or alters one, update every repository-owned consumer, contract test, invocation, and migration requirement in the same authorized task. A new optional argument or operation is compatible only when existing invocations retain their behavior. Human-facing wording may evolve unless the owning skill or an exact-string consumer declares it stable. Internal functions and source filenames are not part of the interface unless a documented consumer or invocation addresses them directly.

## Keep Ownership Clear

- Store each script and its adjacent contract test directly under `<skill>/scripts` as `<script-name>.<extension>` and `<script-name>.test.<extension>`, with `<script-name>` in lower kebab case unless the language or its tooling requires another form. Rename the pair together. Store shared implementation helpers and their adjacent tests under `<skill>/scripts/helpers`, and keep executable entrypoints out of it. Do not create a per-script directory. Place any launcher at the repository’s prescribed command surface without relocating or duplicating the implementation.
- Let the skill own the source, tests, purpose, invocation, and interface. When script evidence indicates that authoritative source, configuration, or documentation should change, the skill defines a separate repair route and enters it only when the current request authorizes those changes.
- Let the repository root own toolchain configuration, dependencies and host language type packages, static validation, and build output policy.
- Don’t give the scripts directory or its `helpers` directory a separate package, manifest, TypeScript configuration, lockfile, or workspace membership.

## Make Interfaces Discoverable

- Support `--help` and describe the script’s purpose, modes, accepted inputs, output behavior, and relevant exit behavior, plus destination selection and overwrite policy for any writes.
- Treat source and configuration as the authority for implemented CLI behavior and serialized schemas. Treat adjacent contract tests as corroborating evidence, and treat `--help`, proposals, runtime argument diagnostics, and workflow documentation as projections. A stale test or projection never authorizes adding or broadening behavior without explicit user authorization.
- Before changing a CLI projection, inspect the exact implementation and relevant adjacent tests for every affected combination, mode, option, or schema. Keep help-only and documentation-only tasks behavior-neutral. When an authorized task changes behavior, align help, source and configuration, tests, and workflow documentation before treating the change as complete.
- Cover help-to-parser agreement with a contract test rather than review alone. Assert for each mode or standalone operation that the options documented for that route exactly match the options its parser accepts. Aligning them without a test leaves the next change free to reintroduce the drift.

## Bound Execution and Output

- Aggregate the complete result while retaining only the bounded details that can be reported. State exact total and omitted counts without storing or emitting every failure body.
- Choose human-readable or structured output according to the consumer’s needs. Don’t require JSON without a consumer that needs it.
- Don’t invent an output filename in the current directory. Use a declared repository destination or require the caller to supply one.
- Keep each operation bounded and deterministic for the same repository state and inputs.
- When a named consumer requires repeated records, prefer one batch or suite invocation over repeated one-record processes. Don’t introduce batch input without that consumer.

## Support Portable Commands

A portable command is a self-contained script that any supported agent can run from any working directory through the installed skill. The script and its adjacent test use only their runtime’s standard library and files under the skill’s `scripts` directory. They need no installed packages, build step, or repository configuration, so an installation that copies the skill rather than linking it can still run them. An external command that gives an operation access to the state it inspects or changes, such as `claude`, `git`, or `gh`, is an interface prerequisite rather than a dependency. Document the invocation as the runtime command followed by `<skill-directory>/scripts/<script-name>.<extension>`, where `<skill-directory>` is the resolved directory containing the owning `SKILL.md`. Add a `PATH` wrapper only when project policy requires one or a concrete consumer needs it.

Apply these command defaults:

- Keep the interface non-interactive and independent of editor actions, MCP servers, agent-specific APIs, conversational state, and calling-agent implementation.
- Support an exit-only `--help` operation that writes help to standard output, performs no other work, and returns status `0`. Reject `--help` combined with operational arguments.
- Reject unknown arguments, missing required arguments, inaccessible required inputs, and invalid input without performing writes.
- Resolve relative paths against the invocation working directory unless the contract documents another base.
- Write requested data, reports, and completed findings to standard output. Reserve standard error for usage and operational diagnostics. Keep default output free of terminal control sequences and order it deterministically.
- For new interfaces, use status `0` when the requested operation completes successfully without check findings, including a query that returns zero records. Use status `1` only when a check or audit completes and reports findings. Use status `2` for invalid invocation, invalid or inaccessible input, and operational failure.

Preserve an established, documented exit status contract when adding portability, adding a launcher, or renaming a command unless an explicit migration is authorized. Document and test any differences from these defaults.

Make network access explicit in the contract. Obtain credentials only through an established machine-local source or external credential store, and never accept literal secrets through command arguments. Treat the caller’s sandbox, permission, access, approval, and mutation boundaries as preconditions. On an access failure, report the attempted operation and the required boundary crossing instead of following an alternate path. Don’t turn an operation that requires native confirmation into an indirect script mutation, or infer mutation authority from writable permissions, an ignored destination, or a previous successful invocation.

Before adding an operation that inspects or changes another project, define how the caller selects that project and its complete scope rather than inferring either from the working directory. Apply every absolute exclusion for machine-local secret material, and never modify that project’s dependencies, toolchain configuration, Git metadata, or agent instructions as an incidental effect. Changes to the script and changes to another project, including updates to its consumers after a breaking interface change, require separate authorization.

Validate a portable command from its canonical repository. Run it from a working directory outside that repository, and confirm that it loads code and bundled files only from the skill’s `scripts` directory. Exercise help, invalid invocation, each defined status, and deterministic ordering. Verify that read operations leave their inputs unchanged and that write operations affect only declared destinations.

Don’t add `--dry-run`, a machine-readable `--contract` operation, standalone distribution, or a universal subcommand hierarchy without a concrete consumer. Give every persistent machine-readable schema an integer version, and reject unsupported versions rather than interpreting them heuristically.

## Test Contracts

- Cover every distinct externally observable refusal and every routine that authorizes a mutation on a correctness claim, such as an accounting, containment, or equivalence proof. For a refusal shared by multiple external routes, keep the detailed refusal cases on the shared path and add one lightweight wiring assertion for each route proving that it reaches that path. Add route-specific detailed cases only when the route changes behavior or a caller relies on that distinction. Assert the refusal a caller would rely on, not just that the operation failed.
- Test each operation’s output and failure behavior, plus destination handling, overwrite policy, and cleanup for any writes.
- Keep durable repository-owned fixture inputs narrow and deterministic under `<skill>/scripts`. Create runtime fixture outputs, disposable repositories, and scratch state in temporary storage outside the repository, and keep the repository’s own Git metadata read-only. Tests that create commits, including fixture commits, still require explicit user authorization under the global **Commit gate**.
- Give each test its own fixtures and run independent tests concurrently, so the suite takes about as long as its slowest test. Launch processes asynchronously because a blocking launch serializes concurrent tests.
- Run focused tests during implementation and after each behaviorally relevant correction. Run the repository’s root static validation once after the consolidated change batch, then rerun it only when a later correction changes an input or configuration that it covers. Direct execution and focused tests don’t replace root typechecking or compilation.

Script tests are agent-owned. Document focused script and test commands in the owning skill or its repair reference, and keep test runs out of the repository’s default test command and commit hooks. CI may run them.

## Choose Dependencies Before Implementation

Apply `intentional-dependency-choice` before choosing a bespoke implementation of a mature general-purpose capability. Such capabilities include cryptography and hashing, shell parsing, structured-data parsing and serialization, Unicode processing, URL handling, and other standards-heavy behavior. Reuse established decisions and approvals. The global **Dependencies** policy still governs implementation and mutating delegation.

A portable command can’t take a package dependency or run an external command to supply such a capability. When its standard library lacks a capability of this kind, narrow the contract or keep the script out of portable use rather than reimplementing standards-heavy behavior.

## Integrate Root Validation

When a script reveals that root validation omits a source category, extend the root contract for that complete category rather than hardcoding one skill path. For example, add `.agents/**` to a TypeScript repository’s root include patterns when they do not cover skill-owned sources. Ensure root static validation covers both the script and its test. Do not broaden configuration prospectively before a real script establishes the need.
