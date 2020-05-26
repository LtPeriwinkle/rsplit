use std::io
use tui::Terminal
use tui::backend::CrosstermBackend
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend);
}
