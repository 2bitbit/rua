//! 本地 JSON 文件存储的实现
//! StorageBackend Trait 的本地文件系统实现，负责读写本地的 commands.json 文件。
use crate::commands::CommandEntry;
use crate::storage::StorageBackend;
use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};
use tokio::fs;

const DEFAULT_CONFIG_FILE_NAME: &str = "commands.json";

pub struct LocalBackend {
    source_path: PathBuf,
}

impl LocalBackend {
    pub fn new() -> Result<Self> {
        // get config_dir
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?
            .join("rua");
        // 确保配置目录存在
        std::fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory at {:?}", &config_dir))?;

        // get config file
        let path = config_dir.join(DEFAULT_CONFIG_FILE_NAME);
        // 确保命令文件存在
        if !path.exists() {
            // 使用 open + write + create 的组合确保文件被创建为空文件
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .with_context(|| format!("Failed to create commands file at {:?}", &path))?;
        }
        Ok(Self { source_path: path })
    }

    async fn read_entries(&self) -> Result<Vec<CommandEntry>> {
        let content = fs::read_to_string(&self.source_path).await?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        serde_json::from_str(&content).with_context(|| {
            format!(
                "Failed to deserialize commands from {:?}",
                &self.source_path
            )
        })
    }

    async fn write_entries(&self, entries: &[CommandEntry]) -> Result<()> {
        let content = serde_json::to_string_pretty(entries)?;
        fs::write(&self.source_path, content)
            .await
            .with_context(|| format!("Failed to write commands to {:?}", &self.source_path))
    }

    // 辅助函数，用于 'ls' 命令打印来源路径
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
}

#[async_trait::async_trait]
impl StorageBackend for LocalBackend {
    async fn get_all(&self) -> Result<Vec<CommandEntry>> {
        self.read_entries().await
    }

    async fn add(&mut self,entry: CommandEntry) -> Result<()> {
        let mut entries = self.read_entries().await?;

        // 如果已存在相同的 key，则先移除，实现更新(upsert)效果
        entries.retain(|e| e.key != entry.key);

        entries.push(entry);
        self.write_entries(&entries).await
    }

    async fn remove(&mut self, keys: &Vec<String>) -> Result<()> {
        let mut entries = self.read_entries().await?;
        for k in keys {
            let initial_len = entries.len();
            entries.retain(|e| e.key != *k); 
            if entries.len() == initial_len {
                return Err(anyhow!(
                    "Key '{}' not found. And all subsequent deletions were not executed.",
                    k
                ));
            }
        }

        self.write_entries(&entries).await
    }
}
