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

    if test $is_macos -eq 0
        scutil --get ComputerName | read --null name
    else
        uname -n | read --null name
    end

    set --local exit_code $pipestatus[1]

    printf '%s' "$name"
    or return

    test $exit_code -eq 0
    or return $exit_code

    set --global __domfiles_cache_pretty_hostname "$name"
end
