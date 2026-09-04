# Prints `$argv` as a shell-quoted command
function __domfiles_print_command
    set --local suppressed (__domfiles_read_boolean_from_env DOMFILES_SUPPRESSED false)
    or return

    if test "$suppressed" = true; and not __domfiles_is_ci
        return 0
    end

    set --local printed_command (string join ' ' -- (string escape -- $argv))
    __domfiles_print_info "\$ $printed_command" >&2
end
