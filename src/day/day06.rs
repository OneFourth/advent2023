use super::Day;

#[derive(Debug, Default)]
pub(crate) struct Day06 {
    races: Vec<(usize, usize)>,

    single_race: (usize, usize),
}

fn calculate_range(total_time: usize, distance: usize) -> usize {
    let total_time = total_time as f64;
    let distance = distance as f64;

    let factor = ((total_time * total_time) - (4.0 * distance)).sqrt();
    let start = (total_time - factor) / 2.0;

    let mut range_start = start.ceil();
    if range_start == start {
        range_start += 1.0;
    }

    let end = (total_time + factor) / 2.0;
    let mut range_end = end.floor();
    if range_end == end {
        range_end -= 1.0;
    }

    range_end as usize - range_start as usize + 1
}

impl Day for Day06 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(6)?;
        let mut data = input.data.lines().map(|s| {
            s.split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        });

        self.races = data
            .next()
            .unwrap()
            .into_iter()
            .zip(data.next().unwrap())
            .collect();
        let [single_time, single_distance] = input
            .data
            .replace("Time:", "")
            .replace("Distance:", "")
            .replace(' ', "")
            .lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.single_race = (single_time, single_distance);

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .races
            .iter()
            .map(|&(t, d)| calculate_range(t, d))
            .product::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        Ok((calculate_range(self.single_race.0, self.single_race.1)).to_string())
    }
}
