struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn from_str(range_str: &str) -> Option<Self> {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Option::None;
        }
        let start_result = parts[0].parse::<u64>();
        let end_result = parts[1].parse::<u64>();
        match (start_result, end_result) {
            (Ok(start), Ok(end)) => Some(Self { start, end }),
            _ => Option::None,
        }
    }

    pub fn sum_invalid_ids(&self, for_part_1: bool) -> u64 {
        let invalid_ids = self.get_invalid_ids(for_part_1);
        invalid_ids.iter().sum()
    }

    fn get_invalid_ids(&self, for_part_1: bool) -> Vec<u64> {
        let mut invalid_ids: Vec<u64> = Vec::new();
        for id in self.start..=self.end {
            let str_id = id.to_string();
            if for_part_1 && self.invalid_id_for_part_1(&str_id) {
                invalid_ids.push(id);
            } else if !for_part_1 && self.invalid_id_for_part_2(&str_id) {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }

    fn invalid_id_for_part_1(&self, str_id: &String) -> bool {
        if str_id.len() < 2 {
            return false;
        }
        let parts: Vec<String> = self.split_str_by(str_id, 2);
        self.are_all_parts_equal(&parts)
    }

    fn invalid_id_for_part_2(&self, str_id: &String) -> bool {
        let length = str_id.len();
        if length < 2 {
            return false;
        }
        let divisible_options = self.id_divisible_by_options(str_id);
        for divisor in divisible_options {
            let parts = self.split_str_by(str_id, divisor);
            if self.are_all_parts_equal(&parts) {
                return true;
            }
        }
        false
    }

    fn split_str_by(&self, str_id: &String, divisor: u64) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        let length = str_id.len() as u64;
        if length < 2 || divisor < 2 || length % divisor != 0 {
            return parts;
        }
        let part_length = length / divisor;
        for i in 0..divisor {
            let start_index = (i * part_length) as usize;
            let end_index = ((i + 1) * part_length) as usize;
            let part = str_id[start_index..end_index].to_string();
            parts.push(part);
        }
        parts
    }

    fn are_all_parts_equal(&self, parts: &Vec<String>) -> bool {
        if parts.is_empty() {
            return false;
        }
        let first_part = &parts[0];
        for part in parts.iter() {
            if part != first_part {
                return false;
            }
        }
        true
    }

    fn id_divisible_by_options(&self, str_id: &String) -> Vec<u64> {
        let mut options: Vec<u64> = Vec::new();
        let length: u64 = str_id.len() as u64;
        for option in 2..=length {
            if length % option == 0 {
                options.push(option as u64);
            }
        }
        options
    }
}

pub fn run() {
    let input = include_str!("input.txt");
    let str_ranges: Vec<&str> = input.split(',').collect();
    let mut invalid_ids_sum_for_part_1: u64 = 0;
    let mut invalid_ids_sum_for_part_2: u64 = 0;
    for str_range in str_ranges {
        if let Some(range) = Range::from_str(str_range) {
            let sum_for_part_1 = range.sum_invalid_ids(true);
            let sum_for_part_2 = range.sum_invalid_ids(false);
            invalid_ids_sum_for_part_1 += sum_for_part_1;
            invalid_ids_sum_for_part_2 += sum_for_part_2;
        }
    }
    println!("Invalid IDs sum for part 1: {}", invalid_ids_sum_for_part_1);
    println!("Invalid IDs sum for part 2: {}", invalid_ids_sum_for_part_2);
}

pub fn print_problem() {
    let problem = include_str!("problem.txt");
    println!("{}", problem);
}
