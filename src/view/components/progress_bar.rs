use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge},
};

use crate::model::state::Model;

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(Line::from(vec![
                    Span::from("[ "),
                    Span::styled("PROGRESS", Style::default().fg(model.theme.primary)),
                    Span::from(" ]"),
                ]))
                .border_style(Style::default().fg(model.theme.border)),
        )
        .gauge_style(
            Style::default()
                .fg(model.theme.primary)
                .bg(model.theme.border),
        )
        .percent(model.progress as u16);

    frame.render_widget(progress_bar, area);
}
