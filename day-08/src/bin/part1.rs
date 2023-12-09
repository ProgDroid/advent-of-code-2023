use log::debug;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn parse_node(line: &str) -> Option<Node> {
    log(format!("Parsing {line}").as_str());

    let Some((node_value_string, node_edges_string)) = line.split_once('=') else {
        return None;
    };

    let node_value = node_value_string.trim();

    let node_edges_string_processed = node_edges_string.trim().replace(['(', ')', ','], "");

    let Some((left_edge, right_edge)) = node_edges_string_processed.split_once(' ') else {
        return None;
    };

    Some(Node {
        value: node_value.to_string(),
        left: left_edge.to_string(),
        right: right_edge.to_string(),
    })
}

fn compute_path_steps(lines: &str) -> u32 {
    log("Parsing Input");

    let mut nodes: Vec<Node> = Vec::default();

    for line in lines.lines().skip(2) {
        if let Some(node) = parse_node(line) {
            nodes.push(node);
        };
    }

    log(format!("Parsed nodes {nodes:?}").as_str());

    let instructions: Vec<Instruction> = lines
        .lines()
        .take(1)
        .flat_map(|line| line.chars().filter_map(Instruction::from_char))
        .collect();

    log(format!("Parsed instructions {instructions:?}").as_str());

    let mut steps = 0;
    let mut current_node = nodes.iter().position(|node| node.value == "AAA").unwrap();
    let final_node = nodes.iter().position(|node| node.value == "ZZZ").unwrap();

    while current_node != final_node {
        for instruction in &instructions {
            if current_node == final_node {
                break;
            }

            steps += 1;

            let node = nodes.get(current_node).unwrap();
            let next_node_value = match instruction {
                Instruction::Left => &node.left,
                Instruction::Right => &node.right,
            };

            current_node = nodes
                .iter()
                .position(|node| node.value == *next_node_value)
                .unwrap();
        }
    }

    steps
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", compute_path_steps(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let result = compute_path_steps(lines);

        assert_eq!(result, 6);
    }
}
