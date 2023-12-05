use std::ops::Range;

use super::Day;

#[derive(Debug)]
struct Card {
    hand: Vec<usize>,
    winning: Vec<usize>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, rest) = value.trim_start_matches("Card").split_once(':').unwrap();
        let (l, r) = rest.split_once('|').unwrap();
        let hand = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let winning = r.split_whitespace().map(|s| s.parse().unwrap()).collect();

        Self { hand, winning }
    }
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
        self.cards = input.data.lines().map(|l| l.into()).collect();

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
