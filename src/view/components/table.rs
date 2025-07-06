use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Cell, HighlightSpacing, Padding, Row, Table},
};

use crate::model::state::{ActivePanel, ActiveTab, Model};

pub fn render(
    model: &mut Model,
    frame: &mut Frame,
    area: Rect,
    active_panel: &ActivePanel,
    active_tab: &ActiveTab,
) {
    let header_style = Style::default().fg(model.theme.success);

    // .bg(model.theme.);

    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(model.theme.primary)
        .bg(model.theme.primary_forground);
    let selected_col_style = Style::default().fg(model.theme.primary);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(model.theme.primary);

    let header = ["Name", "Speed", "Size", "Progress", "ETA", "Status"]
        .into_iter()
        .map(|c| Cell::from(Text::from(c.to_ascii_uppercase().to_string())))
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let tasks = match active_tab {
        ActiveTab::Single => model.downloader.single.state.filtered_downloads.clone(),
        ActiveTab::Batch => model.downloader.batch.state.filtered_downloads.clone(),
        ActiveTab::Playlist => model.downloader.playlist.state.filtered_downloads.clone(),
        _ => Default::default(),
    };

    let rows = tasks.iter().enumerate().map(|(i, data)| {
        // let color = match i % 2 {
        //     0 => model.theme.muted,
        //     _ => model.theme.muted,
        // };

        let item = [
            Text::from(data.title.to_string()),
            Text::from(data.download_speed.clone()),
            Text::from(data.file_size.clone()),
            Text::from(format!("{:.0}%", data.progress_percent)),
            Text::from(data.estimated_time.clone()),
            Text::from(data.status.to_string()),
        ];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(model.theme.forground))
            .height(3)
    });

    let t = Table::new(
        rows,
        [
            Constraint::Min(30),    // name
            Constraint::Length(15), // speed
            Constraint::Length(15), // size
            Constraint::Length(10), // progress
            Constraint::Length(10), // eta
            Constraint::Length(15), // status
        ],
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(Line::from(vec![
                Span::from("[ "),
                Span::styled(
                    format!("{} TASKS", active_tab.to_string().to_uppercase()),
                    Style::default().fg(model.theme.primary),
                ),
                Span::from(" ]"),
            ]))
            .border_style(Style::default().fg(match active_panel {
                ActivePanel::Content => model.theme.border_active,
                _ => model.theme.border,
            }))
            .padding(Padding::new(0, 0, 1, 0)),
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .column_highlight_style(selected_col_style)
    .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from("  "))
    .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(
        t,
        area,
        match active_tab {
            ActiveTab::Single => &mut model.downloader.single.state.table_state,
            ActiveTab::Batch => &mut model.downloader.batch.state.table_state,
            ActiveTab::Playlist => &mut model.downloader.playlist.state.table_state,
            _ => &mut model.downloader.single.state.table_state,
        },
    );
}
