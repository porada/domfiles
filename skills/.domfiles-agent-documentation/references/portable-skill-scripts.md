# Portable Skill Scripts

Use this reference for a script owned by a portable skill—a skill installed for use across target projects rather than scoped to one repository. Apply the [general skill-owned script policy](skill-owned-scripts.md) in addition to this reference.

A portable skill script exposes an agent-neutral [observable interface](skill-owned-scripts.md#define-the-observable-interface). Any supported agent can invoke that documented interface against an explicitly selected target without installing the script’s build configuration, dependencies, source, or toolchain in that target. Portability spans agents and targets rather than requiring standalone distribution.

The script therefore runs from the repository that canonically owns it. Confirm that the supported installation keeps that repository reachable, because an installation that copies the skill rather than linking it leaves the script without a host. A skill whose supported installation cannot reach its host stays documentation-only.

Updating a target-owned consumer for a breaking interface change requires separate authorization for that target.

## Separate the Host and Target

- Treat the repository that canonically owns the script as the host. The host owns source, tests, dependencies, lockfiles, compilation, and root validation.
- Treat a project, repository, or path inspected or changed by an operation as its target. Do not infer a target from the host working directory.
- Require a single-target project-wide operation to accept `--root <target-root>` as its path base and `--scope <scope-manifest>` as its agent-resolved input boundary. Require a narrower operation to select its complete scope through explicit input paths. For a multi-target operation, use role-qualified selectors and document each target’s role and path base.
- Treat a target root as location rather than authorization. Reject a missing selector before reading target content.
- Resolve the scope manifest to a finite set of target-relative paths, and reject traversal, path-base ambiguity, and symlink escape before reading an authorized path.
- Leave semantic resolution of each target’s `AGENTS.md`, authority model, approvals, protected paths, and absolute exclusions to the invoking agent. A script may inventory or structurally validate only paths the invoking agent selected through the applicable scope manifest or the operation’s explicit input-path arguments.
- Do not require the target to install dependencies, add manifests, change package-manager state, register build targets, or expose repository-specific commands.
- Do not apply host policy to the target merely because the script executes in the host.
- Resolve relative command-line paths against the invocation working directory unless a documented manifest-relative contract governs them. Distinguish host paths, target paths, and artifact destinations in help and diagnostics.

A host-maintenance operation that consumes no separate target may omit a target selector. Name and document that scope explicitly so it cannot be mistaken for a project-targeted operation.

## Apply the Common Command Contract

- Keep the interface noninteractive and independent of editor actions, MCP servers, agent-specific APIs, conversational state, and calling-agent implementation.
- Support an exit-only `--help` operation that writes help to standard output, performs no other work, and returns status `0`. Reject `--help` combined with operational arguments.
- Reject unknown arguments, missing required arguments, inaccessible required inputs, and invalid input without performing writes.
- Write requested data, reports, and completed findings to standard output. Reserve standard error for usage and operational diagnostics. Keep default output free of terminal-control sequences and order it deterministically.
- Use status `0` when the requested operation completes successfully without check findings. Use status `1` only when a check or audit completes and reports findings. Use status `2` for invalid invocation, invalid or inaccessible input, and operational failure.

A query that successfully returns zero records is status `0`. Status `1` represents a policy-defined discrepancy, not an empty result.

## Bound Target Effects

- Keep host maintenance and target mutation as separate operations. Authorization to update the hosted script does not authorize changing a target, and authorization to change a target does not authorize updating the host.
- Never modify target dependencies, toolchain configuration, Git metadata, or agent instructions as an incidental effect.
- Exclude managed, vendored, generated, and third-party material unless the resolved scope includes it under applicable target policy. Apply every absolute exclusion for machine-local secret material, and never treat explicit scope inclusion as authority to override one.
- Make network access explicit in the operation contract. Obtain credentials only through an established machine-local source or external credential store, and never accept literal secrets through command arguments.

## Preserve Agent and Target Boundaries

- Treat caller sandbox, permission, access, approval, mutation, and submission boundaries as part of the operation’s preconditions rather than obstacles to bypass.
- Do not turn an operation requiring native confirmation or additional access into an indirect script or terminal mutation. A script may prepare or validate authorized external staging state, then must stop and report the native or user-authorized action still required.
- On an access failure, report the selected target, attempted operation, and required boundary crossing. Do not copy the target, follow an alternate path, or reformulate the invocation merely to evade the boundary.
- Do not infer mutation authority from target existence, writable permissions, an ignored destination, or a previous successful invocation.

## Validate Portability

- Run contract tests from the host against target fixtures outside the owning skill directory.
- Test invocation from a working directory other than the target, target paths containing spaces and non-ASCII characters, missing and inaccessible targets, missing project-wide scope, traversal attempts, symlink escape, and paths omitted by the resolved scope.
- Verify that read operations leave host and target unchanged and that write operations affect only declared destinations.
- Exercise help, invalid invocation, each defined status, and deterministic ordering.
- Register source and adjacent tests in the host’s authoritative validation without requiring target integration.

## Avoid Speculative Interfaces

Do not require a universal subcommand hierarchy, JSON output, `--dry-run`, a `PATH`-installed wrapper, standalone distribution, or a machine-readable `--contract` operation without a concrete consumer. Add structured output only when an identified consumer needs it, then give every persistent machine-readable schema an integer version and reject unsupported versions rather than interpreting them heuristically. Add a shared interface only when it makes supported agent invocation or composition materially more deterministic than the owning skill’s documented command contract.
