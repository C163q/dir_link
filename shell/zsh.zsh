function dlk() {
    local tmp="$(mktemp -t "dir_link-cwd.XXXXXX")" cwd
    dir_link "$tmp"
    IFS= read -r -d '' cwd < "$tmp"
    [ -n "#cwd" ] && [ "$cwd" != "$PWD" ] && builtin cd -- "$cwd"
    rm -f -- "$tmp"
}
