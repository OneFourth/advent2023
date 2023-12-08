use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{alt, delimited, preceded, separated, separated_pair};
use winnow::{PResult, Parser};

use super::Day;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn parse_colour(i: &mut &str) -> PResult<Colour> {
    alt((
        "red".value(Colour::Red),
        "green".value(Colour::Green),
        "blue".value(Colour::Blue),
    ))
    .parse_next(i)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Turn(usize, usize, usize);

fn parse_turn(i: &mut &str) -> PResult<Turn> {
    let parts: HashMap<_, usize> = separated(
        1..=3,
        separated_pair(preceded(space1, digit1.parse_to()), space1, parse_colour)
            .map(|(d, c)| (c, d)),
        ',',
    )
    .parse_next(i)?;

    Ok(Turn(
        *parts.get(&Colour::Red).unwrap_or(&0),
        *parts.get(&Colour::Green).unwrap_or(&0),
        *parts.get(&Colour::Blue).unwrap_or(&0),
    ))
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

fn parse_game(i: &mut &str) -> PResult<Game> {
    let id = delimited(("Game", space1), digit1, ':')
        .parse_to()
        .parse_next(i)?;

    let sequence = separated(1.., parse_turn, ';').parse_next(i)?;

    Ok(Game { id, sequence })
}

#[derive(Debug, Default)]
pub(crate) struct Day02 {
    games: Vec<Game>,
}

impl Day for Day02 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(2)?;
        self.games = separated(1.., parse_game, newline)
            .parse(input.data.trim())
            .map_err(|e| eyre!(e.to_string()))?;

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
