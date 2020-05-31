use std::{env, fs, io, process};
use tui::{backend::TermionBackend, terminal::Terminal};
use termion::{clear::All, cursor::Goto};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::widgets::{Table, Borders, Block, Row, List, Text};
use tui::layout::{Rect, Constraint};
use tui::style::{Style, Color, Modifier};

static good: tui::style::Color = Color::Green;
static standard: tui::style::Color = Color::White;

fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a splits file");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

fn draw_timer(mut terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>) -> Result<(), io::Error> {
    terminal.draw(|mut t| {
        let area = Rect::new(0, 0, 35, 18);
        let timer = Table::new(
            ["Split", "Time"].iter(),
            vec![
                Row::StyledData(["12345678901234567890", "Time1"].iter(), Style::default().fg(good)),
                Row::Data(["Split 2", "Time2"].iter()),
                Row::Data(["Split 3", "Time3"].iter()),
                Row::Data(["Split 4", "12345678912"].iter())
            ].into_iter()
        )
        .block(Block::default().title("RSplit").borders(Borders::LEFT | Borders::TOP).title_style(Style::default().modifier(Modifier::BOLD)))
        .header_style(Style::default().fg(Color::White).modifier(Modifier::UNDERLINED))
        .widths(&[Constraint::Length(20), Constraint::Length(11)])
        .style(Style::default().fg(standard))
        .column_spacing(2);
        t.render_widget(timer, area)
    })
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let splits_json = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    print!("{}{}", All, Goto(1, 1));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    draw_timer(terminal)
}
