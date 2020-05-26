use std::io::{stdout, Write};
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() {
//    let stdout = io::stdout();
//    let backend = CrosstermBackend::new(stdout);
//    let mut terminal = Terminal::new(backend);
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    );
}
