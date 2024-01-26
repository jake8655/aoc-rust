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
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn is_winning_number(&self, number: u32) -> bool {
        self.winning_numbers.contains(&number)
    }

    fn get_card_score(&self) -> u32 {
        let mut score = 0;

        for number in self.numbers.iter() {
            if self.is_winning_number(*number) {
                if score == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }
}

fn solve(input: &str) -> u32 {
    let mut cards = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split("| ");
        let mut first_part = parts.next().unwrap().split(": ");
        let second_part = parts.next().unwrap();

        let winning_numbers = first_part
            .nth(1)
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

        cards.push(Card::new(winning_numbers, numbers));
    });

    let score = cards.iter().map(|card| card.get_card_score()).sum::<u32>();

    score
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

        assert_eq!(output, 13);
    }
}
