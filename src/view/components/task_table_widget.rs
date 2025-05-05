use std::collections::VecDeque;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{
        Modifier, Style,
        palette::tailwind::{self, Palette},
    },
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Padding, Row, StatefulWidget, Table,
        TableState,
    },
};

use crate::model::{state::FocusedBlock, task::Task};

pub struct TaskTable {
    title: String,
    primary: Palette,
    secondary: Palette,
}

#[derive(Clone)]
pub struct TaskTableState {
    pub(crate) tasks: VecDeque<Task>,
    pub(crate) table_state: TableState,
    pub(crate) focused_block: FocusedBlock,
}

impl TaskTable {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            primary: tailwind::NEUTRAL,
            secondary: tailwind::PURPLE,
        }
    }
    pub fn primary(mut self, palette: &Palette) -> Self {
        self.primary = palette.to_owned();
        self
    }
    pub fn secondary(mut self, palette: &Palette) -> Self {
        self.secondary = palette.to_owned();
        self
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

impl StatefulWidget for TaskTable {
    type State = TaskTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_style = Style::default()
            .fg(self.secondary.c100)
            .bg(self.secondary.c950);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.secondary.c800)
            .bg(self.secondary.c100);
        let selected_col_style = Style::default().fg(self.secondary.c600);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.secondary.c800);

        let header = ["Name", "Speed", "Size", "Progress", "ETA", "Status"]
            .into_iter()
            .map(|c| Cell::from(Text::from(c.to_ascii_uppercase().to_string())))
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = state.tasks.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.primary.c900,
                _ => self.primary.c950,
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
                .style(Style::new().fg(self.primary.c100).bg(color))
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
                        format!("{} TASKS", self.title),
                        Style::default().fg(self.secondary.c500),
                    ),
                    Span::from(" ]"),
                ]))
                .border_style(Style::default().fg(match state.focused_block {
                    FocusedBlock::Content => self.secondary.c800,
                    _ => self.secondary.c950,
                }))
                .padding(Padding::new(0, 0, 1, 0)),
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from("  "))
        .highlight_spacing(HighlightSpacing::Always);

        t.render(area, buf, &mut state.table_state);
    }
}
