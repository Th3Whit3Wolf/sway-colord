use anyhow::{anyhow, Result};
use lighting::Lighting;
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub alacritty: Alacritty,
    pub gsettings: GSettings,
    pub lighting: Lighting,
    pub vscode: VSCode,
}

impl Config {
    pub fn read_config(path: &str) -> Config {
        match from_reader(File::open(path).expect("Failed opening config.ron")) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        }
    }
    pub fn get() -> Result<Config> {
        let local_conf: Option<PathBuf> =
            xdg::BaseDirectories::with_prefix("sway-colord")?.find_config_file("config.ron");
        let conf = if local_conf.is_some() {
            dbg!("Local config used");
            Config::read_config(
                local_conf
                    .ok_or(anyhow!("Unable to read ~/.config/sway-colord/config.ron"))?
                    .to_str()
                    .ok_or(anyhow!("Unable to convert PathBuf to &str"))?,
            )
        } else if Path::new(GLOBAL_CONF).is_file() {
            dbg!("Global config used");
            Config::read_config(GLOBAL_CONF)
        } else {
            dbg!("Default config used");
            Config {
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
                    monitor: Some(Monitor{
                        device: String::from("amdgpu_bl0"),
                        light_perc: 50,
                        dark_perc: 20
                    }
                    ),
                    keyboard: Some(Keyboard{
                            device: String::from("asus::kbd_backlight"),
                            light_perc: 0,
                            dark_perc: 34,
                    }
                    )

                },
                vscode: VSCode {
                    light_theme: Some(String::from("Spacemacs - light")),
                    dark_theme: Some(String::from("Spacemacs - dark")),
                },
            }
        };
        Ok(conf)
    }
    pub async fn set_light_mode(&self) -> Result<()> {
        if self.alacritty.is_some() {
            self.alacritty.light_mode()?;
        }
        self.gsettings.light_mode().await?;
        if self.vscode.is_some() {
            self.vscode.light_mode()?;
        }

        Ok(())
    }
    pub async fn set_dark_mode(&self) -> Result<()> {
        if self.alacritty.is_some() {
            self.alacritty.dark_mode()?;
        }
        self.gsettings.dark_mode().await?;
        if self.vscode.is_some() {
            self.vscode.dark_mode()?;
        }
        Ok(())
    }
}
