function up --description 'Prevent the system and disk from idle sleeping'
    __domfiles_continue_on_macos (status function); or return

    if set --query argv[1]
        __domfiles_print_and_run caffeinate -im $argv
        return $status
    end

    command pgrep -q -f '(^|/)caffeinate -im$'
    set --local pgrep_status $status

    if test $pgrep_status -eq 0
        __domfiles_print_info 'The system is already caffeinated'
        return
    end

    if test $pgrep_status -ne 1
        return $pgrep_status
    end

    __domfiles_print_and_run caffeinate -im
end
