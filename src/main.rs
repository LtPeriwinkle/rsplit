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
    //since for some reason the env args output gives an array with the command also in it, grab filename out of the array
    let splits = &args[1];
    Ok(splits.to_string())
}

fn splits_to_print<'a>(split_vec: &'a Vec<&str>, line: usize) -> Vec<&'a str> {
    //makes sure that out-of-bounds doesnt happen
    if split_vec.len() < 18 {
        //i want a better way to do this but im lazy and this was fast to write
        return split_vec.to_vec();
    } else {
        //my brain hurts
        let end = if split_vec.len() < line + 18 {split_vec.len() - 1} else {line + 18};
        //i have no idea why line works but it does. thank you rust forum user nemo157.
        let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
        print_vec
    }

}

/* why did i write this it is completely useless (i think) lol
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
*/

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
    let mut current_line = 0;

    //make sure we arent printing over other stuff
    out.execute(Clear(All)).unwrap();

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

    //gave the loop a lifetime while i remembered how to do that in case i need to put another loop inside of it
    'main: loop {
        //introduce a new scope to print new rows each iteration
        {
            let table_rows = splits_to_print(&rows, current_line);
            if current_line == table_rows.len() {
                break 'main;
            }
            queue_table_row(table_rows[current_line], "time", &mut out, current_line as u16).unwrap();
            current_line += 1;

            //makes crossterm do all the stuff we queued in the queue_table_row()
            out.flush()?;
        }
    }
    //makes it so that anything you do in the terminal after use this isnt weirdly colored
    out.execute(SetForegroundColor(RESET)).unwrap();
    Ok(())
}
