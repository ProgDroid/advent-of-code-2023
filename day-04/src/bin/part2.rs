use log::debug;

const DELIMITER_CARD_ID: &str = ":";
const DELIMITER_CARD: &str = "|";

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: Vec<u8>,
    owned_numbers: Vec<u8>,
}

impl Card {
    fn count_matching(&self) -> usize {
        self.owned_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn parse_card(line: &str) -> Option<Card> {
    log(format!("Parsing line {line}").as_str());

    let Some((card_id_string, cards_string)) = line.split_once(DELIMITER_CARD_ID) else {
        return None;
    };

    let card_id = if let Some(id) = card_id_string.split_whitespace().last() {
        if let Ok(id) = id.parse::<usize>() {
            id
        } else {
            log("Could not parse card ID");
            return None;
        }
    } else {
        log("Card ID string empty");
        return None;
    };

    let Some((winning_card, owned_card)) = cards_string.split_once(DELIMITER_CARD) else {
        log("Could not split card string");
        return None;
    };

    let winning_numbers: Vec<u8> = winning_card
        .split_whitespace()
        .filter_map(|number| number.parse::<u8>().ok())
        .collect();

    let owned_numbers: Vec<u8> = owned_card
        .split_whitespace()
        .filter_map(|number| number.parse::<u8>().ok())
        .collect();

    Some(Card {
        id: card_id,
        winning_numbers,
        owned_numbers,
    })
}

fn count_total_cards(original_lines: Vec<&str>, lines_to_count: &[&str]) -> usize {
    let mut copied_lines: Vec<&str> = Vec::default();

    let mut total_lines: usize = 0;

    for line in lines_to_count {
        total_lines += 1;

        let Some(card) = parse_card(line) else {
            continue;
        };

        let matching_numbers = card.count_matching();

        for n in 0..matching_numbers {
            if let Some(line) = original_lines.get(card.id + n) {
                copied_lines.push(line);
            }
        }
    }

    if !copied_lines.is_empty() {
        total_lines += count_total_cards(original_lines, &copied_lines);
    }

    total_lines
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    let lines: Vec<&str> = input.lines().collect();

    println!("{}", count_total_cards(input.lines().collect(), &lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<&str> = input.lines().collect();

        let result = count_total_cards(input.lines().collect(), &lines);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_parse_card() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let result = parse_card(line);

        let expected = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            owned_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(expected, result);
    }
}
