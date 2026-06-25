function fish_prompt
    set -l exit_code $status

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
    __fish_prompt_caret $exit_code

    set_color $fish_color_normal
end

function __fish_prompt_newline
    printf '\n'
end

function __fish_prompt_host
    test -n "$SSH_CONNECTION"; and printf '%s@%s ' $USER (hostname -s)
end

function __fish_prompt_pwd
    set -l fish_prompt_pwd_dir_length 0
    test (id -u) -eq 0; and set_color $fish_color_cwd_root
    printf '%s ' (prompt_pwd)
end

function __fish_prompt_git
    set -g __fish_git_prompt_showdirtystate 1
    set -g __fish_git_prompt_char_dirtystate '·'
    set -g __fish_git_prompt_char_stagedstate '·'
    set -g __fish_git_prompt_char_stateseparator ''

    fish_git_prompt '%s'
end

function __fish_prompt_caret
    test $argv[1] -ne 0; and set_color $fish_color_error
    printf '$ '
end
