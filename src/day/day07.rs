use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::ascii::{alphanumeric1, digit1, newline, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

use super::Day;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair(char),
    TwoPair(char, char),
    ThreeOfAKind(char),
    FullHouse(char, char),
    FourOfAKind(char),
    FiveOfAKind(char),
}

impl std::cmp::PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn get_order(item: &HandType) -> usize {
            match item {
                HandType::HighCard => 1,
                HandType::OnePair(_) => 2,
                HandType::TwoPair(_, _) => 3,
                HandType::ThreeOfAKind(_) => 4,
                HandType::FullHouse(_, _) => 5,
                HandType::FourOfAKind(_) => 6,
                HandType::FiveOfAKind(_) => 7,
            }
        }

        Some(get_order(self).cmp(&get_order(other)))
    }
}

impl std::cmp::Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::str::FromStr for HandType {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = map.into_iter().collect();
        counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        use HandType::*;

        match counts[..] {
            [(c, 5)] => Ok(FiveOfAKind(c)),
            [(c, 4), _] => Ok(FourOfAKind(c)),
            [(c1, 3), (c2, 2)] => Ok(FullHouse(c1, c2)),
            [(c, 3), (_, 1), (_, 1)] => Ok(ThreeOfAKind(c)),
            [(c1, 2), (c2, 2), (_, 1)] => Ok(TwoPair(c1, c2)),
            [(c, 2), (_, 1), (_, 1), (_, 1)] => Ok(OnePair(c)),
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Ok(HighCard),
            _ => Err(eyre!("could not parse hand type {s}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: String,
    kind: HandType,
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ordering = self.kind.cmp(&other.kind).then_with(|| {
            const ORDER: [char; 13] = [
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ];

            for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
                let o = ORDER
                    .iter()
                    .position(|&c| c == c1)
                    .cmp(&ORDER.iter().position(|&c| c == c2));

                if !o.is_eq() {
                    return o;
                }
            }

            std::cmp::Ordering::Equal
        });

        Some(ordering)
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::str::FromStr for Hand {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s.to_string(),
            kind: s.parse()?,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day07 {
    hands: Vec<(Hand, usize)>,
}

fn parse_line(i: &mut &str) -> PResult<(Hand, usize)> {
    separated_pair(alphanumeric1.parse_to::<Hand>(), space1, digit1.parse_to()).parse_next(i)
}

impl Day for Day07 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(7)?;
        self.hands = separated(1.., parse_line, newline)
            .parse(input.data.trim())
            .map_err(|e| eyre!(e.to_string()))?;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let mut hands = self.hands.clone();
        hands.sort();

        Ok(hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        todo!()
    }
}
