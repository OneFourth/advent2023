use color_eyre::eyre::eyre;
use color_eyre::Result;

mod day01;

pub trait Day {
    fn setup(&mut self) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}

pub fn get(day: u8) -> Result<impl Day> {
    let day = match day {
        1 => day01::Day01::default(),
        /*
        2 => Day02,
        3 => Day03,
        4 => Day04,
        5 => Day05,
        6 => Day06,
        7 => Day07,
        8 => Day08,
        9 => Day09,
        10 => Day10,
        11 => Day11,
        12 => Day12,
        13 => Day13,
        14 => Day14,
        15 => Day15,
        16 => Day16,
        17 => Day17,
        18 => Day18,
        19 => Day19,
        20 => Day20,
        21 => Day21,
        22 => Day22,
        23 => Day23,
        24 => Day24,
        25 => Day25,
        */
        _ => return Err(eyre!("Invalid day!")),
    };

    Ok(day)
}
