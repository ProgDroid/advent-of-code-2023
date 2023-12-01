use log::debug;

use core::fmt;
use std::{env::args, fs::read_to_string, num::ParseIntError};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    fn iterator() -> impl Iterator<Item = Self> {
        [
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
        ]
        .iter()
        .copied()
    }

    const fn to_char(self) -> char {
        match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "one"),
            Self::Two => write!(f, "two"),
            Self::Three => write!(f, "three"),
            Self::Four => write!(f, "four"),
            Self::Five => write!(f, "five"),
            Self::Six => write!(f, "six"),
            Self::Seven => write!(f, "seven"),
            Self::Eight => write!(f, "eight"),
            Self::Nine => write!(f, "nine"),
        }
    }
}

impl TryFrom<&str> for Number {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            "three" => Ok(Self::Three),
            "four" => Ok(Self::Four),
            "five" => Ok(Self::Five),
            "six" => Ok(Self::Six),
            "seven" => Ok(Self::Seven),
            "eight" => Ok(Self::Eight),
            "nine" => Ok(Self::Nine),
            _ => Err("Could not convert string into a number"),
        }
    }
}

impl TryFrom<char> for Number {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err("Could not convert char into a number"),
        }
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn numbers_from_token(token: &str) -> Vec<Number> {
    let mut vec: Vec<(usize, Number)> = Number::iterator()
        .flat_map(|number| {
            let indices: Vec<(usize, &str)> = token.match_indices(&number.to_string()).collect();
            indices
                .iter()
                .map(|(index, _)| (*index, number))
                .collect::<Vec<(usize, Number)>>()
        })
        .collect();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    vec.iter().map(|(_, b)| *b).collect()
}

fn number_from_string(source: &str) -> Result<u32, ParseIntError> {
    let mut numbers: Vec<Number> = Vec::default();

    let mut current_token = String::default();

    for c in source.chars() {
        if c.is_numeric() {
            numbers.append(&mut numbers_from_token(current_token.as_str()));

            current_token = String::default();
            match Number::try_from(c) {
                Ok(number) => numbers.push(number),
                Err(e) => log(e),
            };
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        numbers.append(&mut numbers_from_token(current_token.as_str()));
    }

    for number in &numbers {
        log(format!("{number}").as_str());
    }

    let mut number_string = String::default();

    if let Some(first_digit) = numbers.first() {
        number_string.push(first_digit.to_char());
    }

    if let Some(last_digit) = numbers.last() {
        number_string.push(last_digit.to_char());
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
        let lines = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            .lines();

        let result = calculate_calibration_values_sum(lines);

        assert_eq!(result, 281);
    }

    #[test]
    fn test_number_from_string() {
        let lines = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected = vec![29, 83, 13, 24, 42, 14, 76];

        let result: Vec<u32> = lines
            .iter()
            .filter_map(|line| number_from_string(line).ok())
            .collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_numbers_from_token() {
        let token = "eightwothree";

        let result: Vec<Number> = numbers_from_token(token);

        let expected = vec![Number::Eight, Number::Two, Number::Three];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_repeated_numbers_from_same_token() {
        let token = "qwsdsixsixabx";

        let result: Vec<Number> = numbers_from_token(token);

        let expected = vec![Number::Six, Number::Six];

        assert_eq!(expected, result);
    }
}
