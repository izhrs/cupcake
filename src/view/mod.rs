pub mod components;
mod layout;

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    model::state::{ActivePanel, ActiveTab, Model},
    view::{
        components::{action_button, add_task, logo, menu, progress_bar, scrollbar, table, tabs},
        layout::LayoutAreas,
    },
};

pub fn draw(
    model: &mut Model,
    frame: &mut Frame,
    active_panel: &ActivePanel,
    active_tab: &ActiveTab,
) {
    let screen = if let Some(color) = model.theme.background {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(model.theme.border))
            .style(Style::default().bg(color).fg(model.theme.forground))
    } else {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(model.theme.border))
            .style(Style::default().fg(model.theme.forground))
    };

    frame.render_widget(&screen, frame.area());

    let layout = LayoutAreas::compute(frame.area());
    action_button::render(model, frame, layout.action_button);
    logo::render(model, frame, layout.logo);
    menu::render(model, frame, layout.menu, active_panel);
    tabs::render(model, frame, layout.tabs, active_tab);

    match active_tab {
        ActiveTab::Single => {
            table::render(model, frame, layout.content, active_panel, active_tab);

            if (model.task_store.single.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content, active_tab);
            }
        }
        ActiveTab::Batch => {
            table::render(model, frame, layout.content, active_panel, active_tab);

            if (model.task_store.batch.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content, active_tab);
            }
        }
        ActiveTab::Playlist => {
            table::render(model, frame, layout.content, active_panel, active_tab);

            if (model.task_store.playlist.tasks.len() * 3) > layout.content.height as usize {
                scrollbar::render(model, frame, layout.content, active_tab);
            }
        }
        _ => {}
    }

    progress_bar::render(model, frame, layout.progress_bar);

    if let ActivePanel::Modal = active_panel {
        add_task::render(model, frame, layout.modal);
    }
}
