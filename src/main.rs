//! Rubus 程序入口：解析CLI，分发任务
//!
//! ## 用户手册
//! 无特殊说明，所有操作都只针对当前源。
//! - `rua`: (无参数) 进入交互式 TUI 列表模式。在此模式下，按下一个快捷键，TUI 将退出并将对应的命令填充到您的命令行。（如果该 key 不存在，则什么也不会填充，单纯地退出 TUI）
//!     - note: commands would automatically refreash according to source everytime you run 'rua' command.
//! - `rua add  <KEY> '<COMMAND>'`: 添加一条新命令到**当前来源**
//! 	- 示例: `rua add 'gs "git status --short"'`
//! - `rua rm  <KEY>`: 删除**当前来源**的一条命令
//! 	- 示例: `rua rm gs`
//! - `rua ls`: 以非交互方式，打印**当前来源**的所有命令。（内容包括）
//! - `rua source <source>`: 设置 TUI 模式的默认命令来源。
//! 	- `rua source default`: 使用默认本地文件。
//! 	- `rua source /path/to/my_commands.json`: 使用指定的本地文件。
//! 	- `rua source 192.168.1.100:7878`: 使用指定的远程服务器。
//!
//! // TODO:
//! - `rua serve [PATH] --port [PORT]`: 启动一个服务器来分发命令列表。 （只有这个命令能设定不同的源进行暴露）（这意味着正在被 `serve` 的 json 文件，要想修改，唯一推荐方式是：在服务器端切换到对应源，执行 `add/rm` 命令，再把源切换回去）
//! - `rua --help`: 打印详细的帮助信息。

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use commands::CommandEntry;
use executor::expand_command;
use storage::StorageBackend;
use storage::local::LocalBackend;
use tui::AVAILABLE_KEYS;

mod cli;
mod commands;
mod config;
mod executor;
mod storage;
mod tui;

#[tokio::main]
/// 不负责命令行逻辑，只负责输出响应到：stdout，stderr
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO: 当下只处理本地后端。基于配置选择后端的逻辑将在后续阶段实现。
    let mut backend = LocalBackend::new()?;

    // subcommand--process  ||  no_subcommand--setup_TUI
    if let Some(command) = cli.command {
        match command {
            Commands::Add { key, command } => {
                if !AVAILABLE_KEYS.contains(&key) {
                    eprint!("Error: Invalid key. Available keys are: a-z, A-Z, 0-9");
                    std::process::exit(1);
                }
                let command = command.join(" ");
                let entry = CommandEntry {
                    key: key.clone(),
                    command: command.clone(),
                };
                backend.add(entry).await?;
                println!("Added/Updated '{}': '{}'", key, command);
            }
            Commands::Rm { keys } => {
                match backend.remove(&keys).await {
                    Ok(_) => println!("Removed '{}'", keys.join(", ")),
                    Err(e) => {
                        // 将错误信息打印到 stderr 并以非零状态码退出
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Commands::Ls => {
                let entries = backend.get_all().await?;
                if entries.is_empty() {
                    println!("No commands found.");
                } else {
                    for entry in entries {
                        let expanded_cmd = expand_command(&entry.command);
                        println!(
                            "{} : {}      (expanded: {})",
                            entry.key, entry.command, expanded_cmd
                        );
                    }
                }
                // 根据设计文档，打印出当前的命令源
                println!("source : {}", backend.source_path().to_string_lossy());
            }
            // TODO:后续阶段的命令
            Commands::Source { .. } => {
                println!("'source' command will be implemented in a future phase.");
            } // Commands::Serve { .. } => {
              //     println!("'serve' command will be implemented in a future phase.");
              // }
        }
    } else {
        // --- 调用 TUI ---
        // 这是 `rua` 默认行为的入口
        use tui::TUI;
        let mut tui = TUI::new().with_context(|| "Can't initialize TUI.")?;
        let entries = backend.get_all().await?;
        if let Some(selected_command) = tui.rua(entries)? {
            // TUI 返回选择的命令，我们将其打印到标准输出
            // Shell 函数会捕获这个输出并填充到命令行
            print!("{}", selected_command);
        }
    }
    Ok(())
}
