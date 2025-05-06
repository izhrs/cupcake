use ratatui::layout::{Constraint, Layout, Rect};

pub struct LayoutAreas {
    pub logo: Rect,
    pub menu: Rect,
    pub tabs: Rect,
    pub action_button: Rect,
    pub content: Rect,
    pub progress_bar: Rect,
    pub modal: Rect,
}

impl LayoutAreas {
    pub fn compute(area: Rect) -> Self {
        let main_layout = Layout::vertical(vec![
            Constraint::Min(10),   // main content
            Constraint::Length(3), // progress bar
        ])
        .split(area);

        let body_layout = Layout::horizontal(vec![
            Constraint::Length(30), // sidebar
            Constraint::Min(10),    // main content area
        ])
        .split(main_layout[0]);

        let sidebar_layout = Layout::vertical(vec![
            Constraint::Length(5), // logo
            Constraint::Min(10),   // menu
        ])
        .split(body_layout[0]);

        let content_layout = Layout::vertical(vec![
            Constraint::Length(3), // tabs + action button
            Constraint::Min(10),   // main table
        ])
        .split(body_layout[1]);

        let action_layout = Layout::horizontal(vec![
            Constraint::Min(20),    // tabs
            Constraint::Length(16), // action button
        ])
        .split(content_layout[0]);

        // take up a half horizontally and vertically
        let modal_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };

        Self {
            logo: sidebar_layout[0],
            menu: sidebar_layout[1],
            tabs: action_layout[0],
            action_button: action_layout[1],
            content: content_layout[1],
            progress_bar: main_layout[1],
            modal: modal_area,
        }
    }
}
