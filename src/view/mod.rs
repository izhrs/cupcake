pub mod components;
mod layout;

use crate::model::state::AppState;
use crate::view::layout::LayoutAreas;

use crate::view::components::{action_button, logo, menu, progress_bar, scrollbar, table, tabs};

use ratatui::{
    Frame,
    style::{Style, palette::tailwind},
    widgets::{Block, Borders},
};

pub fn draw(model: &mut AppState, frame: &mut Frame) {
    let screen = Block::default().borders(Borders::NONE).style(
        Style::default()
            .fg(tailwind::NEUTRAL.c500)
            .bg(tailwind::NEUTRAL.c950),
    );

    frame.render_widget(&screen, frame.area());

    let layout = LayoutAreas::compute(frame.area());
    action_button::render(model, frame, layout.action_button);
    logo::render(model, frame, layout.logo);
    menu::render(model, frame, layout.menu);
    tabs::render(model, frame, layout.tabs);
    table::render(model, frame, layout.content);
    scrollbar::render(model, frame, layout.content);
    progress_bar::render(model, frame, layout.progress_bar);
}
