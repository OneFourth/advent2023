use super::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Round,
    Cube,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Grid {
    stride: usize,
    height: usize,
    data: Vec<Tile>,
}

#[derive(Debug, Default)]
pub(crate) struct Day14 {
    grid: Grid,
}

impl Grid {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.stride + x
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.data[self.index(x, y)]
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let index1 = self.index(x1, y1);
        let index2 = self.index(x2, y2);
        self.data.swap(index1, index2);
    }

    fn shift_up(&mut self) {
        for y in 1..self.height {
            for x in 0..self.stride {
                match (self.get(x, y), self.get(x, y - 1)) {
                    (Tile::Round, Tile::Empty) => self.swap(x, y, x, y - 1),
                    _ => {}
                }
            }
        }
    }

    fn shift_right(&mut self) {
        for y in 0..self.height {
            for x in 0..(self.stride - 1) {
                match (self.get(x, y), self.get(x + 1, y)) {
                    (Tile::Round, Tile::Empty) => self.swap(x, y, x + 1, y),
                    _ => {}
                }
            }
        }
    }

    fn shift_down(&mut self) {
        for y in 0..(self.height - 1) {
            for x in 0..self.stride {
                match (self.get(x, y), self.get(x, y + 1)) {
                    (Tile::Round, Tile::Empty) => self.swap(x, y, x, y + 1),
                    _ => {}
                }
            }
        }
    }

    fn shift_left(&mut self) {
        for y in 0..self.height {
            for x in 1..self.stride {
                match (self.get(x, y), self.get(x - 1, y)) {
                    (Tile::Round, Tile::Empty) => self.swap(x, y, x - 1, y),
                    _ => {}
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &t)| (t == Tile::Round).then_some(self.height - (i / self.stride)))
            .sum()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, t) in self.data.iter().enumerate() {
            match t {
                Tile::Empty => write!(f, ".")?,
                Tile::Round => write!(f, "O")?,
                Tile::Cube => write!(f, "#")?,
            }

            if i % self.stride == self.stride - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Day for Day14 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(14)?;
        let (stride, data) =
            input
                .data
                .char_indices()
                .fold((None, Vec::new()), |(mut stride, mut data), (i, c)| {
                    match c {
                        '.' => data.push(Tile::Empty),
                        'O' => data.push(Tile::Round),
                        '#' => data.push(Tile::Cube),
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
        let mut old_grid = self.grid.clone();
        let mut grid = self.grid.clone();

        loop {
            grid.shift_up();
            if old_grid == grid {
                break;
            } else {
                old_grid = grid.clone();
            }
        }

        Ok(grid.total_load().to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut grid = self.grid.clone();

        let direction = [
            Grid::shift_up,
            Grid::shift_left,
            Grid::shift_down,
            Grid::shift_right,
        ];

        let cycle = |grid: &mut Grid| {
            for func in &direction {
                let mut old_grid = grid.clone();
                loop {
                    func(grid);

                    if old_grid == *grid {
                        break;
                    } else {
                        old_grid = grid.clone();
                    }
                }
            }
        };

        let total = 1000000000;
        for i in 0..total {
            if i % 100000 == 0 {
                println!("{:0.2}%", 100.0 * (i as f64 / total as f64));
            }
            cycle(&mut grid);
        }

        Ok(grid.total_load().to_string())
    }
}
