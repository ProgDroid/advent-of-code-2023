use log::debug;

use std::{env::args, fs::read_to_string, num::ParseIntError};

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn number_from_string(source: &str) -> Result<u32, ParseIntError> {
    let numeric_digits: Vec<char> = source.chars().filter(|char| char.is_numeric()).collect();

    let mut number_string = String::default();

    if let Some(first_digit) = numeric_digits.first() {
        number_string.push(*first_digit);
    }

    if let Some(last_digit) = numeric_digits.last() {
        number_string.push(*last_digit);
    }

    number_string.parse::<u32>()
}

fn calculate_calibration_values_sum<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    lines
        .filter_map(|line| {
            log(format!("Parsing {line}").as_str());
            let result = number_from_string(line);

            log(format!("Got {}\n", result.as_ref().unwrap_or(&0)).as_str());

            result.ok()
        })
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let args: Vec<String> = args().collect();

    args.get(1).map_or_else(
        || {
            eprintln!("No input file path provided");
        },
        |file_name| {
            match read_to_string(file_name) {
                Ok(lines) => println!(
                    "Answer: {}",
                    calculate_calibration_values_sum(lines.lines())
                ),
                Err(e) => eprintln!("Could not load input file {file_name}: {e}"),
            };
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            .lines();

        let result = calculate_calibration_values_sum(lines);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_number_from_string() {
        let lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let expected = vec![12, 38, 15, 77];

        let result: Vec<u32> = lines
            .iter()
            .filter_map(|line| number_from_string(line).ok())
            .collect();

        assert_eq!(expected, result);
    }
}
