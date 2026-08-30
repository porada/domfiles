# Prints the given text as a warning
function __domfiles_print_warning
    __domfiles_printf '\033[1m▴ %s\033[0m\n' $argv
end
