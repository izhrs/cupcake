use dirs;

use ratatui::{
    style::{Modifier, Style, palette::tailwind},
    text::Text,
};

use tui_input::Input;
use tui_tree_widget::{TreeItem, TreeState};

use crate::model::task::TaskStore;

#[derive(Default)]
pub struct AppState {
    pub(crate) focused_block: FocusedBlock, // focused window
    pub(crate) progress: f32,
    pub(crate) task_store: TaskStore,
    pub(crate) selected_tab: SelectedTab,
    pub(crate) menu_state: TreeState<&'static str>,
    pub(crate) menu_items: Vec<TreeItem<'static, &'static str>>,
    pub(crate) input_state: InputState,
    pub(crate) theme: Theme,
    pub(crate) running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: true,
            focused_block: FocusedBlock::default(),
            progress: 0.0,
            task_store: TaskStore::new().load().unwrap_or(TaskStore::default()),
            selected_tab: SelectedTab::default(),
            menu_state: TreeState::default(),
            theme: Theme::default(),
            input_state: InputState::default(),
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
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) enum FocusedBlock {
    #[default]
    Content,
    Menu,
    Modal,
}

#[derive(Default, Clone, Copy, Debug)]
pub(crate) enum SelectedTab {
    #[default]
    Single,
    Batch,
    Playlist,
    Settings,
    About,
}

impl std::fmt::Display for SelectedTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectedTab::Single => write!(f, "Single"),
            SelectedTab::Batch => write!(f, "Batch"),
            SelectedTab::Playlist => write!(f, "Playlist"),
            SelectedTab::Settings => write!(f, "Settings"),
            SelectedTab::About => write!(f, "About"),
        }
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn previous(self) -> Self {
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
    pub(crate) fn next(self) -> Self {
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
            .unwrap()
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

pub struct Theme {
    pub(crate) primary: tailwind::Palette,
    pub(crate) secondary: tailwind::Palette,
    pub(crate) accent: tailwind::Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: tailwind::NEUTRAL,
            secondary: tailwind::PURPLE,
            accent: tailwind::GREEN,
        }
    }
}
