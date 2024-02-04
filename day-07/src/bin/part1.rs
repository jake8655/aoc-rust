const CARD_ORDER: &str = "23456789TJQKA";

fn main() {
    let input = include_str!("./input");
    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn strength(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }

    fn cmp(&self, other: &HandType) -> std::cmp::Ordering {
        self.strength().cmp(&other.strength())
    }
}

#[derive(Debug)]
struct Card {
    value: char,
}

impl Card {
    fn cmp(&self, other: &Card) -> std::cmp::Ordering {
        CARD_ORDER
            .find(self.value)
            .cmp(&CARD_ORDER.find(other.value))
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Hand {
        let hand_type = Hand::get_hand_type(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn get_hand_type(cards: &[Card; 5]) -> HandType {
        let mut hand_type = HandType::HighCard;

        let mut counts = [0; 13];

        for card in cards.iter() {
            counts[CARD_ORDER.find(card.value).unwrap()] += 1;
        }

        let mut pairs = 0;

        for count in counts {
            match count {
                5 => {
                    hand_type = HandType::FiveOfAKind;
                    break;
                }
                4 => {
                    hand_type = HandType::FourOfAKind;
                    break;
                }
                3 => {
                    hand_type = HandType::ThreeOfAKind;
                }
                2 => {
                    pairs += 1;
                }
                _ => {}
            }
        }

        if pairs == 2 {
            hand_type = HandType::TwoPair;
        } else if pairs == 1 {
            if hand_type == HandType::ThreeOfAKind {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::OnePair;
            }
        }

        hand_type
    }
}

fn order_hands(hands: &mut [Hand]) {
    hands.sort_by(|a, b| {
        if a.hand_type != b.hand_type {
            a.hand_type.cmp(&b.hand_type)
        } else {
            let inequal_card = a
                .cards
                .iter()
                .zip(b.cards.iter())
                .find(|(a, b)| a.value != b.value)
                .unwrap();

            inequal_card.0.cmp(inequal_card.1)
        }
    });
}

fn solve(input: &str) -> u32 {
    let lines = input.lines();

    let mut hands: Vec<Hand> = lines
        .into_iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cards: Vec<Card> = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| Card { value: c })
                .collect();
            let bid = parts.next().unwrap().parse().unwrap();

            Hand::new(cards.try_into().unwrap(), bid)
        })
        .collect();

    order_hands(&mut hands);

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output = solve(input);

        assert_eq!(output, 6440);
    }
}
