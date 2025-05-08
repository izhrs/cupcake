use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Style,
    widgets::{Scrollbar, ScrollbarOrientation},
};

use crate::model::state::{Model, SelectedTab};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_style(Style::default().fg(model.theme.secondary.c900)),
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        match model.selected_tab {
            SelectedTab::Single => &mut model.task_store.single.scroll_state,
            SelectedTab::Batch => &mut model.task_store.batch.scroll_state,
            SelectedTab::Playlist => &mut model.task_store.playlist.scroll_state,
            _ => &mut model.task_store.single.scroll_state,
        },
    )
}
