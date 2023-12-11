use itertools::Itertools;
use std::{cmp::Ordering, fmt::Debug};

struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn score_first(&self, other: &Hand) -> Ordering {
        for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }

    fn score_part_1(&self) -> HandType {
        let mut counts = [0; 13];
        for &c in &self.cards {
            counts[13 - c as usize] += 1;
        }

        if counts.iter().any(|&c| c == 5) {
            HandType::FiveOfAKind
        } else if counts.iter().any(|&c| c == 4) {
            HandType::FourOfAKind
        } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            HandType::FullHouse
        } else if counts.iter().any(|&c| c == 3) {
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|&&c| c == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.iter().any(|&c| c == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn score_part_2(&self) -> HandType {
        let mut counts = [0; 13];
        for &c in &self.cards {
            counts[13 - c as usize] += 1;
        }

        let jokers = counts[12];
        let counts = counts[0..12]
            .iter()
            .copied()
            .filter(|x| *x != 0)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        if counts.len() <= 1 || counts[0] + jokers == 5 {
            HandType::FiveOfAKind
        } else if counts[0] + jokers == 4 {
            HandType::FourOfAKind
        } else if ((counts[0] + jokers == 3) && (counts[1] == 2))
            || ((counts[0] == 3) && (counts[1] + jokers == 2))
        {
            HandType::FullHouse
        } else if counts[0] + jokers == 3 {
            HandType::ThreeOfAKind
        } else if (counts[0] + jokers == 2 && counts[1] == 2)
            || (counts[0] == 2 && counts[1] + jokers == 2)
        {
            HandType::TwoPair
        } else if counts[0] + jokers == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

fn parse(input: &str, mappings: &'static str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_at(5);

        let cards = cards
            .as_bytes()
            .iter()
            .map(|&c| 13 - (mappings.find(c as char).unwrap() as u8))
            .collect();

        let bid = bid.trim().parse().unwrap();

        hands.push(Hand { cards, bid });
    }
    hands
}

fn solve(mut hands: Vec<Hand>, score: fn(&Hand) -> HandType) -> usize {
    hands.sort_by(|a, b| score(a).cmp(&score(b)).then_with(|| b.score_first(a)));

    let solved = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, h)| h.bid as usize * (i + 1))
        .sum::<usize>();

    solved
}

fn part_1() {
    let input = include_str!("my_input.txt");

    let hands = parse(input, "AKQJT98765432");
    let solved = solve(hands, Hand::score_part_1);

    println!("Part 1: {}", solved);
}

fn part_2() {
    let input = include_str!("my_input.txt");

    let hands = parse(input, "AKQT98765432J");
    let solved = solve(hands, Hand::score_part_2);

    println!("Part 2: {}", solved);
}

fn main() {
    part_1();
    part_2();
}
