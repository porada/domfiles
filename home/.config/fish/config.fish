# Resolve config paths
set --global DOMFILES_FISH_CONFIG_DIR (path dirname (path resolve (status filename)))
set --global DOMFILES (path resolve "$DOMFILES_FISH_CONFIG_DIR/../../..")

# Resolve common paths
set --global DOMFILES_BIN_DIR "$DOMFILES/bin"
set --global DOMFILES_PROJECTS_DIR "$HOME/Projects"

# Set the default editor
set --global --export EDITOR vim -c startinsert

# Don’t clear the screen after a `less` session
set --global --export LESS -FRX

# Don’t keep history between `less` sessions
set --global --export LESSHISTFILE -

# Configure `brew`
set --global --export HOMEBREW_NO_AUTO_UPDATE 1
set --global --export HOMEBREW_NO_ENV_HINTS 1

# Configure `node`
set --global --export NODE_OPTIONS '--trace-uncaught --unhandled-rejections=strict'
set --global --export NODE_REPL_HISTORY "$HOME/.node-history"

# Set `npm` config paths
set --global --export npm_config_globalconfig "$HOME/.config/npm/global.npmrc"
set --global --export npm_config_userconfig "$HOME/.config/npm/user.npmrc"

# Set `pnpm` config paths
set --global --export PNPM_HOME "$HOME/Library/pnpm"
set --global --export pnpm_config_npmrc_auth_file "$npm_config_userconfig"

# Set `zizmor` config path
set --global --export ZIZMOR_CONFIG "$HOME/.config/zizmor/config.yaml"

# Opt out of telemetry
set --global --export DISABLE_TELEMETRY 1
set --global --export DO_NOT_TRACK 1
set --global --export HOMEBREW_NO_ANALYTICS 1
set --global --export VERCEL_TELEMETRY_DISABLED 1

# Set `$PATH`
fish_add_path --path --move /opt/homebrew/sbin
fish_add_path --path --move /opt/homebrew/bin
fish_add_path --path --move "$HOME/.cargo/bin"
fish_add_path --path --move "$PNPM_HOME/bin"
fish_add_path --path --move "$DOMFILES_BIN_DIR"

# Load interactive configuration
if status is-interactive
    source "$DOMFILES_FISH_CONFIG_DIR/aliases.fish"
    source "$DOMFILES_FISH_CONFIG_DIR/colors.fish"
end

# Load local configuration
source "$DOMFILES_FISH_CONFIG_DIR/local.fish"
