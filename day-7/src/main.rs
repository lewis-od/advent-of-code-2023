use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let file = fs::read_to_string(file_name).unwrap();

    let mut hands: Vec<Hand> = file.lines().map(|line| Hand::parse(line)).collect();
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| ((rank as u32) + 1) * hand.bet)
        .sum();
    println!("Winnings: {winnings}")
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: u32,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for (ours, theirs) in self.cards.iter().zip(other.cards.iter()) {
            let comparison = ours.cmp(theirs);
            if comparison != Ordering::Equal {
                return comparison;
            }
        }
        Ordering::Equal
    }
}

impl Hand {
    fn parse(row: &str) -> Hand {
        let mut parts = row.split_ascii_whitespace();
        let cards = parts.next().unwrap();
        let bet = parts.next().unwrap();

        let cards: Vec<Card> = cards
            .chars()
            .map(|character| Card::parse(character))
            .collect();
        let hand_type = Hand::get_type(&cards);
        let bet: u32 = bet.parse().unwrap();
        Hand {
            cards,
            hand_type,
            bet,
        }
    }

    fn get_type(cards: &Vec<Card>) -> HandType {
        let mut counts: HashMap<Card, u32> = HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(*card).or_insert(0);
            *count += 1
        }

        if counts.contains_key(&Card::Joker) && *counts.get(&Card::Joker).unwrap() != 5 {
            let (most_common_card, max_count) = counts
                .iter()
                .filter(|(card, _)| *card != &Card::Joker)
                .max_by_key(|(_card, count)| *count)
                .unwrap_or_else(|| {
                    let first_non_joker = counts
                        .keys()
                        .filter(|card| *card != &Card::Joker)
                        .next()
                        .unwrap();
                    (first_non_joker, &counts[first_non_joker])
                });
            counts.insert(*most_common_card, max_count + counts[&Card::Joker]);
        }

        Hand::get_type_from_counts(counts)
    }

    fn get_type_from_counts(counts: HashMap<Card, u32>) -> HandType {
        if counts.values().any(|count| *count == 5) {
            HandType::FiveOfaKind
        } else if counts.values().any(|count| *count == 4) {
            HandType::FourOfAKind
        } else if counts.values().any(|count| *count == 3) {
            if counts.values().any(|count| *count == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else {
            let num_pairs = counts.values().filter(|count| **count == 2).count();
            if num_pairs == 2 {
                HandType::TwoPair
            } else if num_pairs == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Card {
    Joker,
    Number(u32),
    Picture(Royal),
    Ace,
}

impl Card {
    const KING: Card = Card::Picture(Royal::King);
    const QUEEN: Card = Card::Picture(Royal::Queen);

    fn parse(character: char) -> Card {
        match character {
            'K' => Card::KING,
            'Q' => Card::QUEEN,
            'J' => Card::Joker,
            'T' => Card::Number(10),
            'A' => Card::Ace,
            _ => Card::Number(character.to_digit(10).unwrap()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Royal {
    Queen,
    King,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfaKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Card::Number;

    #[test]
    fn should_order_number_cards() {
        let mut cards = vec![Number(3), Number(8), Number(5)];

        cards.sort();

        let expected = vec![Number(3), Number(5), Number(8)];
        assert_eq!(expected, cards);
    }

    #[test]
    fn should_order_number_and_picture_cards() {
        let mut cards = vec![Number(3), Card::KING, Number(8), Card::QUEEN, Number(5)];

        cards.sort();

        let expected = vec![Number(3), Number(5), Number(8), Card::QUEEN, Card::KING];
        assert_eq!(expected, cards);
    }

    #[test]
    fn should_order_hands_by_type_then_cards() {
        let one = Hand {
            cards: vec![Number(3), Number(2), Number(10), Number(3), Card::KING],
            hand_type: HandType::OnePair,
            bet: 0,
        };
        let two = Hand {
            cards: vec![Card::KING, Number(10), Card::QUEEN, Card::QUEEN, Number(10)],
            hand_type: HandType::TwoPair,
            bet: 0,
        };
        let three = Hand {
            cards: vec![Card::KING, Card::KING, Number(6), Number(7), Number(7)],
            hand_type: HandType::TwoPair,
            bet: 0,
        };
        let mut cards = vec![one.clone(), three.clone(), two.clone()];

        cards.sort();

        let expected = vec![one, two, three];
        assert_eq!(expected, cards);
    }

    #[test]
    fn should_order_hands_by_type_then_cards_with_jokers() {
        let one = Hand {
            cards: vec![Number(3), Number(2), Number(10), Number(3), Card::KING],
            hand_type: HandType::OnePair,
            bet: 0,
        };
        let two = Hand {
            cards: vec![Card::KING, Card::Joker, Number(10), Card::Joker, Number(10)],
            hand_type: HandType::FourOfAKind,
            bet: 0,
        };
        let three = Hand {
            cards: vec![Card::KING, Card::Joker, Card::KING, Card::Joker, Number(10)],
            hand_type: HandType::FourOfAKind,
            bet: 0,
        };
        let four = Hand {
            cards: vec![Card::KING, Number(2), Number(2), Number(2), Number(2)],
            hand_type: HandType::FourOfAKind,
            bet: 0,
        };
        let mut cards = vec![three.clone(), one.clone(), four.clone(), two.clone()];

        cards.sort();

        let expected = vec![one, two, three, four];
        assert_eq!(expected, cards);
    }

    #[test]
    fn should_calculate_five_of_a_kind() {
        let cards = vec![Number(3), Number(3), Number(3), Number(3), Number(3)];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::FiveOfaKind, hand_type);
    }

    #[test]
    fn should_calculate_four_of_a_kind() {
        let cards = vec![Number(3), Number(2), Number(3), Number(3), Number(3)];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::FourOfAKind, hand_type);
    }

    #[test]
    fn should_calculate_full_house() {
        let cards = vec![Number(3), Card::KING, Number(3), Number(3), Card::KING];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::FullHouse, hand_type);
    }

    #[test]
    fn should_calculate_three_of_a_kind() {
        let cards = vec![Number(3), Card::KING, Number(3), Number(3), Card::QUEEN];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::ThreeOfAKind, hand_type);
    }

    #[test]
    fn should_calculate_two_pair() {
        let cards = vec![Number(3), Card::KING, Card::QUEEN, Number(3), Card::QUEEN];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::TwoPair, hand_type);
    }

    #[test]
    fn should_calculate_one_pair() {
        let cards = vec![Number(3), Card::KING, Card::QUEEN, Number(3), Number(8)];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::OnePair, hand_type);
    }

    #[test]
    fn should_calculate_high_card() {
        let cards = vec![Number(3), Card::KING, Card::QUEEN, Number(9), Number(8)];

        let hand_type = Hand::get_type(&cards);

        assert_eq!(HandType::HighCard, hand_type);
    }

    #[test]
    fn should_parse_hand() {
        let row = "T55Q5 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(10), Number(5), Number(5), Card::QUEEN, Number(5)],
            hand_type: HandType::ThreeOfAKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_with_joker_four_of_a_kind() {
        let row = "T55J5 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(10), Number(5), Number(5), Card::Joker, Number(5)],
            hand_type: HandType::FourOfAKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_with_joker_one_pair() {
        let row = "T58J2 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(10), Number(5), Number(8), Card::Joker, Number(2)],
            hand_type: HandType::OnePair,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_with_joker_three_of_a_kind() {
        let row = "TT8J2 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(10), Number(10), Number(8), Card::Joker, Number(2)],
            hand_type: HandType::ThreeOfAKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_with_joker_five_of_a_kind() {
        let row = "TTTJT 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(10), Number(10), Number(10), Card::Joker, Number(10)],
            hand_type: HandType::FiveOfaKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_with_joker_full_house() {
        let row = "88J55 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Number(8), Number(8), Card::Joker, Number(5), Number(5)],
            hand_type: HandType::FullHouse,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_of_all_jokers() {
        let row = "JJJJJ 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker,
            ],
            hand_type: HandType::FiveOfaKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_of_four_jokers() {
        let row = "JJ5JJ 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![
                Card::Joker,
                Card::Joker,
                Number(5),
                Card::Joker,
                Card::Joker,
            ],
            hand_type: HandType::FiveOfaKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }

    #[test]
    fn should_parse_hand_of_three_jokers() {
        let row = "JJ5J4 684";

        let hand = Hand::parse(row);

        let expected_hand = Hand {
            cards: vec![Card::Joker, Card::Joker, Number(5), Card::Joker, Number(4)],
            hand_type: HandType::FourOfAKind,
            bet: 684,
        };
        assert_eq!(expected_hand, hand);
    }
}
