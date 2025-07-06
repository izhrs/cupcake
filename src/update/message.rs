use crate::model::{
    downloader::DownloadTask,
    state::{ActivePanel, ActiveTab},
};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone)]
pub enum Message {
    Quit,

    // Content actions
    FocusMenu,
    SwitchNextTab,
    SwitchPreviousTab,
    SelectNextRowSingle,
    SelectNextRowBatch,
    SelectNextRowPlaylist,
    SelectPreviousRowSingle,
    SelectPreviousRowBatch,
    SelectPreviousRowPlaylist,
    ProgressUp,
    ProgressDown,
    UpdateProgressSingle,

    // Menu actions
    FocusContent,
    ToggleSelected,
    CollapseMenuItem,
    ExpandMenuItem,
    SelectPrevMenuItem,
    SelectNextMenuItem,
    SelectFirstMenuItem,
    SelectLastMenuItem,
    ApplyCategoryFilterSingle,
    ApplyCategoryFilterBatch,
    ApplyCategoryFilterPlaylist,

    // Modal actions
    OpenAddTaskModal,
    ToggleFocusedInput,
    HandleInputEvent(Event),
    AddTaskSingle,
    CloseModal,

    UpdateDownloadStatus(DownloadTask),
}

impl Message {
    pub fn from_event(
        event: Event,
        active_panel: &ActivePanel,
        active_tab: &ActiveTab,
    ) -> Option<Self> {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return None;
            }

            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(Message::Quit);
                }
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(Message::OpenAddTaskModal);
                }
                _ => {}
            }

            match active_panel {
                ActivePanel::Content => match key.code {
                    KeyCode::Char('a') => Some(Message::OpenAddTaskModal),
                    KeyCode::Char('q') => Some(Message::Quit),
                    KeyCode::Left | KeyCode::Char('h')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        Some(Message::FocusMenu)
                    }
                    KeyCode::Tab | KeyCode::Char('L') => Some(Message::SwitchNextTab),
                    KeyCode::BackTab | KeyCode::Char('H') => Some(Message::SwitchPreviousTab),
                    KeyCode::Up | KeyCode::Char('k') => match active_tab {
                        ActiveTab::Single => Some(Message::SelectPreviousRowSingle),
                        ActiveTab::Batch => Some(Message::SelectPreviousRowBatch),
                        ActiveTab::Playlist => Some(Message::SelectPreviousRowPlaylist),
                        _ => None,
                    },
                    KeyCode::Down | KeyCode::Char('j') => match active_tab {
                        ActiveTab::Single => Some(Message::SelectNextRowSingle),
                        ActiveTab::Batch => Some(Message::SelectNextRowBatch),
                        ActiveTab::Playlist => Some(Message::SelectNextRowPlaylist),
                        _ => None,
                    },
                    KeyCode::Right => Some(Message::ProgressUp),
                    KeyCode::Left => Some(Message::ProgressDown),
                    _ => None,
                },

                ActivePanel::Menu => match key.code {
                    KeyCode::Char('a') => Some(Message::OpenAddTaskModal),
                    KeyCode::Char('q') => Some(Message::Quit),
                    KeyCode::Right | KeyCode::Char('l')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        Some(Message::FocusContent)
                    }
                    KeyCode::Enter => match active_tab {
                        ActiveTab::Single => Some(Message::ApplyCategoryFilterSingle),
                        ActiveTab::Batch => Some(Message::ApplyCategoryFilterBatch),
                        ActiveTab::Playlist => Some(Message::ApplyCategoryFilterPlaylist),
                        _ => None,
                    },
                    KeyCode::Char(' ') => Some(Message::ToggleSelected),
                    KeyCode::Left | KeyCode::Char('h') => Some(Message::CollapseMenuItem),
                    KeyCode::Right | KeyCode::Char('l') => Some(Message::ExpandMenuItem),
                    KeyCode::Down | KeyCode::Char('j') => Some(Message::SelectNextMenuItem),
                    KeyCode::Up | KeyCode::Char('k') => Some(Message::SelectPrevMenuItem),
                    KeyCode::Home => Some(Message::SelectFirstMenuItem),
                    KeyCode::End => Some(Message::SelectLastMenuItem),
                    _ => None,
                },

                ActivePanel::Modal => match key.code {
                    KeyCode::Esc => Some(Message::CloseModal),
                    KeyCode::Enter => match active_tab {
                        ActiveTab::Single => Some(Message::AddTaskSingle),
                        _ => None,
                    },
                    KeyCode::Up | KeyCode::Down | KeyCode::Tab | KeyCode::BackTab => {
                        Some(Message::ToggleFocusedInput)
                    }
                    _ => Some(Message::HandleInputEvent(event)),
                },
            }
        } else {
            None
        }
    }
}
