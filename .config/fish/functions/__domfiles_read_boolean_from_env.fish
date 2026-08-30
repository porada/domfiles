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

    set --local variable_name $argv[1]
    set --local value

    if set --query --export "$variable_name"
        set value $$variable_name
    end

    if test -z "$value"
        echo "$argv[2]"
        return 0
    end

    for accepted in 1 on true yes
        if string match --quiet --ignore-case -- "$accepted" "$value"
            echo true
            return 0
        end
    end

    for accepted in 0 false no off
        if string match --quiet --ignore-case -- "$accepted" "$value"
            echo false
            return 0
        end
    end

    __domfiles_print_error \
        "`$variable_name` has an unsupported boolean value. Leave it empty to use the default"
    return 1
end
