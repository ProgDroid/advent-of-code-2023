use log::debug;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Step {
    x: usize,
    y: usize,
    direction: Direction,
    heat_loss: usize,
    steps: usize,
}

#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn string(&self) -> &str {
        match self {
            Self::North => "n",
            Self::West => "w",
            Self::South => "s",
            Self::East => "e",
        }
    }
}

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn cache_key(step: Step) -> String {
    format!(
        "{}_{}_{}_{}",
        step.x,
        step.y,
        step.direction.string(),
        step.steps
    )
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn compute_least_heat_loss(grid: &Vec<Vec<usize>>, min_steps: usize, max_steps: usize) -> usize {
    let grid_height = grid.len();
    let grid_width = grid.get(0).unwrap().len();

    let start_east = Step {
        x: 0,
        y: 0,
        direction: Direction::East,
        heat_loss: 0,
        steps: 0,
    };

    let start_south = Step {
        x: 0,
        y: 0,
        direction: Direction::South,
        heat_loss: 0,
        steps: 0,
    };

    let mut priority_queue = PriorityQueue::new();

    priority_queue.push(start_east, Reverse(0));
    priority_queue.push(start_south, Reverse(0));

    let mut visited: HashSet<String> = HashSet::new();

    visited.insert(cache_key(start_east));
    visited.insert(cache_key(start_south));

    while !priority_queue.is_empty() {
        let current = priority_queue.pop().unwrap().0;

        log(format!("Popped: {current:?}"));

        if current.x == (grid_width - 1) && current.y == (grid_height - 1) {
            if current.steps < min_steps {
                continue;
            }

            return current.heat_loss;
        }

        let mut next_steps: Vec<(isize, isize, Direction)> = Vec::new();

        match current.direction {
            Direction::North => {
                next_steps.push((current.x as isize, current.y as isize - 1, Direction::North));
                next_steps.push((current.x as isize - 1, current.y as isize, Direction::West));
                next_steps.push((current.x as isize + 1, current.y as isize, Direction::East));
            }
            Direction::West => {
                next_steps.push((current.x as isize - 1, current.y as isize, Direction::West));
                next_steps.push((current.x as isize, current.y as isize + 1, Direction::South));
                next_steps.push((current.x as isize, current.y as isize - 1, Direction::North));
            }
            Direction::South => {
                next_steps.push((current.x as isize, current.y as isize + 1, Direction::South));
                next_steps.push((current.x as isize - 1, current.y as isize, Direction::West));
                next_steps.push((current.x as isize + 1, current.y as isize, Direction::East));
            }
            Direction::East => {
                next_steps.push((current.x as isize + 1, current.y as isize, Direction::East));
                next_steps.push((current.x as isize, current.y as isize + 1, Direction::South));
                next_steps.push((current.x as isize, current.y as isize - 1, Direction::North));
            }
        };

        for next_step in next_steps {
            if next_step.0 < 0
                || next_step.1 < 0
                || next_step.0 >= grid_width as isize
                || next_step.1 >= grid_height as isize
            {
                continue;
            }

            if current.steps >= max_steps && current.direction == next_step.2 {
                continue;
            }

            if current.steps < min_steps && current.direction != next_step.2 {
                continue;
            }

            let step = Step {
                x: next_step.0 as usize,
                y: next_step.1 as usize,
                direction: next_step.2,
                heat_loss: current.heat_loss
                    + grid
                        .get(next_step.1 as usize)
                        .unwrap()
                        .get(next_step.0 as usize)
                        .unwrap(),
                steps: if current.direction == next_step.2 {
                    current.steps + 1
                } else {
                    1
                },
            };

            let key = cache_key(step);

            if !visited.contains(&key) {
                visited.insert(key);
                priority_queue.push(step, Reverse(step.heat_loss));
            }
        }
    }

    0
}

fn compute_path(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|number| number.to_string().parse::<usize>().ok())
                .collect()
        })
        .collect();

    let result = compute_least_heat_loss(&grid, 4, 10);

    log(format!("Result: {result}"));

    result
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", compute_path(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let result = compute_path(lines);

        assert_eq!(result, 94);
    }

    #[test]
    fn test_new_example() {
        let lines = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let result = compute_path(lines);

        assert_eq!(result, 71);
    }
}
