use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Directed;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};

fn find_loop(graph: &Graph<(i32, i32), &str, Directed>, start: NodeIndex) -> Vec<NodeIndex> {
    let mut dfs = Dfs::new(graph, start);
    let mut visited_nodes = Vec::new();

    while let Some(node) = dfs.next(graph) {
        visited_nodes.push(node);
    }
    visited_nodes
}

fn build_graph(
    input: &str,
) -> (
    Graph<(i32, i32), &str>,
    HashMap<(i32, i32), NodeIndex>,
    Vec<Vec<char>>,
    Option<NodeIndex>,
) {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut graph = Graph::<(i32, i32), &str, Directed>::new();
    let mut nodes = HashMap::<(i32, i32), NodeIndex>::new();
    let mut start_position: Option<NodeIndex> = None;

    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let node = graph.add_node((x as i32, y as i32));
            nodes.insert((x as i32, y as i32), node);

            if value == 'S' {
                start_position = Some(node);
            }
        }
    }

    for (y, row) in map.iter().enumerate() {
        let y = y as i32;
        for (x, &value) in row.iter().enumerate() {
            let x = x as i32;
            let current_node = nodes.get(&(x, y)).unwrap();
            let (others1, others2) = match value {
                '-' => (nodes.get(&(x + 1, y)), nodes.get(&(x - 1, y))),
                '|' => (nodes.get(&(x, y + 1)), nodes.get(&(x, y - 1))),
                'L' => (nodes.get(&(x, y - 1)), nodes.get(&(x + 1, y))),
                'J' => (nodes.get(&(x - 1, y)), nodes.get(&(x, y - 1))),
                '7' => (nodes.get(&(x - 1, y)), nodes.get(&(x, y + 1))),
                'F' => (nodes.get(&(x, y + 1)), nodes.get(&(x + 1, y))),
                _ => (None, None),
            };

            if let Some(other1) = others1 {
                graph.add_edge(*current_node, *other1, "1");
                if *other1 == start_position.unwrap() {
                    graph.add_edge(*other1, *current_node, "1");
                }
            }
            if let Some(other2) = others2 {
                graph.add_edge(*current_node, *other2, "2");
                if *other2 == start_position.unwrap() {
                    graph.add_edge(*other2, *current_node, "2");
                }
            }
        }
    }

    (graph, nodes, map, start_position)
}

fn display_loop(
    graph: Graph<(i32, i32), &str>,
    start_position: Option<NodeIndex>,
    visited_nodes: Vec<NodeIndex>,
    map: Vec<Vec<char>>,
) {
    let visited_coordinates: HashSet<_> = visited_nodes
        .iter()
        .map(|node_index| graph[*node_index])
        .collect();
    let start_coordinates = graph[start_position.unwrap()];

    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let value = match value {
                'F' => '┌',
                'J' => '┘',
                'L' => '└',
                '7' => '┐',
                _ => value,
            };
            if (x as i32, y as i32) == start_coordinates {
                print!("\x1b[1;34m{}\x1b[0m", value); // print in blue and bold if start position
            } else if visited_coordinates.contains(&(x as i32, y as i32)) {
                print!("\x1b[31m{}\x1b[0m", value); // print in red if visited
            } else {
                print!("{}", value)
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn shoelace_formula(positions: &Vec<(f64, f64)>) -> f64 {
    let mut a = 0.0;
    let mut b = 0.0;
    let len = positions.len();

    for x in 0..len {
        a += positions[x].1 * positions[(x + 1) % len].0;
        b += positions[x].0 * positions[(x + 1) % len].1;
    }

    (f64::from(a - b).abs() / 2.0) - (len as f64 / 2.0) + 1.0
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let (graph, _, map, start_position) = build_graph(input);

    // Formatting
    let visited_nodes = find_loop(&graph, start_position.unwrap());
    display_loop(graph, start_position, visited_nodes.clone(), map);

    (visited_nodes.clone().len() / 2) as i32
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let (mut graph, nodes, map, start_position) = build_graph(input);

    // Formatting
    let visited_nodes = find_loop(&graph, start_position.unwrap());

    display_loop(
        graph.clone(),
        start_position,
        visited_nodes.clone(),
        map.clone(),
    );

    let polygon: Vec<(f64, f64)> = visited_nodes
        .iter()
        .map(|&node| {
            let (x, y) = graph[node];

            (x as f64, y as f64)
        })
        .collect();

    shoelace_formula(&polygon).round() as i32
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(input), 4);
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part2_test() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);

        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(input), 8);

        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input), 4);
    }
}
