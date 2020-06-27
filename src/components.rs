use serde::{Serialize, Deserialize};
use crossterm::style::Color;
use crossterm::event::{read, Event, KeyCode};

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

//i bet there is a crate to do this better but guess what i just smashed my face into my keyboard and it works
//so i dont feel like changing it
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

        if s >= 60 {
            remain_s = s % 60;
            s -= remain_s;
            min = s / 60;

            if min >= 60 {
                remain_min = min % 60;
                min -= remain_min;
                hr = min / 60;
            } else { remain_min = min; hr = 0; }
        } else { remain_s = s; remain_min = 0; hr = 0; }
    } else { remain_ms = new_ms; remain_s = 0; remain_min = 0; hr = 0; }
    return (hr, remain_min, remain_s, remain_ms);

}

//the function called in spawned thread to poll for events, currently only polls for enter key
pub fn handle_events() -> u8 {

    let event = read().expect("something broke");

    if event == Event::Key(KeyCode::Enter.into()) {
        return 0;
    } else {
        return 1;
    }
}

//gets rid of unnecessary 0s to print a nicer looking time
pub fn format(numbers: (usize, usize, usize, usize)) -> String {
    let mut formatted_string = String::new();
    if numbers.0 != 0 {
        formatted_string.push_str(&format!("{}:", numbers.0));
    }
    if numbers.1 != 0 {
        formatted_string.push_str(&format!("{}:", numbers.1));
    }
    formatted_string.push_str(&format!("{:02}.{:03}", numbers.2, numbers.3));
    formatted_string

}
