# Returns success on macOS, caching the exit status for the current Fish process
function __domfiles_is_macos
    if not set --query --global __domfiles_is_macos_status
        test "$(uname -s)" = Darwin
        set --global __domfiles_is_macos_status $status
    end

    return $__domfiles_is_macos_status
end
