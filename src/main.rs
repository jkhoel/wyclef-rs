use anyhow::Context;
use std::env;
use std::time::Duration;

use wyclef_rs::app::tui::start_tui;
use wyclef_rs::app::*;
use wyclef_rs::get_app_arguments_from_program_args;
use wyclef_rs::models::clef::CompactLogEventsFormatFile;

fn main() -> anyhow::Result<()> {
    let args = &env::args().collect::<Vec<String>>();
    let app_args =
        get_app_arguments_from_program_args(args).context("unable to get program arguments")?;

    let log_events =
        CompactLogEventsFormatFile::new(&app_args.file_path).context("unable to read log file")?;

    let tick_rate = Duration::from_millis(250);

    let mut terminal = create_terminal().context("unable to set up a terminal for the App")?;
    let app = App::new(app_args, log_events.events.iter().collect());

    start_tui(&mut terminal, app, tick_rate)
        .context("unable to start the terminal user interface")?;
    destroy_terminal(terminal).context("an error occurred when cleaning up the app terminal")
}
