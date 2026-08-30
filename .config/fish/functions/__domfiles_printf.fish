# Prints the given text, replacing `$HOME` with `~`
function __domfiles_printf
    if not set --query argv[1]
        return 0
    end

    set --local format $argv[1]
    set --erase argv[1]

    if test -z "$format"
        return 0
    end

    set --local text "$argv"
    set text "$(string replace --all -- "$HOME" '~' "$text")"

    printf "$format" "$text"
end
