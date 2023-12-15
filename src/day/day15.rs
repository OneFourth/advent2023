use color_eyre::eyre::eyre;
use winnow::ascii::{alpha1, digit1};
use winnow::combinator::{alt, separated, separated_pair, terminated};
use winnow::{PResult, Parser};

use super::Day;

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Hash(Vec<u8>);

#[derive(Debug, Default, Clone)]
struct Lens {
    label: String,
    focal: usize,
}

#[derive(Debug)]
enum Operation {
    Equals(Lens),
    Minus(String),
}

fn parse_operation(i: &mut &str) -> PResult<Operation> {
    alt((
        separated_pair(alpha1, '=', digit1.parse_to()).map(|(label, focal): (&str, usize)| {
            Operation::Equals(Lens {
                label: label.to_string(),
                focal,
            })
        }),
        terminated(alpha1, '-').map(|label: &str| Operation::Minus(label.to_string())),
    ))
    .parse_next(i)
}

fn get_hash(values: &[u8]) -> usize {
    values
        .iter()
        .copied()
        .fold(0usize, |acc, v| ((acc + v as usize) * 17) % 256)
}

#[derive(Debug, Default)]
pub(crate) struct Day15 {
    hash: Hash,
    operations: Vec<Operation>,
}

impl Day for Day15 {
    fn setup(&mut self) -> color_eyre::eyre::Result<()> {
        let input = crate::input::Input::get(15)?;
        self.hash = Hash(input.data.trim().as_bytes().to_vec());
        self.operations = separated(1.., parse_operation, ',')
            .parse(input.data.trim())
            .map_err(|e| eyre!(e.to_string()))?;

        Ok(())
    }

    fn part1(&self) -> color_eyre::eyre::Result<String> {
        Ok(self
            .hash
            .0
            .split(|&b| b == b',')
            .map(get_hash)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self) -> color_eyre::eyre::Result<String> {
        let mut map: HashMap<usize, Vec<Lens>> = (0..256).map(|v| (v, Vec::new())).collect();
        let mut reverse_lookup: HashMap<String, usize> = HashMap::new();

        for op in &self.operations {
            match op {
                Operation::Equals(lens) => {
                    let box_index = get_hash(lens.label.as_bytes());
                    let b = map.get_mut(&box_index).unwrap();
                    if let Some(l) = b.iter_mut().find(|v| v.label == lens.label) {
                        l.focal = lens.focal;
                    } else {
                        b.push(lens.clone());
                        reverse_lookup.insert(lens.label.clone(), box_index);
                    }
                }
                Operation::Minus(label) => {
                    if let Some(box_index) = reverse_lookup.remove(label.as_str()) {
                        let b = map.get_mut(&box_index).unwrap();
                        let index = b.iter().position(|v| v.label == *label).unwrap();
                        b.remove(index);
                    }
                }
            }
        }

        Ok(map
            .iter()
            .map(|(k, b)| {
                (k + 1)
                    * b.iter()
                        .enumerate()
                        .map(|(i, l)| (i + 1) * l.focal)
                        .sum::<usize>()
            })
            .sum::<usize>()
            .to_string())
    }
}
