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

# Clone a repository into `~/Projects` and navigate into it
function clone
    cd -P "$DOMFILES_PROJECTS_DIR"; or return

    if test (count $argv) -eq 1
        git clone $argv && cd (basename $argv .git)
    else
        git clone $argv
    end
end
