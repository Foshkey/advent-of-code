use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

type CamelCards = BTreeMap<(HandType, u32), u16>;

fn get_total_winnings(input: &str, joker: bool) -> u64 {
    let mut hands = CamelCards::new();
    input
        .lines()
        .for_each(|l| insert_hand(l, &mut hands, joker));

    // hands should be ordered from weakest to strongest now,
    // simply loop through while increasing rank
    let mut total: u64 = 0;
    let mut rank: u64 = 1;
    let bids = hands.values();
    for &bid in bids {
        total += rank * bid as u64;
        rank += 1;
    }
    total
}

fn insert_hand(line: &str, hands: &mut CamelCards, joker: bool) {
    let Some((hand, bid_str)) = line.split_once(' ') else {
        panic!("Could not read line {}", line)
    };

    let hand_type = if joker { get_type_joker(hand) } else { get_type(hand) };
    let hand_value = get_value(hand, joker);
    let bid = bid_str.parse::<u16>().unwrap();

    hands.insert((hand_type, hand_value), bid);
}

fn get_value(hand: &str, joker: bool) -> u32 {
    // This converts the hand into a unique value using base 15
    let mapping = &[
        ('T', 'A'),
        ('J', if joker { '1' } else { 'B' }),
        ('Q', 'C'),
        ('K', 'D'),
        ('A', 'E'),
    ];
    let hex_str: String = hand
        .chars()
        .map(|c| {
            mapping
                .iter()
                .find(|&&(from, _)| from == c)
                .map_or(c, |&(_, to)| to)
        })
        .collect();
    u32::from_str_radix(&hex_str, 15).unwrap()
}

fn get_joker_mutations(hand: &str) -> [String; 12] {
    // Create mutations of the hand, replacing the joker card with each of the other cards.
    // Note that we don't need to permeate, considering jokers of the same "value" will always
    // be higher than jokers of different values. E.g. 23JJ2 would equate to 23222 as the highest
    // rather than 23322.
    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    cards
        .map(|card| {
            hand.chars()
                .map(|c| if c == 'J' { card } else { c })
                .collect::<String>()
        })
}

fn get_type_joker(hand: &str) -> HandType {
    get_joker_mutations(hand)
        .iter()
        .fold(HandType::HighCard, |highest_type, mutation| {
            let mut_type = get_type(mutation);
            if mut_type > highest_type { mut_type } else { highest_type}
        })
}

fn get_type(hand: &str) -> HandType {
    let mut matches_map: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        // If we already counted this, then skip
        if matches_map.contains_key(&c) {
            continue;
        }

        // Count matches and insert it into the map
        let matching = hand.chars().filter(|&ch| c == ch).count();
        matches_map.insert(c, matching);
    }

    let matches: Vec<&usize> = matches_map.values().collect();

    if matches.contains(&&5usize) {
        return HandType::FiveOfAKind;
    }

    if matches.contains(&&4usize) {
        return HandType::FourOfAKind;
    }

    if matches.contains(&&2usize) && matches.contains(&&3usize) {
        return HandType::FullHouse;
    }

    if matches.contains(&&3usize) {
        return HandType::ThreeOfAKind;
    }

    if matches.iter().filter(|&&&x| x == 2usize).count() >= 2 {
        return HandType::TwoPair;
    }

    if matches.contains(&&2usize) {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    println!("Part 1: {}", get_total_winnings(input, false));
    println!("Part 2: {}", get_total_winnings(input, true));
    println!("Executed in {} μs", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        assert_eq!(6440, get_total_winnings(input, false));
    }

    #[test]
    fn test_example_joker() {
        let input = include_str!("example.txt");
        assert_eq!(5905, get_total_winnings(input, true));
    }

    #[test]
    fn test_get_type() {
        assert_eq!(HandType::FiveOfAKind, get_type("AAAAA"));
        assert_eq!(HandType::FourOfAKind, get_type("AA8AA"));
        assert_eq!(HandType::FullHouse, get_type("23332"));
        assert_eq!(HandType::ThreeOfAKind, get_type("TTT98"));
        assert_eq!(HandType::TwoPair, get_type("23432"));
        assert_eq!(HandType::OnePair, get_type("A23A4"));
        assert_eq!(HandType::HighCard, get_type("23456"));
    }

    #[test]
    fn test_get_type_joker() {
        assert_eq!(HandType::FiveOfAKind, get_type_joker("AAJAA"));
        assert_eq!(HandType::FourOfAKind, get_type_joker("AA8JA"));
        assert_eq!(HandType::FullHouse, get_type_joker("233J2"));
        assert_eq!(HandType::FourOfAKind, get_type_joker("233JJ"));
        assert_eq!(HandType::FourOfAKind, get_type_joker("2J3J2"));
        assert_eq!(HandType::FourOfAKind, get_type_joker("2333J"));
        assert_eq!(HandType::ThreeOfAKind, get_type_joker("TJT98"));
        assert_eq!(HandType::ThreeOfAKind, get_type_joker("2J432"));
        assert_eq!(HandType::ThreeOfAKind, get_type_joker("2J43J"));
        assert_eq!(HandType::ThreeOfAKind, get_type_joker("AA1J2"));
        assert_eq!(HandType::TwoPair, get_type_joker("A2A24"));
        assert_eq!(HandType::OnePair, get_type_joker("A23J4"));
        assert_eq!(HandType::HighCard, get_type_joker("23456"));
    }
}
