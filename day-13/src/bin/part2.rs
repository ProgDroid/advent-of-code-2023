use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

struct Grid {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Grid {
    #[allow(clippy::too_many_lines)]
    fn summarise(&self) -> u32 {
        let possible_middle_pairs_rows: Vec<(usize, usize)> = self
            .rows
            .iter()
            .enumerate()
            .zip(self.rows.iter().enumerate().skip(1))
            .filter_map(|(a, b)| {
                let differences = a.1.chars().zip(b.1.chars()).filter(|(a, b)| a != b).count();

                if a.1 == b.1 || differences == 1 {
                    Some((a.0, b.0))
                } else {
                    None
                }
            })
            .collect();

        let mut difference_spotted: Option<(usize, usize)> = None;

        let results_rows: Vec<(u32, bool)> = possible_middle_pairs_rows
            .iter()
            .enumerate()
            .map(|(index_row, (end, start))| {
                let mut count = 0;

                difference_spotted = None;

                for n in 0..=*end {
                    if let Some(left) = self.rows.get(end - n) {
                        if let Some(right) = self.rows.get(start + n) {
                            let differences = left
                                .chars()
                                .zip(right.chars())
                                .filter(|(a, b)| *a != *b)
                                .count();

                            let mut index_column: Option<usize> = None;

                            if differences == 1 {
                                if let Some(((column, _), _)) = left
                                    .chars()
                                    .enumerate()
                                    .zip(right.chars())
                                    .find(|(a, b)| a.1 != *b)
                                {
                                    index_column = Some(column);
                                }
                            }

                            if left != right
                                && (differences != 1
                                    || difference_spotted.is_some()
                                        && difference_spotted.unwrap()
                                            != (index_row, index_column.unwrap()))
                            {
                                difference_spotted = None;
                                count = 0;
                                break;
                            }

                            if differences == 1 {
                                difference_spotted = Some((index_row, index_column.unwrap()));
                            }
                        }
                    }

                    count += 1;
                }

                (count, difference_spotted.is_some())
            })
            .filter(|result| result.0 != 0 && result.1)
            .collect();

        let possible_middle_pairs_columns: Vec<(usize, usize)> = self
            .columns
            .iter()
            .enumerate()
            .zip(self.columns.iter().enumerate().skip(1))
            .filter_map(|(a, b)| {
                let differences = a.1.chars().zip(b.1.chars()).filter(|(a, b)| a != b).count();

                if a.1 == b.1 || differences == 1 {
                    Some((a.0, b.0))
                } else {
                    None
                }
            })
            .collect();

        let results_columns: Vec<(u32, bool)> = possible_middle_pairs_columns
            .iter()
            .enumerate()
            .map(|(index_column, (end, start))| {
                let mut count = 0;

                difference_spotted = None;

                for n in 0..=*end {
                    if let Some(left) = self.columns.get(end - n) {
                        if let Some(right) = self.columns.get(start + n) {
                            let differences = left
                                .chars()
                                .zip(right.chars())
                                .filter(|(a, b)| *a != *b)
                                .count();

                            let mut index_row: Option<usize> = None;

                            if differences == 1 {
                                if let Some(((row, _), _)) = left
                                    .chars()
                                    .enumerate()
                                    .zip(right.chars())
                                    .find(|(a, b)| a.1 != *b)
                                {
                                    index_row = Some(row);
                                }
                            }

                            if left != right
                                && (differences != 1
                                    || difference_spotted.is_some()
                                        && difference_spotted.unwrap()
                                            != (index_row.unwrap(), index_column))
                            {
                                difference_spotted = None;
                                count = 0;
                                break;
                            }

                            if differences == 1 {
                                difference_spotted = Some((index_row.unwrap(), index_column));
                            }
                        }
                    }

                    count += 1;
                }

                (count, difference_spotted.is_some())
            })
            .filter(|result| result.0 != 0 && result.1)
            .collect();

        println!("Results Columns {results_columns:?}");
        println!("Results Rows {results_rows:?}");

        results_columns.iter().map(|result| result.0).sum::<u32>()
            + 100 * results_rows.iter().map(|result| result.0).sum::<u32>()
    }
}

fn parse(grid: &str) -> Grid {
    log(format!("Parsing grid\n{grid}").as_str());

    let (t_rows, r_rows) = std::sync::mpsc::channel();
    let (t_columns, r_columns) = std::sync::mpsc::channel();

    let grid_rows = String::from(grid);
    let grid_columns = String::from(grid);

    std::thread::spawn(move || {
        let rows: Vec<String> = grid_rows
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        t_rows.send(rows).unwrap();
    });

    std::thread::spawn(move || {
        let line_length = grid_columns.lines().next().unwrap().len();

        let mut columns: Vec<String> = Vec::default();

        for n in 0..line_length {
            let column: String = grid_columns
                .chars()
                .filter(|c| *c != '\n')
                .skip(n)
                .step_by(line_length)
                .collect();

            columns.push(column);
        }

        t_columns.send(columns).unwrap();
    });

    let rows = r_rows.recv().unwrap();
    let columns = r_columns.recv().unwrap();

    Grid { rows, columns }
}

fn summarise(lines: &str) -> u32 {
    lines
        .split("\n\n")
        .map(parse)
        .map(|grid| grid.summarise())
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", summarise(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = summarise(lines);

        assert_eq!(result, 400);
    }
}
