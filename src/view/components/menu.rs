use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Padding},
};

use tui_tree_widget::Tree;

use crate::model::state::{ActivePanel, Model};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect, active_panel: &ActivePanel) {
    let widget = Tree::new(&model.menu_items)
        .expect("all item identifiers must be unique")
        .block(
            Block::default()
                // .title_bottom(format!("{:?}", model.menu_state.selected()))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(match active_panel {
                    ActivePanel::Menu => model.theme.border_active,
                    _ => model.theme.border,
                }))
                .padding(Padding::symmetric(1, 1)),
        )
        .style(Style::default().fg(model.theme.forground))
        .node_open_symbol("  ")
        .node_closed_symbol("  ")
        .node_no_children_symbol(" ")
        .highlight_style(
            Style::new()
                .fg(model.theme.primary_forground)
                .bg(model.theme.primary)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    frame.render_stateful_widget(widget, area, &mut model.menu_state);
}
