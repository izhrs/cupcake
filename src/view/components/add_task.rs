use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};

use crate::model::state::AppState;

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    Clear.render(area, frame.buffer_mut());

    let modal = Paragraph::new(
        Line::from(Span::styled(
            "ADD NEW TASK",
            Style::default().fg(model.theme.secondary.c500),
        ))
        .alignment(Alignment::Center),
    )
    .add_modifier(Modifier::BOLD)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(model.theme.secondary.c700))
            .style(Style::default().bg(model.theme.primary.c950))
            .padding(Padding::uniform(1)),
    );

    frame.render_widget(modal, area);
}
