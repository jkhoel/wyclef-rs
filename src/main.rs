use ratatui::{backend::CrosstermBackend, style, Terminal};
use std::env;
use std::io::{stdout, Stdout};

use anyhow::Context;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders};

#[derive(Debug, PartialEq)]
struct ProgramArguments {
    file_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = &env::args().collect::<Vec<String>>();
    let program_args = get_program_args(args).context("unable to get program arguments")?;

    run_terminal_with_args(&program_args)
}

// TODO: Put terminal functions in own module
fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("failed to enable raw mode")?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("could not create terminal")
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn run_terminal_with_args(program_args: &ProgramArguments) -> anyhow::Result<()> {
    let mut terminal = setup_terminal().context("unable to set up terminal")?;

    let main_title = format!("Reading File: {}", &program_args.file_path);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(main_title.as_str())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(style::Color::Cyan));

            f.render_widget(block, size);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    restore_terminal(terminal)?;
                    break;
                }
                KeyCode::Char('f') => {
                    restore_terminal(terminal)?;
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

// TODO: End of terminal commands

fn get_program_args(args: &[String]) -> anyhow::Result<ProgramArguments> {
    let file_path = (&args.len() > &1).then(|| &args[1]).context("Please provide a path to a log file!")?;

    Ok(ProgramArguments {
        file_path: file_path.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::get_program_args;

    #[test]
    fn throws_error_when_file_name_args_is_missing() {
        let args = vec!["not-used-for-this-test".to_string()];
        let result = get_program_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn can_resolve_file_path_from_args() {
        let args = vec![
            "not-used-for-this-test".to_string(),
            "test.jlog".to_string(),
        ];

        let result = get_program_args(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().file_path, "test.jlog");
    }
}
