use std::collections::HashMap;

const CARD_ORDER: &str = "J23456789TQKA";

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

#[derive(Debug, Hash, PartialEq, Eq)]
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
        let mut card_count = HashMap::new();

        for card in cards {
            *card_count.entry(card).or_insert(0) += 1;
        }

        match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    if card_count.contains_key(&Card { value: 'J' }) {
                        // Four of a kind upgraded to five of a kind because
                        // of the joker.
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else if card_count.contains_key(&Card { value: 'J' }) {
                    // Full house upgraded to five of a kind because of the
                    // joker.
                    HandType::FiveOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    if card_count.contains_key(&Card { value: 'J' }) {
                        // Three of a kind upgraded to four of a kind because
                        // of the joker.
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    card_count
                        .get(&Card { value: 'J' })
                        .map_or(HandType::TwoPair, |&count| {
                            if count == 2 {
                                // Two pair upgraded to four of a kind because
                                // of the joker.
                                HandType::FourOfAKind
                            } else {
                                // Two pair upgraded to full house because of
                                // the joker.
                                HandType::FullHouse
                            }
                        })
                }
            }
            4 => {
                if card_count.contains_key(&Card { value: 'J' }) {
                    // One pair upgraded to three of a kind because of the
                    // joker.
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if card_count.contains_key(&Card { value: 'J' }) {
                    // High card upgraded to one pair because of the joker.
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => unreachable!("Impossible hand: {:?}", cards),
        }
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

        assert_eq!(output, 5905);
    }
}
