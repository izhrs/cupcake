use std::path::PathBuf;

use color_eyre::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Modifier, Style, Stylize, palette::tailwind},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, Gauge, HighlightSpacing, Padding, Paragraph, Row,
        Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Tabs,
    },
};

use tui_tree_widget::{Tree, TreeItem, TreeState};
use url::Url;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Clone)]
enum TaskStaus {
    Running,
    Paused,
    Queued,
    Finished,
    Failed,
}

impl std::fmt::Display for TaskStaus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            TaskStaus::Running => "Running ",
            TaskStaus::Paused => "Paused",
            TaskStaus::Queued => "Queued",
            TaskStaus::Finished => "Finished",
            TaskStaus::Failed => "Failed",
        };
        write!(f, "{}", status)
    }
}

#[derive(Clone)]
struct Task {
    name: String,
    source: Url,
    destination: PathBuf,
    speed: f32,
    size: f32,
    progress: f32,
    eta: String,
    status: TaskStaus,
}

enum FocusedBlock {
    Content,
    Menu,
    Modal,
}

struct TaskState {
    tasks: Vec<Task>,
}

impl TaskState {
    fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}

struct App {
    focused_block: FocusedBlock, // focused window
    table_state: TableState,
    scrollbar_state: ScrollbarState,
    progress: f32,
    task_items: Vec<Task>, // this is just for storing original tasks
    task_state: TaskState, // this is for performing filter operations
    selected_tab: SelectedTab,
    menu_state: TreeState<&'static str>,
    menu_items: Vec<TreeItem<'static, &'static str>>,
    running: bool,
}

#[derive(Default, Clone, Copy)]
enum SelectedTab {
    #[default]
    Single,
    Batch,
    Playlist,
    Settings,
    About,
}

impl App {
    fn new() -> Self {
        let dummy_items = vec![
            Task {
                name: "ubuntu-22.04-desktop-amd64.iso".to_string(),
                source: Url::parse(
                    "https://releases.ubuntu.com/22.04/ubuntu-22.04-desktop-amd64.iso",
                )
                .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/ubuntu-22.04-desktop-amd64.iso"),
                speed: 1.0,
                size: 3700.0,
                progress: 0.5,
                eta: "1m".to_string(),
                status: TaskStaus::Running,
            },
            Task {
                name: "big_buck_bunny_1080p.mp4".to_string(),
                source: Url::parse("https://www.example.com/big_buck_bunny_1080p.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/big_buck_bunny_1080p.mp4"),
                speed: 2.0,
                size: 825.0,
                progress: 0.75,
                eta: "2m".to_string(),
                status: TaskStaus::Paused,
            },
            Task {
                name: "node_modules.tar.gz".to_string(),
                source: Url::parse("https://www.example.com/node_modules.tar.gz").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/node_modules.tar.gz"),
                speed: 0.5,
                size: 154.0,
                progress: 0.25,
                eta: "3m".to_string(),
                status: TaskStaus::Queued,
            },
            Task {
                name: "linux-kernel-6.2.0.tar".to_string(),
                source: Url::parse("https://www.example.com/linux-kernel-6.2.0.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/linux-kernel-6.2.0.tar"),
                speed: 3.0,
                size: 1200.0,
                progress: 1.0,
                eta: "0m".to_string(),
                status: TaskStaus::Finished,
            },
            Task {
                name: "game_assets.zip".to_string(),
                source: Url::parse("https://www.example.com/game_assets.zip").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/game_assets.zip"),
                speed: 0.0,
                size: 4500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "fedora-38-x86_64.iso".to_string(),
                source: Url::parse("https://releases.fedoraproject.org/38/fedora-38-x86_64.iso")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/fedora-38-x86_64.iso"),
                speed: 0.0,
                size: 2800.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "movie_collection.tar".to_string(),
                source: Url::parse("https://www.example.com/movie_collection.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/movie_collection.tar"),
                speed: 0.0,
                size: 15000.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "project_backup_2023-10-15.zip".to_string(),
                source: Url::parse("https://www.example.com/project_backup_2023-10-15.zip")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/project_backup_2023-10-15.zip"),
                speed: 0.0,
                size: 750.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "archlinux-2023.05.01-x86_64.iso".to_string(),
                source: Url::parse(
                    "https://archlinux.org/iso/2023.05.01/archlinux-2023.05.01-x86_64.iso",
                )
                .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/archlinux-2023.05.01-x86_64.iso"),
                speed: 0.0,
                size: 850.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "4k_nature_documentary.mkv".to_string(),
                source: Url::parse("https://www.example.com/4k_nature_documentary.mkv").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/4k_nature_documentary.mkv"),
                speed: 0.0,
                size: 12000.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "debian-11-amd64.iso".to_string(),
                source: Url::parse("https://www.example.com/debian-11-amd64.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/debian-11-amd64.iso"),
                speed: 0.0,
                size: 3200.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "sample_video_1080p_60fps.mp4".to_string(),
                source: Url::parse("https://www.example.com/sample_video_1080p_60fps.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/sample_video_1080p_60fps.mp4"),
                speed: 0.0,
                size: 1500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "website_templates.zip".to_string(),
                source: Url::parse("https://www.example.com/website_templates.zip").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/website_templates.zip"),
                speed: 0.0,
                size: 225.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "react_native_project.tar.gz".to_string(),
                source: Url::parse("https://www.example.com/react_native_project.tar.gz").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/react_native_project.tar.gz"),
                speed: 0.0,
                size: 180.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "centos-stream-9-x86_64.iso".to_string(),
                source: Url::parse("https://www.example.com/centos-stream-9-x86_64.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/centos-stream-9-x86_64.iso"),
                speed: 0.0,
                size: 2500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "conference_recordings.tar".to_string(),
                source: Url::parse("https://www.example.com/conference_recordings.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/conference_recordings.tar"),
                speed: 0.0,
                size: 8500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "anime_series_s01_complete.mkv".to_string(),
                source: Url::parse("https://www.example.com/anime_series_s01_complete.mkv")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/anime_series_s01_complete.mkv"),
                speed: 0.0,
                size: 5800.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "docker_images.tar".to_string(),
                source: Url::parse("https://www.example.com/docker_images.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/docker_images.tar"),
                speed: 0.0,
                size: 3400.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "kali-linux-2023.2-live.iso".to_string(),
                source: Url::parse("https://www.example.com/kali-linux-2023.2-live.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/kali-linux-2023.2-live.iso"),
                speed: 0.0,
                size: 4100.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "machine_learning_datasets.zip".to_string(),
                source: Url::parse("https://www.example.com/machine_learning_datasets.zip")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/machine_learning_datasets.zip"),
                speed: 0.0,
                size: 2700.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStaus::Failed,
            },
            Task {
                name: "Emily_Wills_4K.mp4".to_string(),

                source: Url::parse("https://www.example.com/Emily_Wills_4K.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/Emily_Wills_4K.mp4"),
                speed: 5.2,
                size: 2439.0,
                progress: 1.00,
                eta: "0m".to_string(),
                status: TaskStaus::Finished,
            },
        ];
        Self {
            running: true,
            focused_block: FocusedBlock::Content,
            table_state: TableState::default(),
            scrollbar_state: ScrollbarState::default(),
            progress: 0.0,
            task_items: dummy_items.clone(),
            task_state: TaskState::new(dummy_items),
            selected_tab: SelectedTab::default(),
            menu_state: TreeState::default(),
            menu_items: vec![
                TreeItem::new(
                    "all",
                    Text::from("ALL DOWNLOADS").style(
                        Style::default()
                            .fg(tailwind::NEUTRAL.c400)
                            .add_modifier(Modifier::BOLD),
                    ),
                    vec![
                        TreeItem::new_leaf("all-music", "󰎆 Music"),
                        TreeItem::new_leaf("all-vids", " Videos"),
                        TreeItem::new_leaf("all-docs", "󰈙 Documents"),
                        TreeItem::new_leaf("all-compressed", " Compressed"),
                        TreeItem::new_leaf("all-programs", " Programs"),
                        TreeItem::new_leaf("all-others", " Others"),
                    ],
                )
                .expect("all item identifiers must be unique"),
                TreeItem::new(
                    "unfinished",
                    Text::from("UNFINISHED").style(
                        Style::default()
                            .fg(tailwind::NEUTRAL.c400)
                            .add_modifier(Modifier::BOLD),
                    ),
                    vec![
                        TreeItem::new_leaf("unfinished-music", "󰎆 Music"),
                        TreeItem::new_leaf("unfinished-vids", " Videos"),
                        TreeItem::new_leaf("unfinished-docs", "󰈙 Documents"),
                        TreeItem::new_leaf("unfinished-compressed", " Compressed"),
                        TreeItem::new_leaf("unfinished-programs", " Programs"),
                        TreeItem::new_leaf("unfinished-others", " Others"),
                    ],
                )
                .expect("all item identifiers must be unique"),
                TreeItem::new(
                    "finished",
                    Text::from("FINISHED").style(
                        Style::default()
                            .fg(tailwind::NEUTRAL.c400)
                            .add_modifier(Modifier::BOLD),
                    ),
                    vec![
                        TreeItem::new_leaf("finished-music", "󰎆 Music"),
                        TreeItem::new_leaf("finished-vids", " Videos"),
                        TreeItem::new_leaf("finished-docs", "󰈙 Documents"),
                        TreeItem::new_leaf("finished-compressed", " Compressed"),
                        TreeItem::new_leaf("finished-programs", " Programs"),
                        TreeItem::new_leaf("finished-others", " Others"),
                    ],
                )
                .expect("all item identifiers must be unique"),
                TreeItem::new(
                    "failed",
                    Text::from("FAILED").style(
                        Style::default()
                            .fg(tailwind::NEUTRAL.c400)
                            .add_modifier(Modifier::BOLD),
                    ),
                    vec![
                        TreeItem::new_leaf("failed-rec", "Recoverable"),
                        TreeItem::new_leaf("failed-unr", "Unrecoverable"),
                    ],
                )
                .expect("all item identifiers must be unique"),
            ],
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Open the "ALL DOWNLOADS" and "UNFINISHED" menu items by default
        // Can't set this state in the constructor because fields are private in the TreeState
        self.menu_state.open(vec!["all"]);

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.focused_block {
                    FocusedBlock::Content => match key.code {
                        KeyCode::Left if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.focused_block = FocusedBlock::Menu;
                        }

                        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.focused_block = FocusedBlock::Modal;
                        }

                        KeyCode::Char('q') => self.running = false,
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
                        KeyCode::Right => {
                            if self.progress < 100.0 {
                                self.progress += 1.0;
                            }
                        }
                        KeyCode::Left => {
                            if self.progress > 0.0 {
                                self.progress -= 1.0;
                            }
                        }
                        _ => {}
                    },
                    FocusedBlock::Menu => match key.code {
                        KeyCode::Right if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.focused_block = FocusedBlock::Content;
                        }
                        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.focused_block = FocusedBlock::Modal;
                        }
                        KeyCode::Enter => {
                            // self.focused_block = FocusedBlock::Content;
                            self.filter_tasks();
                        }
                        KeyCode::Char('q') => self.focused_block = FocusedBlock::Content,
                        KeyCode::Char(' ') => {
                            self.menu_state.toggle_selected();
                        }
                        KeyCode::Left => {
                            self.menu_state.key_left();
                        }
                        KeyCode::Right => {
                            self.menu_state.key_right();
                        }
                        KeyCode::Down => {
                            self.menu_state.key_down();
                        }
                        KeyCode::Up => {
                            self.menu_state.key_up();
                        }

                        KeyCode::Home => {
                            self.menu_state.select_first();
                        }
                        KeyCode::End => {
                            self.menu_state.select_last();
                        }
                        _ => {}
                    },
                    FocusedBlock::Modal => match key.code {
                        KeyCode::Esc => {
                            self.focused_block = FocusedBlock::Content;
                        }
                        _ => {}
                    },
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let screen = Block::default().borders(Borders::NONE).style(
            Style::default()
                .fg(tailwind::NEUTRAL.c500)
                .bg(tailwind::NEUTRAL.c950),
        );

        frame.render_widget(&screen, frame.area());

        // Entire screen layout
        let main_layout = Layout::vertical(vec![
            Constraint::Min(10),   // everything
            Constraint::Length(3), // progress bar (bottom)
        ])
        .split(screen.inner(frame.area()));

        let body_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(30), // sidebar (menu and logo)
                Constraint::Min(10),    // tab - button and content
            ])
            .split(main_layout[0]);

        let sidebar_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Length(5), // logo
                Constraint::Min(10),   // menu
            ])
            .split(body_layout[0]);

        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3), // tabs and button
                Constraint::Min(10),   // main content (all tasks are rendered here)
            ])
            .split(body_layout[1]);

        let action_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(20), // tabs
                Constraint::Max(12), // add task button (action button)
            ])
            .flex(ratatui::layout::Flex::SpaceBetween)
            .split(content_layout[0]);

        self.render_logo(frame, sidebar_layout[0]);
        self.render_menu(frame, sidebar_layout[1]);

        self.render_tabs(frame, action_layout[0]);
        self.render_action_button(frame, action_layout[1]);

        self.render_table(frame, content_layout[1]);
        self.render_scrollbar(frame, content_layout[1]);

        self.render_progress_bar(frame, main_layout[1]);
    }

    fn render_action_button(&self, frame: &mut Frame, area: Rect) {
        let button = Paragraph::new("ADD TASK")
            .style(
                Style::default()
                    .fg(tailwind::GREEN.c600)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(tailwind::GREEN.c600)),
            );
        frame.render_widget(button, area);
    }

    fn render_logo(&self, frame: &mut Frame, area: Rect) {
        let logo = Paragraph::new(
            Line::from(vec![
                Span::styled("C ", Style::default().fg(tailwind::PURPLE.c500)),
                Span::styled("U ", Style::default().fg(tailwind::GREEN.c500)),
                Span::styled("P ", Style::default().fg(tailwind::BLUE.c500)),
                Span::styled("C ", Style::default().fg(tailwind::ORANGE.c500)),
                Span::styled("A ", Style::default().fg(tailwind::VIOLET.c500)),
                Span::styled("K ", Style::default().fg(tailwind::CYAN.c500)),
                Span::styled("E", Style::default().fg(tailwind::ROSE.c500)),
            ])
            .alignment(Alignment::Center),
        )
        .add_modifier(Modifier::BOLD)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(tailwind::PURPLE.c950))
                .padding(Padding::uniform(1)),
        );

        frame.render_widget(logo, area);
    }

    fn render_menu(&mut self, frame: &mut Frame, area: Rect) {
        let widget = Tree::new(&self.menu_items)
            .expect("all item identifiers must be unique")
            .block(
                Block::default()
                    .title_bottom(format!("{:?}", self.menu_state.selected()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(match self.focused_block {
                        FocusedBlock::Menu => tailwind::PURPLE.c800,
                        _ => tailwind::PURPLE.c950,
                    }))
                    .padding(Padding::symmetric(2, 1)),
            )
            .highlight_style(
                Style::new()
                    .fg(tailwind::PURPLE.c500)
                    .bg(tailwind::NEUTRAL.c900)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("");
        frame.render_stateful_widget(widget, area, &mut self.menu_state);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["SINGLE", "BATCH", "PLAYLIST", "SETTINGS", "ABOUT"])
            .select(self.selected_tab as usize)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(tailwind::PURPLE.c950)),
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
            .fg(tailwind::PURPLE.c100)
            .bg(tailwind::PURPLE.c950);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::PURPLE.c800)
            .bg(tailwind::PURPLE.c100);
        let selected_col_style = Style::default().fg(tailwind::PURPLE.c600);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::PURPLE.c800);

        let header = ["Name", "Speed", "Size", "Progress", "ETA", "Status"]
            .into_iter()
            .map(|c| Cell::from(Text::from(c.to_ascii_uppercase().to_string())))
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.task_state.tasks.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => tailwind::NEUTRAL.c900,
                _ => tailwind::NEUTRAL.c950,
            };

            let item = [
                Text::from(data.name.to_string()),
                Text::from(format!("{:.2} MB/s", data.speed)),
                Text::from(format!("{:.0} MB", data.size)),
                Text::from(format!("{:.0} %", data.progress * 100.0)),
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
                    Span::styled("TASKS", Style::default().fg(tailwind::PURPLE.c500)),
                    Span::from(" ]"),
                ]))
                .border_style(Style::default().fg(match self.focused_block {
                    FocusedBlock::Content => tailwind::PURPLE.c800,
                    _ => tailwind::PURPLE.c950,
                }))
                .padding(Padding::new(0, 0, 1, 0)),
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from("  "))
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scrollbar_state,
        );
    }

    fn render_progress_bar(&self, frame: &mut Frame, area: Rect) {
        let progress_bar = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title(Line::from(vec![
                        Span::from("[ "),
                        Span::styled("PROGRESS", Style::default().fg(tailwind::PURPLE.c500)),
                        Span::from(" ]"),
                    ]))
                    .border_style(Style::default().fg(tailwind::PURPLE.c950)),
            )
            .gauge_style(
                Style::default()
                    .fg(match self.progress {
                        0.0..=25.0 => tailwind::PURPLE.c800,
                        25.0..=50.0 => tailwind::PURPLE.c700,
                        50.0..=75.0 => tailwind::PURPLE.c600,
                        _ => tailwind::PURPLE.c500,
                    })
                    .bg(tailwind::PURPLE.c950),
            )
            .percent(self.progress as u16);
        frame.render_widget(progress_bar, area);
    }

    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.task_state.tasks.len() - 1 {
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
                    self.task_state.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * 4);
    }

    fn filter_tasks(&mut self) {
        match self.menu_state.selected().len() {
            1 => match self.menu_state.selected()[0] {
                "unfinished" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed))
                        .cloned()
                        .collect();
                }
                "finished" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| matches!(t.status, TaskStaus::Finished))
                        .cloned()
                        .collect();
                }
                "failed" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| matches!(t.status, TaskStaus::Failed))
                        .cloned()
                        .collect();
                }
                _ => self.task_state.tasks = self.task_items.clone(),
            },

            2 => match self.menu_state.selected()[1] {
                "all-music" | "finished-music" | "unfinished-music" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| {
                            (match self.menu_state.selected()[1] {
                                "finished-music" => matches!(t.status, TaskStaus::Finished),
                                "unfinished-music" => {
                                    !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".mp3", ".wav", ".aac", ".ogg", ".m4a", ".flac", ".wma", ".aiff",
                                ".opus", ".dsd",
                            ]
                            .iter()
                            .any(|&ext| t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                }
                "all-vids" | "finished-vids" | "unfinished-vids" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| {
                            (match self.menu_state.selected()[1] {
                                "finished-vids" => matches!(t.status, TaskStaus::Finished),
                                "unfinished-vids" => {
                                    !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv", ".webm", ".mpeg",
                                ".mpg", ".3gp",
                            ]
                            .iter()
                            .any(|&ext| t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                }
                "all-docs" | "finished-docs" | "unfinished-docs" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| {
                            (match self.menu_state.selected()[1] {
                                "finished-docs" => matches!(t.status, TaskStaus::Finished),
                                "unfinished-docs" => {
                                    !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".txt",
                                ".rtf", ".odt",
                            ]
                            .iter()
                            .any(|&ext| t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                }
                "all-compressed" | "finished-compressed" | "unfinished-compressed" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| {
                            (match self.menu_state.selected()[1] {
                                "finished-compressed" => matches!(t.status, TaskStaus::Finished),
                                "unfinished-compressed" => {
                                    !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".zip", ".rar", ".tar", ".gz", ".7z", ".bz2", ".xz", ".iso",
                                ".tgz", ".z",
                            ]
                            .iter()
                            .any(|&ext| t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                }
                "all-programs" | "finished-programs" | "unfinished-programs" => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| {
                            (match self.menu_state.selected()[1] {
                                "finished-programs" => matches!(t.status, TaskStaus::Finished),
                                "unfinished-programs" => {
                                    !matches!(t.status, TaskStaus::Finished | TaskStaus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".exe", ".dll", ".msi", ".app", ".dmg", ".deb", ".rpm", ".sh",
                                ".bin", ".jar", ".apk", ".xapk",
                            ]
                            .iter()
                            .any(|&ext| t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                }
                _ => {
                    self.task_state.tasks = self
                        .task_items
                        .iter()
                        .filter(|&t| matches!(t.status, TaskStaus::Failed))
                        .cloned()
                        .collect();
                }
            },
            _ => {}
        }
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        match previous_index {
            0 => SelectedTab::Single,
            1 => SelectedTab::Batch,
            2 => SelectedTab::Playlist,
            3 => SelectedTab::Settings,
            4 => SelectedTab::About,
            _ => SelectedTab::Single,
        }
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        match next_index {
            0 => SelectedTab::Single,
            1 => SelectedTab::Batch,
            2 => SelectedTab::Playlist,
            3 => SelectedTab::Settings,
            4 => SelectedTab::About,
            _ => SelectedTab::Single,
        }
    }
}
