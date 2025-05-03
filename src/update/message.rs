use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::model::state::FocusedBlock;

pub enum Msg {
    Quit,
    Content(ContentMsg),
    Menu(MenuMsg),
    Modal(ModalMsg),
}

pub enum ContentMsg {
    FocusMenu,
    SwitchNextTab,
    SwitchPreviousTab,
    SelectNextRow,
    SelectPreviousRow,
    ProgressUp,
    ProgressDown,
}

pub enum MenuMsg {
    FocusContent,
    ToggleSelected,
    CollapseMenuItem,
    ExpandMenuItem,
    SelectPrevMenuItem,
    SelectNextMenuItem,
    SelectFirstMenuItem,
    SelectLastMenuItem,
    ApplyMenuFilter,
}

pub enum ModalMsg {
    AddTask,
    ConfirmDeleteTask,
    Close,
}

impl Msg {
    pub fn from_event(event: Event, focused: FocusedBlock) -> Option<Self> {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return None;
            }

            match key.code {
                KeyCode::Esc if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(Msg::Quit);
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(Msg::Quit);
                }
                KeyCode::Char('q') => {
                    return Some(Msg::Quit);
                }
                _ => {}
            }

            match focused {
                FocusedBlock::Content => match key.code {
                    KeyCode::Left if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        Some(Msg::Content(ContentMsg::FocusMenu))
                    }
                    KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        Some(Msg::Modal(ModalMsg::AddTask))
                    }
                    KeyCode::Tab => Some(Msg::Content(ContentMsg::SwitchNextTab)),
                    KeyCode::BackTab => Some(Msg::Content(ContentMsg::SwitchPreviousTab)),
                    KeyCode::Up => Some(Msg::Content(ContentMsg::SelectPreviousRow)),
                    KeyCode::Down => Some(Msg::Content(ContentMsg::SelectNextRow)),
                    KeyCode::Right => Some(Msg::Content(ContentMsg::ProgressUp)),
                    KeyCode::Left => Some(Msg::Content(ContentMsg::ProgressDown)),
                    _ => None,
                },

                FocusedBlock::Menu => match key.code {
                    KeyCode::Right if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        Some(Msg::Menu(MenuMsg::FocusContent))
                    }
                    KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        Some(Msg::Modal(ModalMsg::AddTask))
                    }
                    KeyCode::Enter => Some(Msg::Menu(MenuMsg::ApplyMenuFilter)),
                    KeyCode::Char(' ') => Some(Msg::Menu(MenuMsg::ToggleSelected)),
                    KeyCode::Left => Some(Msg::Menu(MenuMsg::CollapseMenuItem)),
                    KeyCode::Right => Some(Msg::Menu(MenuMsg::ExpandMenuItem)),
                    KeyCode::Down => Some(Msg::Menu(MenuMsg::SelectNextMenuItem)),
                    KeyCode::Up => Some(Msg::Menu(MenuMsg::SelectPrevMenuItem)),
                    KeyCode::Home => Some(Msg::Menu(MenuMsg::SelectFirstMenuItem)),
                    KeyCode::End => Some(Msg::Menu(MenuMsg::SelectLastMenuItem)),
                    _ => None,
                },

                FocusedBlock::Modal => match key.code {
                    KeyCode::Esc => Some(Msg::Modal(ModalMsg::Close)),
                    KeyCode::Enter => Some(Msg::Modal(ModalMsg::ConfirmDeleteTask)),
                    _ => None,
                },
            }
        } else {
            None
        }
    }
}
