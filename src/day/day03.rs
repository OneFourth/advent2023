use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use rangemap::RangeInclusiveMap;

use super::Day;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SymbolKind {
    Number(u32),
    Symbol(char),
}

#[derive(Debug, Default)]
struct Map {
    items: Vec<RangeInclusiveMap<usize, SymbolKind>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut items = Vec::new();

        for s in value.split('\n') {
            let mut number: Option<(RangeInclusive<usize>, u32)> = None;

            let mut line = RangeInclusiveMap::new();

            for (x, c) in s.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    if let Some((r, n)) = number {
                        number = Some((*r.start()..=x, (n * 10) + d));
                    } else {
                        number = Some((x..=x, d));
                    }
                } else {
                    if let Some((r, n)) = number {
                        line.insert(r, SymbolKind::Number(n));
                        number = None;
                    }

                    if c != '.' {
                        line.insert(x..=x, SymbolKind::Symbol(c));
                    }
                }
            }

            if let Some((r, n)) = number {
                line.insert(r, SymbolKind::Number(n));
            }

            items.push(line);
        }

        Self { items }
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<(&RangeInclusive<usize>, &SymbolKind)> {
        self.items.get(y).and_then(|v| v.get_key_value(&x))
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day03 {
    map: Map,
}

impl Day for Day03 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(3)?;
        self.map = input.data.as_str().into();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let positions = self.map.items.iter().enumerate().flat_map(|(y, v)| {
            v.iter().filter_map(move |(x, s)| match s {
                SymbolKind::Symbol(_) => Some((*x.start(), y)),
                _ => None,
            })
        });

        let surrounding = positions.flat_map(|(x, y)| {
            let mut output = Vec::new();
            if x != 0 {
                if y != 0 {
                    output.push((x - 1, y - 1));
                }
                output.push((x - 1, y));
                output.push((x - 1, y + 1));
            }
            if y != 0 {
                output.push((x, y - 1));
                output.push((x + 1, y - 1));
            }
            output.push((x + 1, y));
            output.push((x, y + 1));
            output.push((x + 1, y + 1));

            output
        });

        let unique = surrounding
            .filter_map(|(x, y)| match self.map.get(x, y) {
                Some((r, SymbolKind::Number(n))) => Some(((r, y), n)),
                _ => None,
            })
            .collect::<HashSet<_>>();

        Ok(unique.iter().map(|(_, n)| *n).sum::<u32>().to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let positions = self.map.items.iter().enumerate().flat_map(|(y, v)| {
            v.iter().filter_map(move |(x, s)| match s {
                SymbolKind::Symbol('*') => Some((*x.start(), y)),
                _ => None,
            })
        });

        let surrounding = positions.flat_map(|(x, y)| {
            let p = (x, y);
            let mut output = Vec::new();
            if x != 0 {
                if y != 0 {
                    output.push((p, (x - 1, y - 1)));
                }
                output.push((p, (x - 1, y)));
                output.push((p, (x - 1, y + 1)));
            }
            if y != 0 {
                output.push((p, (x, y - 1)));
                output.push((p, (x + 1, y - 1)));
            }
            output.push((p, (x + 1, y)));
            output.push((p, (x, y + 1)));
            output.push((p, (x + 1, y + 1)));

            output
        });

        let mut gears: HashMap<_, HashSet<_>> = HashMap::new();
        for (p, (x, y)) in surrounding {
            let e = gears.entry(p).or_default();
            if let Some((r, SymbolKind::Number(n))) = self.map.get(x, y) {
                e.insert((r, n));
            }
        }

        Ok(gears
            .iter()
            .filter_map(|(_, v)| {
                if v.len() == 2 {
                    Some(v.iter().map(|(_, n)| *n).product::<u32>())
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string())
    }
}
