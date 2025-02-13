use std::env;
use std::path::Path;
use std::process::Command;
use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod config;
mod ui;
use crate::{
    app::{App, Screen},
    config::{Config, Item},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);

    let mut terminal = Terminal::new(backend)?;

    let config = Config::new();

    let mut app = App::new(&config.title, &config.items);

    if !app.list.items.is_empty() {
        app.list.state.select_first();
    }

    let exe = env::current_exe().unwrap();
    let dir = Path::new(&exe).parent().unwrap();

    assert!(env::set_current_dir(dir).is_ok());

    let response = run(&mut terminal, &mut app);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = response {
        println!("{err:?}");
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App<Item>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.screen {
                Screen::Main => match key.code {
                    KeyCode::Up => {
                        app.list.state.select_previous();
                    }
                    KeyCode::Down => {
                        app.list.state.select_next();
                    }
                    KeyCode::Enter => {
                        if let Some(i) = app.list.state.selected() {
                            let item = &app.list.items[i];
                            let command = &item.command.to_string();

                            let _output = if cfg!(target_os = "windows") {
                                Command::new("cmd")
                                    .args(["/C", command])
                                    .output()
                                    .expect("failed to execute process")
                            } else {
                                Command::new("sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("failed to execute process")
                            };
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
            }
        }
    }
}
