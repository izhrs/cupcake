pub mod components;
mod layout;

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    model::state::{Model, FocusedBlock, SelectedTab},
    view::{
        components::{action_button, add_task, logo, menu, progress_bar, scrollbar, table, tabs},
        layout::LayoutAreas,
    },
};

pub fn draw(model: &mut Model, frame: &mut Frame) {
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

            if (model.task_store.single.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content);
            }
        }
        SelectedTab::Batch => {
            table::render(model, frame, layout.content);

            if (model.task_store.batch.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content);
            }
        }
        SelectedTab::Playlist => {
            table::render(model, frame, layout.content);

            if (model.task_store.playlist.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content);
            }
        }
        _ => {}
    }

    progress_bar::render(model, frame, layout.progress_bar);

    match model.focused_block {
        FocusedBlock::Modal => add_task::render(model, frame, layout.modal),
        _ => {}
    }
}
