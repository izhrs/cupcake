pub(crate) mod message;

use std::path::PathBuf;

use tui_input::backend::crossterm::EventHandler;

use crate::{
    model::{
        state::{FocusedBlock, FocusedInput, InputState, Model, SelectedTab},
        task::TaskState,
    },
    update::message::{ContentMsg, MenuMsg, ModalMsg, Msg},
};

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Quit => {
            model.task_store.save().expect("failed to save tasks");
            model.running = false;
        }

        Msg::Content(msg) => match msg {
            ContentMsg::FocusMenu => model.focused_block = FocusedBlock::Menu,
            ContentMsg::SwitchNextTab => model.selected_tab = model.selected_tab.next(),
            ContentMsg::SwitchPreviousTab => model.selected_tab = model.selected_tab.previous(),
            ContentMsg::SelectNextRow => match model.selected_tab {
                SelectedTab::Single => model.task_store.single.next_row(),
                SelectedTab::Batch => model.task_store.batch.next_row(),
                SelectedTab::Playlist => model.task_store.playlist.next_row(),
                _ => {}
            },
            ContentMsg::SelectPreviousRow => match model.selected_tab {
                SelectedTab::Single => model.task_store.single.previous_row(),
                SelectedTab::Batch => model.task_store.batch.previous_row(),
                SelectedTab::Playlist => model.task_store.playlist.previous_row(),
                _ => {}
            },
            ContentMsg::ProgressUp => {
                if model.progress < 100.0 {
                    model.progress += 1.0;
                }
            }
            ContentMsg::ProgressDown => {
                if model.progress > 0.0 {
                    model.progress -= 1.0;
                }
            }
        },

        Msg::Menu(msg) => match msg {
            MenuMsg::FocusContent => model.focused_block = FocusedBlock::Content,
            MenuMsg::ToggleSelected => {
                model.menu_state.toggle_selected();
            }
            MenuMsg::CollapseMenuItem => {
                model.menu_state.key_left();
            }
            MenuMsg::ExpandMenuItem => {
                model.menu_state.key_right();
            }
            MenuMsg::SelectPrevMenuItem => {
                model.menu_state.key_up();
            }
            MenuMsg::SelectNextMenuItem => {
                model.menu_state.key_down();
            }
            MenuMsg::SelectFirstMenuItem => {
                model.menu_state.select_first();
            }
            MenuMsg::SelectLastMenuItem => {
                model.menu_state.select_last();
            }
            MenuMsg::ApplyMenuFilter => match model.selected_tab {
                SelectedTab::Single => model
                    .task_store
                    .single
                    .apply_menu_filter(model.menu_state.selected().to_owned()),
                SelectedTab::Batch => model
                    .task_store
                    .batch
                    .apply_menu_filter(model.menu_state.selected().to_owned()),
                SelectedTab::Playlist => model
                    .task_store
                    .playlist
                    .apply_menu_filter(model.menu_state.selected().to_owned()),
                _ => {}
            },
        },

        Msg::Modal(msg) => match msg {
            ModalMsg::OpenAddTaskModal => model.focused_block = FocusedBlock::Modal,
            ModalMsg::ToggleFocusedInput => match model.input_state.focused {
                FocusedInput::Destination => model.input_state.focused = FocusedInput::Source,
                FocusedInput::Source => model.input_state.focused = FocusedInput::Destination,
            },
            ModalMsg::HandleInputEvent(e) => match model.input_state.focused {
                FocusedInput::Source => {
                    model.input_state.source.handle_event(&e);
                }
                FocusedInput::Destination => {
                    model.input_state.destination.handle_event(&e);
                }
            },
            ModalMsg::AddTask => match model.selected_tab {
                SelectedTab::Single => {
                    if let Ok(task) = TaskState::extract_data(
                        model.input_state.source.value(),
                        PathBuf::from(model.input_state.destination.value()),
                    ) {
                        model.task_store.single.add_task(task).unwrap();
                        model.focused_block = FocusedBlock::Content;
                        model.input_state = InputState::new();
                    }
                }
                _ => {}
            },
            ModalMsg::Close => {
                model.input_state = InputState::new(); // clear the contents
                model.focused_block = FocusedBlock::Content
            }
        },
    }
}
