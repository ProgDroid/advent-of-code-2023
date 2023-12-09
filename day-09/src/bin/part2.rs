use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn calculate_differences(values: &[i64]) -> Vec<i64> {
    log(format!("Calculating differences {values:?}").as_str());

    let mut differences: Vec<i64> = Vec::default();

    for n in 1..values.len() {
        differences.push(values[n] - values[n - 1]);
    }

    log(format!("Calculated differences {differences:?}").as_str());

    differences
}

fn extrapolate_next_value(values: &[i64]) -> i64 {
    log(format!("Extrapolating next value for {values:?}").as_str());

    let mut list_of_differences: Vec<Vec<i64>> = Vec::default();

    let mut differences = calculate_differences(values);
    list_of_differences.push(differences.clone());

    while !differences.iter().all(|difference| *difference == 0) {
        differences = calculate_differences(&differences);
        list_of_differences.push(differences.clone());
    }

    let mut current_number = 0;

    for difference_list in list_of_differences.iter().rev() {
        current_number = difference_list.first().unwrap() - current_number;
    }

    let next_value = values.first().unwrap() - current_number;

    log(format!("Next value is {next_value}").as_str());

    next_value
}

fn parse_line(line: &str) -> Vec<i64> {
    log(format!("Parsing line {line}").as_str());

    line.split_whitespace()
        .filter_map(|number| number.parse::<i64>().ok())
        .collect()
}

fn sum_extrapolated_values<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    lines
        .map(|line| extrapolate_next_value(&parse_line(line)))
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", sum_extrapolated_values(input.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = sum_extrapolated_values(lines.lines());

        assert_eq!(result, 2);
    }

    #[test]
    fn test_extrapolate_next_value() {
        let values = vec![0, 3, 6, 9, 12, 15];

        let result = extrapolate_next_value(&values);

        assert_eq!(result, -3);

        let values = vec![1, 3, 6, 10, 15, 21];

        let result = extrapolate_next_value(&values);

        assert_eq!(result, 0);

        let values = vec![10, 13, 16, 21, 30, 45];

        let result = extrapolate_next_value(&values);

        assert_eq!(result, 5);
    }
}
