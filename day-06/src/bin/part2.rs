use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn binary_search(list: &[usize], value: usize, max: usize) -> usize {
    log(format!("Checking list {list:?}").as_str());

    if list.len() == 1 {
        return *list.first().unwrap();
    }

    let index = list.len() / 2;

    log(format!("Checking index {index:?}").as_str());

    let list_value = list.get(index).unwrap();

    log(format!("Checking calculation {list_value:?} * ({max:?} - {list_value:?})").as_str());

    if list_value * (max - list_value) <= value {
        binary_search(&list[index..], value, max)
    } else {
        binary_search(&list[0..index], value, max)
    }
}

fn power_ways_to_win_races(input: &str) -> usize {
    let Some((times_string, distances_string)) = input.split_once('\n') else {
        log("Could not split input into 2 lines");
        return 0;
    };

    let time: String = times_string.chars().filter(|c| c.is_numeric()).collect();

    let Ok(time) = time.parse::<usize>() else {
        return 0;
    };

    let distance: String = distances_string
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    let Ok(distance) = distance.parse::<usize>() else {
        return 0;
    };

    let maximum_non_win = binary_search(&(1..time).collect::<Vec<usize>>(), distance, time);

    log(format!("Maximum non win: {maximum_non_win:?}").as_str());

    // add 1 for time + 0
    // subtract 2 for holding 0 and holding the whole time
    let ways_of_beating_record = (time + 1) - 2 - (maximum_non_win * 2);

    log(format!("Ways of beating record: {ways_of_beating_record:?}").as_str());

    // subtract holding 0 and holding the whole time
    ways_of_beating_record
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", power_ways_to_win_races(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "Time:      7  15   30
        Distance:  9  40  200";

        let result = power_ways_to_win_races(lines);

        assert_eq!(result, 71503);
    }
}
