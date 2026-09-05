[![](https://img.shields.io/badge/shell-fish-informational?style=flat-square)](https://fishshell.com) [![](https://img.shields.io/badge/system-macOS-informational?style=flat-square)](https://www.apple.com/os/macos/)

# ~

The dotfiles I use across all my machines.

## Install

```sh
git clone https://github.com/porada/domfiles.git ~/.domfiles
```

Cloning over HTTPS is recommended to avoid authentication issues on a fresh system. [Once SSH is set up](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent), `domfiles sync` automatically configures the remote to connect over SSH.

```sh
~/.domfiles/home/.local/bin/domfiles-sync
```

## License

MIT © [Dom Porada](https://dom.engineering)
