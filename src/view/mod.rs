pub mod components;
mod layout;

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    model::state::{AppState, FocusedBlock, SelectedTab},
    view::{
        components::{action_button, add_task, logo, menu, progress_bar, scrollbar, table, tabs},
        layout::LayoutAreas,
    },
};

pub fn draw(model: &mut AppState, frame: &mut Frame) {
    let screen = Block::default().borders(Borders::NONE).style(
        Style::default()
            .fg(model.theme.primary.c500)
            .bg(model.theme.primary.c950),
    );

    frame.render_widget(&screen, frame.area());

    let layout = LayoutAreas::compute(frame.area());
    action_button::render(model, frame, layout.action_button);
    logo::render(model, frame, layout.logo);
    menu::render(model, frame, layout.menu);
    tabs::render(model, frame, layout.tabs);
    match model.selected_tab {
        SelectedTab::Single => {
            table::render(model, frame, layout.content);
            scrollbar::render(model, frame, layout.content);
        }
        _ => {}
    }
    progress_bar::render(model, frame, layout.progress_bar);

    match model.focused_block {
        FocusedBlock::Modal => add_task::render(model, frame, layout.modal),
        _ => {}
    }
}
