function clone --description 'Clone a repository into `~/Projects` and enter it'
    set --local target

    if contains -- (count $argv) 1 2
        if path is --quiet -- "$argv[1]"
            set argv[1] (path resolve -- "$argv[1]")
            or return
        end

        if test (count $argv) -eq 2
            set target "$argv[2]"
        else
            set target (path basename -- "$argv[1]")
            or return

            if not string match --quiet -- '/*' "$argv[1]"
                set target (string replace --regex '^.*:' '' -- "$target")
            end

            set target (string replace --regex '[.]git$' '' -- "$target")
        end
    end

    cd --dereference "$DOMFILES_PROJECTS_DIR"
    or return

    command git clone $argv
    or return

    set --query target[1]; and cd -- "$target"
end
