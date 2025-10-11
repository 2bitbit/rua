# ----------------------rua part--------------------------------
# Part 1: The 'precmd' Hook
# This is the core mechanism that allows inserting text into the next prompt.
# (It is more direct than the key-press simulation used in PowerShell.)
_rua_insert_on_next_prompt() { # This function runs just before each command prompt is displayed.
    # Check if our special global variable 'RUA_SELECTED_COMMAND' has been set by the 'rua' function.
    if [[ -n "$RUA_SELECTED_COMMAND" ]]; then
        # If it is set, push its content into the Zsh Line Editor (ZLE) buffer.
        # This makes the command appear on the next line, ready to be executed.
        print -z "$RUA_SELECTED_COMMAND"
        # Unset the variable so this logic only runs once after 'rua' is called.
        unset RUA_SELECTED_COMMAND
    fi
}
# Register the function to run as a 'precmd' hook.
autoload -Uz add-zsh-hook
add-zsh-hook precmd _rua_insert_on_next_prompt

# Part 2: The 'rua' Function
rua() {
    # Define the path to your executable.
    local rua_executable="/home/finnwsl/repos/rua/target/debug/rua""<path/to/your/rua.exe e.g. /home/finnwsl/repos/rua/target/debug/rua>"

    # Check the number of arguments passed to the function.
    if (( $# == 0 )); then   # In Zsh, '$#' holds the count of arguments.
        # --- Mode 1: Interactive Fill ---
        # No arguments were provided, so we run the interactive TUI.
        # 1. Execute rua and capture its standard output (stdout).
        #    The selected command from the TUI will be stored in this variable.
        local selected_command
        selected_command=$("$rua_executable")

        # 2. Check if a command was actually selected (the output is not an empty string).
        #    In Zsh, '-n' tests for a non-zero length string.
        if [[ -n "$selected_command" ]]; then
            # 3. If a command was selected, assign it to our global variable.
            #    The '_rua_insert_on_next_prompt' hook will see this variable and place its content into the prompt.
            RUA_SELECTED_COMMAND="$selected_command"
        fi
    else
        # --- Mode 2: Normal Command Execution ---
        # Arguments like 'add' or 'ls' were provided.
        # Execute the command directly with all the provided arguments.
        # In Zsh, '"$@"' expands to all arguments, correctly handling spaces.
        "$rua_executable" "$@"
    fi
}
# ----------------------rua part--------------------------------