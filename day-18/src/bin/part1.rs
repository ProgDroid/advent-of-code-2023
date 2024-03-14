use log::debug;

struct DiggingInstruction {
    direction: Direction,
    count: i32,
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
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
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => Self::Up, // "U"
        }
    }
}

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn shoelace(trenches: &[Point], perimeter: i32) -> u32 {
    let length = trenches.len();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..length - 1 {
        let current = trenches.get(i).unwrap();
        let next = trenches.get(i + 1).unwrap();

        sum1 += current.x * next.y;
        sum2 += current.y * next.x;
    }

    let last = trenches.last().unwrap();
    let first = trenches.first().unwrap();

    sum1 += last.x * first.y;
    sum2 += first.x * last.y;

    (sum1 - sum2).unsigned_abs() / 2 + u32::try_from(perimeter / 2).unwrap() + 1
}

fn parse_instruction(instruction: &str) -> DiggingInstruction {
    let (direction_string, count_and_colour_string) = instruction.split_once(' ').unwrap();

    let (count, _) = count_and_colour_string.split_once(' ').unwrap();

    DiggingInstruction {
        direction: Direction::from_str(direction_string),
        count: count.parse::<i32>().unwrap(),
    }
}

fn calculate_area(input: &str) -> u32 {
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

        assert_eq!(result, 62);
    }
}
