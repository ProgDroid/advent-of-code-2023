use std::collections::HashSet;

use log::debug;

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn new_directions(c: char, direction: Direction) -> Vec<Direction> {
    log(format!(
        "New Direction for char {c} and direction {direction:?}"
    ));

    let mut directions = Vec::default();

    match c {
        '\\' => match direction {
            Direction::North => directions.push(Direction::West),
            Direction::West => directions.push(Direction::North),
            Direction::South => directions.push(Direction::East),
            Direction::East => directions.push(Direction::South),
        },
        '/' => match direction {
            Direction::North => directions.push(Direction::East),
            Direction::East => directions.push(Direction::North),
            Direction::South => directions.push(Direction::West),
            Direction::West => directions.push(Direction::South),
        },
        '|' => match direction {
            Direction::North | Direction::South => directions.push(direction),
            Direction::East | Direction::West => {
                directions.push(Direction::North);
                directions.push(Direction::South);
            }
        },
        '-' => match direction {
            Direction::East | Direction::West => directions.push(direction),
            Direction::North | Direction::South => {
                directions.push(Direction::East);
                directions.push(Direction::West);
            }
        },
        _ => directions.push(direction),
    }

    directions
}

fn trace_beam(
    already_seen: &mut HashSet<(usize, usize, Direction)>,
    grid: &Vec<Vec<char>>,
    starting_position: (usize, usize),
    direction: Direction,
) -> HashSet<(usize, usize)> {
    log(format!(
        "Tracing beam at {starting_position:?} and direction {direction:?}"
    ));

    if already_seen.contains(&(starting_position.0, starting_position.1, direction)) {
        return HashSet::default();
    }

    already_seen.insert((starting_position.0, starting_position.1, direction));

    let mut energised_tiles: HashSet<(usize, usize)> = HashSet::default();

    energised_tiles.insert(starting_position);

    let mut next_directions: Vec<Direction> = vec![direction];

    let grid_width = grid.first().unwrap().len();
    let grid_height = grid.len();

    let mut current_position = starting_position;

    while next_directions == vec![direction] {
        log(format!("Current Position {current_position:?}"));

        if current_position.0 == 0 && direction == Direction::West
            || current_position.0 == grid_width - 1 && direction == Direction::East
            || current_position.1 == 0 && direction == Direction::North
            || current_position.1 == grid_height - 1 && direction == Direction::South
        {
            break;
        }

        let new_position = match direction {
            Direction::North => (current_position.0, current_position.1 - 1),
            Direction::South => (current_position.0, current_position.1 + 1),
            Direction::East => (current_position.0 + 1, current_position.1),
            Direction::West => (current_position.0 - 1, current_position.1),
        };

        log(format!("New Position {new_position:?}"));

        let row = grid.get(new_position.1);

        if row.is_none() {
            break;
        }

        let row = row.unwrap();

        let char_at_new_position = row.get(new_position.0);

        if char_at_new_position.is_none() {
            break;
        }

        let char_at_new_position = char_at_new_position.unwrap();

        current_position = new_position;
        energised_tiles.insert(new_position);
        next_directions = new_directions(*char_at_new_position, direction);

        log(format!("Next Directions {next_directions:?}"));

        if next_directions.len() > 1 {
            break;
        }
    }

    if !next_directions.is_empty() {
        for next_direction in next_directions {
            energised_tiles.extend(trace_beam(
                already_seen,
                grid,
                current_position,
                next_direction,
            ));
        }
    }

    log(format!("Energised Tiles {energised_tiles:?}"));

    energised_tiles
}

fn count_energised_tiles(input: &str) -> usize {
    let mut energised_tiles: HashSet<(usize, usize)> = HashSet::default();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut already_seen: HashSet<(usize, usize, Direction)> = HashSet::default();

    let mut starting_position = (0, 0);
    let mut starting_direction = Direction::East;

    if let Some(c) = grid.first().unwrap().first() {
        if *c != '.' {
            let new_directions = new_directions(*c, starting_direction);

            if new_directions != vec![Direction::East] {
                energised_tiles.insert((0, 0));
                starting_direction = Direction::South;
                starting_position = (0, 1);
            }
        }
    }

    energised_tiles.extend(trace_beam(
        &mut already_seen,
        &grid,
        starting_position,
        starting_direction,
    ));

    energised_tiles.len()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", count_energised_tiles(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let result = count_energised_tiles(lines);

        assert_eq!(result, 46);
    }
}
