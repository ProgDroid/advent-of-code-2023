use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

struct Grid {
    rows: Vec<u32>,
    columns: Vec<u32>,
}

impl Grid {
    fn summarise(&self) -> u32 {
        let possible_middle_pairs_rows: Vec<(usize, usize)> = self
            .rows
            .iter()
            .enumerate()
            .zip(self.rows.iter().enumerate().skip(1))
            .filter(|(a, b)| a.1 == b.1)
            .map(|(a, b)| (a.0, b.0))
            .collect();

        let results_rows: Vec<u32> = possible_middle_pairs_rows
            .iter()
            .map(|(end, start)| {
                let mut count = 0;

                for n in 0..=*end {
                    if let Some(left) = self.rows.get(end - n) {
                        if let Some(right) = self.rows.get(start + n) {
                            if left != right {
                                count = 0;
                                break;
                            }
                        }
                    }

                    count += 1;
                }

                count
            })
            .filter(|result| *result != 0)
            .collect();

        let possible_middle_pairs_columns: Vec<(usize, usize)> = self
            .columns
            .iter()
            .enumerate()
            .zip(self.columns.iter().enumerate().skip(1))
            .filter(|(a, b)| a.1 == b.1)
            .map(|(a, b)| (a.0, b.0))
            .collect();

        let results_columns: Vec<u32> = possible_middle_pairs_columns
            .iter()
            .map(|(end, start)| {
                let mut count = 0;

                for n in 0..=*end {
                    if let Some(left) = self.columns.get(end - n) {
                        if let Some(right) = self.columns.get(start + n) {
                            if left != right {
                                count = 0;
                                break;
                            }
                        }
                    }

                    count += 1;
                }

                count
            })
            .filter(|result| *result != 0)
            .collect();

        results_columns.iter().sum::<u32>() + 100 * results_rows.iter().sum::<u32>()
    }
}

fn parse(grid: &str) -> Grid {
    log(format!("Parsing grid\n{grid}").as_str());

    let (t_rows, r_rows) = std::sync::mpsc::channel();
    let (t_columns, r_columns) = std::sync::mpsc::channel();

    let grid_rows = String::from(grid);
    let grid_columns = String::from(grid);

    std::thread::spawn(move || {
        let rows: Vec<u32> = grid_rows
            .lines()
            .map(|string| {
                u32::from_str_radix(&string.replace('.', "0").replace('#', "1"), 2).unwrap()
            })
            .collect();

        t_rows.send(rows).unwrap();
    });

    std::thread::spawn(move || {
        let line_length = grid_columns.lines().next().unwrap().len();

        let mut columns: Vec<u32> = Vec::default();

        for n in 0..line_length {
            let column: String = grid_columns
                .chars()
                .filter(|c| *c != '\n')
                .skip(n)
                .step_by(line_length)
                .collect();

            columns
                .push(u32::from_str_radix(&column.replace('.', "0").replace('#', "1"), 2).unwrap());
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

        assert_eq!(result, 405);
    }
}
