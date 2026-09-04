# Reset all abbreviations
set --local abbreviations (abbr --list)
set --query abbreviations[1]; and abbr --erase $abbreviations

# Shorten frequently used commands
abbr --add c cargo
abbr --add g git
abbr --add k killall
abbr --add n npm
abbr --add o open
abbr --add p pnpm
abbr --add y yarn

# Show hidden files by default when using `ls`
alias ls 'ls -A'

# Ensure `npx` goes through `pnpm`
alias npx 'pnpm dlx'
