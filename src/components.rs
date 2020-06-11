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
