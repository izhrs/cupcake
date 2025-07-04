use std::sync::{Arc, atomic::AtomicBool};

use ratatui::{
    style::{Modifier, Style, palette::tailwind},
    text::Text,
};

use tokio::sync::{RwLock, mpsc::UnboundedSender};
use tui_input::Input;
use tui_tree_widget::{TreeItem, TreeState};

use crate::{model::task::TaskStore, model::theme::Theme, update::message::Message};

pub struct Model {
    pub message_tx: Option<UnboundedSender<Message>>,
    pub(crate) active_panel: Arc<RwLock<ActivePanel>>, // focused window
    pub(crate) progress: f32,
    pub(crate) task_store: TaskStore,
    pub(crate) active_tab: Arc<RwLock<ActiveTab>>,
    pub(crate) menu_state: TreeState<&'static str>,
    pub(crate) menu_items: Vec<TreeItem<'static, &'static str>>,
    pub(crate) input_state: InputState,
    pub(crate) theme: Theme,
    pub(crate) running: Arc<AtomicBool>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            message_tx: None,
            running: Arc::new(AtomicBool::new(true)),
            active_panel: Arc::new(RwLock::new(ActivePanel::default())),
            active_tab: Arc::new(RwLock::new(ActiveTab::default())),
            progress: 0.0,
            task_store: TaskStore::new().load().unwrap_or(TaskStore::default()),
            menu_state: TreeState::default(),
            theme: Theme::default(),
            input_state: InputState::default(),
            menu_items: Default::default(),
        }
    }
}

impl Model {
    pub fn new(message_tx: UnboundedSender<Message>) -> Self {
        Self {
            message_tx: Some(message_tx),
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
            ..Default::default()
        }
    }

    pub async fn previous_tab(&mut self) {
        let mut tab = self.active_tab.write().await;
        *tab = tab.previous();
    }

    pub async fn next_tab(&mut self) {
        let mut tab = self.active_tab.write().await;
        *tab = tab.next();
    }

    pub async fn focus_content(&mut self) {
        let mut active_panel = self.active_panel.write().await;
        *active_panel = ActivePanel::Content;
    }

    pub async fn focus_menu(&mut self) {
        let mut active_panel = self.active_panel.write().await;
        *active_panel = ActivePanel::Menu;
    }

    pub async fn focus_modal(&mut self) {
        let mut active_panel = self.active_panel.write().await;
        *active_panel = ActivePanel::Modal;
    }

    pub async fn close_modal(&mut self) {
        self.input_state = InputState::new();
        let mut active_panel = self.active_panel.write().await;
        *active_panel = ActivePanel::Content;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) enum ActivePanel {
    #[default]
    Content,
    Menu,
    Modal,
}

#[derive(Default, Clone, Copy, Debug)]
pub(crate) enum ActiveTab {
    #[default]
    Single,
    Batch,
    Playlist,
    Settings,
    About,
}

impl std::fmt::Display for ActiveTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveTab::Single => write!(f, "Single"),
            ActiveTab::Batch => write!(f, "Batch"),
            ActiveTab::Playlist => write!(f, "Playlist"),
            ActiveTab::Settings => write!(f, "Settings"),
            ActiveTab::About => write!(f, "About"),
        }
    }
}

impl ActiveTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        match previous_index {
            0 => ActiveTab::Single,
            1 => ActiveTab::Batch,
            2 => ActiveTab::Playlist,
            3 => ActiveTab::Settings,
            4 => ActiveTab::About,
            _ => ActiveTab::Single,
        }
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub(crate) fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        match next_index {
            0 => ActiveTab::Single,
            1 => ActiveTab::Batch,
            2 => ActiveTab::Playlist,
            3 => ActiveTab::Settings,
            4 => ActiveTab::About,
            _ => ActiveTab::Single,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FocusedInput {
    #[default]
    Source,
    Destination,
}

#[derive(Debug, Clone)]
pub struct InputState {
    pub(crate) source: Input,
    pub(crate) destination: Input,
    pub(crate) focused: FocusedInput,
}

impl Default for InputState {
    fn default() -> Self {
        let download_dir = dirs::download_dir()
            .unwrap_or("".into())
            .to_str()
            .unwrap_or("")
            .to_string();

        Self {
            destination: Input::new(download_dir),
            source: Input::default(),
            focused: FocusedInput::default(),
        }
    }
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
}
