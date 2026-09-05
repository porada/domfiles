# Requests confirmation with `$argv[1]`. Returns success when confirmed
function __domfiles_confirm
    if test (count $argv) -ne 1
        __domfiles_print_error '`__domfiles_confirm` requires one argument'
        return 1
    end

    set --local whitespace (printf ' \t')

    while true
        __domfiles_print_warning --inline -- "$argv[1]"
        __domfiles_print_info --inline -- y/n

        read --local --prompt-str '' response
        or return 1

        set response (string trim --chars "$whitespace" -- "$response")

        switch "$response"
            case y Y
                return 0
            case n N
                return 1
        end
    end
end
