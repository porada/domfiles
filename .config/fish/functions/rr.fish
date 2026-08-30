function rr --description 'Recursively remove files or directories that match the given pattern'
    if test (count $argv) -ne 1
        __domfiles_print_error '`rr` requires one argument'
        return 1
    end

    if not __domfiles_confirm "Remove all instances of `$argv[1]`?"
        return 0
    end

    __domfiles_print_and_run find . -mindepth 1 -name "$argv[1]" -prune \
        -exec rm -rf '{}' + -print
end
