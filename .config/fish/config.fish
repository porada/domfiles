# Resolve config paths
set --global DOMFILES_FISH_CONFIG_DIR (path dirname (path resolve (status filename)))
set --global DOMFILES_CONFIG_DIR (path dirname "$DOMFILES_FISH_CONFIG_DIR")
set --global DOMFILES_GIT_CONFIG_DIR "$DOMFILES_CONFIG_DIR/git"
set --global DOMFILES_NPM_CONFIG_DIR "$DOMFILES_CONFIG_DIR/npm"
set --global DOMFILES_ZED_CONFIG_DIR "$DOMFILES_CONFIG_DIR/zed"

# Resolve common paths
set --global DOMFILES (path dirname "$DOMFILES_CONFIG_DIR")
set --global DOMFILES_BIN_DIR "$DOMFILES/bin"
set --global DOMFILES_HOME_DIR "$DOMFILES/home"
set --global DOMFILES_PROJECTS_DIR "$HOME/Projects"
set --global DOMFILES_SKILLS_DIR "$DOMFILES/skills"

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
set --global --export npm_config_globalconfig "$DOMFILES_NPM_CONFIG_DIR/global.npmrc"
set --global --export npm_config_userconfig "$DOMFILES_NPM_CONFIG_DIR/user.npmrc"

# Set `pnpm` config paths
set --global --export PNPM_HOME "$HOME/Library/pnpm"
set --global --export pnpm_config_npmrc_auth_file "$npm_config_userconfig"

# Set `zizmor` config path
set --global --export ZIZMOR_CONFIG "$DOMFILES_CONFIG_DIR/zizmor.yaml"

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
source "$DOMFILES_FISH_CONFIG_DIR/local.fish" >/dev/null 2>&1
