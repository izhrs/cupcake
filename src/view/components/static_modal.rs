/// display info or error modal
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::model::state::{ModalType, Model};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect, modal_type: &ModalType) {
    Clear.render(area, frame.buffer_mut());

    let block = Block::bordered()
        .border_style(Style::default().fg(match modal_type {
            ModalType::Info => model.theme.success,
            ModalType::Error => model.theme.destructive,
            _ => model.theme.muted, // unreachable case
        }))
        .style(match model.theme.background {
            Some(color) => Style::default().bg(color).fg(model.theme.forground),
            None => Style::default().fg(model.theme.forground),
        });

    let centered_layout = Layout::vertical(vec![Constraint::Min(1)])
        .flex(ratatui::layout::Flex::Center)
        .split(area);

    let paragraph = Paragraph::new(model.modal_prompt.clone())
        .style(Style::default().fg(match modal_type {
            ModalType::Info => model.theme.success,
            ModalType::Error => model.theme.destructive,
            _ => model.theme.muted, // unreachable case
        }))
        .block(block);
    frame.render_widget(paragraph, centered_layout[0]);
}
