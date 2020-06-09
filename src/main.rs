use std::{env, fs, process, iter::FromIterator, thread::sleep, time::Duration};
use std::io::{Write, stdout, Error};
use crossterm::Result as cross_result;
use crossterm::{QueueableCommand, execute, cursor::MoveTo};
use crossterm::style::{Print, Color, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType::All};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Split {
    name: String,
    time: u32,
}

//these are the colors that the timer will use for ahead/behind/gold/other stuff
static GOOD: Color = Color::Green;
static STANDARD: Color = Color::White;
static BAD: Color = Color::Red;
static GOLD: Color = Color::Yellow;
static RESET: Color = Color::Reset;

//makes sure that an argument was actually provided
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a file to read splits from");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

fn queue_table_row(row: u16, to_print: &String) -> cross_result<()> {
    let mut stdout = stdout();
    stdout.queue(MoveTo(1, row))?
        .queue(SetForegroundColor(GOOD))?
        .queue(Print(to_print))?
        .queue(MoveTo(20, row))?;
    Ok(())
}

//new soon-to-be print a timer function
fn print_to_terminal(stdout: &mut std::io::Stdout, to_print: Vec<String>) -> cross_result<()> {
    stdout.queue(Clear(All))?;
    queue_table_row(1, &to_print[0])?;
    stdout.flush()?;
    Ok(())
}

fn splits_to_print<'a>(split_vec: &'a Vec<String>, line: usize) -> Vec<String> {
    let end = line + 1;
    //i have no idea why line works but it does. thank you rust forum user nemo157.
    let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
    print_vec
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut out = stdout();
    let second = Duration::new(1, 0);
    let mut current_line = 0;
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let json_raw = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(2);
    });
    let json_as_splits: Vec<Split> = serde_json::from_str(&json_raw)?;
    /*(print_to_terminal(out).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(3);
    });*/
    let mut rows = Vec::new();
    for i in json_as_splits {
        let split = i.name;
        rows.push(split);
    }
    loop {
        {
            let table_rows = splits_to_print(&rows, current_line);
            sleep(second);
            current_line += 1;
            print_to_terminal(&mut out, table_rows).unwrap();
        }
        if current_line == 1 {
            break;
        }
    }
    execute!(stdout(), SetForegroundColor(RESET)).unwrap();
    Ok(())
}
