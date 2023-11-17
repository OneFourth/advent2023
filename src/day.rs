use super::error::{AdventError, Result};
use super::input::Input;

#[derive(Debug)]
pub struct Day {
    day: u8,
    input: Input,
}

impl Day {
    pub fn new(day: u8) -> Result<Self> {
        if (0..25).contains(&day) {
            let input = Input::get(day)?;
            Ok(Self { day, input })
        } else {
            Err(AdventError::InvalidDay)
        }
    }
}
