use std::collections::HashMap;

use lazy_regex::regex_captures;
use num::integer::lcm;

pub fn solve(input: Vec<String>) {
    let (instructions, nodes) = parse(input);
    println!(
        "Day 8, part one: {}",
        part_one(instructions.clone(), nodes.clone())
    );
    println!("Day 8, part two: {}", part_two(instructions, nodes));
}

fn part_one(instructions: Vec<Instruction>, nodes: HashMap<Node, (Node, Node)>) -> usize {
    let first_node: Node = String::from("AAA");
    let last_node: Node = String::from("ZZZ");

    let mut next_node = first_node;
    let mut counter = 0usize;
    //
    //todo get value from inst and use that as the tuple output.
    for i in instructions.iter().cycle() {
        counter += 1;
        let i = *i as usize;
        let node = nodes.get(&next_node).expect("next node");
        next_node = match i {
            0 => node.0.clone(),
            1 => node.1.clone(),
            _ => unreachable!(),
        };
        if next_node == last_node {
            break;
        }
    }
    counter
}

// So, at first I tried to brute-force the solution (thinking rust would do it).
// Then, I luckily thought about the test input and how the two starting points (11A, 22A)
// had their steps (2,3) converge as their LCM, and that worked.
// After thinking about it and looking it up, I now see that there were several other
// assumptions made:
//  - each XXA only reaches one XXZ
//  - same amount of steps from XXA to XXZ than XXZ to XXZ again.
//  - (probably more that I'm not seeing?)
// So... yeah, bit of a lucky guess that I'm still not waiting for the brute-force method.
fn part_two(instructions: Vec<Instruction>, nodes: HashMap<Node, (Node, Node)>) -> usize {
    let starting_nodes: Vec<Node> = nodes
        .clone()
        .into_iter()
        .filter(|(k, (_, _))| k.is_starting())
        .map(|(node, (_, _))| node)
        .collect();

    let mut ending_steps: Vec<usize> = Vec::new();

    //todo get value from inst and use that as the tuple output.
    'outer: for n in starting_nodes {
        let mut n_counter = 0;
        let mut next_step = n.clone();
        for i in instructions.iter().cycle() {
            n_counter += 1;
            let i = *i as usize;
            let node = nodes.get(&next_step).expect("next node");
            let node = match i {
                0 => node.0.clone(),
                1 => node.1.clone(),
                _ => unreachable!(),
            };

            if node.is_ending() {
                ending_steps.push(n_counter);
                continue 'outer;
            }
            next_step = node;
        }
    }

    ending_steps.into_iter().fold(1, lcm)
}

fn parse(input: Vec<String>) -> (Vec<Instruction>, HashMap<Node, (Node, Node)>) {
    let (raw_instructions, raw_nodes) = input
        .split_first()
        .map(|(f, r)| (f.to_owned(), r.to_vec().split_off(1)))
        .expect("valid input");

    let instructions = raw_instructions
        .chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => unreachable!(),
        })
        .collect::<Vec<Instruction>>();

    let nodes = raw_nodes.iter().fold(HashMap::new(), |mut acc, curr| {
        let (_, node_a, node_b, node_c) =
            regex_captures!(r#"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)"#, curr)
                .expect("valid node line");
        let (node_a, node_b, node_c): (Node, Node, Node) =
            (node_a.into(), node_b.into(), node_c.into());
        acc.entry(node_a).or_insert((node_b, node_c));
        acc
    });

    (instructions, nodes)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

type Node = String;

trait NodeTraits {
    fn is_starting(&self) -> bool;
    fn is_ending(&self) -> bool;
}

impl NodeTraits for Node {
    fn is_starting(&self) -> bool {
        self.chars().last().unwrap() == 'A'
    }

    fn is_ending(&self) -> bool {
        self.chars().last().unwrap() == 'Z'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_8_part_one() {
        let input = [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let input2 = [
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let (instructions, nodes) = parse(input);
        let (instructions2, nodes2) = parse(input2);
        assert_eq!(part_one(instructions, nodes), 2);
        assert_eq!(part_one(instructions2, nodes2), 6);
    }

    #[test]
    fn test_day_8_part_two() {
        let input = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let (instructions, nodes) = parse(input);
        assert_eq!(part_two(instructions, nodes), 6);
    }
}
