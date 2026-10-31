mod day1;

use inquire::Select;
use std::collections::HashMap;
use std::io::{Write, stdout};

fn clear_screen() {
    // \x1B[2J clears the screen, \x1B[1;1H moves the cursor to the top-left corner
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}
fn main() {
    clear_screen();
    println!("Welcome to Advent of Code 2025!");
    println!("Select a day to continue:");
    let days = HashMap::from([("day1", day1::run)]);
    let options = days.keys().cloned().collect::<Vec<&str>>();
    let selection = Select::new("Select a day:", options).prompt().unwrap();
    days.get(selection).unwrap()();
}
