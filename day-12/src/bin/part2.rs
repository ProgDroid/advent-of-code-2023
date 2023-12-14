use rayon::prelude::*;
use std::collections::HashMap;

use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

#[allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::needless_pass_by_value
)]
fn arrange(line: String, groups: &[usize], cache: &mut HashMap<String, usize>) -> usize {
    log(format!("Arranging line {line}").as_str());

    let key = format!("{}{:?}", &line, &groups);

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    if groups.is_empty() {
        return usize::from(!line.chars().any(|c| c == '#'));
    }

    let mut arrangements = 0;

    let next_groups = Vec::from(&groups[1..]);

    let iterations: i64 = line.len() as i64 - next_groups.iter().sum::<usize>() as i64
        + next_groups.len() as i64
        - *groups.first().unwrap() as i64
        + 1;

    if iterations <= 0 {
        cache.insert(key, arrangements);

        return arrangements;
    }

    let iterations = iterations as usize;

    for n in 0..iterations {
        let operational = String::from(".").repeat(n);

        let damaged = String::from("#").repeat(*groups.first().unwrap());

        let to_test = format!("{}{}.", &operational, &damaged);

        let difference = line
            .chars()
            .zip(to_test.chars())
            .filter(|(a, b)| a != b && *a != '?')
            .count();

        if difference == 0 {
            let next_line = line
                .get(to_test.len()..)
                .map_or_else(String::new, std::string::ToString::to_string);

            arrangements += arrange(next_line, &next_groups, cache);
        }
    }

    cache.insert(key, arrangements);

    arrangements
}

fn possible_arrangements(line: &str) -> usize {
    log(format!("Parsing line {line}").as_str());

    let Some((springs, groups)) = line.split_once(' ') else {
        return 0;
    };

    let mut unfolded_springs = String::default();

    let groups: Vec<usize> = groups
        .split(',')
        .filter_map(|number| number.parse::<usize>().ok())
        .collect();

    let mut unfolded_groups: Vec<usize> = Vec::default();

    for n in 0..5 {
        if n > 0 {
            unfolded_springs.push('?');
        }

        unfolded_springs.push_str(springs);

        unfolded_groups.append(&mut groups.clone());
    }

    let mut cache: HashMap<String, usize> = HashMap::default();

    arrange(unfolded_springs, &unfolded_groups, &mut cache)
}

fn sum_possible_arrangements(lines: &str) -> usize {
    lines.par_lines().map(possible_arrangements).sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", sum_possible_arrangements(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = sum_possible_arrangements(lines);

        assert_eq!(result, 525_152);
    }

    #[test]
    fn test_single_line() {
        let line = "???.### 1,1,3";

        let result = possible_arrangements(line);

        assert_eq!(result, 1);

        let line = ".??..??...?##. 1,1,3";

        let result = possible_arrangements(line);

        assert_eq!(result, 16384);

        let line = "?#?#?#?#?#?#?#? 1,3,1,6";

        let result = possible_arrangements(line);

        assert_eq!(result, 1);

        let line = "????.#...#... 4,1,1";

        let result = possible_arrangements(line);

        assert_eq!(result, 16);

        let line = "????.######..#####. 1,6,5";

        let result = possible_arrangements(line);

        assert_eq!(result, 2500);

        let line = "?###???????? 3,2,1";

        let result = possible_arrangements(line);

        assert_eq!(result, 506_250);
    }
}
