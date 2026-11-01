mod day;
mod day1;

use crate::day::load_days;
use inquire::Select;
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
    let days = load_days();
    let options = days.keys().cloned().collect::<Vec<&str>>();
    let selection = Select::new("Select a day:", options).prompt().unwrap();
    let day_fn = days.get(selection);
    match day_fn {
        Some(day) => day.print_result(),
        None => println!("Invalid selection"),
    };
}
