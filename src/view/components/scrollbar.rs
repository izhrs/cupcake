use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Style,
    widgets::{Scrollbar, ScrollbarOrientation},
};

use crate::model::state::{ActiveTab, Model};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect, active_tab: &ActiveTab) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_style(Style::default().fg(model.theme.primary)),
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        match active_tab {
            ActiveTab::Single => &mut model.downloader.single.state.scroll_state,
            ActiveTab::Batch => &mut model.downloader.batch.state.scroll_state,
            ActiveTab::Playlist => &mut model.downloader.playlist.state.scroll_state,
            _ => &mut model.downloader.single.state.scroll_state,
        },
    )
}
