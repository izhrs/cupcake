use std::path::PathBuf;
use url::Url;

#[derive(Clone)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) source: Url,
    pub(crate) destination: PathBuf,
    pub(crate) speed: f32,
    pub(crate) size: f32,
    pub(crate) progress: f32,
    pub(crate) eta: String,
    pub(crate) status: TaskStatus,
}

#[derive(Clone)]
pub enum TaskStatus {
    Running,
    Paused,
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
        write!(f, "{}", status)
    }
}

pub struct TaskState {
    db: Vec<Task>,
    pub(crate) tasks: Vec<Task>,
}

impl TaskState {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            db: tasks.clone(),
            tasks,
        }
    }

    /// Filter tasks based on the selected menu item
    /// # Arguments
    /// * `selected_menu_item` - A slice of strings representing the selected menu item
    /// coming from tui_tree_widget::TreeState.selected()
    /// All the &str are Tree identifiers
    pub(crate) fn apply_menu_filter(&mut self, selected_menu_item: Vec<&str>) {
        match selected_menu_item.len() {
            1 => match selected_menu_item[0] {
                identifier => {
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
                }
            },

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
                }
            },
            _ => {}
        }
    }
}
