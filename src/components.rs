use serde::{Serialize, Deserialize};
use crossterm::style::Color;

#[derive(Serialize, Deserialize)]
pub struct Split<'a> {
    pub name: &'a str,
    pub time: u32,
}

//these are the colors that the timer will use for ahead/behind/gold/other stuff
pub static GOOD: Color = Color::Green;
/*pub static STANDARD: Color = Color::White;
pub static BAD: Color = Color::Red;
pub static GOLD: Color = Color::Yellow;*/
pub static RESET: Color = Color::Reset;

//makes sure that an argument was actually provided and returns an error that is used to stop program later if no argument
pub fn check_args(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please specify a file to read splits from");
    }
    //since for some reason args().collect() gives an array with the command also in it, grab filename out of the array
    let splits = &args[1];
    Ok(splits.to_string())
}
