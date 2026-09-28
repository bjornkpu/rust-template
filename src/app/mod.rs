//! Elm-style state. `update` is the only place state changes, and it does no IO: side effects
//! come back as `Command`s for `io::terminal` to execute.

/// A key the app reacts to. `io::terminal` converts crossterm keys into these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Esc,
}

/// Everything that can happen to the app. Drop `Copy` once a variant carries owned data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Key(Key),
    Tick,
}

/// Side effects `io::terminal` executes. `Quit` instead of `exit`.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Quit,
}

#[derive(Debug, Default)]
pub struct App {
    pub count: u32,
    pub ticks: u64,
}

impl App {
    pub fn update(&mut self, message: Message) -> Vec<Command> {
        match message {
            Message::Key(Key::Char('q') | Key::Esc) => return vec![Command::Quit],
            Message::Key(Key::Up) => self.count = self.count.saturating_add(1),
            Message::Key(Key::Down) => self.count = self.count.saturating_sub(1),
            Message::Key(Key::Char(_)) => {}
            Message::Tick => self.ticks = self.ticks.saturating_add(1),
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(messages: Vec<Message>) -> (App, Vec<Command>) {
        let mut app = App::default();
        let commands = messages.into_iter().flat_map(|m| app.update(m)).collect();
        (app, commands)
    }

    #[test]
    fn up_and_down_move_the_count_and_never_underflow() {
        let (app, commands) = run(vec![
            Message::Key(Key::Up),
            Message::Key(Key::Up),
            Message::Key(Key::Down),
            Message::Key(Key::Down),
            Message::Key(Key::Down),
        ]);
        assert_eq!(app.count, 0);
        assert!(commands.is_empty());
    }

    #[test]
    fn q_and_esc_quit() {
        assert_eq!(
            run(vec![Message::Key(Key::Char('q'))]).1,
            vec![Command::Quit]
        );
        assert_eq!(run(vec![Message::Key(Key::Esc)]).1, vec![Command::Quit]);
    }

    #[test]
    fn ticks_are_counted() {
        let (app, _) = run(vec![Message::Tick, Message::Tick]);
        assert_eq!(app.ticks, 2);
    }
}
