1. `nano ~/.zshrc` or `vim ~/.zshrc`  

2. Append the following content and replace the value of rua_executable with the path to your rua executable: 
```zsh
# ----------------------rua part--------------------------------
_rua_insert_on_next_prompt() {
    if [[ -n "$RUA_SELECTED_COMMAND" ]]; then
        print -z "$RUA_SELECTED_COMMAND"
        unset RUA_SELECTED_COMMAND
    fi
}
autoload -Uz add-zsh-hook
add-zsh-hook precmd _rua_insert_on_next_prompt

rua() {
    local rua_executable="<path/to/your/rua.exe e.g. /home/finnwsl/repos/rua/target/debug/rua>"
    if (( $# == 0 )); then   
        local selected_command
        selected_command=$("$rua_executable")
        if [[ -n "$selected_command" ]]; then
            RUA_SELECTED_COMMAND="$selected_command"
        fi
    else
        "$rua_executable" "$@"
    fi
}
# ----------------------rua part--------------------------------
```

3. Save and restart your shell, then it'll work. Enjoy rua!