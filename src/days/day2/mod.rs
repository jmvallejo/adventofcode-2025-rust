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

    pub fn sum_invalid_ids(&self) -> u64 {
        let invalid_ids = self.get_invalid_ids();
        invalid_ids.iter().sum()
    }

    fn get_invalid_ids(&self) -> Vec<u64> {
        let mut invalid_ids: Vec<u64> = Vec::new();
        for id in self.start..=self.end {
            let str_id = id.to_string();
            let length = str_id.len();
            if length < 2 || length % 2 != 0 {
                continue;
            }
            let mid = length / 2;
            let left_str = str_id[..mid].to_string();
            let right_str = str_id[mid..].to_string();
            if left_str == right_str {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }
}

pub fn run() {
    let input = include_str!("input.txt");
    let str_ranges: Vec<&str> = input.split(',').collect();
    let mut invalid_ids_sum: u64 = 0;
    for str_range in str_ranges {
        if let Some(range) = Range::from_str(str_range) {
            let sum = range.sum_invalid_ids();
            invalid_ids_sum += sum;
        }
    }
    println!("Invalid IDs sum: {}", invalid_ids_sum);
}

pub fn print_problem() {
    let problem = include_str!("problem.txt");
    println!("{}", problem);
}
