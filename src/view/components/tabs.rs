use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::model::state::{ActiveTab, Model};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect, active_tab: &ActiveTab) {
    let tabs = Tabs::new(vec!["SINGLE", "BATCH", "PLAYLIST", "SETTINGS", "ABOUT"])
        .select(*active_tab as usize)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(model.theme.secondary.c950)),
        )
        .highlight_style(
            Style::default()
                .fg(model.theme.secondary.c500)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|")
        .style(Style::default());

    frame.render_widget(tabs, area);
}
