# Prints the given text as informational output
function __domfiles_print_info
    __domfiles_printf '\033[2m%s\033[0m\n' $argv
end
