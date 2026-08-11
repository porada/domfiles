# Reset all abbreviations
abbr --erase (abbr --list) >/dev/null 2>&1

# Shorten frequently used commands
abbr c cargo
abbr g git
abbr k killall
abbr n npm
abbr o open
abbr p pnpm
abbr y yarn

# Show hidden files by default when using `ls`
alias ls 'ls -A'

# Ensure `npx` goes through `pnpm`
alias npx 'pnpm dlx'

# Clone a repository into `~/Projects`
function clone
    if test (count $argv) -eq 1; and test -e "$argv[1]"
        set argv[1] (path resolve "$argv[1]")
    end

    cd -P "$DOMFILES_PROJECTS_DIR"; or return

    if test (count $argv) -eq 1
        set -l repository (basename "$argv[1]")

        if string match --quiet '*.git' "$repository"
            set repository (string replace -r '^.*:' '' -- "$repository")
            set repository (basename "$repository" .git)
        end

        git clone "$argv[1]" && cd "$repository"
    else
        git clone $argv
    end
end
