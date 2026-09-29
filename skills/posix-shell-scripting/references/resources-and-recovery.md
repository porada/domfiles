# Resources and Recovery

## Temporary Resources

Use the first boundary that preserves the contract. The sequence moves from direct transfer to durable staging:

1. Pass separate arguments directly.
2. Stream through standard input, standard output, a pipe, or redirection.
3. Use command substitution for suitable scalar text.
4. Reuse a destination or work directory the operation already owns.
5. Create a temporary resource only when the receiving interface requires a pathname, exact data cannot survive a shell variable, the operation needs multiple passes, statuses must be checked independently, or atomic replacement or rollback requires staging.

Do not introduce a temporary resource merely to simplify quoting or control flow.

When temporary storage is necessary and the target provides `mktemp`, keep creation, use, and cleanup in one subshell-isolated operation. Set `umask 077` immediately before creating a private directory under `${TMPDIR:-/tmp}`. Do not change the mask for an entire executable merely because the entrypoint owns cleanup. Never check whether a predictable pathname is unused and then create it because that sequence has a race.

## Recovery and Compensation

Follow the script’s declared partial-failure and recovery contract. A multi-step mutation does not by itself require recovery infrastructure or imply atomicity. When the contract requires recovery, capture the original state needed for exact restoration before the first mutation. Reject preexisting operation state that compensation could destroy, along with any condition that would make destructive compensation unsafe. Establish how the workflow will prove ownership and unchanged state for every shared resource it may compensate.

Compensate only effects the workflow can identify as its own. An attempted command does not prove that its effect occurred. A before-and-after inventory does not establish ownership of a new shared entry. Rely on a creation-returned stable identifier alone only when the identified resource is immutable and the creation result proves ownership. If ownership or safe compensation without overwriting others’ changes cannot be established, leave the shared resource intact and stop.

When the established concurrency or risk model or a promised atomic guarantee requires atomic state protection for a mutable shared resource, hold an exclusive lock from the ownership and state checks through compensation, or use a conditional mutation that succeeds only if the resource still matches the observed version. Do not add these mechanisms merely to strengthen recoverability for ordinary cooperative work.

Capture the original failure status before recovery. If recovery succeeds, return that original status. If recovery fails, report both failures, return the recovery failure’s status, and preserve any resource needed for manual recovery. Verify the recovered state before restoring user data or starting another mutation.

Write a completion marker only after establishing the exact condition it represents, including any recovery that condition requires.

Repeat any remaining destructive-safety preflight after earlier recovery steps because they may have changed the inspected state. When atomic state protection is required, perform the decisive mutable-state check under the lock or as part of the conditional mutation, not as a separate preflight.

## Cleanup Ownership

Let an executable entrypoint own global cleanup traps. A sourceable library should release its resources before returning instead of replacing the caller’s trap configuration. Keep trap bodies simple, and preserve the operation’s status deliberately.
