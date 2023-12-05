use color_eyre::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub trait Day {
    fn setup(&mut self) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}

pub fn get(day: u8) -> Box<dyn Day> {
    match day {
        1 => Box::<day01::Day01>::default(),
        2 => Box::<day02::Day02>::default(),
        3 => Box::<day03::Day03>::default(),
        4 => Box::<day04::Day04>::default(),
        5 => Box::<day05::Day05>::default(),
        6 => Box::<day06::Day06>::default(),
        7 => Box::<day07::Day07>::default(),
        8 => Box::<day08::Day08>::default(),
        9 => Box::<day09::Day09>::default(),
        10 => Box::<day10::Day10>::default(),
        11 => Box::<day11::Day11>::default(),
        12 => Box::<day12::Day12>::default(),
        13 => Box::<day13::Day13>::default(),
        14 => Box::<day14::Day14>::default(),
        15 => Box::<day15::Day15>::default(),
        16 => Box::<day16::Day16>::default(),
        17 => Box::<day17::Day17>::default(),
        18 => Box::<day18::Day18>::default(),
        19 => Box::<day19::Day19>::default(),
        20 => Box::<day20::Day20>::default(),
        21 => Box::<day21::Day21>::default(),
        22 => Box::<day22::Day22>::default(),
        23 => Box::<day23::Day23>::default(),
        24 => Box::<day24::Day24>::default(),
        25 => Box::<day25::Day25>::default(),
        _ => panic!("Invalid day"),
    }
}
