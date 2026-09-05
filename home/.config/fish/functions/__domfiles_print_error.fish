# Prints the given text as an error
function __domfiles_print_error
    __domfiles_print_styled '0;31' '× %s\n' -- $argv >&2
end
