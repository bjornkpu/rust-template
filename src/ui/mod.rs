//! Pure render: reads `App`, never changes it.

use ratatui::Frame;
use ratatui::widgets::{Block, Paragraph};

use crate::app::App;

pub fn draw(app: &App, frame: &mut Frame) {
    let text = format!("count: {}\n\nup/down to change, q to quit", app.count);
    let block = Block::bordered().title(" rust-template ");
    frame.render_widget(Paragraph::new(text).block(block), frame.area());
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::*;

    #[test]
    fn home_screen() {
        let app = App { count: 3, ticks: 0 };
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| draw(&app, frame)).unwrap();
        insta::assert_snapshot!(terminal.backend());
    }
}
