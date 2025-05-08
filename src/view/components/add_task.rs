use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Style, palette::tailwind},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::model::state::{Model, FocusedInput};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    Clear.render(area, frame.buffer_mut());

    let block = Block::bordered()
        .border_style(Style::default().fg(model.theme.secondary.c700))
        .title(
            Line::from(vec![
                Span::from("[ "),
                Span::styled(
                    "ADD A NEW TASK",
                    Style::default().fg(model.theme.secondary.c500),
                ),
                Span::from(" ]"),
            ])
            .centered(),
        )
        .style(Style::default().bg(model.theme.primary.c950));

    frame.render_widget(block, area);

    let layout = Layout::vertical(vec![
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(5),
    ])
    .split(area);

    let width = layout[1].width.max(3) - 5;
    let scroll = model.input_state.source.visual_scroll(width as usize);

    let source_input = Paragraph::new(model.input_state.source.value())
        .style(Style::default().fg(match model.input_state.focused {
            FocusedInput::Source => model.theme.primary.c400,
            _ => model.theme.primary.c700,
        }))
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("[ Source URL ]"));

    frame.render_widget(source_input, layout[1].inner(Margin::new(1, 0)));

    let destination_input = Paragraph::new(model.input_state.destination.value())
        .style(Style::default().fg(match model.input_state.focused {
            FocusedInput::Destination => model.theme.primary.c400,
            _ => model.theme.primary.c700,
        }))
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("[ Download Path ]"));

    frame.render_widget(destination_input, layout[2].inner(Margin::new(1, 0)));

    match model.input_state.focused {
        FocusedInput::Source => {
            let x = model.input_state.source.visual_cursor().max(scroll) - scroll + 2;
            frame.set_cursor_position((layout[1].x + x as u16, layout[1].y + 1));
        }
        FocusedInput::Destination => {
            let x = model.input_state.destination.visual_cursor().max(scroll) - scroll + 2;
            frame.set_cursor_position((layout[2].x + x as u16, layout[2].y + 1));
        }
    }

    // weird trick to center but all I know is constraint!
    let button_layout = Layout::horizontal(vec![
        Constraint::Min(1),
        Constraint::Length(14), //button
        Constraint::Length(1),  // spacer
        Constraint::Length(14), //button
        Constraint::Min(1),
    ])
    .split(layout[4].inner(Margin {
        horizontal: 1,
        vertical: 1,
    }));

    let cancel_button = Paragraph::new("CENCEL 󱊷")
        .style(Style::default().fg(tailwind::RED.c500))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::bordered());

    let submit_button = Paragraph::new("SUBMIT 󰌑")
        .style(Style::default().fg(tailwind::GREEN.c500))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::bordered());

    frame.render_widget(cancel_button, button_layout[1]);
    frame.render_widget(submit_button, button_layout[3]);
}
