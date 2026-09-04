# Prints a normalized boolean. Returns failure when `$argv[1]` is unsupported
function __domfiles_normalize_boolean
    if test (count $argv) -ne 1
        __domfiles_print_error '`__domfiles_normalize_boolean` requires one value'
        return 1
    end

    for accepted in 1 true on yes
        if string match --quiet --ignore-case -- "$accepted" "$argv[1]"
            echo true
            return 0
        end
    end

    for accepted in 0 false off no
        if string match --quiet --ignore-case -- "$accepted" "$argv[1]"
            echo false
            return 0
        end
    end

    return 1
end
