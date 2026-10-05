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

Global installation is recommended for the best experience. Both [`skills`](https://www.skills.sh/docs/cli) and [`gh skill`](https://cli.github.com/manual/gh_skill) install only the skills you choose. Neither sets up any other tooling or configuration from this repository.

## Available Skills

### Shell Scripting

Work with the shell, not against it.

- [**fish-shell-scripting**](fish-shell-scripting)
- [**posix-shell-scripting**](posix-shell-scripting)

### Writing

Bring meaning into focus and make technical writing easy to follow.

- [**human-facing-writing**](human-facing-writing)
- [**release-notes-for-humans**](release-notes-for-humans)

### Agent Coordination

Keep agent work organized and carry context between conversations.

- [**agent-task-directories**](agent-task-directories)
- [**agent-task-relay**](agent-task-relay)
- [**verify-findings**](verify-findings)

### Dependencies

Choose dependencies deliberately and understand their impact.

- [**intentional-dependency-choice**](intentional-dependency-choice)

### Git and GitHub

Keep commit history clear and GitHub work focused.

- [**sensible-commit-flow**](sensible-commit-flow)
- [**sensible-contribution-flow**](sensible-contribution-flow)
- [**simple-github-cli**](simple-github-cli)

## Internal Skills

`.dom-*` skills add my personal conventions on top of published skills, while `.domfiles-*` skills depend on this repository’s [configuration](../home) or need more development. None of those are suitable for standalone installation.

## License

MIT © [Dom Porada](https://dom.engineering)
