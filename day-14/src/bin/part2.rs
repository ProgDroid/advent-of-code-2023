use std::collections::HashMap;

use log::debug;

const CYCLES: usize = 1_000_000_000;

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn tilt_north(lines: &str) -> Vec<String> {
    log(format!("Tilting North {lines}"));

    let line_length = lines.lines().next().unwrap().len();

    let mut transposed_grid: Vec<String> = Vec::default();

    for n in 0..line_length {
        let column: String = lines
            .chars()
            .filter(|c| *c != '\n')
            .skip(n)
            .step_by(line_length)
            .collect();

        transposed_grid.push(column);
    }

    log(format!("Transposed Grid {transposed_grid:?}"));

    let mut tilted: Vec<String> = vec![String::new(); lines.lines().count()];

    for column in transposed_grid {
        log(format!("Column {column}"));

        let sections: Vec<&str> = column.split('#').collect();

        log(format!("Sections {sections:?}"));

        let mut new_column: Vec<String> = Vec::default();

        for section in sections {
            let rock_count = section.match_indices('O').count();

            new_column.push(format!(
                "{}{}",
                "O".repeat(rock_count),
                ".".repeat(section.len() - rock_count)
            ));

            log(format!("Current Column {new_column:?}"));
        }

        let new_column = new_column.join("#");

        log(format!("New Column {new_column}"));

        tilted = tilted
            .iter()
            .zip(new_column.chars())
            .map(|(row, c)| {
                let mut new_row = row.clone();
                new_row.push(c);
                new_row
            })
            .collect();
    }

    log(format!("Tilted result {tilted:?}"));

    tilted
}

fn tilt_west(lines: &str) -> Vec<String> {
    log(format!("Tilting West {lines}"));

    let tilted: Vec<String> = lines
        .lines()
        .map(|line| {
            log(format!("Row {line}"));

            let sections: Vec<&str> = line.split('#').collect();

            log(format!("Sections {sections:?}"));

            let mut new_line: Vec<String> = Vec::default();

            for section in sections {
                let rock_count = section.match_indices('O').count();

                new_line.push(format!(
                    "{}{}",
                    "O".repeat(rock_count),
                    ".".repeat(section.len() - rock_count)
                ));

                log(format!("Current Row {new_line:?}"));
            }

            let new_line = new_line.join("#");

            log(format!("New Column {new_line}"));

            new_line
        })
        .collect();

    log(format!("Tilted result {tilted:?}"));

    tilted
}

fn tilt_south(lines: &str) -> Vec<String> {
    log(format!("Tilting South {lines}"));

    let line_length = lines.lines().next().unwrap().len();

    let mut transposed_grid: Vec<String> = Vec::default();

    for n in 0..line_length {
        let column: String = lines
            .chars()
            .filter(|c| *c != '\n')
            .skip(n)
            .step_by(line_length)
            .collect();

        transposed_grid.push(column);
    }

    log(format!("Transposed Grid {transposed_grid:?}"));

    let mut tilted: Vec<String> = vec![String::new(); lines.lines().count()];

    for column in transposed_grid {
        log(format!("Column {column}"));

        let sections: Vec<&str> = column.split('#').collect();

        log(format!("Sections {sections:?}"));

        let mut new_column: Vec<String> = Vec::default();

        for section in sections {
            let rock_count = section.match_indices('O').count();

            new_column.push(format!(
                "{}{}",
                ".".repeat(section.len() - rock_count),
                "O".repeat(rock_count),
            ));

            log(format!("Current Column {new_column:?}"));
        }

        let new_column = new_column.join("#");

        log(format!("New Column {new_column}"));

        tilted = tilted
            .iter()
            .zip(new_column.chars())
            .map(|(row, c)| {
                let mut new_row = row.clone();
                new_row.push(c);
                new_row
            })
            .collect();
    }

    log(format!("Tilted result {tilted:?}"));

    tilted
}

fn tilt_east(lines: &str) -> Vec<String> {
    log(format!("Tilting East {lines}"));

    let tilted: Vec<String> = lines
        .lines()
        .map(|line| {
            log(format!("Row {line}"));

            let sections: Vec<&str> = line.split('#').collect();

            log(format!("Sections {sections:?}"));

            let mut new_line: Vec<String> = Vec::default();

            for section in sections {
                let rock_count = section.match_indices('O').count();

                new_line.push(format!(
                    "{}{}",
                    ".".repeat(section.len() - rock_count),
                    "O".repeat(rock_count),
                ));

                log(format!("Current Row {new_line:?}"));
            }

            let new_line = new_line.join("#");

            log(format!("New Column {new_line}"));

            new_line
        })
        .collect();

    log(format!("Tilted result {tilted:?}"));

    tilted
}

fn compute_load(lines: &str) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut tilted_grid: Vec<String> = lines
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    let mut last_cycle: Vec<String> = Vec::default();

    for n in 0..CYCLES {
        tilted_grid = tilt_north(&tilted_grid.join("\n"));

        tilted_grid = tilt_west(&tilted_grid.join("\n"));

        tilted_grid = tilt_south(&tilted_grid.join("\n"));

        tilted_grid = tilt_east(&tilted_grid.join("\n"));

        if let Some(cached_grid_index) = cache.get(&tilted_grid.join("\n")) {
            if (CYCLES - n) % (n - cached_grid_index) == 0 {
                tilted_grid = last_cycle;
                break;
            }
        }

        last_cycle = tilted_grid.clone();

        cache.insert(tilted_grid.join("\n"), n);
    }

    let multiplier = tilted_grid.len();

    tilted_grid
        .iter()
        .enumerate()
        .map(|(index, line)| line.match_indices('O').count() * (multiplier - index))
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", compute_load(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let result = compute_load(lines);

        assert_eq!(result, 64);
    }

    #[test]
    fn test_tilt() {
        let lines = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let expected: Vec<String> = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        let mut result: Vec<String> = lines
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        result = tilt_north(&result.join("\n"));
        result = tilt_west(&result.join("\n"));
        result = tilt_south(&result.join("\n"));
        result = tilt_east(&result.join("\n"));

        assert_eq!(result, expected);

        let expected: Vec<String> = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        result = tilt_north(&result.join("\n"));
        result = tilt_west(&result.join("\n"));
        result = tilt_south(&result.join("\n"));
        result = tilt_east(&result.join("\n"));

        assert_eq!(result, expected);

        let expected: Vec<String> = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        result = tilt_north(&result.join("\n"));
        result = tilt_west(&result.join("\n"));
        result = tilt_south(&result.join("\n"));
        result = tilt_east(&result.join("\n"));

        assert_eq!(result, expected);
    }
}
