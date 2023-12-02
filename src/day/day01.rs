use super::Day;

#[derive(Debug)]
struct Line(Vec<u8>);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self(value.bytes().collect())
    }
}

impl Line {
    fn digits(&self) -> DigitIter {
        DigitIter { values: &self.0 }
    }

    fn digits_and_words(&self) -> DigitAndWordIter {
        DigitAndWordIter { values: &self.0 }
    }

    fn calibration(&self) -> u8 {
        let tens = self.digits().next().unwrap();
        let ones = self.digits().last().unwrap();

        tens * 10 + ones
    }

    fn calibration2(&self) -> u8 {
        let tens = self.digits_and_words().next().unwrap();
        let ones = self.digits_and_words().last().unwrap();

        tens * 10 + ones
    }
}

struct DigitIter<'a> {
    values: &'a [u8],
}

impl Iterator for DigitIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((c, new_values)) = self.values.split_first() {
            self.values = new_values;
            if let Some(d) = c.is_ascii_digit().then_some(c - b'0') {
                return Some(d);
            }
        }

        None
    }
}

struct DigitAndWordIter<'a> {
    values: &'a [u8],
}

impl Iterator for DigitAndWordIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.values.is_empty() {
            const CONVERT: [(&[u8], u8); 9] = [
                (b"one", 1),
                (b"two", 2),
                (b"three", 3),
                (b"four", 4),
                (b"five", 5),
                (b"six", 6),
                (b"seven", 7),
                (b"eight", 8),
                (b"nine", 9),
            ];

            let digit = CONVERT
                .iter()
                .find_map(|(word, digit)| self.values.starts_with(word).then_some(*digit))
                .or_else(|| {
                    self.values
                        .first()
                        .and_then(|c| c.is_ascii_digit().then_some(c - b'0'))
                });

            self.values = &self.values[1..];

            if digit.is_some() {
                return digit;
            }
        }

        None
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day01 {
    values: Vec<Line>,
}

impl Day for Day01 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        self.values = crate::input::Input::get(1)?
            .data
            .lines()
            .map(|s| s.into())
            .collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .values
            .iter()
            .map(|v| v.calibration() as usize)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .values
            .iter()
            .map(|v| v.calibration2() as usize)
            .sum::<usize>()
            .to_string())
    }
}
