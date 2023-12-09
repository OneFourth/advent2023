use super::Day;

#[derive(Debug)]
struct Sequence(Vec<isize>);

impl Sequence {
    fn get_difference(&self) -> Self {
        Sequence(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    fn get_next(&self) -> isize {
        if self.0.iter().all(|&v| v == 0) {
            0
        } else {
            self.0.last().unwrap() + self.get_difference().get_next()
        }
    }

    fn get_prev(&self) -> isize {
        if self.0.iter().all(|&v| v == 0) {
            0
        } else {
            self.0.first().unwrap() - self.get_difference().get_prev()
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day09 {
    sequences: Vec<Sequence>,
}

impl Day for Day09 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(9)?;
        self.sequences = input
            .data
            .trim()
            .lines()
            .map(|l| {
                Sequence(
                    l.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .sequences
            .iter()
            .map(|s| s.get_next())
            .sum::<isize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .sequences
            .iter()
            .map(|s| s.get_prev())
            .sum::<isize>()
            .to_string())
    }
}
