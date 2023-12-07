use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    fn cmp(&self, other: &Self, part2: bool) -> Ordering {
        if part2 {
            match (self, other) {
                (Card::CJ, Card::CJ) => Ordering::Equal,
                (Card::CJ, _) => Ordering::Less,
                (_, Card::CJ) => Ordering::Greater,
                _ => Ord::cmp(self, other),
            }
        } else {
            Ord::cmp(self, other)
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::CT,
            'J' => Card::CJ,
            'Q' => Card::CQ,
            'K' => Card::CK,
            'A' => Card::CA,
            _ => panic!("Unknown Card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand(Card, Card, Card, Card, Card);

impl Hand {
    fn get_type_part1(&self) -> Type {
        let mut counts = HashMap::new();
        for &card in &[self.0, self.1, self.2, self.3, self.4] {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.values().collect_vec();
        counts.sort();

        match counts[..] {
            [5] => Type::FiveOfAKind,
            [1, 4] => Type::FourOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 1, 3] => Type::ThreeOfAKind,
            [1, 2, 2] => Type::TwoPair,
            [1, 1, 1, 2] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => panic!("Invalid Hand"),
        }
    }

    fn get_type_part2(&self) -> Type {
        let mut counts = HashMap::new();
        let mut jokers = 0;

        for &card in &[self.0, self.1, self.2, self.3, self.4] {
            if card == Card::CJ {
                jokers += 1;
            } else {
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        let mut counts = counts.values().copied().collect_vec();
        counts.sort();
        if counts.is_empty() {
            counts = vec![jokers];
        } else {
            *counts.last_mut().unwrap() += jokers;
        }

        if counts.iter().any(|i| *i == 5) {
            Type::FiveOfAKind
        } else if counts.iter().any(|i| *i == 4) {
            Type::FourOfAKind
        } else if counts.iter().any(|i| *i == 3) {
            if counts.iter().any(|i| *i == 2) {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        } else if counts.iter().any(|i| *i == 2) {
            if counts.iter().filter(|i| **i == 1).count() == 1 {
                Type::TwoPair
            } else {
                Type::OnePair
            }
        } else {
            Type::HighCard
        }
    }

    fn cmp(&self, other: &Self, part2: bool) -> Ordering {
        self.0
            .cmp(&other.0, part2)
            .then(self.1.cmp(&other.1, part2))
            .then(self.2.cmp(&other.2, part2))
            .then(self.3.cmp(&other.3, part2))
            .then(self.4.cmp(&other.4, part2))
    }
}

impl From<(Card, Card, Card, Card, Card)> for Hand {
    fn from(value: (Card, Card, Card, Card, Card)) -> Self {
        Self(value.0, value.1, value.2, value.3, value.4)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round(Hand, u32);

impl Round {
    fn cmp(&self, other: &Self, part2: bool) -> Ordering {
        if part2 {
            self.0.get_type_part2()
        } else {
            self.0.get_type_part1()
        }
        .cmp(&if part2 {
            other.0.get_type_part2()
        } else {
            other.0.get_type_part1()
        })
        .then(self.0.cmp(&other.0, part2))
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand
                .chars()
                .map(|c| c.into())
                .collect_tuple::<(Card, Card, Card, Card, Card)>()
                .unwrap()
                .into();
            Round(hand, bid.trim().parse().unwrap())
        })
        .collect_vec()
}

#[aoc(day7, part1)]
fn part1(input: &[Round]) -> u32 {
    let mut input = input.to_vec();
    input.sort_by(|a, b| a.cmp(b, false));
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, round)| acc + (rank as u32 + 1) * round.1)
}

#[aoc(day7, part2)]
fn part2(input: &[Round]) -> u32 {
    let mut input = input.to_vec();
    input.sort_by(|a, b| a.cmp(b, true));
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, round)| acc + (rank as u32 + 1) * round.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(
            parse(input),
            vec![
                Round(Hand(Card::C3, Card::C2, Card::CT, Card::C3, Card::CK), 765),
                Round(Hand(Card::CT, Card::C5, Card::C5, Card::CJ, Card::C5), 684),
                Round(Hand(Card::CK, Card::CK, Card::C6, Card::C7, Card::C7), 28),
                Round(Hand(Card::CK, Card::CT, Card::CJ, Card::CJ, Card::CT), 220),
                Round(Hand(Card::CQ, Card::CQ, Card::CQ, Card::CJ, Card::CA), 483),
            ]
        )
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(part1(&parse(input)), 6440);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(part2(&parse(input)), 5905);
    }
}
