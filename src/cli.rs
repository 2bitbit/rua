//！ CLI 定义
use crate::config::Source;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rua", version, about = "一个智能的命令行快捷启动器")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 添加一条新命令到指定的存储源
    Add {
        /// 命令的快捷键
        key: String,
        /// 完整的命令内容 (可以包含空格)
        #[clap(required = true)]
        command: Vec<String>,
    },
    /// 从指定的存储源删除一条命令
    Rm {
        /// 命令的快捷键
        keys: Vec<String>,
    },
    /// 列出当前存储源的所有命令
    Ls,
    /// 设置默认的命令来源 (default, file path, or ip:port)
    Source {
        /// 来源
        source: Source,
    },
    // // 启动一个服务器来分发命令列表 // TODO
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