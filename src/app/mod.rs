use anyhow::Context;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Stdout};

use ratatui::prelude::*;

use crate::app::state::StatefulList;
use crate::models::clef::ClefEvent;
use crate::AppArguments;

pub mod state;
pub mod tui;

#[derive(Debug)]
pub struct App<'a> {
    pub args: AppArguments,
    pub items: StatefulList<&'a ClefEvent>,
    //events: Vec<(&'a str, &'a str)>,
}

impl<'a> App<'a> {
    pub fn new(args: AppArguments, items: Vec<&'a ClefEvent>) -> App<'a> {
        App {
            args,
            items: StatefulList::with_items(items),
        }
    }
}

pub fn create_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("failed to enable raw mode")?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("could not create app terminal")
}

pub fn destroy_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}
