function ip --description 'Show local and public IP addresses'
    set --local interfaces (command ifconfig -a inet)
    or return

    string match --regex --groups-only -- '^\s*inet\s+(\S+)' $interfaces |
        string match --invert -- '127.0.0.1'

    set --local public_ip (command dig +short myip.opendns.com @resolver1.opendns.com 2>/dev/null)
    or begin
        __domfiles_print_error 'Failed to resolve the public IP address'
        return 1
    end

    if not set --query public_ip[1]
        __domfiles_print_error 'No public IP address was returned'
        return 1
    end

    printf '%s\n' $public_ip
end
