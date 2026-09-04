# Returns success on CI
function __domfiles_is_ci
    test -n "$CI"; or test -n "$GITHUB_ACTIONS"
end
