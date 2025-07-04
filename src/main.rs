mod model;
mod update;
mod view;

use std::sync::{Arc, atomic::Ordering};

use color_eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{model::state::Model, update::message::Message, view::draw};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    let app_result = App::new(tx).run(terminal, rx).await;
    ratatui::restore();
    app_result
}

struct App {
    state: Model,
}

impl App {
    fn new(message_tx: UnboundedSender<Message>) -> Self {
        Self {
            state: Model::new(message_tx),
        }
    }

    async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        mut message_rx: UnboundedReceiver<Message>,
    ) -> Result<()> {
        self.handle_events();

        self.state.menu_state.open(vec!["all"]);

        let panel = Arc::clone(&self.state.active_panel);
        let tab = Arc::clone(&self.state.active_tab);

        while self.state.running.load(Ordering::Relaxed) {
            let active_panel = {
                let lock = panel.read().await;
                *lock
            };

            let active_tab = {
                let lock = tab.read().await;
                *lock
            };

            terminal.draw(|frame| draw(&mut self.state, frame, &active_panel, &active_tab))?;

            if let Some(message) = message_rx.recv().await {
                update::update(&mut self.state, message).await;
            }
        }
        Ok(())
    }

    /// Handles terminal events in a separate thread.
    fn handle_events(&self) {
        let event_tx = self.state.message_tx.clone().unwrap();
        let panel = Arc::clone(&self.state.active_panel);
        let tab = Arc::clone(&self.state.active_tab);
        let running = Arc::clone(&self.state.running);

        tokio::spawn(async move {
            while running.load(Ordering::Relaxed) {
                let event = crossterm::event::read().unwrap();

                let active_panel = {
                    let lock = panel.read().await;
                    *lock
                };

                let active_tab = {
                    let lock = tab.read().await;
                    *lock
                };

                if let Some(message) = Message::from_event(event, &active_panel, &active_tab) {
                    event_tx.send(message.clone()).unwrap();
                    if let Message::Quit = message {
                        break;
                    }
                }
            }
        });
    }
}
