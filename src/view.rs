use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};
use std::io::{stdout, Write};
pub fn color(color: Color, str: &str) {
    execute!(stdout(), SetForegroundColor(color), Print(str), ResetColor).unwrap();
}
pub fn print(str: String) -> () {
    execute!(stdout(), Print(str)).unwrap();
}
