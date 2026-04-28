# Resolve config paths
set -g DOMFILES_FISH_CONFIG_DIR (path dirname (path resolve (status filename)))
set -g DOMFILES_CONFIG_DIR (path dirname "$DOMFILES_FISH_CONFIG_DIR")
set -g DOMFILES_NPM_CONFIG_DIR "$DOMFILES_CONFIG_DIR/npm"

# Resolve common paths
set -g DOMFILES (path dirname "$DOMFILES_CONFIG_DIR")
set -g DOMFILES_BIN_DIR "$DOMFILES/bin"
set -g DOMFILES_PROJECTS_DIR "$HOME/Projects"

# Set the default editor
set -x EDITOR 'vim -c startinsert'

# Don’t clear the screen after a `less` session
set -x LESS -FRX
set -x PAGER "less $LESS"
set -x MANPAGER "less $LESS"

# Don’t keep history between `less` sessions
set -x LESSHISTFILE -

# Configure `node`
set -x NODE_OPTIONS '--trace-uncaught --unhandled-rejections=strict'

# Enable persistent REPL history for `node`
set -x NODE_REPL_HISTORY "$HOME/.node_history"
set -x NODE_REPL_MODE sloppy

# Set `npm` config paths
set -x npm_config_globalconfig "$DOMFILES_NPM_CONFIG_DIR/global.npmrc"
set -x npm_config_userconfig "$DOMFILES_NPM_CONFIG_DIR/user.npmrc"

# Set `pnpm` config paths
set -x PNPM_HOME "$HOME/Library/pnpm"
set -x pnpm_config_npmrc_auth_file "$npm_config_userconfig"

# Opt out of telemetry
set -x DO_NOT_TRACK 1
set -x HOMEBREW_NO_ANALYTICS 1

# Disable automatic `brew` updates
set -x HOMEBREW_NO_AUTO_UPDATE 1
set -x HOMEBREW_NO_ENV_HINTS 1

# Set `$PATH`
fish_add_path --path --move /opt/homebrew/sbin
fish_add_path --path --move /opt/homebrew/bin
fish_add_path --path --move "$PNPM_HOME"
fish_add_path --path --move "$PNPM_HOME/bin"
fish_add_path --path --move "$DOMFILES_BIN_DIR"

# Load domfiles
. "$DOMFILES_FISH_CONFIG_DIR/aliases.fish"
. "$DOMFILES_FISH_CONFIG_DIR/colors.fish"
. "$DOMFILES_FISH_CONFIG_DIR/local.fish" >/dev/null 2>&1
