# Prints text with format `$argv[2]`. Applies style `$argv[1]` only to
# terminal output. Replaces `$HOME` with `~`. `--inline` uses a trailing space
# instead of a newline
function __domfiles_print_styled
    if not set --query argv[1]
        return 0
    end

    if not set --query argv[2]
        __domfiles_print_error '`__domfiles_print_styled` requires a style and format'
        return 1
    end

    set --local style $argv[1]
    set --local format $argv[2]
    set --erase argv[1..2]

    if set --query argv[1]; and test "$argv[1]" = --inline
        set format "$(string replace --regex -- '(?:\\\\n)?\\z' ' ' "$format")"
        set --erase argv[1]
    end

    if set --query argv[1]; and test "$argv[1]" = --
        set --erase argv[1]
    end

    if test -z "$format"
        return 0
    end

    set --local text "$argv"
    set text "$(string replace --all -- "$HOME" '~' "$text")"

    # Run `test` externally to follow function-local redirections
    if not command test -t 1
        set style ''
    end

    if test -n "$style"
        printf '\033[%sm' "$style"
        or return
    end

    printf "$format" "$text"
    or return

    if test -n "$style"
        printf '\033[0m'
    end
end
