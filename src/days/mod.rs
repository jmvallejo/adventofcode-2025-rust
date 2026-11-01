mod day1;

use std::collections::HashMap;

pub struct Day {
    run: fn(),
    print_problem: fn(),
}

impl Day {
    pub fn new(run: fn(), print_problem: fn()) -> Self {
        Self { run, print_problem }
    }

    pub fn print_result(&self) {
        (self.print_problem)();
        (self.run)();
    }
}

pub fn load_days() -> HashMap<&'static str, Day> {
    HashMap::from([
        ("day1", Day::new(day1::run, day1::print_problem)),
        // Add more days here as needed
    ])
}
