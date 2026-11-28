# Interrupted Sync Recovery

Rerun the interrupted sync command first. Before starting a stage, `domfiles sync` checks the checkout for an interrupted repository update. If recovery reports an unexpected change, move it aside or resolve the reported issue and retry. If a check stops recovery before rollback, keep the marker and saved refs, resolve what prevented the check, and rerun the command. To keep the checkout in its current state, verify the files and staged changes before removing the marker named in the message.

If the update stopped while Git was rewriting `domfiles`, `domfiles-sync`, `domlib`, or a sync module, sync may be unable to start recovery. In that case, use the checkout’s marker and private refs to restore it with Git alone.

1. Wait for sync and its Git processes to exit, and keep other syncs from starting during the restore. From the root of the affected checkout, run these commands in a POSIX shell such as `sh` to find its marker and private refs.

    ```sh
    checkout=$(pwd -P)
    id=$(printf '%s' "$checkout" | cksum | awk '{ print $1 "-" $2 }')
    marker="$HOME/.local/state/domfiles/update-$id"
    refs="refs/domfiles-sync/$id"
    awk '
      NR == 2 { print "Checkout path:", $0 }
      NR == 3 { print "Phase:", $0 }
      NR == 4 { print "Original commit:", $0 }
      NR == 5 { print "Saved changes commit:", $0 }
      NR == 6 { print "Original working tree:", $0 }
      NR == 8 { print "Updated commit:", $0 }
    ' "$marker"
    git symbolic-ref --quiet HEAD
    git rev-parse HEAD
    ```

    Confirm that **Checkout path** matches the output of `pwd -P`, then use **Phase** to choose the next step. **Saved changes commit** is blank if the checkout had no tracked changes to restore.

    - `switching` means the switch may be incomplete. Continue with step 2 only if `HEAD` is on `refs/heads/master` at the original or updated commit. If `HEAD` is detached, on another branch, or at a different commit, leave it in place and keep the marker and refs. Inspect the original commit with `git show --stat "$refs/original"` and any saved changes with `git stash show -p "$refs/saved"`. Reconcile the saved state with the checkout by hand, then go to step 4.
    - `prepared` means the switch had yet to start. `switched` means it finished. `restored` means sync already rolled it back. For any of these three phases, skip directly to step 4. Some private refs may already be gone.

2. Check `git status --short --untracked-files=all`. If another Git operation is in progress, finish resolving it first. Back up later edits and any untracked or ignored files you want to keep outside the checkout. The reset replaces tracked files and can overwrite files in their way. Verify that `git rev-parse --verify "$refs/original"` matches **Original commit**. If **Saved changes commit** has a value, verify that `git rev-parse --verify "$refs/saved"` matches it. That commit holds the original staged and unstaged changes.

    Compare **Original working tree** with the update result to identify files the switch may have added.

    ```sh
    git diff --no-renames --diff-filter=A --name-only "$(sed -n '6p' "$marker")" "$refs/updated" --
    ```

3. Restore the original commit. If Git reports a lock file, first confirm that no process is using it. Remove only a stale lock and retry the command.

    ```sh
    git reset --hard "$refs/original"
    ```

    Before cleaning up an added path, check its parent directories from the checkout root down. Skip that path if a parent is missing, is a file, or is a symlink. If all parents are directories and the switch left an untracked file at the added path, move it outside the checkout. Use `rmdir` to remove any empty directories the update created along that path, starting with the file’s parent and working back toward the checkout root. Stop at the first non-empty or pre-existing directory. Keep unrelated untracked files and directories.

    If **Saved changes commit** had a value in step 1, reapply the saved changes with their staged state.

    ```sh
    git stash apply --index "$refs/saved"
    ```

4. Check `git status --short --untracked-files=all`, `git diff --cached`, and `git diff`. Once the branch, staged changes, and files are as you want them, remove the marker and release any remaining private refs. The next repository update cleans up the private worktree and scratch directory before reusing them.

    ```sh
    rm -- "$marker"
    ```

    ```sh
    git update-ref -d "$refs/original"
    git update-ref -d "$refs/saved"
    git update-ref -d "$refs/updated"
    ```
