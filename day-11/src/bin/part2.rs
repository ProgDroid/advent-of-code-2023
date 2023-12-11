use log::debug;

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn sum_galaxy_pair_distances(lines: &str, expansion_rate: usize) -> usize {
    let expansion_rate = expansion_rate - 1;

    let Some((first_line, _)) = lines.split_once('\n') else {
        return 0;
    };

    let mut galaxies: Vec<(usize, usize)> = Vec::default();

    let mut row_offset = 0;

    let original_line_length = first_line.len();

    for (index_row, line) in lines.lines().enumerate() {
        log(format!("Parsing line {line}").as_str());

        let galaxy_columns: Vec<usize> = line.match_indices('#').map(|(index, _)| index).collect();

        if galaxy_columns.is_empty() {
            log("Expanding vertically");

            row_offset += expansion_rate;
            continue;
        }

        for index_column in galaxy_columns {
            galaxies.push((index_row + row_offset, index_column));
        }
    }

    log(format!("Vertically Expanded {galaxies:?}").as_str());

    let mut column_offset = 0;

    for n in 0..original_line_length {
        let to_check = n + column_offset;

        if !galaxies.iter().any(|galaxy| galaxy.1 == to_check) {
            column_offset += expansion_rate;

            log("Expanding horizontally");

            galaxies = galaxies
                .iter()
                .map(|galaxy| {
                    if galaxy.1 > to_check {
                        (galaxy.0, galaxy.1 + expansion_rate)
                    } else {
                        *galaxy
                    }
                })
                .collect();
        }
    }

    log(format!("Horizontally Expanded {galaxies:?}").as_str());

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, galaxy)| {
            let distances: Vec<usize> = galaxies
                .iter()
                .skip(index + 1)
                .map(|other| {
                    (if other.0 > galaxy.0 {
                        other.0 - galaxy.0
                    } else {
                        galaxy.0 - other.0
                    }) + (if other.1 > galaxy.1 {
                        other.1 - galaxy.1
                    } else {
                        galaxy.1 - other.1
                    })
                })
                .collect();

            log(format!("Distances {distances:?}").as_str());

            distances
        })
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", sum_galaxy_pair_distances(input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = sum_galaxy_pair_distances(lines, 2);

        assert_eq!(result, 374);

        let lines = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = sum_galaxy_pair_distances(lines, 10);

        assert_eq!(result, 1030);

        let lines = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = sum_galaxy_pair_distances(lines, 100);

        assert_eq!(result, 8410);
    }
}
