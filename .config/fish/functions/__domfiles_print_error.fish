# Prints the given text as an error
function __domfiles_print_error
    __domfiles_printf '\033[0;31m× %s\033[0m\n' $argv >&2
end
