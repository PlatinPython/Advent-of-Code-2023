use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardPart1 {
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

impl From<char> for CardPart1 {
    fn from(value: char) -> Self {
        match value {
            '2' => CardPart1::C2,
            '3' => CardPart1::C3,
            '4' => CardPart1::C4,
            '5' => CardPart1::C5,
            '6' => CardPart1::C6,
            '7' => CardPart1::C7,
            '8' => CardPart1::C8,
            '9' => CardPart1::C9,
            'T' => CardPart1::CT,
            'J' => CardPart1::CJ,
            'Q' => CardPart1::CQ,
            'K' => CardPart1::CK,
            'A' => CardPart1::CA,
            _ => panic!("Unknown Card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardPart2 {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl CardPart2 {
    fn values() -> [CardPart2; 12] {
        [
            CardPart2::C2,
            CardPart2::C3,
            CardPart2::C4,
            CardPart2::C5,
            CardPart2::C6,
            CardPart2::C7,
            CardPart2::C8,
            CardPart2::C9,
            CardPart2::CT,
            CardPart2::CQ,
            CardPart2::CK,
            CardPart2::CA,
        ]
    }
}

impl From<char> for CardPart2 {
    fn from(value: char) -> Self {
        match value {
            'J' => CardPart2::CJ,
            '2' => CardPart2::C2,
            '3' => CardPart2::C3,
            '4' => CardPart2::C4,
            '5' => CardPart2::C5,
            '6' => CardPart2::C6,
            '7' => CardPart2::C7,
            '8' => CardPart2::C8,
            '9' => CardPart2::C9,
            'T' => CardPart2::CT,
            'Q' => CardPart2::CQ,
            'K' => CardPart2::CK,
            'A' => CardPart2::CA,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct HandPart1(CardPart1, CardPart1, CardPart1, CardPart1, CardPart1);

impl HandPart1 {
    fn get_type(&self) -> Type {
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
}

impl From<(CardPart1, CardPart1, CardPart1, CardPart1, CardPart1)> for HandPart1 {
    fn from(value: (CardPart1, CardPart1, CardPart1, CardPart1, CardPart1)) -> Self {
        Self(value.0, value.1, value.2, value.3, value.4)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct HandPart2(CardPart2, CardPart2, CardPart2, CardPart2, CardPart2);

impl HandPart2 {
    fn get_type(&self) -> Type {
        // let mut counts = HashMap::new();
        // let mut jokers = 0;
        //
        // for &card in &[self.0, self.1, self.2, self.3, self.4] {
        //     if card == CardPart2::CJ {
        //         jokers += 1;
        //     } else {
        //         *counts.entry(card).or_insert(0) += 1;
        //     }
        // }
        //
        // if jokers > 0 {
        //     let mut counts = counts.values().collect_vec();
        //     counts.sort();
        //     counts.reverse();
        //
        //     match counts[..] {
        //         [4] if jokers >= 1 => Type::FiveOfAKind,
        //         [3] if jokers >= 2 => Type::FiveOfAKind,
        //         [2] if jokers >= 3 => Type::FiveOfAKind,
        //         [1] if jokers >= 4 => Type::FiveOfAKind,
        //         [] if jokers >= 5 => Type::FiveOfAKind,
        //         [3, _] if jokers >= 1 => Type::FourOfAKind,
        //         [2, _] if jokers >= 2 => Type::FourOfAKind,
        //         [1, _] if jokers >= 3 => Type::FourOfAKind,
        //         [_] if jokers >= 4 => Type::FourOfAKind,
        //         [2, 2] if jokers >= 1 => Type::FullHouse,
        //         [2, 1] if jokers >= 2 => Type::FullHouse,
        //         [1, 1] if jokers >= 3 => Type::FullHouse,
        //         // [1] if jokers >= 4 => Type::FullHouse,
        //         // [] if jokers >= 5 => Type::FullHouse,
        //         [2, _] if jokers >= 1 => Type::ThreeOfAKind,
        //         [1, _] if jokers >= 2 => Type::ThreeOfAKind,
        //         [_] if jokers >= 3 => Type::ThreeOfAKind,
        //         [2, 1, _] if jokers >= 1 => Type::TwoPair,
        //         [1, 1, _] if jokers >= 2 => Type::TwoPair,
        //         [2, _] if jokers >= 2 => Type::TwoPair,
        //         [1, _] if jokers >= 3 => Type::TwoPair,
        //         [_] if jokers >= 4 => Type::TwoPair,
        //         [1, _] if jokers >= 1 => Type::OnePair,
        //         [_] if jokers >= 2 => Type::OnePair,
        //         _ => Type::HighCard,
        //     }
        // } else {
        //     let mut counts = counts.values().collect_vec();
        //     counts.sort();
        //
        //     match counts[..] {
        //         [5] => Type::FiveOfAKind,
        //         [1, 4] => Type::FourOfAKind,
        //         [2, 3] => Type::FullHouse,
        //         [1, 1, 3] => Type::ThreeOfAKind,
        //         [1, 2, 2] => Type::TwoPair,
        //         [1, 1, 1, 2] => Type::OnePair,
        //         [1, 1, 1, 1, 1] => Type::HighCard,
        //         _ => panic!("Invalid Hand"),
        //     }
        // }

        let mut hands = vec![*self];
        while hands.iter().any(|hand| {
            hand.0 == CardPart2::CJ
                || hand.1 == CardPart2::CJ
                || hand.2 == CardPart2::CJ
                || hand.3 == CardPart2::CJ
                || hand.4 == CardPart2::CJ
        }) {
            let mut new_hands = vec![];
            for hand in &hands {
                if hand.0 == CardPart2::CJ {
                    new_hands.append(
                        &mut CardPart2::values()
                            .map(|card| HandPart2(card, hand.1, hand.2, hand.3, hand.4))
                            .to_vec(),
                    )
                } else if hand.1 == CardPart2::CJ {
                    new_hands.append(
                        &mut CardPart2::values()
                            .map(|card| HandPart2(hand.0, card, hand.2, hand.3, hand.4))
                            .to_vec(),
                    )
                } else if hand.2 == CardPart2::CJ {
                    new_hands.append(
                        &mut CardPart2::values()
                            .map(|card| HandPart2(hand.0, hand.1, card, hand.3, hand.4))
                            .to_vec(),
                    )
                } else if hand.3 == CardPart2::CJ {
                    new_hands.append(
                        &mut CardPart2::values()
                            .map(|card| HandPart2(hand.0, hand.1, hand.2, card, hand.4))
                            .to_vec(),
                    )
                } else if hand.4 == CardPart2::CJ {
                    new_hands.append(
                        &mut CardPart2::values()
                            .map(|card| HandPart2(hand.0, hand.1, hand.2, hand.3, card))
                            .to_vec(),
                    )
                }
            }
            hands = new_hands;
        }
        hands.iter().fold(Type::HighCard, |acc, hand| {
            let mut counts = HashMap::new();
            for &card in &[hand.0, hand.1, hand.2, hand.3, hand.4] {
                *counts.entry(card).or_insert(0) += 1;
            }

            let mut counts = counts.values().collect_vec();
            counts.sort();

            let hand_type = match counts[..] {
                [5] => Type::FiveOfAKind,
                [1, 4] => Type::FourOfAKind,
                [2, 3] => Type::FullHouse,
                [1, 1, 3] => Type::ThreeOfAKind,
                [1, 2, 2] => Type::TwoPair,
                [1, 1, 1, 2] => Type::OnePair,
                [1, 1, 1, 1, 1] => Type::HighCard,
                _ => panic!("Invalid Hand"),
            };

            if hand_type > acc {
                hand_type
            } else {
                acc
            }
        })
    }
}

impl From<(CardPart2, CardPart2, CardPart2, CardPart2, CardPart2)> for HandPart2 {
    fn from(value: (CardPart2, CardPart2, CardPart2, CardPart2, CardPart2)) -> Self {
        Self(value.0, value.1, value.2, value.3, value.4)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RoundPart1(HandPart1, u32);

impl PartialOrd for RoundPart1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoundPart1 {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.0.get_type().cmp(&other.0.get_type());
        if ord != Ordering::Equal {
            return ord;
        }
        self.0.cmp(&other.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RoundPart2(HandPart2, u32);

impl PartialOrd for RoundPart2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoundPart2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.0.get_type().cmp(&other.0.get_type());
        if ord != Ordering::Equal {
            return ord;
        }
        self.0.cmp(&other.0)
    }
}

#[aoc_generator(day7, part1)]
fn parse_part1(input: &str) -> Vec<RoundPart1> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand
                .chars()
                .map(|c| c.into())
                .collect_tuple::<(CardPart1, CardPart1, CardPart1, CardPart1, CardPart1)>()
                .unwrap()
                .into();
            RoundPart1(hand, bid.trim().parse().unwrap())
        })
        .collect_vec()
}

#[aoc_generator(day7, part2)]
fn parse_part2(input: &str) -> Vec<RoundPart2> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand
                .chars()
                .map(|c| c.into())
                .collect_tuple::<(CardPart2, CardPart2, CardPart2, CardPart2, CardPart2)>()
                .unwrap()
                .into();
            RoundPart2(hand, bid.trim().parse().unwrap())
        })
        .collect_vec()
}

#[aoc(day7, part1)]
fn part1(input: &[RoundPart1]) -> u32 {
    let mut input = input.to_vec();
    input.sort();
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, round)| acc + (rank as u32 + 1) * round.1)
}

#[aoc(day7, part2)]
fn part2(input: &[RoundPart2]) -> u32 {
    let mut input = input.to_vec();
    input.sort();
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
            parse_part1(input),
            vec![
                RoundPart1(
                    HandPart1(
                        CardPart1::C3,
                        CardPart1::C2,
                        CardPart1::CT,
                        CardPart1::C3,
                        CardPart1::CK
                    ),
                    765
                ),
                RoundPart1(
                    HandPart1(
                        CardPart1::CT,
                        CardPart1::C5,
                        CardPart1::C5,
                        CardPart1::CJ,
                        CardPart1::C5
                    ),
                    684
                ),
                RoundPart1(
                    HandPart1(
                        CardPart1::CK,
                        CardPart1::CK,
                        CardPart1::C6,
                        CardPart1::C7,
                        CardPart1::C7
                    ),
                    28
                ),
                RoundPart1(
                    HandPart1(
                        CardPart1::CK,
                        CardPart1::CT,
                        CardPart1::CJ,
                        CardPart1::CJ,
                        CardPart1::CT
                    ),
                    220
                ),
                RoundPart1(
                    HandPart1(
                        CardPart1::CQ,
                        CardPart1::CQ,
                        CardPart1::CQ,
                        CardPart1::CJ,
                        CardPart1::CA
                    ),
                    483
                ),
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

        assert_eq!(part1(&parse_part1(input)), 6440);
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

        assert_eq!(part2(&parse_part2(input)), 5905);
    }
}
