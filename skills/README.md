[![](https://skills.sh/b/porada/domfiles)](https://www.skills.sh/porada/domfiles)

# Agent Skills

A collection of independently installable skills I use as my daily driver. Also available through [skills.sh](https://www.skills.sh/porada/domfiles).

## Install

```sh
npx skills add porada/domfiles --global
```

```sh
gh skill install porada/domfiles --scope user
```

Global installation is recommended for the best experience. Both `skills` and `gh skill` install only the skills you choose. Neither sets up any other tooling or configuration from this repository.

## Featured Skills

### [agent-task-relay](agent-task-relay)

Paste findings from another thread and see your agent take it from there. Or relay work to another agent just as easily.

```sh
npx skills add porada/domfiles --skill agent-task-relay
```

```sh
gh skill install porada/domfiles agent-task-relay
```

### [human-facing-writing](human-facing-writing)

Raise the standard of every piece your agent writes, from general long-form text to small technical copy. No more slop under your name.

```sh
npx skills add porada/domfiles --skill human-facing-writing
```

```sh
gh skill install porada/domfiles human-facing-writing
```

### [fish-shell-scripting](fish-shell-scripting)

Write Fish as intended: without bashisms.

```sh
npx skills add porada/domfiles --skill fish-shell-scripting
```

```sh
gh skill install porada/domfiles fish-shell-scripting
```

## Other Skills

- [**posix-shell-scripting**](posix-shell-scripting)
- [**release-notes-for-humans**](release-notes-for-humans)
- [**simple-github-cli**](simple-github-cli)

The `.domfiles-*` skills are tied to this repository’s agent configuration. They aren’t ready for standalone installation.

## License

MIT © [Dom Porada](https://dom.engineering)
