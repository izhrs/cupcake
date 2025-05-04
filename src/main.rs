mod model;
mod update;
mod view;

use color_eyre::Result;
use ratatui::DefaultTerminal;

use crate::{model::state::AppState, update::message::Msg, view::draw};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    state: AppState,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.state.menu_state.open(vec!["all"]);

        while self.state.running {
            terminal.draw(|frame| draw(&mut self.state, frame))?;

            let event = crossterm::event::read()?;

            if let Some(msg) = Msg::from_event(event, &self.state.focused_block) {
                update::update(&mut self.state, msg);
            }
        }
        Ok(())
    }
}
