use std::{env, fs, io, process, iter::FromIterator, thread::sleep, time::Duration};
use tui::{backend::TermionBackend, terminal::Terminal};
use termion::{clear::All, cursor::Goto};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::widgets::{Table, Borders, Block, Row, Paragraph, Text};
use tui::layout::{Rect, Constraint, Alignment};
use tui::style::{Style, Color, Modifier};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Split {
    name: String,
    time: u32,
}

//i really really did not want to end up typing this multiple times
type Output<'a> = &'a mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>;

//these are the colors that the timer will use for ahead/behind/gold/other stuff
static GOOD: Color = Color::LightGreen;
static STANDARD: Color = Color::White;
static BAD: Color = Color::LightRed;
static GOLD: Color = Color::LightYellow;

//makes sure that an argument was actually provided
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a splits file");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

//draws the timer window to the terminal, including area with splits, title, and soon-to-be current time
fn draw_timer(terminal: Output, rows: Vec<tui::widgets::Row<core::slice::Iter<&str>>>) -> Result<(), io::Error> {
    terminal.draw(|mut t| {
        let area = Rect::new(0, 0, 35, 18);
        let time_table = Table::new(
            ["Split", "Time"].iter(),
            rows.into_iter()
        )
        .block(Block::default().title("RSplit").borders(Borders::LEFT | Borders::TOP).title_style(Style::default().modifier(Modifier::BOLD)))
        .header_style(Style::default().fg(STANDARD).modifier(Modifier::UNDERLINED))
        .widths(&[Constraint::Length(20), Constraint::Length(11)])
        .style(Style::default().fg(STANDARD))
        .column_spacing(2);
        t.render_widget(time_table, area);
        let time_area = Rect::new(0, 19, 35, 2);
        let text = [Text::raw("words")];
        let time = Paragraph::new(text.iter())
            .alignment(Alignment::Right);
        t.render_widget(time, time_area)
    })
}

fn splits_to_print<'a>(split_vec: &'a Vec<tui::widgets::Row<core::slice::Iter<'a, &str>>>, line: usize) -> std::vec::Vec<tui::widgets::Row<std::slice::Iter<'a, &'a str>>> {
    let end = line + 3;
    //i have no idea why line works but it does. thank you rust forum user nemo157.
    let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
    print_vec
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let second = Duration::new(1, 0);
    let mut current_line = 0;
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let splits_json = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let spl: Vec<Split> = serde_json::from_str(&splits_json)?;
    print!("{}{}", All, Goto(1, 1));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let array = [&spl[0].name, "time1"];
    let stuff = vec![Row::StyledData(array.iter(), Style::default().fg(GOOD)),
        Row::StyledData(["2", "Time2"].iter(), Style::default().fg(BAD)),
        Row::StyledData(["3", "Time3"].iter(), Style::default().fg(GOOD)),
        Row::StyledData(["4", "Time4"].iter(), Style::default().fg(GOLD)),
        Row::StyledData(["5", "Time5"].iter(), Style::default().fg(BAD)),
        Row::StyledData(["6", "Time6"].iter(), Style::default().fg(GOOD)),
        Row::StyledData(["7", "Time7"].iter(), Style::default().fg(GOLD))];
    loop {
        {
            let table_rows = splits_to_print(&stuff, current_line);
            sleep(second);
            current_line += 1;
            draw_timer(&mut terminal, table_rows);
        }
        if current_line == 5 {
            break;
        }
    }
    print!("{}", Goto(1, 21));
    Ok(())
}
