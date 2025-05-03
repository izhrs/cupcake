use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style, palette::tailwind},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::model::state::AppState;

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(vec!["SINGLE", "BATCH", "PLAYLIST", "SETTINGS", "ABOUT"])
        .select(model.selected_tab as usize)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(tailwind::PURPLE.c950)),
        )
        .highlight_style(
            Style::default()
                .fg(tailwind::PURPLE.c500)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|")
        .style(Style::default());

    frame.render_widget(tabs, area);
}
