#![allow(
    dead_code
)]

use std::{
    env,
    fs::{File},
    io::{BufRead, BufReader, Error},
    collections::HashMap,
};

fn check_unwrap_io_error(line_result: Result<String, Error>) -> String {
    return match line_result {
        Ok(n) => n,
        Err(err) => panic!("Problem reading from the input file: {:?}", err),
    };
}

fn validate_input_range<T>(value: T, boundaries: [T; 2] ) 
where
    T: std::fmt::Display + std::cmp::PartialOrd,
{
    assert!(
        value >= boundaries[0] && value <= boundaries[1],
        "Invalid number encountered in the input. Expected a value between {} and {}, but `{}` found.",
        boundaries[0], boundaries[1], value
    );
}

fn compute_answers(wanted_counts: &mut Vec<u64>) -> HashMap<u64, u32> {
    let upper_cap: u32 = 1_000_000_007;
    let mut virus_count: u32;
    let mut virus_count_prev: u32 = 11;
    let mut previous_copies_counts: [u32; 3] = [2, 3, 5];
    let max_clicks_count = wanted_counts.iter().max().unwrap();
    let mut answers:HashMap<u64, u32> = HashMap::new();
    // answer for four is added explicitly as the computational logic for it is different
    answers.insert(4, 11);

    for click_idx in 5..max_clicks_count + 1 {
        virus_count = (virus_count_prev + previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]) % upper_cap;
        previous_copies_counts[((click_idx + 4) % 3) as usize] = (previous_copies_counts[0] + previous_copies_counts[1] + previous_copies_counts[2]) % upper_cap;
        virus_count_prev = virus_count;

        if wanted_counts.contains(&click_idx) {
            answers.insert(click_idx, virus_count);
        }
    }

    answers
}

fn print_results(wanted_counts: Vec<u64>, answers: HashMap<u64, u32>) {
    for i in &wanted_counts {
        println!("{}", &answers[i]);
    }
}

fn get_virus_counts(filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(filename)?;
    let reader = BufReader::new(input_file);
    let mut is_first_row = true;
    let mut wanted_counts: Vec<u64> = vec![];
    let mut rows_count: u32 = 0;

    let question_count_boundaries: [u32; 2] = [1, 1_000];
    let click_count_boundaries: [u64; 2] = [4, 10_000_000_000];

    for line in reader.lines() {
        if is_first_row {
            let line_result = check_unwrap_io_error(line);
            let line_unsigned_result = line_result.parse::<u32>();

            let value = match line_unsigned_result {
                Ok(n) => n,
                Err(err) => panic!("Invalid character or number encountered in the input {:?}.", err),
            };        

            // validate that the first row contains a number of questions within the specified range
            validate_input_range(value, question_count_boundaries);
            rows_count = value;
            is_first_row = false;
        }
        else {
            let line_result = check_unwrap_io_error(line);
            let line_unsigned_result = line_result.parse::<u64>();

            let value = match line_unsigned_result {
                Ok(n) => n,
                Err(err) => panic!("Invalid character or number encountered in the input {:?}.", err),
            };        

            // validate that the question rows contain numbers of clicks within the specified range
            validate_input_range(value, click_count_boundaries);
            wanted_counts.push(value);
        }
    }

    assert_eq!(
        wanted_counts.len(), rows_count as usize,
        "Invalid number of questions. Expected {}, but {} found.",
        rows_count, wanted_counts.len()
    );

    let answers:HashMap<u64, u32> = compute_answers(&mut wanted_counts);
    print_results(wanted_counts, answers);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2, "Expected one argument with the filepath to input, got {} argument(s).", args.len() - 1);

    get_virus_counts(&args[1])
}


// TESTS

#[cfg(test)]
mod tests {
    use crate::compute_answers;
    use crate::get_virus_counts;
    use crate::check_unwrap_io_error;
    use crate::validate_input_range;

    #[test]
    fn test_input_1() {
        match get_virus_counts(&"../../test_inputs/input_1".to_string()) {
            Ok(n) => n,
            Err(err) => panic!("Problem reading from the input file: {:?}", err),
        };
    }

    #[test]
    #[should_panic]
    fn test_input_2() {
        // a negative number found in the input
        match get_virus_counts(&"../../test_inputs/input_2".to_string()) {
            Ok(n) => n,
            Err(err) => panic!("Problem reading from the input file: {:?}", err),
        };
    }

    #[test]
    fn test_input_3() {
        match get_virus_counts(&"../../test_inputs/input_3".to_string()) {
            Ok(n) => n,
            Err(err) => panic!("Problem reading from the input file: {:?}", err),
        };
    }

    #[test]
    fn test_input_4() {
        match get_virus_counts(&"../../test_inputs/input_4".to_string()) {
            Ok(n) => n,
            Err(err) => panic!("{:?}", err),
        };
    }

    #[test]
    #[should_panic]
    fn test_input_5() {
        // number of questions above the specified limit
        match get_virus_counts(&"../../test_inputs/input_5".to_string()) {
            Ok(n) => n,
            Err(err) => panic!("{:?}", err),
        };
    }

    #[test]
    fn test_check_unwrap_io_error() {
        assert_eq!(check_unwrap_io_error(std::result::Result::Ok("42".to_string())), "42");
    }

    #[test]
    fn test_validate_input_range() {
        let boundaries: [u32; 2] = [1, 1_000];
        let value: u32 = 42;
        validate_input_range(value, boundaries);
    }

    #[test]
    #[should_panic]
    fn test_validate_input_range_below() {
        let boundaries: [u32; 2] = [1, 1_000];
        let value: u32 = 0;
        validate_input_range(value, boundaries);
    }

    #[test]
    #[should_panic]
    fn test_validate_input_range_above() {
        let boundaries: [u32; 2] = [1, 1_000];
        let value: u32 = 1_001;
        validate_input_range(value, boundaries);
    }

    #[test]
    fn test_compute_answers() {
        let mut wanted_counts: Vec<u64> = vec![];
        for value in 4..100 {
            wanted_counts.push(value);
        }

        let answers = compute_answers(&mut wanted_counts);

        for value in 4..100 {
            assert_ne!(answers[&value], 0);
        }
    }
}
