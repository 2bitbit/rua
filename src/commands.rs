//! 核心数据结构 (CommandEntry)：存储的基本单位。
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandEntry {
    pub key: String,
    pub command:String,
    // 未来可扩展字段
    // pub description: Option<String>,
    // pub tags: Vec<String>,
}