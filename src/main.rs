#![allow(array_into_iter)]
use std::{env, fs, io};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::{terminal_size, clear::All, cursor::Goto};
use tui::widgets::{Widget, Table, Borders, Block, Row};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Color};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args[1]);
    print!("{}{}", All, Goto(1, 1));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    //let splits = fs::read_to_string(&args[1]);
    terminal.draw(|mut f| {
        let area = Rect::new(0, 0, 75, 25);
        let test = Table::new(
            ["Split", "Delta", "Time"].iter(),
            vec![
                Row::Data(["Split 1", "Diff1", "Time1"].iter()),
                Row::Data(["Split 2", "Diff2", "Time2"].iter()),
                Row::Data(["Split 3", "Diff3", "Time3"].iter()),
                Row::Data(["Split 4", "Diff4", "Time4"].iter())
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
