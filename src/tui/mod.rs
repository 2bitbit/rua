//! 交互界面 (`rua` 默认模式的 TUI)

// Define a macro that acts as your reusable template.
// The format string is a literal inside the macro, which the compiler can check.
#[macro_export]
macro_rules! format_list_item {
    ($key:expr, $command:expr, $expanded:expr) => {
        format!("{}: {:80}    | expanded: {}", $key, $command, $expanded)
    };
}

mod core;
mod ui;

use crate::commands::CommandEntry;
// use crossterm::event::KeyCode;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::collections::HashSet;
use std::io::{self};
use std::sync::LazyLock;

pub struct TUI {
    is_running: bool,
    renderer: Terminal<CrosstermBackend<io::Stderr>>, // Draw its TUI on Standard Error (stderr) .Print its final, clean result to Standard Output (stdout). To avoid garbled text in stdout.
    entries: Vec<CommandEntry>,
}



pub const AVAILABLE_KEYS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let base_keys: HashSet<String> = HashSet::new();
    // let base_keys = [
    // TODO:
    // KeyCode::Up,
    // KeyCode::Down,
    // KeyCode::Left,
    // KeyCode::Right,
    // KeyCode::BackTab,
    // KeyCode::Backspace,
    // KeyCode::Delete,
    // KeyCode::End,
    // KeyCode::Enter,
    // KeyCode::Home,
    // KeyCode::Insert,
    // KeyCode::PageDown,
    // KeyCode::PageUp,
    // KeyCode::Tab,
    // ];
    let transfrom_closure = |c: char| c.to_string();
    base_keys
        .into_iter()
        .chain(('a'..='z').map(transfrom_closure))
        .chain(('A'..='Z').map(transfrom_closure))
        .chain(('0'..='9').map(transfrom_closure))
        .collect()
});
