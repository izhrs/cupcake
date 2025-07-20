use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::model::state::Model;

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    Clear.render(area, frame.buffer_mut());

    let block = Block::bordered()
        .border_style(Style::default().fg(model.theme.border_active))
        .title(
            Line::from(vec![
                Span::from("[ "),
                Span::styled("ADD A NEW TASK", Style::default().fg(model.theme.primary)),
                Span::from(" ]"),
            ])
            .centered(),
        )
        .style(match model.theme.background {
            Some(color) => Style::default().bg(color).fg(model.theme.forground),
            None => Style::default().fg(model.theme.forground),
        });

    frame.render_widget(block, area);

    let layout = Layout::vertical(vec![
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(5),
    ])
    .split(area);

    let width = layout[1].width.max(3) - 5;
    let scroll = model.input_state.source.visual_scroll(width as usize);

    let source_input = Paragraph::new(model.input_state.source.value())
        .style(Style::default().fg(model.theme.forground))
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("[ URL ]"));

    frame.render_widget(source_input, layout[1].inner(Margin::new(1, 0)));

    let x = model.input_state.source.visual_cursor().max(scroll) - scroll + 2;
    frame.set_cursor_position((layout[1].x + x as u16, layout[1].y + 1));

    // weird trick to center but all I know is constraint!
    let button_layout = Layout::horizontal(vec![
        Constraint::Min(1),
        Constraint::Length(14), //button
        Constraint::Length(1),  // spacer
        Constraint::Length(14), //button
        Constraint::Min(1),
    ])
    .split(layout[3].inner(Margin {
        horizontal: 1,
        vertical: 1,
    }));

    let cancel_button = Paragraph::new("CENCEL 󱊷")
        .style(Style::default().fg(model.theme.destructive))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::bordered());

    let submit_button = Paragraph::new("SUBMIT 󰌑")
        .style(Style::default().fg(model.theme.success))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::bordered());

    frame.render_widget(cancel_button, button_layout[1]);
    frame.render_widget(submit_button, button_layout[3]);
}
