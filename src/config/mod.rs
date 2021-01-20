use anyhow::Result;
use lighting::Lighting;
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
mod gsettings;
mod lighting;
mod utils;
mod vscode;

#[cfg(test)]
mod tests;

pub use self::alacritty::Alacritty;
pub use self::gsettings::GSettings;
use self::lighting::{Keyboard, Monitor};
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
    pub alacritty: Alacritty,
    pub gsettings: GSettings,
    pub lighting: Lighting,
    pub vscode: VSCode,
}

impl Config {
    pub fn default() -> Config {
        Config {
            timechange: TimeChange::Solar(52.4045, 0.5613),
            alacritty: Alacritty {
                light_theme: Some(String::from("light")),
                dark_theme: Some(String::from("dark")),
            },
            gsettings: GSettings {
                dark_gtk_theme: Some(String::from("Space-Dark")),
                dark_icon_theme: Some(String::from("Space-Dark")),
                dark_cursor_theme: None,
                dark_font_name: None,
                light_gtk_theme: Some(String::from("Space-Light")),
                light_icon_theme: Some(String::from("Space-Light")),
                light_cursor_theme: None,
                light_font_name: None,
            },
            lighting: Lighting {
                monitor: Some(Monitor {
                    device: String::from("amdgpu_bl0"),
                    light_perc: 50,
                    dark_perc: 20,
                }),
                keyboard: Some(Keyboard {
                    device: String::from("asus::kbd_backlight"),
                    light_perc: 0,
                    dark_perc: 34,
                }),
            },
            vscode: VSCode {
                light_theme: Some(String::from("Spacemacs - light")),
                dark_theme: Some(String::from("Spacemacs - dark")),
            },
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
        if self.alacritty.is_some() {
            self.alacritty.light_mode()?;
        }
        if self.gsettings.is_some() {
            self.gsettings.light_mode()?;
        }
        if self.vscode.is_some() {
            self.vscode.light_mode()?;
        }
        self.lighting.light_mode()?;
        Ok(())
    }
    pub fn set_dark_mode(&self) -> Result<()> {
        if self.alacritty.is_some() {
            self.alacritty.dark_mode()?;
        }
        if self.gsettings.is_some() {
            self.gsettings.dark_mode()?;
        }
        if self.vscode.is_some() {
            self.vscode.dark_mode()?;
        }
        self.lighting.dark_mode()?;
        Ok(())
    }
}
