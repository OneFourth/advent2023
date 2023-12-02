use std::collections::HashMap;

use super::Day;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl From<&str> for Colour {
    fn from(value: &str) -> Self {
        use Colour::*;
        match value {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => panic!("Invalid colour"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Turn(usize, usize, usize);

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let parts: HashMap<_, _> = value
            .split(',')
            .map(|s| {
                let (count, colour) = s.trim().split_once(' ').unwrap();
                let colour: Colour = colour.into();
                (colour, count.parse().unwrap())
            })
            .collect();

        Turn(
            *parts.get(&Colour::Red).unwrap_or(&0),
            *parts.get(&Colour::Green).unwrap_or(&0),
            *parts.get(&Colour::Blue).unwrap_or(&0),
        )
    }
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    sequence: Vec<Turn>,
}

impl Game {
    fn max(&self, colour: Colour) -> usize {
        self.sequence
            .iter()
            .copied()
            .map(|t| {
                let Turn(r, g, b) = t;
                match colour {
                    Colour::Red => r,
                    Colour::Green => g,
                    Colour::Blue => b,
                }
            })
            .max()
            .unwrap_or(0)
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_id, rest) = value.split_once(':').unwrap();
        let id: usize = game_id.trim_start_matches("Game ").parse().unwrap();

        let sequence = rest.trim().split(';').map(|cubes| cubes.into()).collect();

        Self { id, sequence }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Day02 {
    games: Vec<Game>,
}

impl Day for Day02 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(2)?;
        self.games = input.data.lines().map(|g| g.into()).collect();

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .games
            .iter()
            .filter_map(|game| {
                let r = game.max(Colour::Red);
                let g = game.max(Colour::Green);
                let b = game.max(Colour::Blue);

                (r <= 12 && g <= 13 && b <= 14).then_some(game.id)
            })
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .games
            .iter()
            .map(|game| {
                let r = game.max(Colour::Red);
                let g = game.max(Colour::Green);
                let b = game.max(Colour::Blue);

                r * g * b
            })
            .sum::<usize>()
            .to_string())
    }
}
