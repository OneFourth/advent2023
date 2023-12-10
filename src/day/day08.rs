use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::ascii::{newline, space1};
use winnow::combinator::{alt, delimited, repeat, separated, separated_pair};
use winnow::token::take;
use winnow::{PResult, Parser};

use super::Day;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_directions(i: &mut &str) -> PResult<Vec<Direction>> {
    repeat(
        1..,
        alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))),
    )
    .parse_next(i)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Name([char; 3]);

fn parse_name(i: &mut &str) -> PResult<Name> {
    take(3usize)
        .map(|s: &str| {
            let mut c = s.chars();
            Name([c.next().unwrap(), c.next().unwrap(), c.next().unwrap()])
        })
        .parse_next(i)
}

type Nodes = HashMap<Name, (Name, Name)>;

#[derive(Debug, Default)]
pub(crate) struct Day08 {
    directions: Vec<Direction>,
    nodes: Nodes,
}

fn parse_nodes(i: &mut &str) -> PResult<Nodes> {
    separated(
        1..,
        separated_pair(
            parse_name,
            (space1, '=', space1),
            delimited(
                '(',
                separated_pair(parse_name, (',', space1), parse_name),
                ')',
            ),
        ),
        newline,
    )
    .parse_next(i)
}

impl Day for Day08 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(8)?;
        (self.directions, self.nodes) =
            separated_pair(parse_directions, (newline, newline), parse_nodes)
                .parse(input.data.trim())
                .map_err(|e| eyre!(e.to_string()))?;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let mut current = &Name(['A', 'A', 'A']);
        let mut direction = self.directions.iter().cycle();
        let mut count = 0;

        while *current != Name(['Z', 'Z', 'Z']) {
            count += 1;
            let node = self.nodes.get(current).unwrap();
            match direction.next().unwrap() {
                Direction::Left => current = &node.0,
                Direction::Right => current = &node.1,
            }
        }

        Ok(count.to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut current: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(k, _)| (k.0[2] == 'A').then_some(k))
            .collect();
        let mut direction = self.directions.iter().cycle();
        let mut count = vec![0u64; current.len()];

        let mut should_loop = true;
        while should_loop {
            let d = direction.next().unwrap();
            should_loop = false;
            for (i, n) in current
                .iter_mut()
                .enumerate()
                .filter(|(_, n)| n.0[2] != 'Z')
            {
                count[i] += 1;
                should_loop = true;
                let node = self.nodes.get(n).unwrap();
                match d {
                    Direction::Left => *n = &node.0,
                    Direction::Right => *n = &node.1,
                }
            }
        }

        Ok(count
            .into_iter()
            .reduce(num::integer::lcm)
            .unwrap()
            .to_string())
    }
}
