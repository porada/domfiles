# Publication audit staging

When an isolated copy of tracked `HEAD` is required:

1. Create a tar archive with `git archive --format=tar --output=<temporary-archive> HEAD`.
2. Keep the archive and extraction destination beneath a writable temporary directory supplied to shell commands by the active agent environment.
3. Extract only with `tar -xf <temporary-archive> -C <temporary-directory>`.
4. Do not use alternate archive formats, refs, paths, or broader extraction options.
