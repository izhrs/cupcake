use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style, Stylize, palette::tailwind},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::model::state::AppState;

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    let logo = Paragraph::new(
        Line::from(vec![
            Span::styled("C ", Style::default().fg(tailwind::PURPLE.c500)),
            Span::styled("U ", Style::default().fg(tailwind::GREEN.c500)),
            Span::styled("P ", Style::default().fg(tailwind::BLUE.c500)),
            Span::styled("C ", Style::default().fg(tailwind::ORANGE.c500)),
            Span::styled("A ", Style::default().fg(tailwind::VIOLET.c500)),
            Span::styled("K ", Style::default().fg(tailwind::CYAN.c500)),
            Span::styled("E", Style::default().fg(tailwind::ROSE.c500)),
        ])
        .alignment(Alignment::Center),
    )
    .add_modifier(Modifier::BOLD)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(model.theme.secondary.c950))
            .padding(Padding::uniform(1)),
    );

    frame.render_widget(logo, area);
}
