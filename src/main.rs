#![allow(array_into_iter)]
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use tui::widgets::{Widget, Table, Borders, Block, Row};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::Style;
use tui::style::Color;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let row_style = Style::default().fg(Color::White);
    terminal.draw(|mut f| {
        //let area: Rect = (0, 0, rect.Result.0, rect.Result.1);
        let area = Rect::new(12, 12, 75, 25);
        let test = Table::new(
            ["Split", "Delta", "Time"].into_iter(),
            vec![
                Row::Data(["Split 1", "Row12", "Row13"].into_iter()),
                Row::Data(["Split 2", "Row22", "Row23"].into_iter()),
                Row::Data(["Split 3", "Row32", "Row33"].into_iter()),
                Row::Data(["Split 4", "Row42", "Row43"].into_iter())
            ].into_iter()
        )
        .block(Block::default().title("Table"))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
        .style(Style::default().fg(Color::White))
        .column_spacing(10);
        f.render_widget(test, area)
    })
}
