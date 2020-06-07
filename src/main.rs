use std::{env, fs, process, iter::FromIterator, thread::sleep, time::Duration};
use std::io::{Write, stdout, Error};
use crossterm::Result as cross_result;
use crossterm::{QueueableCommand, ExecutableCommand};
use crossterm::style::Print;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Split {
    name: String,
    time: u32,
}

/*i really really did not want to end up typing this multiple times
type Output<'a> = &'a mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>;

//these are the colors that the timer will use for ahead/behind/gold/other stuff
static GOOD: Color = Color::LightGreen;
static STANDARD: Color = Color::White;
static BAD: Color = Color::LightRed;
static GOLD: Color = Color::LightYellow;*/

//makes sure that an argument was actually provided
fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a file to read splits from");
    }
    let splits = &args[1];
    Ok(splits.to_string())
}

fn print_to_terminal(mut stdout: std::io::Stdout, rows: Vec<&str>) -> cross_result<()> {
    stdout.queue(Print("something is being printed"))?;
    stdout.flush()?;
    Ok(())
}

fn splits_to_print(split_vec: Vec<&str>, line: usize) -> Vec<&str> {
    let end = line + 1;
    //i have no idea why line works but it does. thank you rust forum user nemo157.
    let print_vec = Vec::from_iter(split_vec[line..end].iter().cloned());
    print_vec
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let out = stdout();
    let second = Duration::new(1, 0);
    let mut current_line = 0;
    let file = check_args(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let json_raw = fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let json_as_splits: Vec<Split> = serde_json::from_str(&json_raw)?;
    print_to_terminal(out).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let mut rows = Vec::new();
    for i in json_as_splits {
        let split = i.name;
        split_vec.push(something);
    }
    loop {
        {
            let table_rows = splits_to_print(&rows, current_line);
            sleep(second);
            current_line += 1;
            draw_timer(&mut terminal, table_rows)?;
        }
        if current_line == 2 {
            break;
        }
    }
    //print!("{}", Goto(1, 21));
    Ok(())
}
