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
