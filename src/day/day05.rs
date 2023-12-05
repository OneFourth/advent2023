use rangemap::RangeMap;
use rayon::prelude::*;

use super::Day;

type InnerMap = RangeMap<usize, usize>;

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<InnerMap>,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut groups = value.split("\n\n");
        let seeds = groups
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let maps = groups
            .map(|s| {
                s.lines()
                    .skip(1)
                    .map(|l| {
                        let mut parts = l.split_whitespace().map(|p| p.parse::<usize>().unwrap());
                        let (dest, source, count) = (
                            parts.next().unwrap(),
                            parts.next().unwrap(),
                            parts.next().unwrap(),
                        );
                        (source..(source + count), dest)
                    })
                    .collect()
            })
            .collect();

        Self { seeds, maps }
    }
}

impl Almanac {
    fn location(&self, seed: usize) -> usize {
        let mut current = seed;
        for map in self.maps.iter() {
            if let Some((k, v)) = map.get_key_value(&current) {
                current = v + current - k.start;
            }
        }

        current
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day05 {
    almanac: Almanac,
}

impl Day for Day05 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(5)?;
        self.almanac = input.data.as_str().into();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .almanac
            .seeds
            .iter()
            .map(|s| self.almanac.location(*s))
            .min()
            .unwrap()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .almanac
            .seeds
            .chunks_exact(2)
            .flat_map(|v| {
                let range = v[0]..(v[0] + v[1]);
                let min = range
                    .into_par_iter()
                    .map(|s| self.almanac.location(s))
                    .min();
                dbg!(min)
            })
            .min()
            .unwrap()
            .to_string())
    }
}
