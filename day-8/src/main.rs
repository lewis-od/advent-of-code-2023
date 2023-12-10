use std::collections::HashMap;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let file = fs::read_to_string(file_name).unwrap();
    let mut lines = file.lines();
    let directions = lines.next().unwrap().chars().cycle();
    lines.next(); // Skip whitespace
    let nodes: HashMap<String, Node> = lines
        .map(|line| Node::parse(line))
        .map(|node| ((&node.label).clone(), node))
        .collect();

    let mut current = nodes.get(Node::START).unwrap();
    let mut num_steps = 0u64;
    for direction in directions {
        let next_label = match direction {
            'L' => &current.left,
            'R' => &current.right,
            _ => panic!("Unknown direction: {direction}"),
        };
        current = nodes.get(next_label).unwrap();
        num_steps += 1;
        if current.label == Node::END {
            break;
        }
    }

    println!("Part 1: {num_steps}");
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    const START: &'static str = "AAA";
    const END: &'static str = "ZZZ";

    fn parse(row: &str) -> Node {
        let parts: Vec<&str> = row.split(" = ").collect();
        let label = parts[0].trim();
        let forks = parts[1].trim();
        let left = &forks[1..=3];
        let right = &forks[6..=8];
        Node {
            label: label.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_node() {
        let row = "BBB = (AAA, ZZZ)";

        let node = Node::parse(row);

        let expected = Node {
            label: "BBB".into(),
            left: "AAA".into(),
            right: "ZZZ".into(),
        };
        assert_eq!(expected, node);
    }
}
