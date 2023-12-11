use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> HashMap<String, Node> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let nodes: Vec<Node> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            let lr = parts[1].replace('(', "").replace(')', "");
            let lr: Vec<&str> = lr.split(", ").collect();
            Node {
                name: parts[0].to_string(),
                left: lr[0].to_string(),
                right: lr[1].to_string(),
            }
        })
        .collect();

    let mut map: HashMap<String, Node> = HashMap::new();
    for node in nodes {
        map.insert(node.name.clone(), node);
    }
    map
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let instructions = lines[0];
    let nodes = parse_input(&lines[1..].join("\n"));
    let mut counter: i32 = 0;
    let mut current_node: &Node = nodes.get("AAA").unwrap();

    for instruction in instructions.chars().cycle() {
        // make it a cycle
        // check if we reached ZZZ
        if current_node.name == "ZZZ" {
            break;
        }

        match instruction {
            'L' => {
                current_node = nodes.get(&current_node.left).unwrap();
            }
            'R' => {
                current_node = nodes.get(&current_node.right).unwrap();
            }
            _ => (),
        }

        counter += 1;
    }

    counter
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let instructions = lines[0];
    let nodes = parse_input(&lines[1..].join("\n"));

    let start_nodes: Vec<Node> = nodes
        .values()
        .filter(|node| node.name.ends_with("A"))
        .cloned()
        .collect();

    start_nodes
        .iter()
        .map(|start_node| {
            let mut counter: i64 = 0;
            let mut current_node = start_node;

            for instruction in instructions.chars().cycle() {
                let next_node_name = match instruction {
                    'L' => &current_node.left,
                    'R' => &current_node.right,
                    _ => continue,
                };
                match nodes.get(next_node_name) {
                    Some(next_node) => current_node = next_node,
                    None => break,
                }
                counter += 1;

                if current_node.name.ends_with("Z") {
                    break;
                }
            }
            counter
        })
        .into_iter()
        .reduce(|val, acc| lcm(val, acc))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part2_test() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part2(input), 6);
    }
}
