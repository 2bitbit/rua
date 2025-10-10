use crate::commands::CommandEntry;
use crate::tui::TUI;
use anyhow::Result;
use crossterm::execute;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::stderr;
use std::time::Duration;
use std::vec;

impl TUI {
    pub fn new() -> Result<Self> {
        // 设置 panic hook，确保即使程序崩溃也能恢复终端状态
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = execute!(stderr(), LeaveAlternateScreen);
            let _ = disable_raw_mode();
            original_hook(panic_info);
        }));

        // 初始化终端
        let renderer = Terminal::new(CrosstermBackend::new(stderr()))?;
        Ok(Self {
            is_running: false,
            renderer: renderer,
            entries: vec![],
        })
    }

    pub fn rua(&mut self, entries: Vec<CommandEntry>) -> Result<Option<String>> {
        // 1. 初始化
        self.enter_interactive_screen()?;
        let mut selected_command = None;
        self.entries = entries;

        // 2. 主循环
        while self.is_running {
            // 2a. 绘制界面
            self.renderer.draw(|frame| {
                let _ = crate::tui::ui::tui_rua(frame, &self.entries);
            })?;
            // 2b. 等待事件 (设置一个超时，以防万一)
            if event::poll(Duration::from_millis(250))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            // 2c. 处理事件 & 更新状态
                            selected_command = Some(self.handle_key_press(key));
                        }
                    }
                    _ => {}
                }
            }
        }

        // 3. 恢复终端
        self.leave_interactive_screen()?;
        Ok(selected_command)
    }
}

impl TUI {
    fn enter_interactive_screen(&mut self) -> Result<()> {
        execute!(stderr(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.is_running = true;
        Ok(())
    }
    fn leave_interactive_screen(&mut self) -> Result<()> {
        execute!(stderr(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        self.is_running = false;
        Ok(())
    }
    fn handle_key_press(&mut self, key: KeyEvent) -> String {
        if key.code == KeyCode::Esc
            || (key.code == KeyCode::Char('c') && key.modifiers == event::KeyModifiers::CONTROL)
        {
            self.is_running = false;
            return "".to_string();
        }
        match key.code {
            KeyCode::Char(c) => {
                if let Some(entry) = self.entries.iter().find(|e| e.key == c.to_string()) {
                    self.is_running = false;
                    entry.command.clone()
                } else {
                    self.is_running = false;
                    "".to_string()
                }
            }
            // ... 在这里可以处理其他按键 ...
            _ => {
                self.is_running = false;
                "".to_string()
            }
        }
    }
}
