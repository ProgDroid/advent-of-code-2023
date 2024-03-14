use log::debug;

struct DiggingInstruction {
    direction: Direction,
    count: isize,
}

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const fn apply(self, instruction: &DiggingInstruction) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        x += match instruction.direction {
            Direction::Up | Direction::Down => 0,
            Direction::Left => -instruction.count,
            Direction::Right => instruction.count,
        };

        y += match instruction.direction {
            Direction::Left | Direction::Right => 0,
            Direction::Up => instruction.count,
            Direction::Down => -instruction.count,
        };

        Self { x, y }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_str(value: &str) -> Self {
        match value {
            "0" => Self::Right,
            "1" => Self::Down,
            "2" => Self::Left,
            _ => Self::Up, // "U"
        }
    }
}

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn shoelace(trenches: &[Point], perimeter: isize) -> usize {
    let length = trenches.len();

    let mut sum = 0;

    for i in 0..length - 1 {
        let current = trenches.get(i).unwrap();
        let next = trenches.get(i + 1).unwrap();

        sum += (current.x * next.y) - (current.y * next.x);
    }

    let last = trenches.last().unwrap();
    let first = trenches.first().unwrap();

    sum += (last.x * first.y) - (first.x * last.y);

    sum.unsigned_abs() / 2 + usize::try_from(perimeter / 2).unwrap() + 1
}

fn parse_instruction(instruction: &str) -> DiggingInstruction {
    let (_, count_and_colour_string) = instruction.split_once(' ').unwrap();

    let (_, hex) = count_and_colour_string.split_once(' ').unwrap();

    let hex = hex.replace('#', "");
    let hex = hex.replace('(', "");
    let hex = hex.replace(')', "");

    let (meters, direction) = hex.split_at(5);

    DiggingInstruction {
        direction: Direction::from_str(direction),
        count: isize::from_str_radix(meters, 16).unwrap(),
    }
}

fn calculate_area(input: &str) -> usize {
    let instructions: Vec<DiggingInstruction> = input.lines().map(parse_instruction).collect();

    let initial = Point { x: 0, y: 0 };
    let mut current = initial;

    let mut trenches: Vec<Point> = Vec::default();

    let mut perimeter = 0;

    for instruction in instructions {
        let new = current.apply(&instruction);
        trenches.push(new);
        current = new;

        perimeter += instruction.count;
    }

    let result = shoelace(&trenches, perimeter);

    log(format!("Result: {result}"));

    result
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", calculate_area(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let result = calculate_area(lines);

        assert_eq!(result, 952_408_144_115);
    }
}
