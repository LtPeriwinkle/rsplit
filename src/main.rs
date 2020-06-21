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

//make the json file into a vec of Splits (from components.rs) and the split names into another vec
fn get_splits<'a>(file: &'a String, _vec: Vec<Split>) -> (Vec<Split<'a>>, Vec<&'a str>) {
    let mut rows = Vec::new();

    let json_as_splits: Vec<Split> = serde_json::from_str(file.as_str()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(3);
    });

    for i in &json_as_splits {
        let split = i.name;
        rows.push(split);
    }

    (json_as_splits, rows)
}

//gets a chunk of the full vec of split names to print each time
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
                queue_table_row(table_rows[current_line], "time", out, current_line as u16)?;
                current_line += 1;

            }
        }
        //makes crossterm do all the stuff queued in queue_table_row() calls
        out.flush()?;
    Ok(())
}

//takes the name of the split, the time that should be displayed for it, and the row in terminal where it should be printed
//and queues a row with name + time into the crossterm buffer
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
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let mut out = stdout();

    //deal with the json stuff, yeah its ugly but it gets the job done
    let json_raw = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(2);
    });
    let useless_item: Vec<Split> = Vec::new();
    let results = get_splits(&json_raw, useless_item);
    let names = results.1;
    //will be used for time comparisons later, not useful right now
    let _split_vec = results.0;

    //make sure we arent printing over other stuff
    out.execute(Clear(All)).unwrap();

    //gave the loop a name because it will eventually have another loop inside and actually need to be a loop
    'main: loop {
        print_timer(&mut out, names).unwrap_or_else(|err| {eprintln!("{}", err); process::exit(3)});
        break 'main;
    }

    //makes it so that anything you do in the terminal after use this isnt weirdly colored, and resets the cursor position
    out.execute(SetForegroundColor(RESET)).expect("what did you do?");
    out.execute(MoveTo(1, 19)).expect("seriously what did you do?");
    Ok(())
}
