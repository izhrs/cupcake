use color_eyre::Result;
use crossterm::event::KeyModifiers;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
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
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let screen = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
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

        self.render_div(frame, inner_layout[0]);

        self.render_tabs(frame, content_layout[0]);
        self.render_div(frame, content_layout[1]);

        self.render_div(frame, outer_layout[1]);
    }

    fn render_div(&self, frame: &mut Frame, area: Rect) {
        let div = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(tailwind::NEUTRAL.c500));
        frame.render_widget(div, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["SIN", "COS", "TAN"])
            .select(self.selected_tab as usize)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(tailwind::PURPLE.c500))
            .divider("|")
            .style(Style::default());

        frame.render_widget(tabs, area);
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
            _ => SelectedTab::Single,
        }
    }
}
