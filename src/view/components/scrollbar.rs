use ratatui::{
    Frame,
    layout::{Margin, Rect},
    widgets::{Scrollbar, ScrollbarOrientation},
};

use crate::model::state::AppState;

pub fn render(model: &mut AppState, frame: &mut Frame, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut model.task_store.single.scroll_state,
    )
}
