use std::collections::HashMap;

use super::Day;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Symbol {
    Empty,
    SymbolChar(char),
    Digit(u8),
}

#[derive(Debug, Default)]
struct Grid {
    stride: usize,
    data: Vec<Symbol>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut data = Vec::with_capacity(value.len());
        let mut stride = None;

        for (i, c) in value.chars().enumerate() {
            use Symbol::*;

            match c {
                '.' => data.push(Empty),
                '\n' => stride = stride.or_else(|| Some(i)),
                d if d.is_digit(10) => {
                    data.push(Digit(d.to_digit(10).unwrap().try_into().unwrap()))
                }
                s => data.push(SymbolChar(s)),
            };
        }

        let stride = stride.unwrap();

        Self { stride, data }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Direction {
    fn next(self) -> Option<Self> {
        match self {
            Direction::TopLeft => Some(Direction::Top),
            Direction::Top => Some(Direction::TopRight),
            Direction::TopRight => Some(Direction::Left),
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::BottomLeft),
            Direction::BottomLeft => Some(Direction::Bottom),
            Direction::Bottom => Some(Direction::BottomRight),
            Direction::BottomRight => None,
        }
    }
}

struct Surrounding {
    center: (usize, usize),
    current: Option<Direction>,
}

impl Surrounding {
    fn new(center: (usize, usize)) -> Self {
        Self {
            center,
            current: Some(Direction::TopLeft),
        }
    }
}

impl Iterator for Surrounding {
    type Item = (Option<usize>, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let rel = match self.current {
            Some(Direction::TopLeft) => (-1, -1),
            Some(Direction::Top) => (0, -1),
            Some(Direction::TopRight) => (1, -1),
            Some(Direction::Left) => (-1, 0),
            Some(Direction::Right) => (1, 0),
            Some(Direction::BottomLeft) => (-1, 1),
            Some(Direction::Bottom) => (0, 1),
            Some(Direction::BottomRight) => (1, 1),
            None => return None,
        };

        let pos = (
            self.center.0.checked_add_signed(rel.0),
            self.center.1.checked_add_signed(rel.1),
        );

        self.current = self.current.unwrap().next();

        Some(pos)
    }
}

impl Grid {
    fn get(&self, (x, y): (Option<usize>, Option<usize>)) -> &Symbol {
        match (x, y) {
            (Some(x), Some(y)) => self.data.get(self.stride * y + x).unwrap_or(&Symbol::Empty),
            _ => &Symbol::Empty,
        }
    }

    fn position(&self, index: usize) -> Option<(usize, usize)> {
        if index == 0 {
            Some((0, 0))
        } else if index >= self.data.len() {
            None
        } else {
            let x = index.rem_euclid(self.stride);
            let y = index.div_euclid(self.stride);

            Some((x, y))
        }
    }

    fn get_full_number(&self, (x, y): (Option<usize>, Option<usize>)) -> ((usize, usize), usize) {
        let mut start = x;
        while let Symbol::Digit(_) = self.get((start, y)) {
            let new_start = start.unwrap().checked_sub(1);
            if let Symbol::Empty = self.get((new_start, y)) {
                break;
            }

            start = new_start;
        }

        let position = (start.unwrap(), y.unwrap());

        let mut number = 0;
        while let Symbol::Digit(d) = self.get((start, y)) {
            start = Some(start.unwrap() + 1);
            number = (number * 10) + (*d as usize);
        }

        (position, number)
    }

    fn surrounding(&self, center: (usize, usize)) -> Surrounding {
        Surrounding::new(center)
    }

    fn find_part_numbers(&self) -> Vec<usize> {
        let number_positions = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                Symbol::SymbolChar(_) => self.position(i),
                _ => None,
            })
            .flat_map(|p| self.surrounding(p))
            .filter(|p| matches!(self.get(*p), Symbol::Digit(_)));

        let numbers: HashMap<_, _> = number_positions
            .map(|np| self.get_full_number(np))
            .collect();

        let mut block = String::new();
        for (i, s) in self.data.iter().enumerate() {
            if i % self.stride == 0 {
                block.push('\n');
            }

            match s {
                Symbol::Empty => block.push('.'),
                Symbol::SymbolChar(c) => block.push(*c),
                Symbol::Digit(_) => block.push('#'),
            }

            if let Some(number) = numbers.get(&self.position(i).unwrap()) {
                block.push_str(&number.to_string());
            }
        }

        std::fs::write("test.txt", block).unwrap();

        numbers.iter().map(|(_, n)| n).copied().collect()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day03 {
    grid: Grid,
}

impl Day for Day03 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(3)?;
        self.grid = input.data.as_str().into();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .grid
            .find_part_numbers()
            .iter()
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        todo!()
    }
}
