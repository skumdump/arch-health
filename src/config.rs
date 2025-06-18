use serde::Deserialize;
use std::fs;
use dirs::config_dir;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub checks: Checks,
    pub output: Output,
}

#[derive(Deserialize, Debug)]
pub struct Checks {
    pub library: bool,
    pub pacman: bool,
    pub audit: bool,
}

#[derive(Deserialize, Debug)]
pub struct Output {
    pub format: String,
}

pub fn load_config() -> Option<Config> {
    let mut paths = vec![];

    if let Some(mut p) = config_dir() {
        p.push("arch-health_/config.toml");
        paths.push(p);
    }

    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join(".archhc.toml"));
    }

    for path in paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                return toml::from_str(&content).ok();
            }
        }
    }
    None
}
