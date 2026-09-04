# Returns success when `$argv[1]` is exactly `true` or `false`
function __domfiles_is_boolean
    if test (count $argv) -ne 1
        __domfiles_print_error '`__domfiles_is_boolean` requires one value'
        return 1
    end

    test "$argv[1]" = true; or test "$argv[1]" = false
end
