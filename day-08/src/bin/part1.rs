fn main() {
    let input = include_str!("./input");
    //     let input = "RL
    //
    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn make_move(&self, m: &Move) -> &str {
        match m {
            Move::Left => self.left,
            Move::Right => self.right,
        }
    }
}

enum Move {
    Left,
    Right,
}

fn solve(input: &str) -> u32 {
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

        nodes.push(Node { name, left, right });
    }

    let mut current_node = nodes.iter().find(|n| n.name == "AAA").unwrap();
    let mut count = 0;

    while current_node.name != "ZZZ" {
        current_node = nodes
            .iter()
            .find(|n| n.name == current_node.make_move(&move_sequence[count % move_sequence.len()]))
            .unwrap();
        count += 1;
    }

    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let output = solve(input);

        assert_eq!(output, 2);
    }
}
