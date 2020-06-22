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

pub fn ms_to_readable<'a>(ms: &usize) -> (usize, usize, usize, usize) {
    let mut new_ms = *ms;
    let mut s: usize;
    let mut min: usize;
    let hr: usize;
    let remain_ms: usize;
    let remain_s: usize;
    let remain_min: usize;

    if new_ms >= 1000 {
        remain_ms = new_ms % 1000;
        new_ms -= remain_ms;
        s = new_ms / 1000;
    }
    else {
        return (0, 0, 0, new_ms);
    }

    if s >= 60 {
        remain_s = s % 60;
        s -= remain_s;
        min = s / 60;
    }
    else {
        return (0, 0, s, new_ms);
    }

    if min >= 60 {
        remain_min = min % 60;
        min -= remain_min;
        hr = min / 60;
    }
    else {
        return (0, min, s, new_ms);
    }
    return (hr, min, s, new_ms);

}
