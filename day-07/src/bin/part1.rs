use std::cmp::Ordering;

use log::debug;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Copy)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardValue {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::Ten),
            'J' => Some(Self::Jack),
            'Q' => Some(Self::Queen),
            'K' => Some(Self::King),
            'A' => Some(Self::Ace),
            _ => None,
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Copy)]
struct Card {
    value: CardValue,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for n in 0..self.cards.len() {
                let self_card = self.cards.get(n).unwrap();
                let other_card = other.cards.get(n).unwrap();

                if self_card == other_card {
                    continue;
                }

                if self_card < other_card {
                    return Ordering::Less;
                }

                if self_card > other_card {
                    return Ordering::Greater;
                }
            }

            return Ordering::Equal;
        }

        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

#[derive(Debug)]
struct CardCount {
    card: CardValue,
    count: usize,
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut found_cards: Vec<CardCount> = vec![
        CardCount {
            card: CardValue::Two,
            count: 0,
        },
        CardCount {
            card: CardValue::Three,
            count: 0,
        },
        CardCount {
            card: CardValue::Four,
            count: 0,
        },
        CardCount {
            card: CardValue::Five,
            count: 0,
        },
        CardCount {
            card: CardValue::Six,
            count: 0,
        },
        CardCount {
            card: CardValue::Seven,
            count: 0,
        },
        CardCount {
            card: CardValue::Eight,
            count: 0,
        },
        CardCount {
            card: CardValue::Nine,
            count: 0,
        },
        CardCount {
            card: CardValue::Ten,
            count: 0,
        },
        CardCount {
            card: CardValue::Jack,
            count: 0,
        },
        CardCount {
            card: CardValue::Queen,
            count: 0,
        },
        CardCount {
            card: CardValue::King,
            count: 0,
        },
        CardCount {
            card: CardValue::Ace,
            count: 0,
        },
    ];

    #[allow(clippy::needless_range_loop)]
    for n in 0..found_cards.len() {
        found_cards[n].count += cards
            .iter()
            .filter(|card| card.value == found_cards[n].card)
            .count();
    }

    let mut count_fives = 0;
    let mut count_fours = 0;
    let mut count_threes = 0;
    let mut count_twos = 0;

    for found in &found_cards {
        match found.count {
            2 => count_twos += 1,
            3 => count_threes += 1,
            4 => count_fours += 1,
            5 => count_fives += 1,
            _ => {}
        }
    }

    if count_fives > 0 {
        return HandType::FiveOfAKind;
    }

    if count_fours > 0 {
        return HandType::FourOfAKind;
    }

    if count_threes > 0 {
        if count_twos > 0 {
            return HandType::FullHouse;
        }

        return HandType::ThreeOfAKind;
    }

    if count_twos > 0 {
        if count_twos == 1 {
            return HandType::OnePair;
        }

        return HandType::TwoPair;
    }

    HandType::HighCard
}

fn parse_hand(line: &str) -> Option<Hand> {
    log(format!("Parsing line {line}").as_str());

    let Some((hand_string, bid_string)) = line.split_once(' ') else {
        return None;
    };

    let cards: Vec<Card> = hand_string
        .chars()
        .map(|c| Card {
            value: CardValue::from_char(c).unwrap(),
        })
        .collect();

    if cards.len() != 5 {
        return None;
    }

    let Ok(bid) = bid_string.parse::<u32>() else {
        return None;
    };

    let hand_type = get_hand_type(&cards);

    let hand = Hand {
        cards,
        hand_type,
        bid,
    };

    log(format!("Got hand {hand:?}").as_str());

    Some(hand)
}

fn calculate_winnings<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    let mut hands: Vec<Hand> = lines.filter_map(parse_hand).collect();

    log(format!("{hands:?}").as_str());

    hands.sort();

    log(format!("Sorted Hands:\n{hands:?}").as_str());

    let mut value = 0;

    for (index, hand) in hands.iter().enumerate() {
        let index_value: u32 = index.try_into().unwrap();

        value += (index_value + 1) * hand.bid;
    }

    value
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", calculate_winnings(input.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = calculate_winnings(lines.lines());

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_card_value_order() {
        assert!(CardValue::Two < CardValue::Three);
    }

    #[test]
    fn test_card_order() {
        assert!(
            Card {
                value: CardValue::Jack
            } < Card {
                value: CardValue::King
            }
        );
    }

    #[test]
    fn test_hand_type_order() {
        assert!(HandType::FourOfAKind < HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_order() {
        assert!(
            Hand {
                cards: vec![
                    Card {
                        value: CardValue::Jack
                    },
                    Card {
                        value: CardValue::Jack
                    },
                    Card {
                        value: CardValue::Jack
                    },
                    Card {
                        value: CardValue::Jack
                    },
                    Card {
                        value: CardValue::Jack
                    }
                ],
                hand_type: HandType::FourOfAKind,
                bid: 100
            } < Hand {
                cards: vec![
                    Card {
                        value: CardValue::King
                    },
                    Card {
                        value: CardValue::King
                    },
                    Card {
                        value: CardValue::Queen
                    },
                    Card {
                        value: CardValue::Jack
                    },
                    Card {
                        value: CardValue::Jack
                    }
                ],
                hand_type: HandType::FiveOfAKind,
                bid: 50
            }
        );
    }
}
