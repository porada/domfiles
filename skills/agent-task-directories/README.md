[![](https://skills.sh/b/porada/domfiles)](https://www.skills.sh/porada/domfiles/agent-task-directories)

# agent-task-directories

Temporary agent files should have a predictable location in your project and be cleaned up when no longer needed. Git-ignored task directories are the default, with an exception for a single short-lived file.

This skill defines task-specific directories for experiments, helper scripts, and notes, with clear rules for ownership, reuse, and cleanup. Agents can retain what they need and remove what they don’t without disturbing another task’s work.

## Install

```sh
npx skills add porada/domfiles --skill agent-task-directories
```

```sh
gh skill install porada/domfiles agent-task-directories
```

## License

MIT © [Dom Porada](https://dom.engineering)
