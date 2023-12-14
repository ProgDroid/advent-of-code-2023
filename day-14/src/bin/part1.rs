use log::debug;

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn tilt(lines: &str) -> Vec<String> {
    log(format!("Tilting {lines}"));

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

fn compute_load(lines: &str) -> usize {
    let tilted_grid: Vec<String> = tilt(lines);

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

        assert_eq!(result, 136);
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

        let expected: Vec<String> = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        let result = tilt(lines);

        assert_eq!(result, expected);
    }
}
