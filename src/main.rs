use std::{env, fs, process, iter::FromIterator, thread::sleep, time::Duration};
use std::io::{Write, stdout, Error};
use crossterm::Result as cross_result;
use crossterm::{QueueableCommand, ExecutableCommand, cursor::MoveTo};
use crossterm::style::{Print, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType::All};
use serde_json;
//file with structs, static vars
mod components;
use components::*;

//makes sure that an argument was actually provided
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a file to read splits from");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

fn splits_to_print<'a>(split_vec: &'a Vec<&str>, line: usize) -> Vec<&'a str> {
    let end = if split_vec.len() > 18 {18} else {split_vec.len()};
    //i have no idea why line works but it does. thank you rust forum user nemo157.
    let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
    print_vec
}

//new soon-to-be print a timer function
fn print_split_names(to_print: Vec<&str>, index: usize, out: &mut std::io::Stdout) -> cross_result<()> {
    let mut counter: u16 = 0;
    loop {
        if counter == to_print.len() as u16 {
            break;
        }
        queue_table_row(&to_print[index], "time", out, counter)?;
        counter += 1;
    }
    Ok(())
}

fn queue_table_row(split_name: &str, time: &str, out: &mut std::io::Stdout, row: u16) -> cross_result<()> {
    out.queue(MoveTo(1, row))?
        .queue(SetForegroundColor(GOOD))?
        .queue(Print(split_name))?
        .queue(MoveTo(20, row))?
        .queue(Print(time))?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut out = stdout();
    out.execute(Clear(All)).unwrap();
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
    let mut rows = Vec::new();
    for i in json_as_splits {
        let split = i.name;
        rows.push(split);
    }
    'main: loop {
        //introduce a new scope to print new rows each iteration
        {
            let table_rows = splits_to_print(&rows, current_line);
            sleep(second);
            if current_line == table_rows.len() {
                break 'main;
            }
            print_split_names(table_rows, current_line, &mut out).unwrap();
            current_line += 1;
            out.flush()?;
        }
    }
    out.execute(SetForegroundColor(RESET)).unwrap();
    Ok(())
}
