# Returns success on macOS
function __domfiles_is_macos
    test "$(uname -s)" = Darwin
end
