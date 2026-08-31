function ffmpeg-wav-png --description 'Convert paired WAV and PNG files in the current directory to videos'
    if not command --query ffmpeg
        __domfiles_print_error '`ffmpeg` is missing'
        return 1
    end

    # Keep the `--preset=<preset>` form and ignore all other arguments
    set --local preset_params

    for param in $argv
        if string match --quiet -- '--preset=*' "$param"
            set --append preset_params "$param"
        end
    end

    argparse 'preset=' -- $preset_params
    set --local preset "$_flag_preset"

    if test -z "$preset"
        __domfiles_print_error '`--preset` is missing'
        return 1
    end

    switch "$preset"
        case instagram
            for audio in ./*.wav
                if not test -f "$audio"; or test -L "$audio"
                    continue
                end

                set --local base $(path change-extension '' -- "$audio")
                set --local image "$base.png"
                set --local video "$base.mov"

                if not test -f "$image"
                    __domfiles_print_error "Missing image for `$audio`"
                    return 1
                end

                __domfiles_print_heading "Generating `$video` for Instagram"

                # Limit Instagram clips to 60 seconds
                __domfiles_print_and_run ffmpeg \
                    -nostdin -y -framerate 1 -loop 1 -i "$image" \
                    -i "$audio" -r 1 \
                    -vf 'format=rgb24,scale=in_range=full:out_range=tv' \
                    -c:v prores_ks -profile:v 3 -pix_fmt yuv444p10le -color_range 1 \
                    -c:a copy -t 60 -shortest "$video"
                or return
            end

        case youtube
            for audio in ./*.wav
                if not test -f "$audio"; or test -L "$audio"
                    continue
                end

                set --local base $(path change-extension '' -- "$audio")
                set --local image "$base.png"
                set --local video "$base.mkv"

                if not test -f "$image"
                    __domfiles_print_error "Missing image for `$audio`"
                    return 1
                end

                __domfiles_print_heading "Generating `$video` for YouTube"

                __domfiles_print_and_run ffmpeg \
                    -nostdin -loop 1 -framerate 1 -i "$image" \
                    -i "$audio" \
                    -vf 'format=yuv420p,eq=gamma=1.18,scale=out_color_matrix=bt709' \
                    -c:v libx264 -crf 0 -preset veryslow -pix_fmt yuv420p \
                    -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 \
                    -c:a copy -shortest -y "$video"
                or return
            end

        case '*'
            __domfiles_print_error "Unsupported preset: `$preset`"
            return 1
    end
end
