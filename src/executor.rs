//! Expands the executable part of a command to its full, absolute path.
use dunce::canonicalize;
use which::which;

/// Takes a full command string.
/// Expands shell variables (~, $VAR) and expands the executable part to its full, absolute path.
pub fn expand_command(command: &str) -> String {
    // Step 1: Expand shell variables like '~' and '$HOME' from the entire command string.
    let expanded_command = match shellexpand::full(command) {
        Ok(cmd) => cmd.to_string(),
        Err(_) => command.to_string(), // On error, fall back to the original
    };

    // Step 2: Split the expanded command into the executable and its arguments.
    let mut parts = expanded_command.split_whitespace();
    let Some(executable) = parts.next() else {
        return String::new(); // Command is empty
    };
    let args = parts;

    // Step 3: Resolve the executable's path.
    // The two branches now return the same type: Result<PathBuf, String>
    let full_path_result = if executable.contains('/') || executable.contains('\\') {
        // canonicalize returns Result<_, std::io::Error>
        // We map the error to a String to unify the types.
        canonicalize(executable).map_err(|e| e.to_string())
    } else {
        // which returns Result<_, which::Error>
        // We also map this error to a String.
        which(executable).map_err(|e| e.to_string())
    };

    match full_path_result {
        Ok(path_buf) => {
            // Step 4: Reconstruct the final command string with the resolved path.
            let mut final_command = path_buf.to_string_lossy().into_owned();
            for arg in args {
                final_command.push(' ');
                final_command.push_str(arg);
            }
            final_command
        }
        Err(_) => {
            // If path resolution fails, return the command after shell expansion.
            expanded_command
        }
    }
}
