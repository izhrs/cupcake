use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, palette::tailwind},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge},
};

use crate::model::state::AppState;

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(Line::from(vec![
                    Span::from("[ "),
                    Span::styled("PROGRESS", Style::default().fg(tailwind::PURPLE.c500)),
                    Span::from(" ]"),
                ]))
                .border_style(Style::default().fg(tailwind::PURPLE.c950)),
        )
        .gauge_style(
            Style::default()
                .fg(match model.progress {
                    0.0..=25.0 => tailwind::PURPLE.c800,
                    25.0..=50.0 => tailwind::PURPLE.c700,
                    50.0..=75.0 => tailwind::PURPLE.c600,
                    _ => tailwind::PURPLE.c500,
                })
                .bg(tailwind::PURPLE.c950),
        )
        .percent(model.progress as u16);

    frame.render_widget(progress_bar, area);
}
