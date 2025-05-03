pub(crate) mod message;

use crate::model::state::{AppState, FocusedBlock};
use crate::update::message::{ContentMsg, MenuMsg, ModalMsg, Msg};

pub fn update(model: &mut AppState, msg: Msg) {
    match msg {
        Msg::Quit => model.running = false,

        Msg::Content(msg) => match msg {
            ContentMsg::FocusMenu => model.focused_block = FocusedBlock::Menu,
            ContentMsg::SwitchNextTab => model.selected_tab = model.selected_tab.next(),
            ContentMsg::SwitchPreviousTab => model.selected_tab = model.selected_tab.previous(),
            ContentMsg::SelectNextRow => model.next_row(),
            ContentMsg::SelectPreviousRow => model.previous_row(),
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
            MenuMsg::ApplyMenuFilter => {
                model
                    .task_state
                    .apply_menu_filter(model.menu_state.selected().to_owned());
            }
        },

        Msg::Modal(msg) => match msg {
            ModalMsg::AddTask => model.focused_block = FocusedBlock::Modal,
            ModalMsg::ConfirmDeleteTask => {
                todo!()
            }
            ModalMsg::Close => model.focused_block = FocusedBlock::Content,
        },
    }
}
