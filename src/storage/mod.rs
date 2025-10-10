//! modmodmodmod  
//! 定义了存储后端的抽象 Trait StorageBackend

use crate::commands::CommandEntry;
use anyhow::Result;
pub mod local;

#[async_trait::async_trait]
pub trait StorageBackend {
    /// 获取所有命令
    async fn get_all(&self) -> Result<Vec<CommandEntry>>;
    /// 添加一条命令
    async fn add(&mut self, entry: CommandEntry) -> Result<()>;
    /// 删除一条命令
    async fn remove(&mut self, keys: &Vec<String>) -> Result<()>;
}
