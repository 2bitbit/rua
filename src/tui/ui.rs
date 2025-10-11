use crate::commands::CommandEntry;
use crate::executor::expand_command;
use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Color, Constraint, Direction, Frame, Layout, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};



/// UI 渲染逻辑--render each frame
pub fn tui_rua(frame: &mut Frame, entries: &Vec<CommandEntry>) -> Result<()> {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let list_items: Vec<ListItem> = entries
        .iter()
        .map(|entry| {
            // 注意：这里我们展示的是展开后的命令，让用户看得更清晰
            let line = format_list_item!(entry.key, entry.command, expand_command(&entry.command));
            ListItem::new(line)
        })
        .collect();
    let list =
        List::new(list_items).block(Block::default().borders(Borders::ALL).title("Commands"));
    frame.render_widget(list, main_layout[0]);

    let footer = Paragraph::new("Press a key to select command, or Esc to quit.")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(footer, main_layout[1]);

    Ok(())
}
