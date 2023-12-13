use color_eyre::eyre::eyre;
use rayon::prelude::*;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{alt, repeat, separated, separated_pair};
use winnow::{PResult, Parser};

use super::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring(i: &mut &str) -> PResult<Spring> {
    alt((
        '.'.value(Spring::Operational),
        '#'.value(Spring::Damaged),
        '?'.value(Spring::Unknown),
    ))
    .parse_next(i)
}

#[derive(Debug)]
struct PartialSequence {
    springs: Vec<Spring>,
    records: Vec<usize>,
}

impl PartialSequence {
    fn possible_sequences(&self) -> PossibleSequences<'_> {
        let unknown_count = self
            .springs
            .iter()
            .filter(|&s| *s == Spring::Unknown)
            .count();

        PossibleSequences {
            max: 2usize.pow(unknown_count as u32),
            current: 0,
            partial: self,
        }
    }

    fn verify(springs: &[Spring], records: &[usize]) -> bool {
        let mut s = springs
            .split(|&s| s == Spring::Operational)
            .filter_map(|s| (!s.is_empty()).then_some(s.len()));

        let mut result = true;
        for &r in records {
            match s.next() {
                Some(v) if r == v => {}
                _ => result = false,
            }
        }

        if s.next().is_some() {
            false
        } else {
            result
        }
    }
}

fn parse_partial(i: &mut &str) -> PResult<PartialSequence> {
    separated_pair(
        repeat(1.., parse_spring),
        space1,
        separated(1.., digit1.parse_to::<usize>(), ','),
    )
    .map(|(springs, records)| PartialSequence { springs, records })
    .parse_next(i)
}

struct PossibleSequences<'a> {
    max: usize,
    current: usize,
    partial: &'a PartialSequence,
}

impl<'a> Iterator for PossibleSequences<'a> {
    type Item = Vec<Spring>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.max {
            let mut springs = self.partial.springs.clone();
            for (i, s) in springs
                .iter_mut()
                .filter(|s| **s == Spring::Unknown)
                .enumerate()
            {
                if self.current & (1 << i) > 0 {
                    *s = Spring::Operational;
                } else {
                    *s = Spring::Damaged;
                }
            }

            self.current += 1;

            if PartialSequence::verify(&springs, &self.partial.records) {
                return Some(springs);
            }
        }

        None
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day12 {
    groups: Vec<PartialSequence>,
}

impl Day for Day12 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(12)?;
        self.groups = separated(1.., parse_partial, newline)
            .parse(input.data.trim())
            .map_err(|e| eyre!(e.to_string()))?;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .groups
            .par_iter()
            .map(|g| g.possible_sequences().count())
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        todo!()
    }
}
