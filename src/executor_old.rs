//! 使用 shellexpand 展开命令中的环境变量和 `~`
//! should not be used in storage. For printing use only.
pub fn expand_command(command: &str) -> String {
    if let Ok(full_command) = shellexpand::full(command) {
        full_command.to_string()
    } else {
        "".to_string() // ignore the error
    }
}
