use anyhow::Result;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};

use std::{
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
    process,
};

mod alacritty;
mod bat;
mod gsettings;
mod lighting;
mod utils;
mod vscode;

#[cfg(test)]
mod tests;

pub use self::alacritty::Alacritty;
pub use self::bat::Bat;
pub use self::gsettings::GSettings;
pub use self::lighting::Lighting;
pub use self::vscode::VSCode;

const GLOBAL_CONF: &str = "/etc/sway-colord/config.ron";
const APP: &str = "sway-colord";
const APP_FILENAME: &str = "config.ron";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TimeChange {
    Rigid(String, String),
    Solar(f64, f64),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub timechange: TimeChange,
    pub alacritty: Option<Alacritty>,
    pub bat: Option<Bat>,
    pub gsettings: Option<GSettings>,
    pub lighting: Option<Lighting>,
    pub vscode: Option<VSCode>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            timechange: TimeChange::Rigid(String::from("07:00:00"), String::from("19:00:00")),
            alacritty: None,
            bat: None,
            gsettings: None,
            lighting: None,
            vscode: None,
        }
    }
    pub fn is_alacritty(&self) -> Option<Alacritty> {
        if let Some(conf) = &self.alacritty {
            if conf.is_some() {
                Some(conf.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_bat(&self) -> Option<Bat> {
        if let Some(conf) = &self.bat {
            if conf.is_some() {
                Some(conf.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_gsettings(&self) -> Option<GSettings> {
        if let Some(conf) = &self.gsettings {
            if conf.is_some() {
                Some(conf.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_lighting(&self) -> Option<Lighting> {
        if let Some(conf) = &self.lighting {
            if conf.is_some() {
                Some(conf.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn is_vscode(&self) -> Option<VSCode> {
        if let Some(conf) = &self.vscode {
            if conf.is_some() {
                Some(conf.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_data() -> PathBuf {
        let mut dir = dirs_next::home_dir().expect("Error: unable to find home directory");
        dir.push(".config");
        dir.push(APP);
        if !dir.exists() {
            match fs::create_dir_all(&dir) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error: Unable to create directory ~/.config/sway-colord");
                    process::exit(1);
                }
            }
        }
        dir.push(APP_FILENAME);
        if !dir.exists() {
            match OpenOptions::new().create(true).write(true).open(&dir) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error: Unable to write to file ~/.config/sway-colord/config.ron");
                    process::exit(1)
                }
            }
        }
        dir.join(APP_FILENAME)
    }
    pub fn get_tmp() -> PathBuf {
        let mut dir = Path::new("/tmp").to_owned();
        dir.push(APP);
        if !dir.exists() {
            match fs::create_dir_all(&dir) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error: Unable to create directory ~/.config/sway-colord");
                    process::exit(1);
                }
            }
        }
        dir
    }
    #[allow(dead_code)]
    pub fn get_cache_dir() -> PathBuf {
        let mut dir = dirs_next::home_dir().expect("Error: unable to find home directory");
        dir.push(".cache");
        dir.push(APP);
        if !dir.exists() {
            match fs::create_dir_all(&dir) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error: Unable to create directory ~/.cache/sway-colord",);
                    process::exit(1)
                }
            }
        }
        dir
    }
    pub fn load() -> Config {
        if let Ok(file) = File::open(Config::get_data()) {
            match from_reader(file) {
                Ok(data) => return data,
                Err(_) => {
                    if let Ok(global) = File::open(Path::new(GLOBAL_CONF)) {
                        match from_reader(global) {
                            Ok(data) => return data,
                            Err(_) => return Config::default(),
                        }
                    } else {
                        return Config::default();
                    }
                }
            }
        } else {
            return Config::default();
        }
    }
    #[allow(dead_code)]
    pub fn save(&self) {
        let mut file = File::create(Config::get_data()).expect("Failed to create config.ron");
        let pretty = PrettyConfig::new()
            .with_depth_limit(2)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true);
        match to_string_pretty(self, pretty) {
            Ok(string) => {
                file.write_all(string.as_bytes())
                    .expect("Error unable to write to data.ron");
            }
            Err(_) => {
                eprintln!("Serializating data failed");
                process::exit(1)
            }
        }
    }
    pub fn set_light_mode(&self) -> Result<()> {
        if let Some(alacritty) = self.is_alacritty() {
            alacritty.light_mode()?;
        }
        if let Some(bat) = self.is_bat() {
            bat.light_mode()?;
        }
        if let Some(gsettings) = self.is_gsettings() {
            gsettings.light_mode()?;
        }
        if let Some(vscode) = self.is_vscode() {
            vscode.light_mode()?;
        }
        if let Some(lighting) = self.is_lighting() {
            lighting.light_mode()?;
        }

        Ok(())
    }
    pub fn set_dark_mode(&self) -> Result<()> {
        if let Some(alacritty) = self.is_alacritty() {
            alacritty.dark_mode()?;
        }
        if let Some(bat) = self.is_bat() {
            bat.dark_mode()?;
        }
        if let Some(gsettings) = self.is_gsettings() {
            gsettings.dark_mode()?;
        }
        if let Some(vscode) = self.is_vscode() {
            vscode.dark_mode()?;
        }
        if let Some(lighting) = self.is_lighting() {
            lighting.dark_mode()?;
        }

        Ok(())
    }
}
