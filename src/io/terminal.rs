//! The TUI's IO shell: owns the terminal, turns crossterm events into `Message`s, runs the
//! draw/update loop and executes `Command`s. Nothing else imports crossterm.

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tokio::sync::mpsc;

use crate::app::{App, Command, Key, Message};
use crate::error::AppError;

const TICK: Duration = Duration::from_millis(250);
/// Messages buffered between producers and the update loop.
const CHANNEL_CAPACITY: usize = 64;

pub fn run() -> Result<(), AppError> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        spawn_input(tx);
        // `ratatui::init` enters raw mode and the alternate screen and installs a panic hook
        // that restores both. The loop returns instead of exiting, so `restore` always runs.
        let mut terminal = ratatui::init();
        let result = event_loop(&mut terminal, rx).await;
        ratatui::restore();
        result
    })
}

async fn event_loop(
    terminal: &mut ratatui::DefaultTerminal,
    mut rx: mpsc::Receiver<Message>,
) -> Result<(), AppError> {
    let mut app = App::default();
    loop {
        terminal.draw(|frame| crate::ui::draw(&app, frame))?;
        let Some(message) = rx.recv().await else {
            return Ok(());
        };
        // One `Command` variant today, so clippy sees a loop that never loops; real apps add more.
        #[allow(clippy::never_loop)]
        for command in app.update(message) {
            match command {
                Command::Quit => return Ok(()),
            }
        }
    }
}

/// Reads keys on a plain thread (crossterm blocks) and sends a `Tick` when none arrive.
fn spawn_input(tx: mpsc::Sender<Message>) {
    std::thread::spawn(move || {
        loop {
            let message = match read_key() {
                Ok(Some(message)) => message,
                Ok(None) => Message::Tick,
                Err(error) => {
                    tracing::warn!(%error, "terminal input stopped");
                    return;
                }
            };
            if tx.blocking_send(message).is_err() {
                return;
            }
        }
    });
}

fn read_key() -> std::io::Result<Option<Message>> {
    if !event::poll(TICK)? {
        return Ok(None);
    }
    let Event::Key(key) = event::read()? else {
        return Ok(None);
    };
    // Windows reports key releases too; acting on both would double every key press.
    if key.kind != KeyEventKind::Press {
        return Ok(None);
    }
    Ok(match key.code {
        KeyCode::Char(c) => Some(Message::Key(Key::Char(c))),
        KeyCode::Up => Some(Message::Key(Key::Up)),
        KeyCode::Down => Some(Message::Key(Key::Down)),
        KeyCode::Esc => Some(Message::Key(Key::Esc)),
        _ => None,
    })
}
