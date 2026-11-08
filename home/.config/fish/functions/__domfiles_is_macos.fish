# Returns success on macOS, status 1 elsewhere, or status 2 if detection fails
function __domfiles_is_macos
    if set --query --global __domfiles_cache_is_macos
        return $__domfiles_cache_is_macos
    end

    set --local platform
    uname -s | read --null platform
    test $pipestatus[1] -eq 0
    or return 2

    string match --quiet --regex -- '\ADarwin\n*\z' "$platform"
    set --local exit_code $status

    set --global __domfiles_cache_is_macos $exit_code
    return $exit_code
end
