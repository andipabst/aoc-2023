use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;
use regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn read_nodes() -> (Cycle<Chars<'static>>, HashMap<String, Node>) {
    let mut lines = include_str!("../input/day8.txt").lines();
    let instructions = lines.next().unwrap().chars().cycle();
    // skip empty line
    lines.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    let nodes_regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    for line in lines {
        let (_, [from, left, right]) = nodes_regex.captures(line).unwrap().extract();

        if let Some(old_node) = nodes.insert(from.to_string(), Node { left: left.to_string(), right: right.to_string() }) {
            panic!("duplicate entry! {:?}", old_node)
        }
    }

    (instructions, nodes)
}

pub(crate) fn part_a() {
    let (instructions, nodes) = read_nodes();

    let mut node = nodes.get("AAA").unwrap();
    let mut steps = 0i64;

    for char in instructions {
        steps += 1;
        let next_node = match char {
            'L' => {
                node.left.clone()
            }
            'R' => {
                node.right.clone()
            }
            _ => { panic!("unknown instruction {char}") }
        };

        if next_node == "ZZZ" {
            println!("{steps}");
            break;
        } else {
            node = nodes.get(&next_node).unwrap()
        }
    }
}

pub(crate) fn part_b() {
    let (instructions, nodes) = read_nodes();

    let mut current_nodes = nodes.iter()
        .filter(|(key, val)| key.ends_with("A"))
        .map(|(key, val)| {
            println!("{}: {:?}", key, val);
            val
        })
        .collect::<Vec<&Node>>();

    let mut steps = 0i64;
    let mut steps_to_z = vec![];

    for char in instructions {
        steps += 1;
        let next_nodes = current_nodes.iter().map(|node| {
            match char {
                'L' => {
                    node.left.clone()
                }
                'R' => {
                    node.right.clone()
                }
                _ => { panic!("unknown instruction {char}") }
            }
        });

        for node in next_nodes.clone().filter(|next_node| next_node.ends_with("Z")) {
            println!("{} steps until {}", steps, node);
            steps_to_z.push(steps);
        }

        current_nodes = next_nodes
            .filter(|next_node| !next_node.ends_with("Z"))
            .map(|next_node| nodes.get(&next_node).unwrap())
            .collect();

        if current_nodes.is_empty() {
            break;
        }
    }

    let mut result = 1i64;
    for s in steps_to_z {
        result = lcm(result, s);
        println!("{result}");
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    let gcd_result = gcd(a, b);
    a / gcd_result * b
}