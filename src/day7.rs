use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid_str) = s.trim().split_once(" ").ok_or(ParseHandError)?;

        let bid = bid_str.parse::<i32>().map_err(|_| ParseHandError)?;

        Ok(Hand { cards: cards.to_string(), bid })
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn get_type(&self) -> Option<Type> {
        let mut letters = HashMap::new();

        for card in self.cards.chars() {
            letters.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }

        match letters.len() {
            1 => { Some(Type::FiveOfAKind) }
            2 => {
                if letters.values().all(|count| count == &4 || count == &1) {
                    Some(Type::FourOfAKind)
                } else if letters.values().all(|count| count == &3 || count == &2) {
                    Some(Type::FullHouse)
                } else {
                    None
                }
            }
            3 => {
                let mut letter_counts: Vec<&i32> = letters.values().collect();
                letter_counts.sort();

                if [1, 2, 2].iter().zip(&letter_counts).all(|(a, b)| &a == b) {
                    Some(Type::TwoPair)
                } else if [1, 1, 3].iter().zip(&letter_counts).all(|(a, b)| &a == b) {
                    Some(Type::ThreeOfAKind)
                } else {
                    None
                }
            }
            4 => { Some(Type::OnePair) }
            5 => { Some(Type::HighCard) }
            _ => { None }
        }
    }
    fn get_type_b(&self) -> Option<Type> {
        let mut letters = HashMap::new();

        for card in self.cards.chars() {
            letters.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }

        match letters.len() {
            1 => { Some(Type::FiveOfAKind) }
            2 => {
                if letters.keys().any(|key| key == &'J') {
                    Some(Type::FiveOfAKind)
                } else if letters.values().all(|count| count == &4 || count == &1) {
                    Some(Type::FourOfAKind)
                } else if letters.values().all(|count| count == &3 || count == &2) {
                    Some(Type::FullHouse)
                } else {
                    None
                }
            }
            3 => {
                let mut letter_counts: Vec<&i32> = letters.values().collect();
                letter_counts.sort();

                if [1, 2, 2].iter().zip(&letter_counts).all(|(a, b)| &a == b) {
                    if letters.iter().any(|(key, value)| key == &'J' && value == &1) {
                        Some(Type::FullHouse)
                    } else if letters.iter().any(|(key, value)| key == &'J' && value == &2) {
                        Some(Type::FourOfAKind)
                    } else {
                        Some(Type::TwoPair)
                    }
                } else if [1, 1, 3].iter().zip(&letter_counts).all(|(a, b)| &a == b) {
                    if letters.keys().any(|key| key == &'J') {
                        Some(Type::FourOfAKind)
                    } else {
                        Some(Type::ThreeOfAKind)
                    }
                } else {
                    None
                }
            }
            4 => {
                if letters.keys().any(|key| key == &'J') {
                    Some(Type::ThreeOfAKind)
                } else {
                    Some(Type::OnePair)
                }
            }
            5 => {
                if letters.keys().any(|key| key == &'J') {
                    Some(Type::OnePair)
                } else {
                    Some(Type::HighCard)
                }
            }
            _ => { None }
        }
    }

    fn get_cards_values(&self) -> Vec<i16> {
        self.cards.chars().map(|c| {
            match c {
                'A' => 1,
                'K' => 2,
                'Q' => 3,
                'J' => 4,
                'T' => 5,
                '9' => 6,
                '8' => 7,
                '7' => 8,
                '6' => 9,
                '5' => 10,
                '4' => 11,
                '3' => 12,
                '2' => 13,
                _ => { panic!("unknown card") }
            }
        }).collect()
    }
    fn get_cards_values_b(&self) -> Vec<i16> {
        self.cards.chars().map(|c| {
            match c {
                'A' => 1,
                'K' => 2,
                'Q' => 3,
                'T' => 5,
                '9' => 6,
                '8' => 7,
                '7' => 8,
                '6' => 9,
                '5' => 10,
                '4' => 11,
                '3' => 12,
                '2' => 13,
                'J' => 14,
                _ => { panic!("unknown card") }
            }
        }).collect()
    }
}


pub(crate) fn part_a() {
    let mut hands: Vec<Hand> = include_str!("../input/day7.txt")
        .lines()
        .map(|l| Hand::from_str(l).unwrap())
        .collect();

    hands.sort_by(|a, b|
        a.get_type().unwrap().cmp(&b.get_type().unwrap())
            .then(a.get_cards_values().cmp(&b.get_cards_values()))
    );
    hands.reverse();

    let mut winnings = 0i64;
    for (i, hand) in hands.iter().enumerate() {
        println!("{}: {:?} {:?}", i + 1, hand, hand.get_type());
        winnings += (i + 1) as i64 * hand.bid as i64;
    }

    println!("\n\n{winnings}")
}

pub(crate) fn part_b() {
    let mut hands: Vec<Hand> = include_str!("../input/day7.txt")
        .lines()
        .map(|l| Hand::from_str(l).unwrap())
        .collect();

    hands.sort_by(|a, b|
        a.get_type_b().unwrap().cmp(&b.get_type_b().unwrap())
            .then(a.get_cards_values_b().cmp(&b.get_cards_values_b()))
    );
    hands.reverse();

    let mut winnings = 0i64;
    for (i, hand) in hands.iter().enumerate() {
        println!("{}: {:?} {:?} ({:?})", i + 1, hand, hand.get_type_b(), hand.get_type());
        winnings += (i + 1) as i64 * hand.bid as i64;
    }

    println!("\n\n{winnings}")
}