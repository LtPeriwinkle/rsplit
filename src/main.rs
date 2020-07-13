use std::{env, fs, process, iter::FromIterator, thread, sync::mpsc, time::Duration};
use std::io::{Write, stdout, Error};
use crossterm::Result as cross_result;
use crossterm::{QueueableCommand, ExecutableCommand, cursor::MoveTo};
use crossterm::style::{Print, SetForegroundColor};
use crossterm::terminal::{SetTitle, Clear, ClearType::{All, CurrentLine}};
use serde_json;
use spin_sleep::{LoopHelper, sleep};

//file with structs, static vars; i didnt want to have them cluttering up this file
mod components;
use components::*;

//make the json file into a vec of Splits (from components.rs) and the split names into another vec
fn get_splits<'a>(file: &'a String) -> (Vec<Split<'a>>, Vec<&'a str>) {
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
fn splits_to_print<'a>(split_vec: &'a Vec<&str>, mut line: usize) -> Vec<&'a str> {
    let length = split_vec.len();
    if length <= 18 {
        let print_vec = split_vec.clone();
        print_vec
    } else if line + 18 > length {
        let end = length;
        line = end - 18;
        //i have no idea why line works but it does, thank you rust forum
        let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
        print_vec
    } else {
        let end = line + 18;
        let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
        print_vec
    }
}

//prints everything that needs to be shown, by queueing timer rows then flushing them at the end
fn print_timer(out: &mut std::io::Stdout, rows: &Vec<&str>, mut current_line: usize, time: &str) -> cross_result<()> {
    let table_rows = splits_to_print(rows, current_line);
    current_line = 0;
    loop {
        if current_line == table_rows.len() {
            break;
        }
        queue_table_row(table_rows[current_line], &time, out, current_line as u16)?;
        current_line += 1;
    }
    //makes crossterm do all the stuff queued in queue_table_row() calls
    out.flush()?;
    Ok(())
}

//takes the name of the split, the time that should be displayed for it, and the row in terminal where it should be printed
//and queues a row with name + time into the crossterm buffer
fn queue_table_row(split_name: &str, time: &str, out: &mut std::io::Stdout, row: u16) -> cross_result<()> {
    out.queue(MoveTo(1, row))?
        .queue(Clear(CurrentLine))?
        .queue(Print(split_name))?
        .queue(MoveTo(20, row))?
        .queue(SetForegroundColor(GOOD))?
        .queue(Print(time))?
        .queue(SetForegroundColor(RESET))?;
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
    let results = get_splits(&json_raw);
    let names = results.1;
    //will be used for time comparisons later, not useful right now
    let _split_vec = results.0;

    //make sure we arent printing over other stuff
    out.execute(Clear(All)).expect("could not initialize terminal").execute(SetTitle("rsplit")).expect("could not initialize terminal");

    //supposed to be more accurate than normal sleep, am using to keep the loop at every 10 ms
    let mut update_timer = LoopHelper::builder().build_with_target_rate(100.0);

    let mut current_line: usize = 0;
    let mut ms: usize = 0;

    //multithreading scary but i couldnt think of anything better
    let (tx, rx) = mpsc::channel();
    let _event_listener = thread::spawn(move ||
        loop {
            sleep(Duration::new(0, 50_000));
            tx.send(handle_events()).unwrap();
        }
    );

    'main: loop {
        'update: loop {
            update_timer.loop_start();
            ms += 10;
            let times = ms_to_readable(&ms);
            //in components.rs, NOT the format!() macro
            let string = format(times);
            print_timer(&mut out, &names, current_line, &string).unwrap_or_else(|err| {eprintln!("{}", err); process::exit(3)});
            let event = rx.try_recv();
            if event == Ok(0) {
                break 'update;
            }
            update_timer.loop_sleep();
        }
        if current_line == names.len() - 1 {
            break 'main;
        } else {
            current_line += 1;
        }
    }

    //makes it so that anything you do in the terminal after use this isnt weirdly colored, and resets the cursor position
    out.execute(SetForegroundColor(RESET)).expect("could not clear terminal").execute(MoveTo(0, 19)).expect("could not clear terminal");
    Ok(())
}
