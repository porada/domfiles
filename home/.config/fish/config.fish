# Set consistent locale
set --local overrides (set --global --names | string match 'LC_*')
set --query overrides[1]; and set --erase --global $overrides
set --global --export LANG C.UTF-8

# Resolve current configuration path
set --local config (status dirname)

# Resolve domfiles’ paths
set --global DOMFILES (path dirname (path dirname (path dirname (path resolve "$config"))))
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

# Configure `gh`
set --global --export GH_NO_UPDATE_NOTIFIER 1

# Configure `node`
set --global --export NODE_OPTIONS '--trace-uncaught --unhandled-rejections=strict'
set --global --export NODE_REPL_HISTORY "$HOME/.node-history"

# Set `npm` config paths
set --global --export npm_config_globalconfig "$HOME/.config/npm/global.npmrc"
set --global --export npm_config_userconfig "$HOME/.config/npm/user.npmrc"
set --global --export pnpm_config_npmrc_auth_file "$npm_config_userconfig"

# Set `zizmor` config path
set --global --export ZIZMOR_CONFIG "$HOME/.config/zizmor/config.yaml"

# Opt out of telemetry
set --global --export DISABLE_TELEMETRY 1
set --global --export DISABLE_ERROR_REPORTING 1
set --global --export DO_NOT_TRACK 1
set --global --export HOMEBREW_NO_ANALYTICS 1
set --global --export VERCEL_TELEMETRY_DISABLED 1

# Set `$PATH`
source "$config/path.fish"

# Load interactive configuration
if status is-interactive
    source "$config/aliases.fish"
    source "$config/colors.fish"
end

# Load local configuration
source "$config/local.fish"
