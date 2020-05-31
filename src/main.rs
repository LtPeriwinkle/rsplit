use std::{env, fs, io, process, iter::FromIterator};
use tui::{backend::TermionBackend, terminal::Terminal};
use termion::{clear::All, cursor::Goto};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::widgets::{Table, Borders, Block, Row};
use tui::layout::{Rect, Constraint};
use tui::style::{Style, Color, Modifier};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Split {
    name: String,
    time: u32,
}
//i really really did not want to end up typing this multiple times
type Output = Terminal<TermionBackend<RawTerminal<io::Stdout>>>;

//these are the colors that the timer will use for ahead/behind/normal or something
static GOOD: Color = Color::Green;
static STANDARD: Color = Color::White;
static BAD: Color = Color::Red;

//makes sure that a file was actually provided
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a splits file");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

//draws the timer window to the terminal
fn draw_timer(mut terminal: Output, rows: Vec<tui::widgets::Row<core::slice::Iter<&str>>>) -> Result<(), io::Error> {
    terminal.draw(|mut t| {
        let area = Rect::new(0, 0, 35, 18);
        let timer = Table::new(
            ["Split", "Time"].iter(),
            rows.into_iter()
        )
        .block(Block::default().title("RSplit").borders(Borders::LEFT | Borders::TOP).title_style(Style::default().modifier(Modifier::BOLD)))
        .header_style(Style::default().fg(STANDARD).modifier(Modifier::UNDERLINED))
        .widths(&[Constraint::Length(20), Constraint::Length(11)])
        .style(Style::default().fg(STANDARD))
        .column_spacing(2);
        t.render_widget(timer, area)
    })
}

fn splits_to_print<'a>(split_vec: &'a Vec<tui::widgets::Row<core::slice::Iter<'a, &str>>>, line: usize) -> std::vec::Vec<tui::widgets::Row<std::slice::Iter<'a, &'a str>>> {
    let end = line + 2;
    //i have no idea why line works but it does. thank you rust forum user nemo157.
    let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
    print_vec
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let current_line = 2;
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let splits_json = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let splits_from_json: Vec<Split> = serde_json::from_str(&splits_json)?;
    print!("{}{}", All, Goto(1, 1));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let stuff = vec![Row::StyledData(["12345678901234567890", "Time1"].iter(), Style::default().fg(GOOD)),
        Row::StyledData(["12345678901234567890", "Time2"].iter(), Style::default().fg(BAD)),
        Row::StyledData(["12345678901234567890", "Time3"].iter(), Style::default().fg(GOOD)),
        Row::StyledData(["12345678901234567890", "Time4"].iter(), Style::default().fg(BAD))];
    let table_rows = splits_to_print(&stuff, current_line);
    draw_timer(terminal, table_rows);
    println!("{:?}", splits_from_json[1]);
    Ok(())
}
