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
                    Span::styled("PROGRESS", Style::default().fg(model.theme.secondary.c500)),
                    Span::from(" ]"),
                ]))
                .border_style(Style::default().fg(model.theme.secondary.c950)),
        )
        .gauge_style(
            Style::default()
                .fg(match model.progress {
                    0.0..=25.0 => model.theme.secondary.c800,
                    25.0..=50.0 => model.theme.secondary.c700,
                    50.0..=75.0 => model.theme.secondary.c600,
                    _ => model.theme.secondary.c500,
                })
                .bg(model.theme.secondary.c950),
        )
        .percent(model.progress as u16);

    frame.render_widget(progress_bar, area);
}
