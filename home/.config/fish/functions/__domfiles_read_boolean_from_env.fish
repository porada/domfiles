# Prints a normalized boolean from environment variable `$argv[1]`. Uses
# `$argv[2]` when unset or empty
function __domfiles_read_boolean_from_env
    if test (count $argv) -ne 2
        __domfiles_print_error \
            '`__domfiles_read_boolean_from_env` requires an environment variable name and boolean default'
        return 1
    end

    if not __domfiles_is_boolean "$argv[2]"
        __domfiles_print_error \
            '`__domfiles_read_boolean_from_env` requires a `true` or `false` default'
        return 1
    end

    # Read the exported value before introducing function-scoped state
    if set --query --export -- "$argv[1]"
        set --function value "$$argv[1]"
    else
        set --function value ''
    end

    if test -z "$value"
        echo "$argv[2]"
        return 0
    end

    if not __domfiles_normalize_boolean "$value"
        __domfiles_print_error "`$argv[1]` has an unsupported boolean value"
        return 1
    end
end
