# Text and Record Boundaries

Choose whether command output represents lines, one opaque document, or delimited records before capturing it. Use `$(command)` for command substitution, including inside double quotes. Use `string split` or `string split0` when another delimiter defines records. Use `string collect` when output must be collected without newline splitting.

```fish
set --local lines $(command tool)
set --local document "$(command tool)"
set --local exact_document $(
    command tool |
        string collect --allow-empty --no-trim-newlines
)
```

A normal command substitution splits on newlines and produces no elements for empty output. A final terminating newline does not create another empty element. A quoted substitution produces exactly one argument but still trims trailing newlines. A final `string collect --allow-empty --no-trim-newlines` preserves empty output as one element and retains trailing newlines.

Treat JSON, SQL, generated source, and similar opaque documents as text rather than line lists unless their interface says otherwise.

Use NUL-delimited streams when records can contain newlines, especially for filenames:

```fish
find . -type f -print0 |
    while read --null file
        process_file $file
    end

set --local files $(
    find . -type f -print0 |
        string split0
)
```

Keep `string split0` as the final pipeline stage when collecting a NUL stream into a Fish list so its element boundaries survive command substitution. Use `path`’s `--null-in` and `--null-out` options while NUL-delimited data remains a stream. Do not send NUL output directly to a terminal or command substitution. Pipe it to a final `string split0` when collecting it.

Direct `path` output captured by command substitution preserves item boundaries, including embedded newlines. An intervening command can serialize those boundaries away. Ordinary `path` standard input remains newline-delimited unless NUL input is selected or detected.

## Official Sources

Expansion and record behavior are documented in the [Fish language](https://fishshell.com/docs/current/language.html), [`path` reference](https://fishshell.com/docs/current/cmds/path.html), [`string collect` reference](https://fishshell.com/docs/current/cmds/string-collect.html), and [`string split0` reference](https://fishshell.com/docs/current/cmds/string-split0.html).
