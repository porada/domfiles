# Treat empty data location overrides as unset
test -z "$PNPM_HOME"; and set --erase --global PNPM_HOME
test -z "$XDG_DATA_HOME"; and set --erase --global XDG_DATA_HOME

# Default to macOS paths
set --local brew_path /opt/homebrew
set --local pnpm_path "$HOME/Library/pnpm"

if not __domfiles_is_macos
    set brew_path /home/linuxbrew/.linuxbrew
    set pnpm_path "$HOME/.local/share/pnpm"
end

# Honor existing data location overrides
if set --query PNPM_HOME
    set pnpm_path "$PNPM_HOME"
else if set --query XDG_DATA_HOME
    set pnpm_path "$XDG_DATA_HOME/pnpm"
end

# Prevent `fish_add_path` from skipping the `pnpm` entry
set --local pnpm_bin_path "$pnpm_path/bin"
path is --type dir "$pnpm_bin_path"; or mkdir -p "$pnpm_bin_path"

# Set `$PATH`
fish_add_path --path --move "$brew_path/sbin"
fish_add_path --path --move "$brew_path/bin"
fish_add_path --path --move "$pnpm_bin_path"
fish_add_path --path --move "$HOME/.cargo/bin"
fish_add_path --path --move "$HOME/.local/bin"
