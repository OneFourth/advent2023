use std::collections::HashMap;

use color_eyre::eyre::{eyre, Report};

use super::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NE),
            'J' => Ok(NW),
            '7' => Ok(SW),
            'F' => Ok(SE),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(eyre!("Invalid token: {value}")),
        }
    }
}

impl Tile {
    fn connections(&self, tiles: &HashMap<Point, Tile>, position: Point) -> Vec<Point> {
        let mut output = Vec::new();

        if self.has_north_connection() {
            let p = position.up();
            if tiles.get(&p).is_some_and(|t| t.has_south_connection()) {
                output.push(p);
            }
        }

        if self.has_west_connection() {
            let p = position.left();
            if tiles.get(&p).is_some_and(|t| t.has_east_connection()) {
                output.push(p);
            }
        }

        if self.has_south_connection() {
            let p = position.down();
            if tiles.get(&p).is_some_and(|t| t.has_north_connection()) {
                output.push(p);
            }
        }

        if self.has_east_connection() {
            let p = position.right();
            if tiles.get(&p).is_some_and(|t| t.has_west_connection()) {
                output.push(p);
            }
        }

        output
    }

    fn has_north_connection(&self) -> bool {
        use Tile::*;

        matches!(self, Vertical | NW | NE | Start)
    }

    fn has_south_connection(&self) -> bool {
        use Tile::*;

        matches!(self, Vertical | SW | SE | Start)
    }

    fn has_east_connection(&self) -> bool {
        use Tile::*;

        matches!(self, Horizontal | NE | SE | Start)
    }

    fn has_west_connection(&self) -> bool {
        use Tile::*;

        matches!(self, Horizontal | NW | SW | Start)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(isize, isize);

impl Point {
    fn up(self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn down(self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn left(self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn right(self) -> Self {
        Self(self.0 + 1, self.1)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day10 {
    map: HashMap<Point, Vec<Point>>,
    tiles: HashMap<Point, Tile>,
    start: Option<Point>,
}

impl Day for Day10 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(10)?;
        self.tiles = input
            .data
            .lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.char_indices()
                    .map(move |(x, c)| (Point(x as isize, y as isize), Tile::try_from(c).unwrap()))
            })
            .collect();

        for (&position, &tile) in self.tiles.iter() {
            self.map
                .insert(position, tile.connections(&self.tiles, position));

            if tile == Tile::Start {
                self.start = Some(position);
            }
        }

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let start = self.start.unwrap();
        let mut seen = HashMap::new();
        let mut to_check = vec![start];

        let mut steps = 0;
        loop {
            let checking: Vec<_> = to_check
                .iter()
                .filter(|&p| !seen.contains_key(p))
                .copied()
                .collect();

            if checking.is_empty() {
                break;
            }

            to_check.clear();

            for p in checking {
                seen.insert(p, steps);

                if let Some(connections) = self.map.get(&p) {
                    to_check.extend(connections.iter());
                }
            }

            steps += 1;
        }

        Ok((steps - 1).to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let start = self.start.unwrap();
        let mut seen = HashMap::new();
        let mut to_check = vec![start];

        let mut steps = 0;
        while let Some(p) = to_check.pop() {
            if seen.contains_key(&p) {
                continue;
            }

            seen.insert(p, steps);

            if let Some(connections) = self.map.get(&p) {
                to_check.extend(connections.iter());
            }

            steps += 1;
        }

        let mut fill = 0;
        let mut winding = 0;
        for y in 0..200 {
            for x in 0..200 {
                let p = Point(x, y);
                if let Some(n) = seen.get(&p) {
                    if let Some(m) = seen.get(&p.down()) {
                        if (steps + n - m) % steps == 1 {
                            winding += 1;
                        } else if (steps + m - n) % steps == 1 {
                            winding -= 1;
                        }
                    }
                } else if winding > 0 {
                    fill += 1;
                }
            }
        }

        Ok(fill.to_string())
    }
}
