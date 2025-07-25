pub mod message;

use std::sync::atomic::Ordering;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    model::state::{FocusedInput, Model},
    update::message::Message,
};

pub async fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Quit => {
            model.downloader.save().expect("failed to save tasks");
            model.running.store(false, Ordering::Relaxed);
        }

        // Content
        Message::FocusMenu => model.focus_menu().await,
        Message::SwitchNextTab => model.next_tab().await,
        Message::SwitchPreviousTab => model.previous_tab().await,
        Message::SelectNextRowSingle => model.downloader.single.next_row(),
        Message::SelectNextRowBatch => model.downloader.batch.next_row(),
        Message::SelectNextRowPlaylist => model.downloader.playlist.next_row(),
        Message::SelectPreviousRowSingle => model.downloader.single.previous_row(),
        Message::SelectPreviousRowBatch => model.downloader.batch.previous_row(),
        Message::SelectPreviousRowPlaylist => model.downloader.playlist.previous_row(),
        Message::ProgressUp => {
            if model.progress < 100.0 {
                model.progress += 1.0;
            }
        }
        Message::ProgressDown => {
            if model.progress > 0.0 {
                model.progress -= 1.0;
            }
        }

        Message::UpdateProgressSingle => {
            model.undate_progress_single();
        }

        // Menu
        Message::FocusContent => model.focus_content().await,
        Message::ToggleSelected => {
            model.menu_state.toggle_selected();
        }
        Message::CollapseMenuItem => {
            model.menu_state.key_left();
        }
        Message::ExpandMenuItem => {
            model.menu_state.key_right();
        }
        Message::SelectPrevMenuItem => {
            model.menu_state.key_up();
        }
        Message::SelectNextMenuItem => {
            model.menu_state.key_down();
        }
        Message::SelectFirstMenuItem => {
            model.menu_state.select_first();
        }
        Message::SelectLastMenuItem => {
            model.menu_state.select_last();
        }

        Message::ApplyCategoryFilterSingle => model
            .downloader
            .single
            .filter_downloads_by_category(model.menu_state.selected().to_owned()),
        Message::ApplyCategoryFilterBatch => model
            .downloader
            .batch
            .filter_downloads_by_category(model.menu_state.selected().to_owned()),
        Message::ApplyCategoryFilterPlaylist => model
            .downloader
            .playlist
            .filter_downloads_by_category(model.menu_state.selected().to_owned()),

        // Modal
        Message::ShowAddTaskModal => model.show_source_input_model().await,
        Message::ToggleFocusedInput => match model.input_state.focused {
            FocusedInput::Destination => model.input_state.focused = FocusedInput::Name,
            FocusedInput::Name => model.input_state.focused = FocusedInput::Destination,
        },
        Message::HandleSourceInputEvent(e) => {
            model.input_state.source.handle_event(&e);
        }
        Message::HandleDestinationInputEvent(e) => match model.input_state.focused {
            FocusedInput::Name => {
                model.input_state.name.handle_event(&e);
            }
            FocusedInput::Destination => {
                model.input_state.destination.handle_event(&e);
            }
        },
        Message::ExtractMetadata => {
            model.extract_metadata().await;
        }

        Message::AddTaskSingle => {
            model.add_task_single().await;
        }

        Message::UpdateDownloadStatus(task) => {
            model.downloader.single.update_download(task);
        }

        Message::CloseModal => model.close_modal().await,
    }
}
