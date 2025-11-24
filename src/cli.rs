//！ CLI 定义
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rua", version, about = "Easily populate your frequently used commands.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    //TODO: /// 添加一条新命令到指定的存储源 (Add a new command to the specified storage source)
    /// 添加一条新命令 (Add a new command)
    Add {
        /// 命令的快捷键
        key: String,
        /// 完整的命令内容 (可以包含空格)
        #[clap(required = true)]
        command: Vec<String>,
    },
    
    /// 更新一条已存在的命令 (Update an existing command)
    Update {
        /// 命令的快捷键
        key: String,
        /// 新的完整命令内容
        #[clap(required = true)]
        command: Vec<String>,
    },

    // TODO: /// 从当前存储源删除一条或多条命令 (Remove one or more commands from the current storage source)
    /// 删除一条或多条命令 (Remove one or more commands)
    Rm {
        /// 命令的快捷键
        #[clap(required = true)]
        keys: Vec<String>,
    },
    // TODO: /// 列出当前存储源的所有命令 (List all commands from the current storage source)
    ///  列出所有命令和存储源 (List all commands and the config file path.)
    Ls,


    // TODO:
    // /// 设置默认的命令来源 (default, file path, or ip:port)
    // Source {
    //     /// 来源
    //     source: Source,
    // },
    // TODO:
    // // 启动一个服务器来分发命令列表
    // #[clap(name = "serve")]
    // Serve {
    //     /// 作为数据源的JSON文件路径
    //     #[clap(default_value = "~/.config/rua/commands.json")]
    //     path: std::path::PathBuf,
    //     /// 服务器监听的端口
    //     #[clap(short, long, default_value = "7878")]
    //     port: u16,
    // },
}