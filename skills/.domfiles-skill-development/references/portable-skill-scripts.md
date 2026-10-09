# Portable Skill Scripts

Apply the [general skill-owned script policy](skill-owned-scripts.md), which selects when this contract applies, in addition to this reference.

A portable skill script exposes an agent-neutral [observable interface](skill-owned-scripts.md#define-observable-interface). Any supported agent can invoke that documented interface with explicitly selected inputs or targets without installing the script’s build configuration, dependencies, source, or toolchain in a target. Portability spans agents and targets rather than requiring standalone distribution.

The canonical repository supplies the script and its build context under the [ownership contract](skill-owned-scripts.md#keep-ownership-clear), not necessarily its runtime working directory. Confirm that the supported installation keeps that repository reachable, because an installation that copies the skill rather than linking it leaves the script without a host. A skill whose supported installation cannot reach its host stays documentation-only.

Updating a target-owned consumer for a breaking interface change requires separate authorization for that target.

## Separate Host and Target

- Treat the repository that canonically owns the script as the host. The host owns source, tests, dependencies, lockfiles, compilation, and root validation.
- Treat a project, repository, or path inspected or changed by an operation as its target. Do not infer a target from the host working directory.
- Require a single-target project-wide operation to accept `--root <target-root>` as its path base and `--scope <scope-manifest>` as its agent-resolved input boundary. Require a narrower operation to select its complete scope through explicit input paths. For a multi-target operation, use role-qualified selectors and document each target’s role and path base.
- Treat a target root as location rather than authorization. Reject a missing selector before reading target content.
- Resolve the scope manifest to a finite set of target-relative paths, and reject traversal, path base ambiguity, and symlink escape before reading an authorized path.
- Leave semantic resolution of each target’s `AGENTS.md`, authority model, approvals, protected paths, and absolute exclusions to the invoking agent. A script may inventory or structurally validate only paths the invoking agent selected through the applicable scope manifest or the operation’s explicit input path arguments.
- Do not require the target to install dependencies, add manifests, change package manager state, register build targets, or expose repository-specific commands.
- Do not apply host policy to the target merely because the host supplies the script.
- Resolve relative command line paths against the invocation working directory unless a documented manifest-relative contract governs them. Distinguish host paths, target paths, and artifact destinations in help and diagnostics.

An operation that consumes only supplied values, or maintains its host without a separate target, may omit target selectors and scope manifests. Name and document that scope explicitly so it cannot be mistaken for a project-targeted operation.

## Apply Common Command Contract

Separate a documented launcher build step from the requested operation. Build preparation may create declared host build outputs before help or argument validation, subject to existing dependency, network, and filesystem permissions. It must not change host source or configuration, or target state. Send build diagnostics to standard error. The operation-level guarantees below exclude only this preparation.

- Keep the interface noninteractive and independent of editor actions, MCP servers, agent-specific APIs, conversational state, and calling-agent implementation.
- Support an exit-only `--help` operation that writes help to standard output, performs no other work, and returns status `0`. Reject `--help` combined with operational arguments.
- Reject unknown arguments, missing required arguments, inaccessible required inputs, and invalid input without performing writes.
- Write requested data, reports, and completed findings to standard output. Reserve standard error for usage and operational diagnostics. Keep default output free of terminal control sequences and order it deterministically.
- For new interfaces, use status `0` when the requested operation completes successfully without check findings, including a query that returns zero records. Use status `1` only when a check or audit completes and reports findings. Use status `2` for invalid invocation, invalid or inaccessible input, and operational failure.

Preserve an established, documented exit status contract when adding portability, adding a launcher, or renaming a command unless an explicit migration is authorized. Document and test any differences from the defaults above.

## Bound Target Effects

- Exclude generated, managed, third-party, and vendored material unless the resolved scope includes it under applicable target policy. Apply every absolute exclusion for machine-local secret material, and never treat explicit scope inclusion as authority to override one.
- Keep host maintenance and target mutation as separate operations. Authorization to update the hosted script does not authorize changing a target, and authorization to change a target does not authorize updating the host.
- Make network access explicit in the operation contract. Obtain credentials only through an established machine-local source or external credential store, and never accept literal secrets through command arguments.
- Never modify target dependencies, toolchain configuration, Git metadata, or agent instructions as an incidental effect.

## Preserve Agent and Target Boundaries

- Treat caller sandbox, permission, access, approval, mutation, and submission boundaries as part of the operation’s preconditions rather than obstacles to bypass.
- Do not turn an operation requiring native confirmation or additional access into an indirect script or terminal mutation. A script may prepare or validate authorized external staging state, then must stop and report the native or user-authorized action still required.
- On an access failure, report the selected target, attempted operation, and required boundary crossing. Do not copy the target, follow an alternate path, or reformulate the invocation merely to evade the boundary.
- Do not infer mutation authority from target existence, writable permissions, an ignored destination, or a previous successful invocation.

## Validate Portability

- Run contract tests from the host. Keep any target fixtures outside the owning skill directory.
- For operations that select filesystem targets, test invocation from a working directory other than the target, target paths containing spaces and non-ASCII characters, missing and inaccessible targets, missing project-wide scope, traversal attempts, symlink escape, and paths omitted by the resolved scope.
- For launchers, test host build selection from an unrelated working directory with conflicting caller build configuration, argument forwarding, runtime working directory, exit status propagation, and output stream separation.
- Verify that build preparation affects only its declared outputs, read operations otherwise leave host and target unchanged, and write operations affect only declared destinations.
- Exercise help, invalid invocation, each defined status, and deterministic ordering.
- Register source and adjacent tests in the host’s authoritative validation without requiring target integration.

## Avoid Speculative Interfaces

A repository command policy may require a `PATH`-installed wrapper. Otherwise, require a concrete consumer before adding one. Do not require `--dry-run`, a machine-readable `--contract` operation, standalone distribution, or a universal subcommand hierarchy without a concrete consumer. When adding structured output, give every persistent machine-readable schema an integer version and reject unsupported versions rather than interpreting them heuristically. Add a shared interface only when it makes supported agent invocation or composition materially more deterministic than the owning skill’s documented command contract.
