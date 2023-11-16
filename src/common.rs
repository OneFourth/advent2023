use std::io::{Read, Write};

#[derive(Debug)]
pub enum AdventError {
    InvalidDay,
    IO(std::io::Error),
    ReqwestError(reqwest::Error),
    Throttled,
}

impl std::fmt::Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AdventError {}

impl From<std::io::Error> for AdventError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<reqwest::Error> for AdventError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

pub type Result<T> = std::result::Result<T, AdventError>;

#[derive(Debug)]
pub struct Input {
    data: String,
}

fn cache_path(day: u8) -> std::path::PathBuf {
    format!("input/{day:02}").into()
}

impl Input {
    fn get(day: u8) -> Result<Self> {
        let cache = cache_path(day);
        if cache.exists() {
            let data = std::fs::read_to_string(cache)?;
            Ok(Input { data })
        } else {
            let input = Self::get_http(day)?;
            input.save(day)?;

            Ok(input)
        }
    }

    fn save(&self, day: u8) -> Result<()> {
        let path = cache_path(day);
        let root = path.parent().unwrap();
        if !root.exists() {
            std::fs::create_dir_all(root)?;
        }
        std::fs::write(path, &self.data)?;

        Ok(())
    }

    fn get_http(day: u8) -> Result<Input> {
        let session = std::fs::read_to_string("session_token")?;
        let session = session.trim();

        let throttle_path = std::path::Path::new("throttle");

        if let Ok(metadata) = throttle_path.metadata() {
            if let Ok(time_since_last) = metadata.modified()?.elapsed() {
                if time_since_last < std::time::Duration::from_secs(1 * 60) {
                    return Err(AdventError::Throttled);
                }
            }
        }

        std::fs::File::create(throttle_path)?;

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("personal tool via Jacques jqschutte@gmail.com")
            .build()?;

        let url = format!("https://adventofcode.com/2023/day/{day}/input");

        let response = client
            .get(url)
            .header("cookie", format!("session={session}"))
            .send()?;

        let data = response.text()?;

        Ok(Input { data })
    }
}

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
