use std::collections::HashSet;

use super::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorF,
    MirrorB,
    SplitterV,
    SplitterH,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Grid {
    stride: usize,
    height: usize,
    data: Vec<Tile>,
}

impl Grid {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.stride + x
    }

    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x < 0 || y < 0 || x as usize >= self.stride || y as usize >= self.height {
            None
        } else {
            Some(self.data[self.index(x as usize, y as usize)])
        }
    }

    fn ray(&self, transform: Transform) -> Ray<'_> {
        Ray {
            grid: self,
            transform,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new_pos(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Transform {
    direction: Direction,
    position: (isize, isize),
}

#[derive(Debug, Copy, Clone)]
struct Ray<'a> {
    grid: &'a Grid,
    transform: Transform,
}

impl<'a> Iterator for Ray<'a> {
    type Item = (Transform, Option<Transform>);

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.transform.direction;
        let position = direction.new_pos(self.transform.position);

        let mut split = None;
        match self.grid.get(position.0, position.1) {
            Some(Tile::Empty) => {
                self.transform = Transform {
                    direction,
                    position,
                };
            }
            Some(Tile::MirrorF) => {
                let direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                self.transform = Transform {
                    direction,
                    position,
                };
            }
            Some(Tile::MirrorB) => {
                let direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };

                self.transform = Transform {
                    direction,
                    position,
                };
            }
            Some(Tile::SplitterV) => match direction {
                Direction::Up | Direction::Down => {
                    self.transform = Transform {
                        direction,
                        position,
                    };
                }
                Direction::Left | Direction::Right => {
                    self.transform = Transform {
                        direction: Direction::Up,
                        position,
                    };

                    split = Some(Transform {
                        direction: Direction::Down,
                        position,
                    });
                }
            },
            Some(Tile::SplitterH) => match direction {
                Direction::Up | Direction::Down => {
                    self.transform = Transform {
                        direction: Direction::Left,
                        position,
                    };

                    split = Some(Transform {
                        direction: Direction::Right,
                        position,
                    });
                }
                Direction::Left | Direction::Right => {
                    self.transform = Transform {
                        direction,
                        position,
                    };
                }
            },
            None => {
                return None;
            }
        }

        Some((self.transform, split))
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day16 {
    grid: Grid,
}

impl Day16 {
    fn get_energized(&self, start: Transform) -> usize {
        let mut energized = HashSet::new();

        let mut rays = Vec::new();
        rays.push(self.grid.ray(start));

        let mut new_rays = Vec::new();
        loop {
            let mut should_break = true;
            for ray in &mut rays {
                if let Some((t1, t2)) = ray.next() {
                    if energized.insert(t1) {
                        should_break = false;
                    }
                    if let Some(t2) = t2 {
                        if energized.insert(t2) {
                            should_break = false;
                            new_rays.push(t2);
                        }
                    }
                }
            }

            new_rays.drain(..).for_each(|t| rays.push(self.grid.ray(t)));

            if should_break {
                break;
            }
        }

        energized
            .iter()
            .map(|e| e.position)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl Day for Day16 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(16)?;
        let (stride, data) =
            input
                .data
                .char_indices()
                .fold((None, Vec::new()), |(mut stride, mut data), (i, c)| {
                    match c {
                        '.' => data.push(Tile::Empty),
                        '/' => data.push(Tile::MirrorF),
                        '\\' => data.push(Tile::MirrorB),
                        '|' => data.push(Tile::SplitterV),
                        '-' => data.push(Tile::SplitterH),
                        '\n' => {
                            stride.get_or_insert(i);
                        }
                        _ => panic!("invalid tile: {c}"),
                    }
                    (stride, data)
                });
        self.grid.stride = stride.unwrap();
        self.grid.height = data.len() / self.grid.stride;
        self.grid.data = data;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let start = Transform {
            position: (-1, 0),
            direction: Direction::Right,
        };

        Ok(self.get_energized(start).to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let top = (0..self.grid.stride).map(|x| Transform {
            direction: Direction::Down,
            position: (x as isize, -1),
        });
        let bottom = (0..self.grid.stride).map(|x| Transform {
            direction: Direction::Down,
            position: (x as isize, self.grid.height as isize),
        });
        let left = (0..self.grid.height).map(|y| Transform {
            direction: Direction::Right,
            position: (-1, y as isize),
        });
        let right = (0..self.grid.height).map(|y| Transform {
            direction: Direction::Left,
            position: (self.grid.stride as isize, y as isize),
        });

        let edges = top.chain(right).chain(bottom).chain(left);

        Ok(edges
            .map(|t| self.get_energized(t))
            .max()
            .unwrap()
            .to_string())
    }
}
