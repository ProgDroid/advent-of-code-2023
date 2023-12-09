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

#[derive(Debug, Clone)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn is_starting(&self) -> bool {
        self.value.ends_with('A')
    }

    fn is_ending(&self) -> bool {
        self.value.ends_with('Z')
    }
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

fn calculate_lcm(lengths: &[usize]) -> usize {
    if lengths.len() == 1 {
        return *lengths.first().unwrap();
    }

    return num::integer::lcm(*lengths.first().unwrap(), calculate_lcm(&lengths[1..]));
}

fn compute_path_steps(lines: &str) -> usize {
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

    let mut current_nodes: Vec<Node> = nodes
        .clone()
        .into_iter()
        .filter(Node::is_starting)
        .collect();

    log(format!("Starting nodes {current_nodes:?}").as_str());

    let mut current_nodes_length = current_nodes.len();

    let mut path_lengths: Vec<usize> = Vec::default();

    'outer: loop {
        for instruction in &instructions {
            let ending_nodes: Vec<Node> = current_nodes
                .clone()
                .into_iter()
                .filter(Node::is_ending)
                .collect();

            for _ in &ending_nodes {
                path_lengths.push(steps);
            }

            if ending_nodes.len() == current_nodes_length {
                break 'outer;
            }

            current_nodes.retain(|current_node| {
                !ending_nodes
                    .iter()
                    .any(|ending_node| ending_node.value == current_node.value)
            });

            current_nodes_length = current_nodes.len();

            steps += 1;

            let mut new_nodes: Vec<Node> = Vec::default();

            for current_node in &current_nodes {
                let comparison_value = match instruction {
                    Instruction::Left => &current_node.left,
                    Instruction::Right => &current_node.right,
                };

                let new_node = nodes
                    .iter()
                    .find(|node| node.value == *comparison_value)
                    .unwrap();

                new_nodes.push(new_node.clone());
            }

            current_nodes = new_nodes;
        }
    }

    calculate_lcm(&path_lengths)
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
        let lines = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let result = compute_path_steps(lines);

        assert_eq!(result, 6);
    }
}
