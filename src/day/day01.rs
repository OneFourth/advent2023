use super::Day;
use crate::input::Input;

#[derive(Debug)]
struct Line {
    calibration: u32,
    calibration2: u32,
}

impl From<String> for Line {
    fn from(mut value: String) -> Self {
        let mut calibration = 0;
        for c in value.chars() {
            if let Some(digit) = c.to_digit(10) {
                calibration += digit * 10;
                break;
            }
        }
        for c in value.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                calibration += digit;
                break;
            }
        }

        value = value.replace("one", "one1one");
        value = value.replace("two", "two2two");
        value = value.replace("three", "three3three");
        value = value.replace("four", "four4four");
        value = value.replace("five", "five5five");
        value = value.replace("six", "six6six");
        value = value.replace("seven", "seven7seven");
        value = value.replace("eight", "eight8eight");
        value = value.replace("nine", "nine9nine");

        let mut calibration2 = 0;
        for c in value.chars() {
            if let Some(digit) = c.to_digit(10) {
                calibration2 += digit * 10;
                break;
            }
        }
        for c in value.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                calibration2 += digit;
                break;
            }
        }

        Self {
            calibration,
            calibration2,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day01 {
    values: Vec<Line>,
}

impl Day for Day01 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        self.values = Input::get(1)?
            .data
            .lines()
            .map(|s| s.to_string().into())
            .collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        let total: u32 = self.values.iter().map(|v| v.calibration).sum();

        Ok(total.to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let total: u32 = self.values.iter().map(|v| v.calibration2).sum();

        Ok(total.to_string())
    }
}
