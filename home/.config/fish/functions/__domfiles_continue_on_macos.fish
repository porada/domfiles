# Returns success on macOS. Otherwise prints a macOS-only notice for command
# `$argv[1]` and returns status 1
function __domfiles_continue_on_macos
    if test (count $argv) -ne 1; or test -z "$argv[1]"
        __domfiles_print_error '`__domfiles_continue_on_macos` requires one command name'
        return 1
    end

    if __domfiles_is_macos
        return
    end

    set --local command (path basename -- "$argv[1]")
    __domfiles_print_info -- "`$command` is a macOS-only command"
    return 1
end
