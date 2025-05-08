use std::collections::VecDeque;

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Cell, HighlightSpacing, Padding, Row, Table},
};

use crate::model::state::{Model, FocusedBlock, SelectedTab};

pub fn render(model: &mut Model, frame: &mut Frame, area: Rect) {
    let header_style = Style::default()
        .fg(model.theme.secondary.c100)
        .bg(model.theme.secondary.c950);
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(model.theme.secondary.c800)
        .bg(model.theme.secondary.c100);
    let selected_col_style = Style::default().fg(model.theme.secondary.c600);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(model.theme.secondary.c800);

    let header = ["Name", "Speed", "Size", "Progress", "ETA", "Status"]
        .into_iter()
        .map(|c| Cell::from(Text::from(c.to_ascii_uppercase().to_string())))
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let tasks = match model.selected_tab {
        SelectedTab::Single => model.task_store.single.tasks.clone(),
        SelectedTab::Batch => model.task_store.batch.tasks.clone(),
        SelectedTab::Playlist => model.task_store.playlist.tasks.clone(),
        _ => VecDeque::new(),
    };

    let rows = tasks.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => model.theme.primary.c900,
            _ => model.theme.primary.c950,
        };

        let item = [
            Text::from(data.name.to_string()),
            Text::from(format!("{:.2} MB/s", data.speed)),
            Text::from(format!("{:.0} MB", data.size)),
            Text::from(format!("{:.0} %", data.progress)),
            Text::from(data.eta.clone()),
            Text::from(data.status.to_string()),
        ];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(model.theme.primary.c100).bg(color))
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
                    format!("{} TASKS", model.selected_tab.to_string().to_uppercase()),
                    Style::default().fg(model.theme.secondary.c500),
                ),
                Span::from(" ]"),
            ]))
            .border_style(Style::default().fg(match model.focused_block {
                FocusedBlock::Content => model.theme.secondary.c800,
                _ => model.theme.secondary.c950,
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
        match model.selected_tab {
            SelectedTab::Single => &mut model.task_store.single.table_state,
            SelectedTab::Batch => &mut model.task_store.batch.table_state,
            SelectedTab::Playlist => &mut model.task_store.playlist.table_state,
            _ => &mut model.task_store.single.table_state,
        },
    );
}
