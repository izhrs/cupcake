use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::model::state::Model;

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    let button = Paragraph::new("ADD TASK ")
        .style(
            Style::default()
                .fg(model.theme.warning)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(model.theme.warning)),
        );

    frame.render_widget(button, area);
}
