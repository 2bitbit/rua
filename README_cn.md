# Rua - 智能命令行启动器

Rua 是一个轻快、简洁的终端工具，它能让你通过一个交互式的、基于快捷键的菜单，来保存并快速调用你常用的 shell 命令。

## 功能演示

!
*(在这里插入一个演示 TUI 操作的 GIF 动图会非常直观)*

## 主要功能

* **交互式界面 (TUI)**: 只需输入 `rua`，即可打开一个清爽的交互式命令列表。按下命令对应的快捷键，即可立即填充到命令行提示符。
* **智能路径展开**: 在列表中，程序会自动展示命令的完整解析路径（例如 `conda` 会显示为 `C:\Users\...\conda.exe`），让你始终清楚将要执行的命令。
* **简易管理**: 通过简单的子命令，轻松添加、删除和列出你的命令别名。
* **跨 Shell 支持**: 只需少量配置，即可在 PowerShell 和 Zsh 中无缝使用。

## 安装与配置

1. 下载可执行文件: 从项目的 [Releases 页面](https://github.com/2bitbit/rua/releases/latest)下载适用于你系统的最新预编译版本。将下载的可执行文件放置在电脑上的任意便捷位置。

2. 配置你的 Shell: 为了让 `rua` 能够将选中的命令自动填充到你的命令行提示符中，你需要在 Shell 的配置文件里添加一个小函数。
    * **PowerShell 用户**: 请遵循 [**scripts/pwsh.md**](https://github.com/2bitbit/rua/blob/main/scripts/pwsh.md) 中的指引进行配置。
    * **Zsh 用户**: 请遵循 [**scripts/zsh.md**](https://github.com/2bitbit/rua/blob/main/scripts/zsh.md) 中的指引进行配置。

## 用户手册

Rua 的使用非常简单。<!-- 所有操作都针对当前设置的命令源。 -->

| 命令 | 描述 |
|---|---|
| `rua` | (无参数) 启动交互式 TUI 界面。按下快捷键来选择对应的命令，该命令会被自动填充到你的命令行中。按 `Esc` 键退出。 |
| `rua add <KEY> '<COMMAND>'` | 添加一条新的命令快捷方式。快捷键 `KEY` 必须是单个字母或数字。<br/>*示例: `rua add g 'git status --short'`* |
| `rua rm <KEYs>...` | 根据快捷键删除一条命令。<br/>*示例: `rua rm a b c d`* |
| `rua ls` | 以非交互的方式，列出所有已保存的命令及其展开后的完整路径。 |
<!-- | `rua source <SOURCE>` | *(计划中功能)* 设置默认的命令来源。来源可以是本地文件或远程服务器。 | -->
<!-- | `rua serve` | *(计划中功能)* 启动一个 HTTP 服务器，用于在不同设备间共享你的命令列表。 | -->

## 从源码构建

如果你希望自行编译本项目：
1. **克隆与构建**:
```sh
# 克隆仓库
git clone [https://github.com/2bitbit/rua.git](https://github.com/2bitbit/rua.git)
cd rua

# 编译 Release 版本
cargo build --release
```
2. **配置 Shell**: 编译完成后，请不要忘记参考上面的“安装与配置”部分，完成对你的 Shell 的配置。

<p align="center"> Colde Joke: 
Rubus means "rua 不死", in contrast to "rua 嘀死"(`Rudis`).
</p>