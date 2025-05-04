use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::model::state::AppState;

pub fn render(modal: &mut AppState, frame: &mut Frame, area: Rect) {
    let button = Paragraph::new("ADD TASK")
        .style(
            Style::default()
                .fg(modal.theme.accent.c600)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(modal.theme.accent.c600)),
        );

    frame.render_widget(button, area);
}
