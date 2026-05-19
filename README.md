[![](https://img.shields.io/badge/shell-fish-informational?style=flat-square)](https://fishshell.com) [![](https://img.shields.io/github/actions/workflow/status/porada/domfiles/test.yaml?style=flat-square)](https://github.com/porada/domfiles/actions/workflows/test.yaml)

# domfiles

Minimal dotfiles for Apple Silicon Macs.

Use them as your own starting point or keep them as-is for effortless long-term maintenance.

## Install

```sh
git clone https://github.com/porada/domfiles.git ~/.domfiles
```

Cloning over HTTPS is recommended to avoid authentication issues on a fresh system. [Once SSH is set up](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent), the remote will be updated automatically to `git@github.com`.

Use `domfiles sync` to link all the dotfiles and keep everything up to date over time:

```sh
~/.domfiles/bin/domfiles sync
```

You can place the repository wherever you like, but `~/.domfiles` is the recommended location.

### Prerequisites

[**Homebrew**](https://brew.sh) is the only required dependency. Make sure it’s installed before setting up the repository.

## License

MIT © [Dom Porada](https://dom.engineering)
