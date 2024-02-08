use std::fmt;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use num_integer::Integer;

fn main() {
    // let input = include_str!("./input");
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node(u8, u8, u8);

impl Node {
    fn ends_with_a(&self) -> bool {
        self.2 == b'A'
    }

    fn ends_with_z(&self) -> bool {
        self.2 == b'Z'
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0 as char, self.1 as char, self.2 as char)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Left,
    Right,
}

#[derive(Debug)]
struct Instructions(Box<[Move]>);

impl Instructions {
    fn new(moves: Vec<Move>) -> Self {
        Instructions(moves.into())
    }

    fn cycle(&self) -> impl Iterator<Item = Move> + '_ {
        self.0.iter().copied().cycle()
    }
}

#[derive(Debug)]
struct DesertMap {
    instructions: Instructions,
    map: HashMap<Node, (Node, Node)>,
}

impl DesertMap {
    fn steps(&self, start: Node) -> u64 {
        let mut steps = 0;
        let mut current = start;
        let mut instructions = self.instructions.cycle();

        while !current.ends_with_z() {
            let next = match instructions.next().unwrap() {
                Move::Left => self.map[&current].0,
                Move::Right => self.map[&current].1,
            };

            current = next;
            steps += 1;
        }

        steps
    }
}

fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let move_sequence_line = lines.next().unwrap();
    let move_sequence: Vec<Move> = move_sequence_line
        .chars()
        .map(|c| match c {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Invalid move"),
        })
        .collect();

    let instructions = Instructions::new(move_sequence);

    lines.next(); // skip empty line

    let mut nodes = Vec::new();

    for line in lines {
        let mut parts = line.split(" = ");
        let name = parts.next().unwrap();
        let mut children = parts.next().unwrap();
        children = &children[1..children.len() - 1];

        let mut children = children.split(", ");
        let left = children.next().unwrap();
        let right = children.next().unwrap();

        nodes.push((name, left, right));
    }

    let mut map = HashMap::new();

    for (name, left, right) in nodes {
        let name = name.as_bytes();
        let left = left.as_bytes();
        let right = right.as_bytes();

        let node = Node(name[0], name[1], name[2]);
        let left = Node(left[0], left[1], left[2]);
        let right = Node(right[0], right[1], right[2]);

        map.insert(node, (left, right));
    }

    let desert_map = DesertMap { instructions, map };

    let a = desert_map
        .map
        .keys()
        .copied()
        .filter(|n| n.ends_with_a())
        .map(|n| desert_map.steps(n))
        .collect::<Vec<_>>();

    dbg!(&a);
    a.into_iter().fold(1, |acc, steps| acc.lcm(&steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let output = solve(input);

        assert_eq!(output, 6);
    }
}
