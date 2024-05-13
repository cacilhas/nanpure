use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
};

use crate::themes::Theme;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub theme: Option<Theme>,
}

pub fn load() -> Config {
    match _load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("[loading {}] {:?}", SETTINGS.to_string(), e);
            Config::default()
        }
    }
}

pub fn save(config: Config) {
    let file = SETTINGS.to_string();
    if let Err(e) = _save(config, &file) {
        eprintln!("[saving {}] {:?}", &file, e);
    } else {
        println!("[saving {}] done", &file);
    }
}

pub fn _save(config: Config, filename: impl Into<String>) -> eyre::Result<()> {
    let filename = filename.into();
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&filename)
    {
        Ok(file) => file,
        _ => File::create(&filename)?,
    };
    let content = toml::to_string(&config)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn _load() -> eyre::Result<Config> {
    let content = fs::read_to_string(SETTINGS.to_string())?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

#[dynamic]
static SETTINGS: String = format!(
    "{}/nanpure.toml",
    env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", env!["HOME"]))
);
