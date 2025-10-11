//! 配置管理 (Source 枚举, 读写 config.toml)  
//! 用于解析和表示 `rua source` 命令的参数。
//! // TODO

/*
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Source {
    Default,
    // TODO:
    // File(PathBuf),
    // Remote(SocketAddr),
}

// 实现 FromStr Trait，以便 clap 能够直接解析
impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Source::Default),

            _ => Ok(Source::File(PathBuf::from(s))),

            // TODO
            // _ if s.contains(':') => s
            //     .parse::<SocketAddr>()
            //     .map(Source::Remote)
            //     .map_err(|e| e.to_string()),
        }
    }
}
*/