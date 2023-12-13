use std::ops::Range;

use super::Day;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    row_wise: Vec<Vec<Tile>>,
    column_wise: Vec<Vec<Tile>>,
}

impl Map {
    fn split_at_row(&self, row: usize) -> (Lines<'_>, Lines<'_>) {
        (
            Lines {
                map: &self.row_wise,
                range: 0..row,
            },
            Lines {
                map: &self.row_wise,
                range: row..self.row_wise.len(),
            },
        )
    }

    fn split_at_column(&self, column: usize) -> (Lines<'_>, Lines<'_>) {
        (
            Lines {
                map: &self.column_wise,
                range: 0..column,
            },
            Lines {
                map: &self.column_wise,
                range: column..self.column_wise.len(),
            },
        )
    }
}

struct Lines<'a> {
    map: &'a [Vec<Tile>],
    range: Range<usize>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.map[self.range.clone()].first() {
            self.range.start += 1;
            Some(line)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Lines<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.map[self.range.clone()].last() {
            self.range.end -= 1;
            Some(line)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day13 {
    maps: Vec<Map>,
}

impl Day for Day13 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(13)?;
        self.maps = input
            .data
            .trim()
            .split("\n\n")
            .map(|b| {
                let row_wise: Vec<Vec<Tile>> = b
                    .lines()
                    .map(|s| {
                        s.chars()
                            .map(|c| match c {
                                '.' => Tile::Ash,
                                '#' => Tile::Rock,
                                _ => panic!("invalid tile"),
                            })
                            .collect()
                    })
                    .collect();

                let column_wise = (0..row_wise[0].len())
                    .map(|x| (0..row_wise.len()).map(|y| row_wise[y][x]).collect())
                    .collect();

                Map {
                    row_wise,
                    column_wise,
                }
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .maps
            .iter()
            .map(|m| {
                (1..m.row_wise.len())
                    .find(|&row| {
                        let (left, right) = m.split_at_row(row);
                        left.rev().zip(right).all(|(l, r)| l == r)
                    })
                    .map(|r| r * 100)
                    .or_else(|| {
                        (1..m.column_wise.len()).find(|&column| {
                            let (left, right) = m.split_at_column(column);
                            left.rev().zip(right).all(|(l, r)| l == r)
                        })
                    })
                    .unwrap_or_default()
            })
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        fn compare((left, right): (Lines, Lines)) -> bool {
            left.rev()
                .zip(right)
                .map(|(l, r)| l.iter().zip(r.iter()).filter(|(a, b)| a != b).count())
                .sum::<usize>()
                == 1
        }

        Ok(self
            .maps
            .iter()
            .map(|m| {
                (1..m.row_wise.len())
                    .find(|&row| compare(m.split_at_row(row)))
                    .map(|r| r * 100)
                    .or_else(|| {
                        (1..m.column_wise.len()).find(|&column| compare(m.split_at_column(column)))
                    })
                    .unwrap()
            })
            .sum::<usize>()
            .to_string())
    }
}
