# Runs `$argv`, printing the command beforehand
function __domfiles_print_and_run
    if not set --query argv[1]
        __domfiles_print_error '`__domfiles_print_and_run` requires at least one argument'
        return 1
    end

    __domfiles_print_command $argv
    or return

    command $argv
end
