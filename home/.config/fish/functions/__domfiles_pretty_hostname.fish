# Prints the human-readable host name
function __domfiles_pretty_hostname
    if set --query --global __domfiles_cache_pretty_hostname
        printf '%s' "$__domfiles_cache_pretty_hostname"
        return
    end

    __domfiles_is_macos
    set --local is_macos $status
    test $is_macos -gt 1
    and return $is_macos

    set --local name
    set --local exit_code 0

    if test $is_macos -eq 0
        scutil --get ComputerName | read --null name
        set exit_code $pipestatus[1]
    else
        test -r /etc/machine-info
        and set name (
            string replace --filter --regex -- '^PRETTY_HOSTNAME=(["\']?)(.*)\1$' '$2' \
                </etc/machine-info
        )[-1]

        if test -n "$name"
            set name "$name"\n
        else
            uname -n | read --null name
            set exit_code $pipestatus[1]
        end
    end

    printf '%s' "$name"
    or return

    test $exit_code -eq 0
    or return $exit_code

    set --global __domfiles_cache_pretty_hostname "$name"
end
