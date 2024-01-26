fn main() {
    let input = include_str!("./input");
    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let ouput = solve(input);

    dbg!(ouput);
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    instances: u32,
}

impl Card {
    fn new(id: u32, winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
            instances: 1,
        }
    }

    fn is_winning_number(&self, number: u32) -> bool {
        self.winning_numbers.contains(&number)
    }

    fn get_winning_amount(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.is_winning_number(**n))
            .count() as u32
    }
}

fn solve(input: &str) -> u32 {
    let mut cards = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split("| ");
        let mut first_part = parts.next().unwrap().split(": ");
        let second_part = parts.next().unwrap();

        let id = first_part
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Failed to parse card id"));

        let winning_numbers = first_part
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Failed to parse winning number: {}", n))
            })
            .collect::<Vec<u32>>();

        let numbers = second_part
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Failed to parse number: {}", n))
            })
            .collect::<Vec<u32>>();

        cards.push(Card::new(id, winning_numbers, numbers));
    });

    for i in 0..cards.len() {
        let card = &cards[i];
        let winning_amount = card.get_winning_amount();

        for id in card.id..card.id + winning_amount {
            cards[id as usize].instances += cards[i].instances;
        }
    }

    let output = cards.iter().fold(0, |mut acc, card| {
        acc += card.instances;
        acc
    });
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let output = solve(input);

        assert_eq!(output, 30);
    }
}
