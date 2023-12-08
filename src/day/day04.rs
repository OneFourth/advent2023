use std::ops::Range;

use color_eyre::eyre::eyre;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{delimited, preceded, separated, separated_pair};
use winnow::{PResult, Parser};

use super::Day;

#[derive(Debug)]
struct Card {
    hand: Vec<usize>,
    winning: Vec<usize>,
}

fn parse_number_list(i: &mut &str) -> PResult<Vec<usize>> {
    separated(1.., digit1.parse_to::<usize>(), space1).parse_next(i)
}

fn parse_card(i: &mut &str) -> PResult<Card> {
    preceded(
        ("Card", space1, digit1, ':'),
        separated_pair(
            delimited(space1, parse_number_list, space1),
            '|',
            preceded(space1, parse_number_list),
        ),
    )
    .map(|(hand, winning)| Card { hand, winning })
    .parse_next(i)
}

impl Card {
    fn count(&self) -> u32 {
        self.hand
            .iter()
            .filter_map(|h| self.winning.iter().find(|w| *w == h))
            .count() as u32
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day04 {
    cards: Vec<Card>,
}

impl Day for Day04 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(4)?;
        self.cards = separated(1.., parse_card, newline)
            .parse(input.data.trim())
            .map_err(|e| eyre!(e.to_string()))?;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .cards
            .iter()
            .map(|c| {
                let count = c.count();

                if count > 0 {
                    2usize.pow(count - 1)
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut total = 0;
        let counts: Vec<_> = self.cards.iter().map(|c| c.count()).collect();
        let mut boosts: Vec<Range<usize>> = Vec::new();

        for (i, &count) in counts.iter().enumerate() {
            let extra_count = boosts.iter().filter(|b| b.contains(&i)).count() + 1;

            for _ in 0..extra_count {
                boosts.push(i + 1..(i + 1 + count as usize));
            }

            total += extra_count;
        }

        Ok(total.to_string())
    }
}
