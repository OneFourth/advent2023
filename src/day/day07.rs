use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::ascii::{alphanumeric1, digit1, newline, space1};
use winnow::combinator::{alt, repeat, separated, separated_pair};
use winnow::{PResult, Parser};

use super::Day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Value {
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

impl Value {
    fn part2_order(&self) -> usize {
        use Value::*;
        match self {
            Jack => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7,
            Nine => 8,
            Ten => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        }
    }
}

fn parse_card(i: &mut &str) -> PResult<Value> {
    alt((
        '2'.value(Value::Two),
        '3'.value(Value::Three),
        '4'.value(Value::Four),
        '5'.value(Value::Five),
        '6'.value(Value::Six),
        '7'.value(Value::Seven),
        '8'.value(Value::Eight),
        '9'.value(Value::Nine),
        'T'.value(Value::Ten),
        'J'.value(Value::Jack),
        'Q'.value(Value::Queen),
        'K'.value(Value::King),
        'A'.value(Value::Ace),
    ))
    .parse_next(i)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: [Value; 5],
}

impl std::str::FromStr for Hand {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = repeat(5, parse_card)
            .parse(s)
            .map_err(|e| eyre!(e.to_string()))?;

        let cards = v.try_into().unwrap();

        Ok(Self { cards })
    }
}

impl Hand {
    fn get_type_part1(&self) -> HandType {
        let mut map = HashMap::new();

        for v in self.cards {
            *map.entry(v).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = map.into_iter().collect();
        counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        use HandType::*;

        match counts[..] {
            [(_, 5)] => FiveOfAKind,
            [(_, 4), _] => FourOfAKind,
            [(_, 3), (_, 2)] => FullHouse,
            [(_, 3), (_, 1), (_, 1)] => ThreeOfAKind,
            [(_, 2), (_, 2), (_, 1)] => TwoPair,
            [(_, 2), (_, 1), (_, 1), (_, 1)] => OnePair,
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => HighCard,
            _ => unreachable!(),
        }
    }

    fn get_type_part2(&self) -> HandType {
        let mut map = HashMap::new();

        for v in self.cards {
            *map.entry(v).or_insert(0) += 1;
        }

        let jacks = *map.get(&Value::Jack).unwrap_or(&0);
        let mut counts: Vec<_> = map.into_iter().collect();
        counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        use HandType::*;

        match jacks {
            0 => match counts[..] {
                [(_, 5)] => FiveOfAKind,
                [(_, 4), _] => FourOfAKind,
                [(_, 3), (_, 2)] => FullHouse,
                [(_, 3), (_, 1), (_, 1)] => ThreeOfAKind,
                [(_, 2), (_, 2), (_, 1)] => TwoPair,
                [(_, 2), (_, 1), (_, 1), (_, 1)] => OnePair,
                [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => HighCard,
                _ => unreachable!(),
            },
            1 => match counts[..] {
                [(_, 4), ..] => FiveOfAKind,
                [(_, 3), ..] => FourOfAKind,
                [(_, 2), (_, 2), ..] => FullHouse,
                [(_, 2), ..] => ThreeOfAKind,
                _ => OnePair,
            },
            2 => match counts[..] {
                [(_, 3), ..] => FiveOfAKind,
                [(_, 2), (_, 2), ..] => FourOfAKind,
                _ => ThreeOfAKind,
            },
            3 => match counts[..] {
                [(Value::Jack, 3), (_, 2)] => FiveOfAKind,
                _ => FourOfAKind,
            },
            4 => FiveOfAKind,
            5 => FiveOfAKind,
            _ => unreachable!(),
        }
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
        hands.sort_by_cached_key(|(h, _)| (h.get_type_part1(), h.cards));

        Ok(hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut hands = self.hands.clone();
        hands.sort_by_cached_key(|(h, _)| {
            (
                h.get_type_part2(),
                h.cards.iter().map(|c| c.part2_order()).collect::<Vec<_>>(),
            )
        });

        Ok(hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum::<usize>()
            .to_string())
    }
}
