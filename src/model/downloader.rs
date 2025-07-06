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
pub struct DownloadTask {
    #[serde(skip)]
    id: u64,
    pub title: String,
    pub source_url: String,
    pub destination_path: PathBuf,
    pub download_speed: String,
    pub file_size: String,
    pub progress_percent: f32,
    pub estimated_time: String,
    pub status: DownloadStatus,
}

impl DownloadTask {
    fn update(&mut self, speed: String, progress: f32, eta: String, status: DownloadStatus) {
        self.progress_percent = progress;
        self.estimated_time = eta;
        self.download_speed = speed;
        self.status = status;
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    #[default]
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            DownloadStatus::Running => "Running ",
            DownloadStatus::Paused => "Paused",
            DownloadStatus::Queued => "Queued",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed => "Failed",
        };
        write!(f, "{status}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DownloadManager {
    downloads: VecDeque<DownloadTask>,
    #[serde(skip)]
    pub(crate) state: DownloadManagerUIState,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DownloadManagerUIState {
    pub(crate) filtered_downloads: VecDeque<DownloadTask>,
    pub(crate) table_state: TableState,
    pub(crate) scroll_state: ScrollbarState,
}

impl DownloadManager {
    // this optimisation is called "bitch, suck my dick"
    pub fn update_download(&mut self, task: DownloadTask) {
        let task_id = task.id;
        if let Some(existing_task) = self.downloads.iter_mut().find(|t| t.id == task_id) {
            existing_task.update(
                task.download_speed.clone(),
                task.progress_percent,
                task.estimated_time.clone(),
                task.status.clone(),
            );
        } else {
            self.downloads.push_back(task.clone());
        }

        if let Some(existing_task) = self
            .state
            .filtered_downloads
            .iter_mut()
            .find(|t| t.id == task_id)
        {
            existing_task.update(
                task.download_speed,
                task.progress_percent,
                task.estimated_time,
                task.status,
            );
        } else {
            self.state.filtered_downloads.push_back(task.clone());
        }
    }

    pub fn filtered_downloads(&self) -> &VecDeque<DownloadTask> {
        &self.state.filtered_downloads
    }

    fn slugify(input: &str, separator: char) -> String {
        input
            .trim()
            .to_lowercase()
            .chars()
            .filter_map(|c| match c {
                'a'..='z' | '0'..='9' | '.' => Some(c), // preserve file extensions
                ' ' | '_' | '-' => Some(separator),
                _ if c.is_ascii() => None,
                _ => Some(separator),
            })
            .collect::<String>()
            .split(separator)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(separator.to_string().as_str())
    }

    fn extract_filename(source: &str) -> Result<String> {
        let name = Command::new("yt-dlp")
            .arg("--no-warnings")
            .arg("--print")
            .arg("filename")
            .arg(source)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?
            .stdout;

        Ok(Self::slugify(String::from_utf8(name)?.as_str(), '_'))
    }

    pub fn start_download(&self, source: &str, destination: PathBuf, tx: UnboundedSender<Message>) {
        let source = source.to_string();
        tokio::spawn(async move {
            if let Err(e) = async {

                let id = rand::random::<u64>();
                let title = Self::extract_filename(&source)?;

                let mut cmd = Command::new("yt-dlp")
                .arg("--no-warnings")
                .arg("--newline")
                .arg("--progress-template")
                .arg("[CUPCAKE] %(progress._percent_str)s %(progress._total_bytes_str)s %(progress._speed_str)s ETA %(progress._eta_str)s")
                .arg(source.clone())
                .arg("-o")
                .arg(format!("{}/{}", destination.to_string_lossy(), title))
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
                            let file_size = parts[1].to_string();
                            let status =  match progress {
                                100.0 => DownloadStatus::Completed,
                                _ if progress > 0.0 => DownloadStatus::Running,
                                _ => DownloadStatus::Queued,
                            };

                            let _ = tx.send(Message::UpdateDownloadStatus(DownloadTask {
                                title: title.clone(),
                                file_size,
                                source_url: source.to_string(),
                                destination_path: destination.join(title.clone()),
                                estimated_time: parts[4].to_string(),
                                download_speed: parts[2].to_string(),
                                status,
                                progress_percent: progress,
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

    pub(crate) fn next_row(&mut self) {
        let len = self.state.filtered_downloads.len();

        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * 3);
    }

    pub(crate) fn previous_row(&mut self) {
        let len = self.state.filtered_downloads.len();

        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * 3);
    }

    /// Filter tasks based on the selected menu item
    /// # Arguments
    /// * `selected_menu_item` - A slice of strings representing the selected menu item
    ///
    /// coming from `tui_tree_widget::TreeState.selected()`
    /// All the &str are Tree identifiers
    // TODO: refactor this and make it idiomatic
    // also, fuck this garbage, right now i just want it to work
    pub fn filter_downloads_by_category(&mut self, selected_menu_item: Vec<&str>) {
        match selected_menu_item.len() {
            1 => {
                let identifier = selected_menu_item[0];
                self.state.filtered_downloads = self
                    .downloads
                    .iter()
                    .filter(|&t| match identifier {
                        "unfinished" => {
                            !matches!(t.status, DownloadStatus::Completed | DownloadStatus::Failed)
                        }
                        "finished" => matches!(t.status, DownloadStatus::Completed),
                        "failed" => matches!(t.status, DownloadStatus::Failed),
                        _ => true,
                    })
                    .cloned()
                    .collect();
                self.state.scroll_state = ScrollbarState::new(
                    (if !self.state.filtered_downloads.is_empty() {
                        self.state.filtered_downloads.len() - 1
                    } else {
                        0
                    }) * 3,
                );
            }

            2 => match selected_menu_item[1] {
                "all-music" | "finished-music" | "unfinished-music" => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-music" => matches!(t.status, DownloadStatus::Completed),
                                "unfinished-music" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
                                _ => true,
                            }) && ([
                                ".mp3", ".wav", ".aac", ".ogg", ".m4a", ".flac", ".wma", ".aiff",
                                ".opus", ".dsd",
                            ]
                            .iter()
                            .any(|&ext| t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-vids" | "finished-vids" | "unfinished-vids" => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-vids" => matches!(t.status, DownloadStatus::Completed),
                                "unfinished-vids" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
                                _ => true,
                            }) && ([
                                ".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv", ".webm", ".mpeg",
                                ".mpg", ".3gp",
                            ]
                            .iter()
                            .any(|&ext| t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-docs" | "finished-docs" | "unfinished-docs" => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-docs" => matches!(t.status, DownloadStatus::Completed),
                                "unfinished-docs" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
                                _ => true,
                            }) && ([
                                ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".txt",
                                ".rtf", ".odt",
                            ]
                            .iter()
                            .any(|&ext| t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-compressed" | "finished-compressed" | "unfinished-compressed" => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-compressed" => {
                                    matches!(t.status, DownloadStatus::Completed)
                                }
                                "unfinished-compressed" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
                                _ => true,
                            }) && ([
                                ".zip", ".rar", ".tar", ".gz", ".7z", ".bz2", ".xz", ".iso",
                                ".tgz", ".z",
                            ]
                            .iter()
                            .any(|&ext| t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                "all-programs" | "finished-programs" | "unfinished-programs" => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match selected_menu_item[1] {
                                "finished-programs" => {
                                    matches!(t.status, DownloadStatus::Completed)
                                }
                                "unfinished-programs" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
                                _ => true,
                            }) && ([
                                ".exe", ".dll", ".msi", ".app", ".dmg", ".deb", ".rpm", ".sh",
                                ".bin", ".jar", ".apk", ".xapk",
                            ]
                            .iter()
                            .any(|&ext| t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
                        } else {
                            0
                        }) * 3,
                    );
                }
                others => {
                    self.state.filtered_downloads = self
                        .downloads
                        .iter()
                        .filter(|&t| {
                            (match others {
                                "finished-others" => matches!(t.status, DownloadStatus::Completed),
                                "unfinished-others" => !matches!(
                                    t.status,
                                    DownloadStatus::Completed | DownloadStatus::Failed
                                ),
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
                            .all(|&ext| !t.title.ends_with(ext)))
                        })
                        .cloned()
                        .collect();
                    self.state.scroll_state = ScrollbarState::new(
                        (if !self.state.filtered_downloads.is_empty() {
                            self.state.filtered_downloads.len() - 1
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
pub struct Downloader {
    pub single: DownloadManager,
    pub batch: DownloadManager,
    pub playlist: DownloadManager,
}

impl Downloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self) -> Result<Self> {
        let path = dirs::data_local_dir().unwrap_or("".into()).join("cupcake");
        fs::create_dir_all(path.clone())?;
        let file = File::open(path.join("tasks.json"))?;
        let store: Downloader = serde_json::from_reader(file)?;

        self.single = DownloadManager {
            downloads: store.single.downloads.clone(),
            state: DownloadManagerUIState {
                filtered_downloads: store.single.downloads.clone(),
                table_state: TableState::default(),
                scroll_state: ScrollbarState::new(
                    (if !store.single.downloads.is_empty() {
                        store.single.downloads.len() - 1
                    } else {
                        0
                    }) * 3,
                ),
            },
        };

        self.batch = DownloadManager {
            downloads: store.batch.downloads.clone(),
            state: DownloadManagerUIState {
                filtered_downloads: store.batch.downloads.clone(),
                table_state: TableState::default(),
                scroll_state: ScrollbarState::new(
                    (if !store.batch.downloads.is_empty() {
                        store.batch.downloads.len() - 1
                    } else {
                        0
                    }) * 3,
                ),
            },
        };

        self.playlist = DownloadManager {
            downloads: store.playlist.downloads.clone(),
            state: DownloadManagerUIState {
                filtered_downloads: store.playlist.downloads.clone(),
                table_state: TableState::default(),
                scroll_state: ScrollbarState::new(
                    (if !store.playlist.downloads.is_empty() {
                        store.playlist.downloads.len() - 1
                    } else {
                        0
                    }) * 3,
                ),
            },
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
