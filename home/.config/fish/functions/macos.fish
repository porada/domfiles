function macos --description 'Configure macOS properties'
    argparse --min-args=1 --max-args=2 --name=macos -- $argv
    or return

    if not contains -- "$argv[1]" hidden shadow
        __domfiles_print_error "Unsupported property. Use `hidden` or `shadow`"
        return 1
    end

    set --local value

    if set --query argv[2]
        set value (__domfiles_normalize_boolean "$argv[2]")
        or begin
            __domfiles_print_error "Expected a boolean value"
            return 1
        end
    end

    switch "$argv[1]"
        case hidden
            # Show or hide hidden files in Finder
            if not set --query value[1]
                __domfiles_print_and_run \
                    defaults delete com.apple.finder AppleShowAllFiles
                or true
            else
                __domfiles_print_and_run \
                    defaults write com.apple.finder AppleShowAllFiles -bool "$value"
                or return
            end

            __domfiles_print_and_run killall Finder; or true

        case shadow
            # Enable or disable window shadows when taking screenshots
            if not set --query value[1]
                __domfiles_print_and_run \
                    defaults delete com.apple.screencapture disable-shadow
                or true
            else
                if test "$value" = true
                    set value false
                else
                    set value true
                end

                __domfiles_print_and_run \
                    defaults write com.apple.screencapture disable-shadow -bool "$value"
                or return
            end

            __domfiles_print_and_run killall SystemUIServer; or true
    end
end
