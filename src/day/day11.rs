use std::collections::HashSet;

use color_eyre::eyre::eyre;

use super::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point(isize, isize);

#[derive(Debug, Default)]
pub(crate) struct Day11 {
    empty_columns: HashSet<isize>,
    empty_rows: HashSet<isize>,
    galaxies: Vec<Point>,
}

impl Day for Day11 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(11)?;

        let mut max_y = 0;
        let mut column_galaxies = HashSet::new();
        for (y, s) in input.data.lines().enumerate() {
            let y = y as isize;
            let mut has_row_galaxy = false;

            for (x, c) in s.char_indices() {
                let x = x as isize;
                match c {
                    '.' => {}
                    '#' => {
                        self.galaxies.push(Point(x, y));
                        column_galaxies.insert(x);
                        has_row_galaxy = true;
                        max_y = max_y.max(y);
                    }
                    _ => return Err(eyre!("Invalid character at ({x}, {y}): {c}")),
                }
            }

            if !has_row_galaxy {
                self.empty_rows.insert(y);
            }
        }

        self.empty_columns = (0..=max_y)
            .filter(|y| !column_galaxies.contains(y))
            .collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let mut sum = 0;

        for (i, &Point(x1, y1)) in self.galaxies.iter().enumerate() {
            for &Point(x2, y2) in self.galaxies.iter().skip(i + 1) {
                let dist = x2.abs_diff(x1) + y2.abs_diff(y1);
                let empty_columns = (x1.min(x2)..=x1.max(x2))
                    .filter(|x| self.empty_columns.contains(x))
                    .count();
                let empty_rows = (y1.min(y2)..=y1.max(y2))
                    .filter(|y| self.empty_rows.contains(y))
                    .count();

                sum += (dist - empty_rows - empty_columns) + (2 * (empty_rows + empty_columns));
            }
        }

        Ok(sum.to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut sum = 0;

        for (i, &Point(x1, y1)) in self.galaxies.iter().enumerate() {
            for &Point(x2, y2) in self.galaxies.iter().skip(i + 1) {
                let dist = x2.abs_diff(x1) + y2.abs_diff(y1);
                let empty_columns = (x1.min(x2)..=x1.max(x2))
                    .filter(|x| self.empty_columns.contains(x))
                    .count();
                let empty_rows = (y1.min(y2)..=y1.max(y2))
                    .filter(|y| self.empty_rows.contains(y))
                    .count();

                sum += (dist - empty_rows - empty_columns)
                    + (1_000_000 * (empty_rows + empty_columns));
            }
        }

        Ok(sum.to_string())
    }
}
