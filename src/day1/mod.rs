struct Dial {
    current_position: i16,
    zero_position_matches: u16,
}

impl Dial {
    pub fn new(start_position: i16) -> Self {
        Self {
            current_position: start_position,
            zero_position_matches: 0,
        }
    }

    pub fn get_matches(&self) -> u16 {
        self.zero_position_matches
    }

    pub fn input_rotation(&mut self, input: &str) {
        if input.len() < 2 {
            return;
        }
        let direction_option = input.chars().nth(0);
        let direction = match direction_option {
            Some('L') => -1,
            Some('R') => 1,
            _ => 0,
        };
        let steps_result = input[1..].parse::<i16>();
        let steps = match steps_result {
            Ok(s) => s * direction,
            Err(_) => 0,
        };
        self.rotate(steps);
    }

    fn rotate(&mut self, steps: i16) {
        let normalized_steps = steps % 100;
        let mut new_position = self.current_position + normalized_steps;
        if new_position < 0 {
            new_position = 100 + new_position;
        } else if new_position > 99 {
            new_position = new_position - 100;
        }
        self.current_position = new_position;
        if self.current_position == 0 {
            self.zero_position_matches += 1;
        }
    }
}

pub fn run() {
    let input = include_str!("input.txt");
    let rotations = input.lines();

    let start_position = 50;
    let mut dial = Dial::new(start_position);
    for rotation in rotations {
        dial.input_rotation(rotation);
    }
    println!("The password is: {}", dial.get_matches());
}

pub fn print_problem() {
    let problem = include_str!("problem.txt");
    println!("{}", problem);
}
