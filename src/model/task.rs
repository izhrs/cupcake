use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

use color_eyre::{Result, eyre::Ok};
use ratatui::widgets::{ScrollbarState, TableState};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::update::message::Message;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Task {
    #[serde(skip)]
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) destination: PathBuf,
    pub(crate) speed: String,
    pub(crate) size: String,
    pub(crate) progress: f32,
    pub(crate) eta: String,
    pub(crate) status: TaskStatus,
}

impl Task {
    fn update(&mut self, speed: String, progress: f32, eta: String, status: TaskStatus) {
        self.progress = progress;
        self.eta = eta;
        self.speed = speed;
        self.status = status;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Running,
    Paused,
    #[default]
    Queued,
    Finished,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            TaskStatus::Running => "Running ",
            TaskStatus::Paused => "Paused",
            TaskStatus::Queued => "Queued",
            TaskStatus::Finished => "Finished",
            TaskStatus::Failed => "Failed",
        };
        write!(f, "{status}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TaskState {
    db: VecDeque<Task>,
    #[serde(skip)]
    pub(crate) tasks: VecDeque<Task>,
    #[serde(skip)]
    pub(crate) table_state: TableState,
    #[serde(skip)]
    pub(crate) scroll_state: ScrollbarState,
}

impl TaskState {
    pub fn default() -> Self {
        Self {
            db: VecDeque::new(),
            tasks: VecDeque::new(),
            table_state: TableState::default(),
            scroll_state: ScrollbarState::default(),
        }
    }

    // this optimisation is called "suck my dick nigga"
    pub fn update_task_state(&mut self, task: Task) {
        let task_id = task.id;
        if let Some(existing_task) = self.db.iter_mut().find(|t| t.id == task_id) {
            existing_task.update(
                task.speed.clone(),
                task.progress,
                task.eta.clone(),
                task.status.clone(),
            );
        } else {
            self.db.push_back(task.clone());
        }

        if let Some(existing_task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            existing_task.update(task.speed, task.progress, task.eta, task.status);
        } else {
            self.tasks.push_back(task.clone());
        }
    }

    pub fn download_task(&self, source: &str, destination: PathBuf, tx: UnboundedSender<Message>) {
        let source = source.to_string();
        tokio::spawn(async move {
            if let Err(e) = async {

                let id = rand::random::<u64>();
                let name = Command::new("yt-dlp")
                    .arg("--no-warnings")
                    .arg("--print")
                    .arg("filename")
                    .arg(source.clone())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?
                    .wait_with_output()?
                    .stdout;

                let name = String::from_utf8(name)?;
                let mut cmd = Command::new("yt-dlp")
                .arg("--no-warnings")
                .arg("--newline")
                .arg("--progress-template")
                .arg("[CUPCAKE] %(progress._percent_str)s %(progress._total_bytes_str)s %(progress._speed_str)s ETA %(progress._eta_str)s")
                .arg(source.clone())
                .arg("-o")
                .arg(destination.to_str().unwrap_or(""))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

                let stdout = cmd.stdout.take().expect("Failed to capture stdout");
                let reader = BufReader::new(stdout);

                for line in reader.lines() {
                    let line = line?;

                    if line.starts_with("[CUPCAKE]") {
                        let parts: Vec<&str> = line[10..].split_whitespace().collect();

                        if parts.len() >= 5 {
                            let progress = parts[0].trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
                            let size = parts[1].to_string();
                            let status =  match progress {
                                100.0 => TaskStatus::Finished,
                                _ if progress > 0.0 => TaskStatus::Running,
                                _ => TaskStatus::Queued,
                            };

                            let _ = tx.send(Message::UpdateTaskStatus(Task {
                                name: name.clone(),
                                size,
                                source: source.to_string(),
                                destination: destination.join(name.clone()),
                                eta: parts[4].to_string(),
                                speed: parts[2].to_string(),
                                status,
                                progress,
                                id,
                            }));
                        }
                    }
                }

                Ok(())
            }.await {
                eprintln!("{e}");
            }
        });
    }

    pub fn next_row(&mut self) {
        let len = self.tasks.len();

        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * 3);
    }

    pub fn previous_row(&mut self) {
        let len = self.tasks.len();

        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * 3);
    }

    /// Filter tasks based on the selected menu item
    /// # Arguments
    /// * `selected_menu_item` - A slice of strings representing the selected menu item
    ///
    /// coming from `tui_tree_widget::TreeState.selected()`
    /// All the &str are Tree identifiers
    // TODO: refactor this and make it idiomatic
    // also, fuck this garbage, right now i just want it to work
    pub(crate) fn apply_menu_filter(&mut self, selected_menu_item: Vec<&str>) {
        match selected_menu_item.len() {
            1 => {
                let identifier = selected_menu_item[0];
                self.tasks = self
                    .db
                    .iter()
                    .filter(|&t| match identifier {
                        "unfinished" => {
                            !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
                        }
                        "finished" => matches!(t.status, TaskStatus::Finished),
                        "failed" => matches!(t.status, TaskStatus::Failed),
                        _ => true,
                    })
                    .cloned()
                    .collect();
                self.scroll_state = ScrollbarState::new(
                    (if !self.tasks.is_empty() {
                        self.tasks.len() - 1
                    } else {
                        0
                    }) * 3,
                );
            }

            2 => match selected_menu_item[1] {
                "all-music" | "finished-music" | "unfinished-music" => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-music" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-music" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
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
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-vids" | "finished-vids" | "unfinished-vids" => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-vids" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-vids" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
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
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-docs" | "finished-docs" | "unfinished-docs" => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-docs" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-docs" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
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
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-compressed" | "finished-compressed" | "unfinished-compressed" => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-compressed" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-compressed" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
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
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-programs" | "finished-programs" | "unfinished-programs" => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-programs" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-programs" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
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
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                others => {
                    self.tasks = self
                        .db
                        .iter()
                        .filter(|&t| {
                            (match others {
                                "finished-others" => matches!(t.status, TaskStatus::Finished),
                                "unfinished-others" => {
                                    !matches!(t.status, TaskStatus::Finished | TaskStatus::Failed)
                                }
                                _ => true,
                            }) && ([
                                ".mp3", ".wav", ".aac", ".ogg", ".m4a", ".flac", ".wma", ".aiff",
                                ".opus", ".dsd", ".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv",
                                ".webm", ".mpeg", ".mpg", ".3gp", ".pdf", ".doc", ".docx", ".xls",
                                ".xlsx", ".ppt", ".pptx", ".txt", ".rtf", ".odt", ".zip", ".rar",
                                ".tar", ".gz", ".7z", ".bz2", ".xz", ".iso", ".tgz", ".z", ".exe",
                                ".dll", ".msi", ".app", ".dmg", ".deb", ".rpm", ".sh", ".bin",
                                ".jar", ".apk", ".xapk",
                            ]
                            .iter()
                            .all(|&ext| !t.name.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.scroll_state = ScrollbarState::new(
                        (if !self.tasks.is_empty() {
                            self.tasks.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
            },
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TaskStore {
    pub single: TaskState,
    pub batch: TaskState,
    pub playlist: TaskState,
}

impl TaskStore {
    pub fn default() -> Self {
        Self {
            single: TaskState::default(),
            batch: TaskState::default(),
            playlist: TaskState::default(),
        }
    }
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self) -> Result<Self> {
        let path = dirs::data_local_dir().unwrap_or("".into()).join("cupcake");
        fs::create_dir_all(path.clone())?;
        let file = File::open(path.join("tasks.json"))?;
        let store: TaskStore = serde_json::from_reader(file)?;

        self.single = TaskState {
            db: store.single.db.clone(),
            tasks: store.single.db.clone(),
            table_state: TableState::default(),
            scroll_state: ScrollbarState::new(
                (if !store.single.db.is_empty() {
                    store.single.db.len() - 1
                } else {
                    0
                }) * 3,
            ),
        };

        self.batch = TaskState {
            db: store.batch.db.clone(),
            tasks: store.batch.db.clone(),
            table_state: TableState::default(),
            scroll_state: ScrollbarState::new(
                (if !store.batch.db.is_empty() {
                    store.batch.db.len() - 1
                } else {
                    0
                }) * 3,
            ),
        };

        self.playlist = TaskState {
            db: store.playlist.db.clone(),
            tasks: store.playlist.db.clone(),
            table_state: TableState::default(),
            scroll_state: ScrollbarState::new(
                (if !store.playlist.db.is_empty() {
                    store.playlist.db.len() - 1
                } else {
                    0
                }) * 3,
            ),
        };

        Ok(self.clone())
    }

    pub fn save(&self) -> Result<()> {
        let path = dirs::data_local_dir()
            .unwrap_or(std::env::current_dir().unwrap())
            .join("cupcake");
        fs::create_dir_all(path.clone())?;
        let file = File::create(path.join("tasks.json"))?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}
