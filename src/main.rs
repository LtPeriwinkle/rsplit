use std::{env, fs, process, iter::FromIterator};
use std::io::{Write, stdout, Error};
use crossterm::Result as cross_result;
use crossterm::{QueueableCommand, ExecutableCommand, cursor::MoveTo};
use crossterm::style::{Print, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType::All};
use serde_json;

//file with structs, static vars; i didnt want to have them cluttering up this file
mod components;
use components::*;

//makes sure that an argument was actually provided and returns an error that is used to stop program later if no argument
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a file to read splits from");
    }
    //since for some reason args().collect() gives an array with the command also in it, grab filename out of the array
    let splits = &args[1];
    Ok(splits.to_string())
}

//rewrote this function to get rid of out of bounds attempts
fn splits_to_print<'a>(split_vec: &'a Vec<&str>, line: usize) -> Vec<&'a str> {
    if split_vec.len() < 18 {
        //i want a better way to do this but im lazy and this was fast to write
        return split_vec.to_vec();
    } else {
        //my brain hurts
        let end = if split_vec.len() < line + 18 {split_vec.len() - 1} else {line + 18};
        //i have no idea why line works but it does, thank you rust forum
        let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
        print_vec
    }
}

//prints everything that needs to be shown, by queueing timer rows then flushing them at the end
fn print_timer(out: &mut std::io::Stdout, rows: Vec<&str>) -> cross_result<()> {
    let mut current_line = 0;
    loop {
            //introduce a new scope to print new rows each iteration
            {
                let table_rows = splits_to_print(&rows, current_line);
                if current_line == table_rows.len() {
                    break;
                }
                queue_table_row(table_rows[current_line], "time", out, current_line as u16).unwrap();
                current_line += 1;

            }
        }
        //makes crossterm do all the stuff we queued in queue_table_row()
        out.flush()?;
    Ok(())
}

//takes the name of the split, the time that should be displayed for it, and the row in terminal where it should be printed
//and queues a row with name + time into the buffer
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
    let mut rows = Vec::new();

    //make sure we arent printing over other stuff
    out.execute(Clear(All)).unwrap();

    //make the json file into a vec of Splits (from components.rs) and the split names into another vec
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let json_raw = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(2);
    });
    let json_as_splits: Vec<Split> = serde_json::from_str(&json_raw)?;
    for i in json_as_splits {
        let split = i.name;
        rows.push(split);
    }

    //gave the loop a name while i remembered how to do that in case i need to put another loop inside of it
    //'main: loop {
    //}
    print_timer(&mut out, rows).unwrap_or_else(|err| {eprintln!("{}", err); process::exit(3)});
    //makes it so that anything you do in the terminal after use this isnt weirdly colored, and resets the cursor position
    out.execute(SetForegroundColor(RESET)).unwrap();
    out.execute(MoveTo(1, 19)).unwrap();
    Ok(())
}
