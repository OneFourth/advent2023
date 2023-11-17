use super::{
    error::{AdventError, Result},
    log,
};

#[derive(Debug)]
pub struct Input {
    data: String,
}

fn cache_path(day: u8) -> std::path::PathBuf {
    format!("input/{day:02}").into()
}

impl Input {
    pub(crate) fn get(day: u8) -> Result<Self> {
        let cache = cache_path(day);
        if cache.exists() {
            log::info(&format!("Using cached input {cache:?}"));
            let data = std::fs::read_to_string(cache)?;
            Ok(Input { data })
        } else {
            Ok(Self::get_http(day)?)
        }
    }

    fn save(&self, day: u8) -> Result<()> {
        let path = cache_path(day);
        let root = path.parent().unwrap();
        if !root.exists() {
            std::fs::create_dir_all(root)?;
        }
        std::fs::write(&path, &self.data)?;

        log::info(&format!("Cached input to {path:?}"));

        Ok(())
    }

    fn get_http(day: u8) -> Result<Input> {
        log::info(&format!("Getting input via http request..."));

        let session = std::fs::read_to_string("session_token")?;
        let session = session.trim();

        let throttle_path = std::path::Path::new("throttle");

        if let Ok(metadata) = throttle_path.metadata() {
            if let Ok(time_since_last) = metadata.modified()?.elapsed() {
                if time_since_last < std::time::Duration::from_secs(15 * 60) {
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
        let input = Input { data };
        input.save(day)?;

        Ok(input)
    }
}
