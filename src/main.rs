use color_eyre::Result;
use crossterm::event::KeyModifiers;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize, palette::tailwind},
    symbols::block,
    text::Text,
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Padding, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Tabs,
    },
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

enum TaskStaus {
    RUNNING,
    PAUSED,
    QUEUED,
    COMPLETED,
    FAILED,
}

impl std::fmt::Display for TaskStaus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            TaskStaus::RUNNING => "Running",
            TaskStaus::PAUSED => "Paused",
            TaskStaus::QUEUED => "Queued",
            TaskStaus::COMPLETED => "Completed",
            TaskStaus::FAILED => "Failed",
        };
        write!(f, "{}", status)
    }
}
struct Task {
    name: String,
    speed: f32,
    size: f32,
    progress: f32,
    eta: String,
    status: TaskStaus,
}

struct App {
    table_state: TableState,
    scrollbar_state: ScrollbarState,
    progress: f32,
    items: Vec<Task>,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy)]
enum SelectedTab {
    #[default]
    Single,
    Playlist,
    Settings,
    About,
}

impl App {
    fn new() -> Self {
        let dummy_items = vec![
            Task {
                name: "Task 1".to_string(),
                speed: 1.0,
                size: 100.0,
                progress: 0.5,
                eta: "1m".to_string(),
                status: TaskStaus::RUNNING,
            },
            Task {
                name: "Task 2".to_string(),
                speed: 2.0,
                size: 200.0,
                progress: 0.75,
                eta: "2m".to_string(),
                status: TaskStaus::PAUSED,
            },
            Task {
                name: "Task 3".to_string(),
                speed: 0.5,
                size: 50.0,
                progress: 0.25,
                eta: "3m".to_string(),
                status: TaskStaus::QUEUED,
            },
            Task {
                name: "Task 4".to_string(),
                speed: 3.0,
                size: 300.0,
                progress: 1.0,
                eta: "0m".to_string(),
                status: TaskStaus::COMPLETED,
            },
            Task {
                name: "Task 5".to_string(),
                speed: 0.0,
                size: 0.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::FAILED,
            },
        ];
        Self {
            table_state: TableState::default(),
            scrollbar_state: ScrollbarState::default(),
            progress: 0.0,
            items: dummy_items,
            selected_tab: SelectedTab::default(),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => {
                            self.selected_tab = self.selected_tab.next();
                        }
                        KeyCode::BackTab => {
                            self.selected_tab = self.selected_tab.previous();
                        }
                        KeyCode::Up => {
                            self.previous_row();
                        }
                        KeyCode::Down => {
                            self.next_row();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let screen = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(tailwind::NEUTRAL.c500));

        frame.render_widget(&screen, frame.area());

        let outer_layout = Layout::vertical(vec![Constraint::Min(5), Constraint::Length(3)])
            .split(screen.inner(frame.area()));

        let inner_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25), // menu
                Constraint::Percentage(75), // table
            ])
            .split(outer_layout[0]);

        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Min(5)])
            .split(inner_layout[1]);

        let action_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Min(38), Constraint::Max(12)])
            .flex(ratatui::layout::Flex::SpaceBetween)
            .split(content_layout[0]);

        self.render_div(frame, inner_layout[0]);

        self.render_tabs(frame, action_layout[0]);
        self.render_action_button(frame, action_layout[1]);
        self.render_table(frame, content_layout[1]);

        self.render_div(frame, outer_layout[1]);
    }

    fn render_action_button(&self, frame: &mut Frame, area: Rect) {
        let button = Paragraph::new("ADD TASK")
            .style(
                Style::default()
                    .fg(tailwind::GREEN.c500)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(tailwind::GREEN.c500)),
            );
        frame.render_widget(button, area);
    }

    fn render_div(&self, frame: &mut Frame, area: Rect) {
        let div = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(tailwind::NEUTRAL.c500));
        frame.render_widget(div, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["SINGLE", "PLAYLIST", "SETTINGS", "ABOUT"])
            .select(self.selected_tab as usize)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            )
            .highlight_style(
                Style::default()
                    .fg(tailwind::PURPLE.c500)
                    .add_modifier(Modifier::BOLD),
            )
            .divider("|")
            .style(Style::default());

        frame.render_widget(tabs, area);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(tailwind::NEUTRAL.c200)
            .bg(tailwind::NEUTRAL.c900);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::PURPLE.c500);
        let selected_col_style = Style::default().fg(tailwind::PURPLE.c600);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::PURPLE.c600);

        let header = ["Name", "Speed", "Size", "Progress", "ETA", "Status"]
            .into_iter()
            .map(|c| Cell::from(Text::from(format!("\n{}\n", c.to_ascii_uppercase()))))
            .collect::<Row>()
            .style(header_style)
            .height(3)
            .bottom_margin(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => tailwind::NEUTRAL.c900,
                _ => tailwind::NEUTRAL.c950,
            };
            let item = [
                Text::from(format!("{}", data.name)),
                Text::from(format!("{:.2} MB/s", data.speed)),
                Text::from(format!("{:.2} MB", data.size)),
                Text::from(format!("{:.2} %", data.progress * 100.0)),
                Text::from(data.eta.clone()),
                Text::from(data.status.to_string()),
            ];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
                .collect::<Row>()
                .style(Style::new().fg(tailwind::NEUTRAL.c100).bg(color))
                .height(3)
        });
        let t = Table::new(
            rows,
            [
                Constraint::Min(5),     // name
                Constraint::Length(10), // speed
                Constraint::Length(10), // size
                Constraint::Length(10), // progress
                Constraint::Length(10), // eta
                Constraint::Length(15), // status
            ],
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title(" TASTKS ")
                .border_style(Style::default().fg(tailwind::NEUTRAL.c500))
                .padding(Padding::uniform(1)),
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from("  "))
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.table_state);
    }

    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * 4);
    }

    fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * 4);
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        match previous_index {
            0 => SelectedTab::Single,
            1 => SelectedTab::Playlist,
            2 => SelectedTab::Settings,
            3 => SelectedTab::About,
            _ => SelectedTab::Single,
        }
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        match next_index {
            0 => SelectedTab::Single,
            1 => SelectedTab::Playlist,
            2 => SelectedTab::Settings,
            3 => SelectedTab::About,
            _ => SelectedTab::Single,
        }
    }
}
