use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style, palette::tailwind},
    widgets::{Block, BorderType, Borders, Padding},
};

use tui_tree_widget::Tree;

use crate::model::state::{AppState, FocusedBlock};

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    let widget = Tree::new(&model.menu_items)
        .expect("all item identifiers must be unique")
        .block(
            Block::default()
                .title_bottom(format!("{:?}", model.menu_state.selected()))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(match model.focused_block {
                    FocusedBlock::Menu => tailwind::PURPLE.c800,
                    _ => tailwind::PURPLE.c950,
                }))
                .padding(Padding::symmetric(2, 1)),
        )
        .highlight_style(
            Style::new()
                .fg(tailwind::PURPLE.c500)
                .bg(tailwind::NEUTRAL.c900)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");
    frame.render_stateful_widget(widget, area, &mut model.menu_state);
}
