#![allow(array_into_iter)]
use std::{env, fs, io, process};
use tui::backend::TermionBackend;
use tui::terminal::Terminal;
use termion::{clear::All, cursor::Goto};
use termion::raw::IntoRawMode;
use tui::widgets::{Table, Borders, Block, Row, List, Text};
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};

static standard: tui::style::Color = Color::White;

fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a splits file");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

fn draw_split_names(mut terminal: tui::terminal::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>) -> Result<(), io::Error> {
    terminal.draw(|mut s| {
        let area = Rect::new(0, 0, 20, 18);
        let splits = ["asdf", "qwerty", "ghjkl"].iter().map(|i| Text::raw(*i));
        let names = List::new(splits)
            .block(Block::default().title("Split Name").title_style(Style::default().modifier(Modifier::BOLD | Modifier::UNDERLINED))
                .borders(Borders::RIGHT | Borders::TOP))
            .style(Style::default().fg(standard));
        s.render_widget(names, area)
    })
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    print!("{}{}", All, Goto(1, 1));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    /*terminal.draw(|mut f| {
        let area = Rect::new(0, 0, 50, 25);
        let test = Table::new(
            ["Split", "Delta", "Time"].iter(),
            vec![
                Row::Data(["Split 1", "Diff1", "Time1"].iter()),
                Row::Data(["Split 2", "Diff2", "Time2"].iter()),
                Row::Data(["Split 3", "Diff3", "Time3"].iter()),
                Row::Data(["Split 4", "Diff4", "Time4"].iter())
            ].into_iter()
        )
        .block(Block::default().title("RSplit").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
        .style(Style::default().fg(Color::White))
        .column_spacing(5);
        f.render_widget(test, area)
    })*/
    draw_split_names(terminal)
}
