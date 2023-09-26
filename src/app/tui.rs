use anyhow::Context;
use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

use crate::app::App;
use crate::models::clef::{ClefEvent, Level};
use ratatui::{prelude::*, widgets::*};

pub fn start_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal
            .draw(|frame| render_tui(frame, &mut app))
            .context("unable to draw terminal user interface")?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Enter => app.items.unselect(),
                        KeyCode::Down => match key.modifiers {
                            KeyModifiers::SHIFT => app.items.next(10),
                            _ => app.items.next(1),
                        },
                        KeyCode::Up => match key.modifiers {
                            KeyModifiers::SHIFT => app.items.previous(10),
                            _ => app.items.previous(1),
                        },
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            //app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn render_tui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    // Prepare data
    let events = generate_events(app);

    let state = &mut app.items.state;
    let args = &app.args.file_path;

    // Create the main block and title
    let main_title = format!("Reading File: {}", args);

    let size = frame.size();
    let block = Block::default()
        .title(main_title.as_str())
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(style::Color::Cyan));

    // Render list of items to show in console into the main block
    let console_items = List::new(events)
        .block(block)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD));
        //.highlight_symbol("> ");

    frame.render_stateful_widget(console_items, size, state);

    // let greeting = Paragraph::new("Hello! (press 'q' to quit)").alignment(ratatui::layout::Alignment::Center);
    // frame.render_widget(greeting, frame.size());
}

fn generate_events<'a>(app: &App<'a>) -> Vec<ListItem<'a>> {
    app.items
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.as_message())
                .style(Style::default().fg(map_level_to_color(i)))
        }) // .modifier(Modifier::BOLD)
        .collect()
}

fn map_level_to_color(event: &ClefEvent) -> Color {
    match event.level {
        Level::Verbose => Color::White,
        Level::Debug => Color::Blue,
        Level::Information => Color::Green,
        Level::Warning => Color::Yellow,
        Level::Error => Color::Red,
        Level::Fatal => Color::Magenta,
    }
}

///////////
//
//
// pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
//     disable_raw_mode().context("failed to disable raw mode")?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )
//     .context("unable to switch to main screen")?;
//     terminal.show_cursor().context("unable to show cursor")
// }
//
// pub fn start_application(
//     terminal_args: &AppArguments,
//     log_events: &CompactLogEventsFormatFile,
// ) -> anyhow::Result<()> {
//     let mut terminal = crate::app::create_terminal().context("unable to set up terminal")?;
//
//     loop {
//         terminal
//             .draw(|f| render_application(f, terminal_args))
//             .context("unable to render application")?;
//
//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Char('q') => {
//                     restore_terminal(terminal)?;
//                     break;
//                 }
//                 KeyCode::Char('f') => {
//                     restore_terminal(terminal)?;
//                     break;
//                 }
//                 _ => {}
//             }
//         }
//     }
//
//     Ok(())
// }
//
// fn render_application(
//     frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>,
//     terminal_args: &AppArguments,
// ) {
//     let main_title = format!("Reading File: {}", &terminal_args.file_path);
//
//     let size = frame.size();
//     let block = Block::default()
//         .title(main_title.as_str())
//         .title_alignment(ratatui::layout::Alignment::Center)
//         .borders(Borders::ALL)
//         .border_style(Style::default().fg(style::Color::Cyan));
//
//     frame.render_widget(block, size);
//
//     // let greeting = Paragraph::new("Hello! (press 'q' to quit)").alignment(ratatui::layout::Alignment::Center);
//     // frame.render_widget(greeting, frame.size());
// }
