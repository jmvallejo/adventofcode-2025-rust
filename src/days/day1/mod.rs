struct Dial {
    current_position: i16,
    zero_position_matches: u16,
    zero_position_passes: u16,
}

impl Dial {
    pub fn new(start_position: i16) -> Self {
        Self {
            current_position: start_position,
            zero_position_matches: 0,
            zero_position_passes: 0,
        }
    }

    pub fn get_matches(&self) -> u16 {
        self.zero_position_matches
    }

    pub fn get_passes(&self) -> u16 {
        self.zero_position_passes
    }

    pub fn input_rotation(&mut self, input: &str) {
        if input.len() < 2 {
            return;
        }
        let direction = match input.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => return,
        };
        let steps = match input.get(1..).unwrap_or("").parse::<i32>() {
            Ok(steps) => steps * direction,
            Err(_) => return,
        };
        self.rotate(steps);
    }

    fn rotate(&mut self, steps: i32) {
        let position = self.current_position as i32;
        let distance = steps.unsigned_abs();

        // Count every click that lands on zero before reducing the movement
        // modulo 100. Starting at zero does not count as a click; it takes a
        // full revolution to return to it.
        let zero_clicks = if steps > 0 {
            (distance + position as u32) / 100
        } else if steps < 0 {
            (distance + ((100 - position) % 100) as u32) / 100
        } else {
            0
        };

        self.zero_position_passes += zero_clicks as u16;
        self.current_position = (position + steps).rem_euclid(100) as i16;

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
    println!(
        "The password using method 0x434C49434B is: {}",
        dial.get_passes()
    );
}

pub fn print_problem() {
    let problem = include_str!("problem.txt");
    println!("{}", problem);
}

#[cfg(test)]
mod tests {
    use super::Dial;

    #[test]
    fn counts_every_click_at_zero_in_the_example() {
        let mut dial = Dial::new(50);
        for rotation in [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ] {
            dial.input_rotation(rotation);
        }

        assert_eq!(dial.get_matches(), 3);
        assert_eq!(dial.get_passes(), 6);
    }

    #[test]
    fn counts_full_revolutions_without_losing_or_double_counting_clicks() {
        let mut from_fifty = Dial::new(50);
        from_fifty.input_rotation("R100");
        assert_eq!(from_fifty.get_passes(), 1);

        let mut from_zero = Dial::new(0);
        from_zero.input_rotation("R200");
        assert_eq!(from_zero.get_passes(), 2);
    }
}
