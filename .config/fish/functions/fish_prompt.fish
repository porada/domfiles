# Renders the prompt. Styles the command marker based on the previous command’s
# exit status
function fish_prompt
    set --local exit_status $status

    set_color $fish_color_normal
    set_color $fish_color_user
    __fish_prompt_newline
    __fish_prompt_host

    set_color $fish_color_normal
    set_color $fish_color_cwd
    __fish_prompt_pwd

    set_color $fish_color_normal
    set_color $fish_color_operator
    __fish_prompt_git
    __fish_prompt_newline

    set_color $fish_color_normal
    set_color $fish_color_comment
    __fish_prompt_caret $exit_status

    set_color $fish_color_normal
end

# Prints a prompt newline
function __fish_prompt_newline
    printf '\n'
end

# Prints remote host context for SSH sessions
function __fish_prompt_host
    test -n "$SSH_CONNECTION"; and printf '%s@%s ' $USER (prompt_hostname)
end

# Prints the working directory with root-user styling
function __fish_prompt_pwd
    fish_is_root_user; and set_color $fish_color_cwd_root
    printf '%s ' (prompt_pwd --dir-length 0)
end

# Prints repository state for the prompt
function __fish_prompt_git
    __fish_git_prompt_char_dirtystate='·' \
        __fish_git_prompt_char_stagedstate='·' \
        __fish_git_prompt_char_stateseparator='' \
        __fish_git_prompt_showdirtystate=1 \
        fish_git_prompt '%s'
end

# Prints the command marker with failure styling
function __fish_prompt_caret
    test "$argv[1]" -ne 0; and set_color $fish_color_error
    printf '$ '
end
