# Clean up legacy abbreviations
abbr --erase (abbr --list) >/dev/null 2>&1

# Shorten frequently used commands
abbr g git
abbr o open
abbr k killall
abbr n npm
abbr p pnpm
abbr y yarn

# Show hidden files by default when using `ls`
alias ls 'ls -A'

# Ensure `npx` goes through `pnpm`
alias npx 'pnpm dlx'

# Clone a repository and navigate into it
function clone
    cd -P "$DOMFILES_PROJECTS_DIR"

    if test (count $argv) -eq 1
        git clone $argv && cd (basename $argv .git)
    else
        git clone $argv
    end
end

# Clean up Fish universal variables
function __domfiles_clean_fish_variables
    for name in (set --names --universal)
        switch "$name"
            case "fish_color_*" "fish_pager_color_*"
                set --erase --universal "$name"
        end
    end

    # Fish 4.8+ no longer creates `__fish_initialized`
    if string match --quiet '4.8.*' "$version"
        set --erase --universal __fish_initialized
    end

    true
end
